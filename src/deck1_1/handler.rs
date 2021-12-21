use actix_web::{get, post, Responder, web, Result, HttpResponse};
use serde::{Serialize,Deserialize};
use crate::Deck;
use std::io::{Read, Write};
use actix_web::http::StatusCode;
use crate::helper::open_file;
use crate::response::SuccessResponse;

#[derive(Serialize)]
pub struct Block1{
    on: Option<String>,
    remote: Option<String>,
    trip_alarm: Option<String>,
    start_fail_alarm: Option<String>,
    stop_fail_alarm: Option<String>,
    air_flow_switch: Option<String>,
    supply_air_temperature: Option<String>,
    rs485_lost_com: Option<String>,
}

#[derive(Serialize)]
pub struct Block2{
    on: Option<String>,
    remote:Option<String>,
    trip_alarm: Option<String>,
    start_fail_alarm: Option<String>,
    stop_fail_alarm: Option<String>,
    hepa_filter_alarm: Option<String>,
    air_flow_switch: Option<String>,
}

#[derive(Serialize)]
pub struct Block3{
    on: Option<String>,
    remote: Option<String>,
    trip_alarm: Option<String>,
    start_fail_alarm: Option<String>,
    stop_fail_alarm: Option<String>,
    hepa_filter_alarm: Option<String>,
    air_flow_switch: Option<String>,
}

#[derive(Serialize)]
pub struct Block4{
    co_sensor: Option<String>,
    co2_sensor: Option<String>,
    co_sensor_fault: Option<String>,
    co2_sensor_fault: Option<String>,
}

#[derive(Serialize)]
pub struct AirBlockAlarm{
    air_block_alarm: Option<String>,
}

#[derive(Serialize)]
pub struct Status {
    block1:Block1,
    block2:Block2,
    block3:Block3,
    block4:Block4,
    ahu_bst_d21:AirBlockAlarm,
    ahu_bst_d12:AirBlockAlarm,
    ahu_bst_d13:AirBlockAlarm,
}


#[get("")]
pub async fn index(deck: web::Data<Deck>) -> Result<impl Responder> {
    let mut contents = String::new();

    open_file(&deck,&deck.files.deck1_1_read,false)?.read_to_string(&mut contents).unwrap();
    let decode_contents: Vec<String> = contents.split(";").map(|s| s.to_string()).collect();

    println!("{:?}",decode_contents.get(0));
    println!("{:?}",decode_contents.get(29));

    let success_response = SuccessResponse{
        code: StatusCode::OK.as_u16(),
        message: "success".to_string(),
        data: Some(Block1{
            on: decode_contents.get(29),
            remote: None,
            trip_alarm: None,
            start_fail_alarm: None,
            stop_fail_alarm: None,
            air_flow_switch: None,
            supply_air_temperature: None,
            rs485_lost_com: None
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

    open_file(&deck,&deck.files.deck1_1_write,false)?.read_to_string(&mut contents).unwrap();
    let mut decode_contents: Vec<String> = contents.split(";").map(|s| s.to_string()).collect();
    decode_contents[0] = form.status.to_string();
    /*****************************************************************************************/

    let write_content = decode_contents.join(";");
    open_file(&deck,&deck.files.deck1_1_write,true)?.write_all(write_content.as_bytes()).unwrap();

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

    open_file(&deck,&deck.files.deck1_1_write,false)?.read_to_string(&mut contents).unwrap();
    let mut decode_contents: Vec<String> = contents.split(";").map(|s| s.to_string()).collect();
    decode_contents[1] = form.status.to_string();
    /*****************************************************************************************/

    let write_content = decode_contents.join(";");
    open_file(&deck,&deck.files.deck1_1_write,true)?.write_all(write_content.as_bytes()).unwrap();


    Ok(HttpResponse::Ok().json(
        SuccessResponse{
            code: StatusCode::OK.as_u16(),
            message: "success".to_string(),
            data: Some(write_content),
            status: "success".to_string()
        }
    ))
}
