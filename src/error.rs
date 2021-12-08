use actix_web::{error, HttpResponse, http::StatusCode, dev::HttpResponseBuilder};
use derive_more::{Display, Error};
use crate::response::ErrorResponse;

#[derive(Debug, Display, Error)]
pub enum DeckError {
    #[display(fmt = "File Not Found: {}", file_name)]
    FileNotFound { file_name: String },
}

impl error::ResponseError for DeckError {
    fn error_response(&self) -> HttpResponse {
        let return_response = ErrorResponse{
            code: 404,
            message: self.to_string(),
            status: "error".to_string()
        };


        HttpResponseBuilder::new(self.status_code()).json(return_response)
    }


    fn status_code(&self) -> StatusCode {
        match *self {
            DeckError::FileNotFound { .. } => StatusCode::NOT_FOUND,
        }
    }
}
