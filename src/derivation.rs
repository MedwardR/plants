use crate::angle::Angle;
use crate::interpreter::Interpreter;
use crate::turtle::Turtle;
use macroquad::color::Color;
use rand::rngs::StdRng;
use rand::{Rng, RngExt, SeedableRng};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct LSystem {
    pub name: String,
    pub desc: String,
    pub state: StateConfig,
    pub rules: RulesConfig,
    pub view: ViewConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct StateConfig {
    pub axiom: String,
    pub n: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct RulesConfig {
    pub productions: Vec<Production>,
    pub homomorphism: Vec<Production>,
}

#[derive(Debug, Clone)]
pub struct Production {
    pub predecessor: String,
    pub successor: String,
    pub probability: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ViewConfig {
    pub angle: Angle,
    pub delta: Angle,

    #[serde(with = "color_serde")]
    pub colors: Vec<Color>,

    pub thickness: f32,
}

impl Default for LSystem {
    fn default() -> Self {
        Self {
            name: String::new(),
            desc: String::new(),
            state: StateConfig::default(),
            rules: RulesConfig::default(),
            view: ViewConfig::default(),
        }
    }
}

impl Default for StateConfig {
    fn default() -> Self {
        Self {
            axiom: String::new(),
            n: 1,
        }
    }
}

impl Default for RulesConfig {
    fn default() -> Self {
        Self {
            productions: Vec::new(),
            homomorphism: Vec::new(),
        }
    }
}

impl Default for ViewConfig {
    fn default() -> Self {
        Self {
            angle: Angle::ZERO,
            delta: Angle::from_degrees(35.0),
            colors: vec![macroquad::color::WHITE],
            thickness: 4.0,
        }
    }
}

impl Production {
    pub fn new(predecessor: impl Into<String>, successor: impl Into<String>) -> Self {
        Self {
            predecessor: predecessor.into(),
            successor: successor.into(),
            probability: None,
        }
    }

    pub fn stochastic(
        predecessor: impl Into<String>,
        probability: f64,
        successor: impl Into<String>,
    ) -> Self {
        Self {
            predecessor: predecessor.into(),
            successor: successor.into(),
            probability: Some(probability),
        }
    }

    fn parse(raw: &str) -> Result<Self, String> {
        let (lhs, rhs) = raw
            .split_once("-->")
            .ok_or_else(|| format!("invalid production: expected '-->': {raw}"))?;

        let lhs = lhs.trim();
        let successor = rhs.trim().to_string();

        let (predecessor, probability) =
            if let Some((predecessor, probability)) = lhs.rsplit_once("--") {
                let probability = probability
                    .trim()
                    .parse::<f64>()
                    .map_err(|_| format!("invalid probability in production: {raw}"))?;

                (predecessor.trim(), Some(probability))
            } else if let Some((probability, predecessor)) = lhs.split_once(':') {
                match probability.trim().parse::<f64>() {
                    Ok(probability) => (predecessor.trim(), Some(probability)),
                    Err(_) => (lhs, None),
                }
            } else {
                (lhs, None)
            };

        if predecessor.is_empty() {
            return Err(format!("production has an empty predecessor: {raw}"));
        }

        Ok(Self {
            predecessor: predecessor.to_string(),
            successor,
            probability,
        })
    }

    fn to_string(&self) -> String {
        match self.probability {
            Some(probability) => {
                format!(
                    "{} -- {} --> {}",
                    self.predecessor, probability, self.successor
                )
            }
            None => {
                format!("{} --> {}", self.predecessor, self.successor)
            }
        }
    }
}

impl Serialize for Production {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for Production {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw = String::deserialize(deserializer)?;
        Self::parse(&raw).map_err(serde::de::Error::custom)
    }
}

impl LSystem {
    pub fn new(axiom: impl Into<String>) -> Self {
        Self {
            state: StateConfig {
                axiom: axiom.into(),
                ..Default::default()
            },
            ..Default::default()
        }
    }

    pub fn from_toml(toml: impl AsRef<str>) -> Result<Self, toml::de::Error> {
        toml::from_str(toml.as_ref())
    }

    pub fn to_toml(&self) -> Result<String, toml::ser::Error> {
        toml::to_string_pretty(self)
    }

    pub fn derive(&self, seed: u64) -> Result<String, LSystemError> {
        let mut rng = StdRng::seed_from_u64(seed);

        let productions = index_productions(&self.rules.productions)?;
        validate_stochastic_rules(&productions)?;

        let homomorphism = index_productions(&self.rules.homomorphism)?;
        validate_homomorphism(&homomorphism)?;

        let mut source = self.state.axiom.clone();

        for _ in 0..self.state.n {
            source = derive_once(&source, &productions, &mut rng);
        }
        source = derive_once(&source, &homomorphism, &mut rng);

        Ok(source)
    }

    pub fn turtle(&self) -> Turtle {
        let mut turtle = Turtle::default();

        turtle.state.rotation = self.view.angle;
        turtle.state.color = self
            .view
            .colors
            .first()
            .copied()
            .unwrap_or(macroquad::color::WHITE);

        turtle
    }

    pub fn interpreter(&self) -> Interpreter {
        Interpreter::new(&self.view)
    }
}

fn index_productions(
    productions: &[Production],
) -> Result<HashMap<char, Vec<&Production>>, LSystemError> {
    let mut indexed: HashMap<char, Vec<&Production>> = HashMap::new();

    for production in productions {
        let mut chars = production.predecessor.chars();

        let predecessor = chars
            .next()
            .ok_or_else(|| LSystemError("production predecessor cannot be empty".into()))?;

        if chars.next().is_some() {
            return Err(LSystemError(format!(
                "this interpreter currently supports single-character predecessors: {}",
                production.predecessor
            )));
        }

        indexed.entry(predecessor).or_default().push(production);
    }

    Ok(indexed)
}

fn validate_stochastic_rules(
    productions: &HashMap<char, Vec<&Production>>,
) -> Result<(), LSystemError> {
    for (predecessor, alternatives) in productions {
        // A single unweighted production is deterministic.
        if alternatives.len() == 1 && alternatives[0].probability.is_none() {
            continue;
        }

        let mut total = 0.0;

        for production in alternatives {
            let probability = production.probability.ok_or_else(|| {
                LSystemError(format!(
                    "all alternatives for '{}' must specify probabilities",
                    predecessor
                ))
            })?;

            if !probability.is_finite() || probability <= 0.0 || probability > 1.0 {
                return Err(LSystemError(format!(
                    "invalid probability {} for '{}'; expected a value in (0, 1]",
                    probability, predecessor
                )));
            }

            total += probability;
        }

        if (total - 1.0).abs() > 1e-6 {
            return Err(LSystemError(format!(
                "probabilities for '{}' sum to {}, expected 1.0",
                predecessor, total
            )));
        }
    }

    Ok(())
}

fn validate_homomorphism(
    homomorphism: &HashMap<char, Vec<&Production>>,
) -> Result<(), LSystemError> {
    for (predecessor, alternatives) in homomorphism {
        if alternatives.len() != 1 {
            return Err(LSystemError(format!(
                "homomorphism must have exactly one rule for '{}'",
                predecessor
            )));
        }

        if alternatives[0].probability.is_some() {
            return Err(LSystemError(format!(
                "homomorphism rules cannot have probabilities: '{}'",
                predecessor
            )));
        }
    }

    Ok(())
}

fn derive_once<R: Rng + ?Sized>(
    source: &str,
    productions: &HashMap<char, Vec<&Production>>,
    rng: &mut R,
) -> String {
    let mut result = String::new();

    for ch in source.chars() {
        if let Some(alternatives) = productions.get(&ch) {
            let selected = select_production(alternatives, rng);
            result.push_str(&selected.successor);
        } else {
            result.push(ch);
        }
    }

    result
}

fn select_production<'a, R: Rng + ?Sized>(
    alternatives: &[&'a Production],
    rng: &mut R,
) -> &'a Production {
    if alternatives.len() == 1 && alternatives[0].probability.is_none() {
        return alternatives[0];
    }

    let sample: f64 = rng.random_range(0.0..1.0);
    let mut cumulative = 0.0;

    for production in alternatives {
        cumulative += production.probability.unwrap_or(0.0);

        if sample < cumulative {
            return production;
        }
    }

    alternatives
        .last()
        .expect("production alternatives cannot be empty")
}

#[derive(Debug, Clone)]
pub struct LSystemError(pub String);

impl fmt::Display for LSystemError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for LSystemError {}

mod color_serde {
    use macroquad::color::Color;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<S>(colors: &Vec<Color>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let hex_colors: Vec<String> = colors
            .iter()
            .map(|c| {
                let channel = |v: f32| (v.clamp(0.0, 1.0) * 255.0).round() as u8;

                format!(
                    "#{:02X}{:02X}{:02X}{:02X}",
                    channel(c.r),
                    channel(c.g),
                    channel(c.b),
                    channel(c.a),
                )
            })
            .collect();

        hex_colors.serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<Color>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let hex_colors = Vec::<String>::deserialize(deserializer)?;

        hex_colors
            .iter()
            .map(|hex| {
                let hex = hex.strip_prefix('#').unwrap_or(hex);

                let value = u32::from_str_radix(hex, 16).map_err(serde::de::Error::custom)?;

                match hex.len() {
                    6 => Ok(Color::from_rgba(
                        (value >> 16) as u8,
                        (value >> 8) as u8,
                        value as u8,
                        255,
                    )),
                    8 => Ok(Color::from_rgba(
                        (value >> 24) as u8,
                        (value >> 16) as u8,
                        (value >> 8) as u8,
                        value as u8,
                    )),
                    _ => Err(serde::de::Error::custom("expected #RRGGBB or #RRGGBBAA")),
                }
            })
            .collect()
    }
}
