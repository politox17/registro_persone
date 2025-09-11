use std::io;
use std::fmt;
use crate::db::registro::Registro; // Import del metodo registro

#[derive(Debug)]
enum Errori {
    InputNonValido,
    StringaNonValida,
    ValoreIntNonValido,
}

// per gestione Errori più robusta
impl fmt::Display for Errori {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        match self {
        Errori::InputNonValido => write!(f, "Input non valido!"),
        Errori::StringaNonValida => write!(f, "Inserire una stringa!"),
        Errori::ValoreIntNonValido => write!(f,"Devi inserire un numero intero!!"),
        }
    }
}

pub fn leggi_input_int() -> Result<i32, Errori> {
   let mut input = String::new();
   io::stdin().read_line(&mut input)
            .map_err(|_| Errori::ValoreIntNonValido)?;

    let cleaned = input.trim(); // Rimuove spazi e newline
    // Lo parsiamo a i32
    match cleaned.parse::<i32> {
        Ok(num) => Ok(num),
        Err(_) => (Errori::ValoreIntNonValido),
    }
}