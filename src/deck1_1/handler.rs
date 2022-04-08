use actix_web::{get, post, Responder, web, Result, HttpResponse};
use serde::{Serialize,Deserialize};
use crate::Deck;
use std::io::{Read, Write};
use actix_web::http::StatusCode;
use crate::helper::open_file;
use crate::response::SuccessResponse;

#[derive(Serialize)]
pub struct AhuBstD21 {
    on: Option<String>,
    remote: Option<String>,
    trip_alarm: Option<String>,
    start_fail_alarm: Option<String>,
    stop_fail_alarm: Option<String>,
    air_flow_switch: Option<String>,
    air_block_alarm: Option<String>,
    supply_air_temperature: Option<String>,
    rs485_lost_com: Option<String>,
}

#[derive(Serialize)]
pub struct AhuBstD12 {
    on: Option<String>,
    remote:Option<String>,
    trip_alarm: Option<String>,
    start_fail_alarm: Option<String>,
    stop_fail_alarm: Option<String>,
    hepa_filter_alarm: Option<String>,
    air_flow_switch: Option<String>,
    air_block_alarm: Option<String>,
}

#[derive(Serialize)]
pub struct AhuBstD13 {
    on: Option<String>,
    remote: Option<String>,
    trip_alarm: Option<String>,
    start_fail_alarm: Option<String>,
    stop_fail_alarm: Option<String>,
    hepa_filter_alarm: Option<String>,
    air_flow_switch: Option<String>,
    air_block_alarm: Option<String>,
}

#[derive(Serialize)]
pub struct Common{
    co_sensor: Option<String>,
    co2_sensor: Option<String>,
    co_sensor_fault: Option<String>,
    co2_sensor_fault: Option<String>,
}

#[derive(Serialize)]
pub struct Status {
    ahu_bst_d21:AhuBstD21,
    ahu_bst_d12:AhuBstD12,
    ahu_bst_d13:AhuBstD13,
    common:Common,
    isolate:Option<String>
}


#[get("")]
pub async fn index(deck: web::Data<Deck>) -> Result<impl Responder> {
    let mut contents = String::new();

    open_file(&deck,&deck.files.deck1_1_read,false)?.read_to_string(&mut contents).unwrap();
    let mut decode_contents: Vec<String> = contents.split(";").map(|s| s.to_string()).collect();

    while decode_contents.len() < 30 {
        decode_contents.push("".to_string());
    }

    let success_response = SuccessResponse{
        code: StatusCode::OK.as_u16(),
        message: "success".to_string(),
        data: Some(Status{
            ahu_bst_d21: AhuBstD21 {
                on: Some(decode_contents[0].to_string()),
                remote: Some(decode_contents[1].to_string()),
                trip_alarm: Some(decode_contents[2].to_string()),
                start_fail_alarm: Some(decode_contents[3].to_string()),
                stop_fail_alarm: Some(decode_contents[4].to_string()),
                air_flow_switch: Some(decode_contents[5].to_string()),
                air_block_alarm: Some(decode_contents[6].to_string()),
                supply_air_temperature: Some(decode_contents[7].to_string()),
                rs485_lost_com: Some(decode_contents[8].to_string()),
            },
            ahu_bst_d12: AhuBstD12 {
                on: Some(decode_contents[9].to_string()),
                remote: Some(decode_contents[10].to_string()),
                trip_alarm: Some(decode_contents[11].to_string()),
                start_fail_alarm: Some(decode_contents[12].to_string()),
                stop_fail_alarm: Some(decode_contents[13].to_string()),
                hepa_filter_alarm: Some(decode_contents[14].to_string()),
                air_flow_switch: Some(decode_contents[15].to_string()),
                air_block_alarm: Some(decode_contents[16].to_string())
            },
            ahu_bst_d13: AhuBstD13 {
                on: Some(decode_contents[17].to_string()),
                remote: Some(decode_contents[18].to_string()),
                trip_alarm: Some(decode_contents[19].to_string()),
                start_fail_alarm: Some(decode_contents[20].to_string()),
                stop_fail_alarm: Some(decode_contents[21].to_string()),
                hepa_filter_alarm: Some(decode_contents[22].to_string()),
                air_flow_switch: Some(decode_contents[23].to_string()),
                air_block_alarm: Some(decode_contents[24].to_string())
            },
            common: Common {
                co_sensor: Some(decode_contents[25].to_string()),
                co2_sensor: Some(decode_contents[26].to_string()),
                co_sensor_fault: Some(decode_contents[27].to_string()),
                co2_sensor_fault: Some(decode_contents[28].to_string())
            },
            isolate: Some(decode_contents[30].to_string()),
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

#[derive(Deserialize)]
pub struct IsolateForm {
    status: u8,
}

#[post("/vesda/isolate")]
pub async fn isolate(form: web::Form<IsolateForm>, deck: web::Data<Deck>) -> Result<impl Responder> {
    let mut contents = String::new();

    open_file(&deck,&deck.files.deck1_1_write,false)?.read_to_string(&mut contents).unwrap();
    let mut decode_contents: Vec<String> = contents.split(";").map(|s| s.to_string()).collect();
    decode_contents[2] = form.status.to_string();
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
