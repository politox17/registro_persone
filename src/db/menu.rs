use std::io;
use std::fmt;
use crate::db::registro::Registro;
use regex::Regex; 

#[derive(Debug)]
pub enum Errori {
    InputNonValido,
    StringaNonValida,
    ValoreIntNonValido,
}

impl fmt::Display for Errori {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Errori::InputNonValido => write!(f, "Input non valido!"),
            Errori::StringaNonValida => write!(f, "Inserire una stringa!"),
            Errori::ValoreIntNonValido => write!(f, "Devi inserire un numero intero!!"),
        }
    }
}

pub fn gestione_registro(registro: &mut Registro) -> Result<(), Errori> {
    println!("\t\t--- Stiamo organizzando un torneo di calcio! Vuoi unirti? Segui le nostre indicazioni! ----");
    println!("=================================================================================================");
    println!("Puoi unirti a qualsiasi età,ma leggi attentamente tutto!!\n\n");
    
    let nome = get_nome()?;
    let cognome = get_cognome()?;
    
    
    registro.aggiungi_iscritti(nome, cognome); // Aggiunta del nome....
    
    Ok(()) // Aggiunto return value
}

pub fn leggi_input_int() -> Result<i32, Errori> {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)
        .map_err(|_| Errori::ValoreIntNonValido)?;

    let cleaned = input.trim();
    match cleaned.parse::<i32>() {
        Ok(num) => Ok(num),
        Err(_) => Err(Errori::ValoreIntNonValido),
    }
}

pub fn leggi_input_string() -> Result<String, Errori> { // Cambiato il tipo di ritorno
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .map_err(|_| Errori::InputNonValido)?;

    let cleaned = input.trim().to_string();

    if cleaned.is_empty() {
        return Err(Errori::InputNonValido);
    }

    let re = Regex::new(r"^[a-zA-ZàèéìòùÀÈÉÌÒÙ\s]+$").unwrap();

    if re.is_match(&cleaned) {
        Ok(cleaned)
    } else {
        Err(Errori::InputNonValido)
    }
}

fn get_nome() -> Result<String, Errori> {
    println!("\t[1] Inserisci il tuo nome: ");
    leggi_input_string() // Ritorno diretto del risultato
}

fn get_cognome() -> Result<String, Errori> {
    println!("\t[2] Inserisci il tuo cognome: ");
    leggi_input_string() // Ritorno diretto del risultato
}