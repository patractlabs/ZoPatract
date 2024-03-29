use std::fmt;
use zopatract_field::Field;
use zopatract_pest_ast::File;

#[derive(Debug)]
pub enum Error {
    Curve(String, String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Curve(expected, found) => write!(
                f,
                "When processing macros: curve `{}` is incompatible with curve `{}`",
                found, expected
            ),
        }
    }
}

pub fn process_macros<'ast, T: Field>(file: File<'ast>) -> Result<File<'ast>, Error> {
    match &file.pragma {
        Some(pragma) => {
            if T::name() != pragma.curve.name {
                Err(Error::Curve(
                    T::name().to_string(),
                    pragma.curve.name.clone(),
                ))
            } else {
                Ok(file)
            }
        }
        None => Ok(file),
    }
}
