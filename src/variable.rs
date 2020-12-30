use std::fmt::{self, Write};

use crate::formatter::Formatter;

use crate::fields::Fields;
use crate::Field;

/// Value type to initialize a variable
#[derive(Debug, Clone)]
pub enum Value {
    /// Empty block
    Block,
    /// Primitive type
    Primitive(String),
    /// Structure type
    Struct {
        /// Structure fields
        fields: Fields,
    },
}

/// Defines a struct.
#[derive(Debug, Clone)]
pub struct Variable {
    name: String,
    typ: String,
    value: Value,
}

impl Variable {
    /// Return a structure definition with the provided name
    pub fn new(name: &str, typ: &str) -> Self {
        Variable {
            name: name.to_owned(),
            typ: typ.to_owned(),
            value: Value::Block,
        }
    }

    /// Set value to the variable
    pub fn set_value(&mut self, value: &str) -> &mut Self {
        self.value = Value::Primitive(value.to_owned());
        self
    }

    /// Add field values to structure
    pub fn push_field(&mut self, field: Field) -> &mut Self {
        loop {
            match &mut self.value {
                Value::Struct { fields, .. } => {
                    fields.push_named(field);
                    break;
                }
                _ => {
                    self.value = Value::Struct {
                        fields: Fields::Empty,
                    };
                    continue;
                }
            }
        }

        self
    }

    /// Formats the struct using the given formatter.
    pub fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        match &self.value {
            Value::Block => {
                write!(fmt, "let {}: {} = {{}}", self.name, self.typ)?;
            }
            Value::Primitive(v) => {
                write!(fmt, "let {}: {} = {}", self.name, self.typ, v)?;
            }
            Value::Struct { fields } => {
                write!(fmt, "let {}: {} = {}", self.name, self.typ, self.typ)?;
                fields.fmt(fmt)?;
            }
        }

        write!(fmt, ";\n")?;
        Ok(())
    }
}
