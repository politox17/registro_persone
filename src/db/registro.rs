use std::collections::HashMap;
use rand::Rng;
use std::fmt;

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
        write!(f, "-- {}: {}", self.nome, self.cognome)
    }
}

impl Registro {
    pub fn new() -> Self {
        Registro {
                iscritti: HashMap::new(), // Nuova istanza
        }
    }
    pub fn aggiungi_iscritti(&mut self, nome: String, cognome: String)  {
           let id = Self::creazione_id();
           let iscritto = Iscritto { nome, cognome };
           self.iscritti.insert(id, iscritto);
           println!("----------------------------------");
           println!("Operazione di aggiunta completata con successo! ID (da non dimenticare): {id}");
}
    pub fn creazione_id() -> i32 {
        let mut random = rand::rng(); // Creazione di un id compreso tra 100 e 999
        let id = random.random_range(100..999);
        id
    }
    pub fn mostra_iscritti(&self) {
        println!("\t\tEcco qua gli iscritti: \n\n");
        
       for (id,iscritto) in &self.iscritti
       {
          println!(" {}: {}",  id, iscritto); // Per mostrare gli iscritti effettivi nel registro
          
       }
    }
    pub fn cerca_per_nome(&self, input: &str) {
          let mut  trovato = false;
          for (id,iscritto) in &self.iscritti {
            // per controllare se 2 stringhe sono case sensitive (eq_ignore_ascii_case)
            if iscritto.nome.eq_ignore_ascii_case(input) {
            println!("Eccolo qua: \n");
            println!("- {id}: {nome}");
            trovato = true;
            }
            if !trovato{
                println!("Nessun iscritto trovato con nome: {nome}");
            }
          }
       }
    pub fn cerca_per_cognome(&self, input: &str) {
        let mut trovato = false;
        for (id, iscritto) in &self.iscritti
        {
            if iscritto.cognome.eq_ignore_ascii_case(input) {
                println!("Eccolo qua: \n");
                println!("- {id}: {cognome}");
                trovato = true;
            }
            if !trovato {
                println!("Nessun iscritto trovato con cognome: {cognome}");
            }
        }
    }
}
