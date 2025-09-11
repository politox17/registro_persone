use std::io;
use std::fmt;
use crate::db::registro::Registro; // Import del metodo registro
 use regex::Regex; 

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

pub fn gestione_registro()-> Result<(), Errori>
{
    println!("\t\t--- Stiamo organizzando un torneo di calcio! Vuoi unirti? Segui le nostre indicazioni ----");
    println!("=================================================================================================");
    println!("Puoi unirti a qualsiasi età, ,ma leggi attentamente tutto!!");
    
}





pub fn leggi_input_int() -> Result<i32, Errori> {
   let mut input = String::new();
   std::io::stdin().read_line(&mut input)
            .map_err(|_| Errori::ValoreIntNonValido)?;

    let cleaned = input.trim(); // Rimuove spazi e newline
    // Lo parsiamo a i32
    match cleaned.parse::<i32> {
        Ok(num) => Ok(num),
        Err(_) => (Errori::ValoreIntNonValido),
    }
}

pub fn leggi_input_string() -> Result<String, Errori> {
     let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .map_err(|_| Errori::InputNonValido)?;

    let cleaned = input.trim().to_string();

    if cleaned.is_empty() {
        return Err(Errori::InputNonValido);
    }

    // Regex: solo lettere (anche con accenti e spazi tra nome e cognome)
    let re = Regex::new(r"^[a-zA-ZàèéìòùÀÈÉÌÒÙ\s]+$").unwrap();

    if re.is_match(&cleaned) {
        Ok(cleaned)
    } else {
        Err(Errori::InputNonValido)
    }
}

fn get_nome() -> Result<String, Errori> {
    println!("\t[1] Inserisci il tuo nome: ");
    leggi_input_string();
}

fn get_cognome() -> Result<String, Errori>
{
    println!("[2] Inserisci il tuo cognome");
    leggi_input_string();
}
