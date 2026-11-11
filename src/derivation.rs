use crate::angle::Angle;
use crate::interpreter::Interpreter;
use crate::turtle::Turtle;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct LSystem {
    pub axiom: String,
    pub n: usize,
    pub rules: HashMap<String, String>,
    pub view: ViewConfig,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ViewConfig {
    pub angle: Angle,
    pub delta: Angle,
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
            delta: Angle::from_degrees(30.0),
        }
    }
}
