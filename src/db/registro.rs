use std::collections::HashMap;

#[derive(Debug)]
pub struct Registro{
   pub iscritti: HashMap<i32, Iscritto>
}


pub struct Iscritto {
    nome: String,
    cognome: String,
}
