use std::fmt::Display;

#[derive(Debug)]
pub struct Tree {
    name: String,
}

impl Display for Tree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("Tree {}", &self.name).as_str())
    }
}

impl Tree {
    pub fn new(name: &str) -> Tree {
        Tree {
            name: String::from(name),
        }
    }
}
