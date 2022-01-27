use actix_web::{App, HttpServer, middleware::Logger};
use crate::router::{deck1_1_service,deck1_2_service,deck5_service};

mod error;
mod response;
mod router;
mod helper;
mod deck1_1;
mod deck1_2;
mod deck5;

#[derive(Clone)]
struct ReadWriteFile {
    deck1_1_read: String,
    deck1_1_write: String,
    deck1_2_read: String,
    deck1_2_write: String,
    deck5_read: String,
    deck5_write: String,
}

#[derive(Clone)]
pub struct Deck {
    file_path: String,
    files: ReadWriteFile,
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    init();

    HttpServer::new(||
        App::new()
            .data(Deck {
                file_path: std::env::var("FILE_DIR").unwrap_or("./".to_string()),
                files: ReadWriteFile {
                    deck1_1_read:  "Deck1_Panel1 Read From PLC.txt".to_string(),
                    deck1_1_write: "Deck1_Panel1 Write To PLC.txt".to_string(),
                    deck1_2_read:  "Deck1_Panel2 Read From PLC.txt".to_string(),
                    deck1_2_write: "Deck1_Panel2 Write To PLC.txt".to_string(),
                    deck5_read:    "Deck5_Panel Read From PLC.txt".to_string(),
                    deck5_write:   "Deck5_Panel Write To PLC.txt".to_string(),
                },
            })
            .wrap(Logger::new("[Route:%r] [Status:%s] [Time:%Dms]"))
            .service(deck1_1_service())
            .service(deck1_2_service())
            .service(deck5_service())
        )
        .bind("0.0.0.1:3008")?
        .run()
        .await
}

fn init() {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    dotenv::dotenv().ok();

    println!("{}","Deck Api Started On http://127.0.0.1:8086/api/maintenance\r\n
Read Deck 1 Panel 1 GET http://127.0.0.1:8086/api/maintenance/deck1-1\r\n
Start Deck 1 Panel 1 POST [status=1] To http://127.0.0.1:8086/api/maintenance/deck1-1/start\r
Reset Deck 1 Panel 1 POST [status=1] To http://127.0.0.1:8086/api/maintenance/deck1-1/reset\r
Isolate VESDA Deck 1 Panel 1 POST [status=1] To http://127.0.0.1:8086/api/maintenance/deck1-1/vesda/isolate\r\n

Read Deck 1 Panel 2 GET http://127.0.0.1:8086/api/maintenance/deck1-2\r
Start Deck 1 Panel 2 POST [status=1] To http://127.0.0.1:8086/api/maintenance/deck1-2/start\r
Reset Deck 1 Panel 2 POST [status=1] To http://127.0.0.1:8086/api/maintenance/deck1-2/reset\r
Isolate VESDA Deck 1 Panel 2 POST [status=1] To http://127.0.0.1:8086/api/maintenance/deck1-2/vesda/isolate\r\n

Read Deck 5 GET http://127.0.0.1:8086/api/maintenance/deck5\r
Start Deck 5 POST [status=1] To http://127.0.0.1:8086/api/maintenance/deck5/start\r
Reset Deck 5 POST [status=1] To http://127.0.0.1:8086/api/maintenance/deck5/reset\r
");
}
