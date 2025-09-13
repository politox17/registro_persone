use std::collections::HashMap;
use rand::Rng;
use std::fmt;
use std::fs::File;
use std::fs::read_to_string;
use std::io::{Write, Read};




#[derive(Debug)]
pub struct Registro{
   pub iscritti: HashMap<i32, Iscritto>
}

#[derive(Debug)]
pub struct Iscritto {
    nome: String,
    cognome: String,
}

impl fmt::Display for Iscritto {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.nome, self.cognome)
    }
}

impl Registro {
    pub fn new() -> Self {
        Registro {
                iscritti: HashMap::new(), // Nuova istanza
        }
    }
    pub fn aggiungi_iscritti(&mut self, nome: String, cognome: String) -> std::io::Result<()> {
    let id = Self::creazione_id();
    let iscritto = Iscritto { nome, cognome };
    self.iscritti.insert(id, iscritto);
    
    // Apri il file in modalità append invece di create (per non sovrascrivere)
    let mut file = File::options()
        .append(true)
        .create(true)
        .open("registro.txt")?;
    
    // Formatta la stringa correttamente
    writeln!(file, "{}: {}", id, self.iscritti.get(&id).unwrap())?;
    
    println!("----------------------------------");
    println!("Operazione di aggiunta completata con successo! ID (da non dimenticare): {id}");
    Ok(())
}
    pub fn creazione_id() -> i32 {
        let mut random = rand::rng(); // Creazione di un id compreso tra 100 e 999
        let id = random.random_range(100..999);
        id
    }
    pub fn mostra_iscritti(&self) {
        println!("\t\tEcco qua gli iscritti: \n\n");
        
       let mut file = std::fs::File::open("registro.txt").unwrap();
       let mut contenuto = String::new();
       file.read_to_string(&mut contenuto).unwrap();
       println!("{}", contenuto);
    }
    pub fn cerca_per_nome(&self, input: &str) {
          let mut  trovato = false;
          for (id,iscritto) in &self.iscritti {
            // per controllare se 2 stringhe sono case sensitive (eq_ignore_ascii_case)
            if iscritto.nome.eq_ignore_ascii_case(input) {
            println!("Eccolo qua: \n");
            println!("- {id}: {iscritto}");
            trovato = true;
            }
            if !trovato{
                println!("Nessun iscritto trovato con nome: {input}");
            }
          }
       }
    pub fn cerca_per_cognome(&self, input: &str) {
        let mut trovato = false;
        for (id, iscritto) in &self.iscritti
        {
            if iscritto.cognome.eq_ignore_ascii_case(input) {
                println!("Eccolo qua: \n");
                println!("- {id}: {iscritto}");
                trovato = true;
            }
            if !trovato {
                println!("Nessun iscritto trovato con cognome: {input}");
            }
        }
    }
    pub fn aggiorna(&mut self, id: i32, nuovo_nome: &str, nuovo_cognome: &str) -> bool {
    if let Some(iscritto) = self.iscritti.get_mut(&id) {
        iscritto.nome = nuovo_nome.to_string();
        iscritto.cognome = nuovo_cognome.to_string();
        true // Aggiornamento riuscito
    } else {
        false // ID non trovato
    }
}
    pub fn elimina(&mut self, id: &i32) -> Option<Iscritto> 
    {
        self.iscritti.remove(&id)
    }
    fn read_line(filename: &str) -> Vec<String>
    {
        let mut result = Vec::new();

        for line in read_to_string(filename).unwrap().line() {
            result.push(line.to_string());
        }
        result
    }
}
