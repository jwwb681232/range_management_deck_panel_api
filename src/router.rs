use actix_web::{Scope, web};
use crate::deck1_1;
use crate::deck1_2;
use crate::deck5;

pub fn deck1_1_service() -> Scope {
    web::scope("/deck1-1")
        .service(deck1_1::handler::index)
        .service(deck1_1::handler::start)
        .service(deck1_1::handler::reset)
}

pub fn deck1_2_service() -> Scope {
    web::scope("/deck1-2")
        .service(deck1_2::handler::index)
        .service(deck1_2::handler::start)
        .service(deck1_2::handler::reset)
}

pub fn deck5_service() -> Scope {
    web::scope("/deck5")
        .service(deck5::handler::index)
        .service(deck5::handler::start)
        .service(deck5::handler::reset)
}
