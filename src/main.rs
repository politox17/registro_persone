use registro_persone::db::menu;
use registro_persone::db::menu::leggi_input_int;
use registro_persone::db::menu::leggi_input_string;
use registro_persone::db::registro::Registro;

fn main() -> Result<(), menu::Errori> {
    
let mut reg = Registro::new();
    loop {
        println!("\t\tBenvenuto nella nostra pagina! Leggi e scegli cosa fare!");
        println!("====================================================================");
        println!("[1] Iscrizione torneo calcio");
        println!("[2] Mostra iscritti");
        println!("[3] Cerca per nome");
        println!("[4] Cerca per cognome");
        println!("[5] Aggiorna");
        println!("[6] Rimuovi");
        println!("[7] Esci\n");
        println!("Selezionare il numero: \n");

        let input = leggi_input_int()?;
        

        match input {
            1 => {
                menu::gestione_registro()?;
                Ok(())
            },
            2 => {
                reg.mostra_iscritti();
                Ok(())
            },
            3 => {
                println!("\tInserire il nome da cercare: \n");
                let nome = leggi_input_string()?;
                reg.cerca_per_nome(&nome);
                Ok(())
            },
            4 => {
                println!("\tInserire il cognome da cercare: \n");
                let cognome = leggi_input_string()?;
                reg.cerca_per_cognome(&cognome);
                Ok(())
            },
            7 => {
                println!("Arrivederci!");
                break Ok(());
            },
            _ => {
                println!("Opzione non valida");
                Ok(())
            }
        }?; // Questo ? propaga eventuali errori dal match
    }
}