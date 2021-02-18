use std::sync::Mutex;

use actix_web::{web, App, HttpRequest, HttpServer, Responder};

use hashstorage::{DBState, Lbase};


async fn index(req: HttpRequest, data: web::Data<DBState>) -> impl Responder {
    let size;
    {
        let db = data.db.lock().unwrap();
        size = db.block_table.size();
    }
    format!("{}\n", size)
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = web::Data::new(DBState {
        db: Mutex::new(Lbase::new("lbase-db")),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(db.clone())
            .route("/", web::get().to(index))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
