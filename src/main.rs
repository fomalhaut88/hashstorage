use std::sync::Mutex;

use actix_web::{web, App, HttpServer, Responder};

use hashstorage::Lbase;


pub struct AppState {
    pub db: Mutex<Lbase>,
}


async fn index(data: web::Data<AppState>) -> impl Responder {
    let size;
    {
        let db = data.db.lock().unwrap();
        size = db.block_table.size();
    }
    format!("{}\n", size)
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let data = web::Data::new(AppState {
        db: Mutex::new(Lbase::new("lbase-db")),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .route("/", web::get().to(index))
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
