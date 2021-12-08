use std::fs::File;
use actix_web::{get, post, Responder, web, Result, HttpResponse};
use serde::{Serialize,Deserialize};
use crate::Deck;
use std::io::{Read, Write};
use actix_web::http::StatusCode;
use actix_web::web::Data;
use crate::error::DeckError;
use crate::response::SuccessResponse;

#[derive(Serialize)]
pub struct Block1{
    on: Option<String>,
    remote: Option<String>,
    trip_alarm: Option<String>,
    start_fail_alarm: Option<String>,
    stop_fail_alarm: Option<String>,
}

#[derive(Serialize)]
pub struct Block2{
    on: Option<String>,
    remote: Option<String>,
    trip_alarm: Option<String>,
    start_fail_alarm: Option<String>,
    stop_fail_alarm: Option<String>,
}

#[derive(Serialize)]
pub struct Status {
    block1:Block1,
    block2:Block2,
    air_flow_measurement: Option<String>,
    leading_monitoring_status: Option<String>,
    supply_air_temperature: Option<String>,
}


#[get("")]
pub async fn index(deck: web::Data<Deck>) -> Result<impl Responder> {
    let mut contents = String::new();

    open_file(&deck,&deck.files.deck5_read,false)?.read_to_string(&mut contents).unwrap();
    let decode_contents: Vec<String> = contents.split(";").map(|s| s.to_string()).collect();

    let success_response = SuccessResponse{
        code: StatusCode::OK.as_u16(),
        message: "success".to_string(),
        data: Some(Status {
            block1: Block1 {
                on:Some(decode_contents[0].to_string()),
                remote: Some(decode_contents[1].to_string()),
                trip_alarm: Some(decode_contents[2].to_string()),
                start_fail_alarm: Some(decode_contents[3].to_string()),
                stop_fail_alarm: Some(decode_contents[4].to_string()),
            },
            block2: Block2 {
                on: Some(decode_contents[5].to_string()),
                remote: Some(decode_contents[6].to_string()),
                trip_alarm: Some(decode_contents[7].to_string()),
                start_fail_alarm: Some(decode_contents[8].to_string()),
                stop_fail_alarm: Some(decode_contents[9].to_string()),
            },
            air_flow_measurement: Some(decode_contents[10].to_string()),
            leading_monitoring_status: Some(decode_contents[11].to_string()),
            supply_air_temperature: Some(decode_contents[12].to_string()),
        }),
        status: "success".to_string()
    };

    Ok(HttpResponse::Ok().json(success_response))

}

#[derive(Deserialize)]
pub struct StartForm {
    status: u8,
}

#[post("/start")]
pub async fn start(form: web::Form<StartForm>, deck: web::Data<Deck>) -> Result<impl Responder> {
    let mut contents = String::new();

    open_file(&deck,&deck.files.deck5_write,false)?.read_to_string(&mut contents).unwrap();
    let mut decode_contents: Vec<String> = contents.split(";").map(|s| s.to_string()).collect();
    decode_contents[0] = form.status.to_string();
    /*****************************************************************************************/

    let write_content = decode_contents.join(";");
    open_file(&deck,&deck.files.deck5_write,true)?.write_all(write_content.as_bytes()).unwrap();

    Ok(HttpResponse::Ok().json(
        SuccessResponse{
            code: StatusCode::OK.as_u16(),
            message: "success".to_string(),
            data: Some(write_content),
            status: "success".to_string()
        }
    ))
}

#[derive(Deserialize)]
pub struct ResetForm {
    status: u8,
}

#[post("/reset")]
pub async fn reset(form: web::Form<ResetForm>, deck: web::Data<Deck>) -> Result<impl Responder> {
    let mut contents = String::new();

    open_file(&deck,&deck.files.deck5_write,false)?.read_to_string(&mut contents).unwrap();
    let mut decode_contents: Vec<String> = contents.split(";").map(|s| s.to_string()).collect();
    decode_contents[1] = form.status.to_string();
    /*****************************************************************************************/

    let write_content = decode_contents.join(";");
    open_file(&deck,&deck.files.deck5_write,true)?.write_all(write_content.as_bytes()).unwrap();


    Ok(HttpResponse::Ok().json(
        SuccessResponse{
            code: StatusCode::OK.as_u16(),
            message: "success".to_string(),
            data: Some(write_content),
            status: "success".to_string()
        }
    ))
}


fn open_file(deck: &Data<Deck>, deck_file: &String, write:bool) -> Result<File,DeckError> {

    if !write {
        std::fs::File::open(format!("{}\\{}", &deck.file_path,deck_file))
            .map_err(|_e| DeckError::FileNotFound { file_name: format!("{}\\{}", &deck.file_path,deck_file) })
    }else{
        std::fs::OpenOptions::new().write(true).open(format!("{}\\{}", &deck.file_path,deck_file))
            .map_err(|_e| DeckError::FileNotFound { file_name: format!("{}\\{}", &deck.file_path,deck_file) })
    }
}
