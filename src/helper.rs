use std::fs::File;
use actix_web::{web::Data};
use crate::Deck;
use crate::error::DeckError;

pub fn open_file(deck: &Data<Deck>, deck_file: &String, write: bool) -> Result<File, DeckError> {
    if !write {
        std::fs::File::open(format!("{}\\{}", &deck.file_path, deck_file))
            .map_err(|_e| DeckError::FileNotFound { file_name: format!("{}\\{}", &deck.file_path, deck_file) })
    } else {
        std::fs::OpenOptions::new().write(true).open(format!("{}\\{}", &deck.file_path, deck_file))
            .map_err(|_e| DeckError::FileNotFound { file_name: format!("{}\\{}", &deck.file_path, deck_file) })
    }
}
