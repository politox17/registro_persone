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
          
       let mut trovati = Vec::new();
        let mut trov_in_mem = false;

        for (id, iscritto) in &self.iscritti {
            if iscritto.nome.eq_ignore_ascii_case(input) { // Correzione: nome invece di cognome
                println!("Trovato: {} -> {}", id, iscritto);
                trovati.push(*id); // Correzione: solo ID
                trov_in_mem = true;
            }
        }

        // Ricerca nel file
        if let Ok(contenuto) = read_to_string("registro.txt") {
            for line in contenuto.lines() {
                if let Some((id_str, nome_cognome)) = line.split_once(": ") { 
                    if let Ok(id) = id_str.parse::<i32>() { 
                        if let Some(nome) = nome_cognome.split_whitespace().nth(0) {
                            if nome.eq_ignore_ascii_case(input) { // Correzione: confronta con input, non con contenuto
                                println!("Trovato nel file: {}", line);
                                trovati.push(id);
                            }
                        }
                    }
                }
            }
        }
        
        if trovati.is_empty() {
            println!("Nessun iscritto trovato con nome: {}", input);
        }
    }
    pub fn cerca_per_cognome(&self, input: &str) {
    let mut trovati = Vec::new();
    let mut trovato_in_memoria = false;
    
    // 1. Cerca nella struct (memoria)
    for (id, iscritto) in &self.iscritti {
        if iscritto.cognome.eq_ignore_ascii_case(input) {
            println!("Trovato in memoria: {} -> {}", id, iscritto);
            trovati.push(*id);
            trovato_in_memoria = true;
        }
    }
    
    // 2. Cerca nel file (per dati che potrebbero non essere in memoria)
    if let Ok(contenuto) = read_to_string("registro.txt") {
        for line in contenuto.lines() {
            if let Some((id_str, nome_cognome)) = line.split_once(": ") {
                if let Ok(id) = id_str.parse::<i32>() {
                    // Estrai il cognome dalla stringa "nome cognome"
                    if let Some(cognome) = nome_cognome.split_whitespace().nth(1) {
                        if cognome.eq_ignore_ascii_case(input) {
                            println!("Trovato nel file: {}", line);
                            trovati.push(id);
                        }
                    }
                }
            }
        }
    }
    
    // 3. Gestisci il caso "non trovato"
    if trovati.is_empty() {
        println!("Nessun iscritto trovato con cognome: {}", input);
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
    elimina(&id);
}
    pub fn elimina(&mut self, id: &i32) -> Option<Iscritto> 
    {
        self.iscritti.remove(&id)
    }
    fn read_lines(filename: &str) -> std::io::Result<Vec<String>> {
    let content = std::fs::read_to_string(filename)?;
    Ok(content.lines().map(|s| s.to_string()).collect())
}
}
