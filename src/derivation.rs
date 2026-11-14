use crate::angle::Angle;
use crate::interpreter::Interpreter;
use crate::turtle::Turtle;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use macroquad::color::Color;

#[derive(Debug, Serialize, Deserialize)]
pub struct LSystem {
    pub axiom: String,
    pub n: usize,
    pub rules: HashMap<String, String>,
    pub view: ViewConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ViewConfig {
    pub angle: Angle,
    pub delta: Angle,
    #[serde(with = "color_serde")]
    pub colors: Vec<Color>,
    pub thickness: f32,
}

impl LSystem {
    pub fn new(axiom: impl Into<String>) -> Self {
        Self {
            axiom: axiom.into(),
            n: 0,
            rules: HashMap::new(),
            view: ViewConfig::default(),
        }
    }

    pub fn from_toml(toml: impl Into<String>) -> Result<Self, toml::de::Error> {
        toml::from_str(toml.into().as_str())
    }

    pub fn add_rule(&mut self, c: char, s: impl Into<String>) {
        self.rules.insert(c.to_string(), s.into());
    }

    pub fn derive(&self) -> String {
        let mut source = self.axiom.clone();

        for _ in 0..self.n {
            let mut result = String::new();

            for ch in source.chars() {
                if let Some(replacement) = self.rules.get(&ch.to_string()) {
                    result += replacement;
                }
                else {
                    result.push(ch);
                }
            }

            source = result;
        }

        source
    }

    pub fn to_toml(&self) -> Result<String, toml::ser::Error> {
        toml::to_string_pretty(&self)
    }

    pub fn turtle(&self) -> Turtle {
        let mut turtle = Turtle::default();
        turtle.state.rotation = self.view.angle;
        turtle.state.color = self.view.colors[0];
        turtle
    }
    
    pub fn interpreter(&self) -> Interpreter {
        Interpreter::new(&self.view)
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

mod color_serde {
    use macroquad::color::Color;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<S>(
        colors: &Vec<Color>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
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

    pub fn deserialize<'de, D>(
        deserializer: D,
    ) -> Result<Vec<Color>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let hex_colors = Vec::<String>::deserialize(deserializer)?;

        hex_colors
            .iter()
            .map(|hex| {
                let hex = hex.strip_prefix('#').unwrap_or(hex);

                let value = u32::from_str_radix(hex, 16)
                    .map_err(serde::de::Error::custom)?;

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
                    _ => Err(serde::de::Error::custom(
                        "expected #RRGGBB or #RRGGBBAA",
                    )),
                }
            })
            .collect()
    }
}
