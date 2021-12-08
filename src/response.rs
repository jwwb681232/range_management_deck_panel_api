use serde::Serialize;

#[derive(Serialize)]
pub struct ErrorResponse {
    pub code: u16,
    pub message: String,
    pub status: String,
}

#[derive(Serialize)]
pub struct SuccessResponse<T> {
    pub code: u16,
    pub message: String,
    pub data:Option<T>,
    pub status: String,
}
