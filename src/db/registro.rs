use std::collections::HashMap;
use rand::Rng;

#[derive(Debug)]
pub struct Registro{
   pub iscritti: HashMap<i32, Iscritto>
}


pub struct Iscritto {
    nome: String,
    cognome: String,
}

impl Registro {
    pub fn new() -> Self {
        iscritti::HashMap::new(),
    }
    pub fn aggiungi_iscritti(&mut self, nome: String) -> i32 {
           let id = Self::creazione_id();
           self.iscritti.insert(id, nome.to_string());
    }
    pub fn creazione_id() -> i32 {
        let mut random = rand::rng();
        let id = random.random_range(100..999);
        id
    }
}
