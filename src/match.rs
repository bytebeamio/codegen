use crate::Formatter;
use crate::body::Body;
use crate::Block;

use std::fmt;
use std::collections::HashMap;
use std::fmt::Write;

/// Defines a struct.
#[derive(Debug, Clone)]
pub struct Match {
    /// Match expression
    pub name: String,
    /// Match branches
    pub branches: HashMap<String, Body> ,
}

impl Match {
    /// Return a structure definition with the provided name
    pub fn new(name: &str) -> Self {
        Match {
            name: name.to_owned(),
            branches: HashMap::new()
        }
    }

    /// Add a new match branch
    pub fn add_branch(&mut self, name: &str, block: Block) {
        self.branches.insert(name.to_owned(), Body::Block(block));
    }

    /// Formats the struct using the given formatter.
    pub fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        write!(fmt, "match {}", self.name)?;

        fmt.block(|fmt| {
            for (branch, body) in self.branches.iter() {
                write!(fmt, "{} =>", branch)?;
                body.fmt(fmt)?;
            }

            Ok(())
        })?;

        Ok(())
    }
}

