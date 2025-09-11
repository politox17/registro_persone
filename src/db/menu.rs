use std::io;
use std::fmt;
use crate::db::registro::Registro; // Import del metodo registro

#[derive(Debug)]
enum Errori {
    InputNonValido,
    StringaNonValida,
}

impl fmt::Display for Errori {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        match self {
        Errori::InputNonValido => write!(f, "Input non valido!"),
        Errori::StringaNonValida => write!(f, "Inserire una stringa!"),
        }
    }
}

pub fn leggi_input_int() -> Result<i32, Errori> {
   
}