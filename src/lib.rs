use std::fmt::Display;

/// Representation of a tree with name, latitude and longitude
#[derive(Debug)]
pub struct Tree {
    name: String,
    description: String,
    _latitude: f64,
    _longitude: f64,
}

impl Display for Tree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            format!(
                "Tree (name={}, description={})",
                &self.name, &self.description
            )
            .as_str(),
        )
    }
}

impl Tree {
    pub fn new(name: &str, desc: &str) -> Tree {
        Tree {
            name: String::from(name),
            description: String::from(desc),
            _latitude: 0.0,
            _longitude: 0.0,
        }
    }
}

// ------------------ TESTS ------------------

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn creation_works() {
        let t = Tree::new("name", "description");

        assert_eq!("name", t.name);
        assert_eq!("description", t.description);
    }

    #[test]
    fn display_string() {
        let t = Tree::new("name", "description");

        let s = format!("{t}");

        assert!(s.contains("Tree"));
        assert!(s.contains("name"));
        assert!(s.contains("description"));
    }
}
