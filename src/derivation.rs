use std::collections::HashMap;

pub struct LSystem {
    pub rules: HashMap<char, String>
}

impl LSystem {
    pub fn new() -> Self {
        Self {
            rules: HashMap::new(),
        }
    }

    pub fn add_rule(&mut self, c: char, s: impl Into<String>) {
        self.rules.insert(c, s.into());
    }

    pub fn derive(&self, axiom: impl Into<String>, iterations: i32) -> String {
        let mut source = axiom.into();

        for _ in 0..iterations {
            let mut result = String::new();

            for ch in source.chars() {
                if let Some(replacement) = self.rules.get(&ch) {
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
}
