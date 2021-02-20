use std::sync::Mutex;

use actix_web::{web, App, HttpServer};

use hashstorage::db::LbaseConnector;
use hashstorage::appstate::AppState;
use hashstorage::views::{version, groups, keys, info,
                         data_get, data_post, data_put};


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let state = web::Data::new(AppState {
        db: Mutex::new(LbaseConnector::new("lbase-db")),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .service(version)
            .service(groups)
            .service(keys)
            .service(info)
            .service(data_get)
            .service(data_post)
            .service(data_put)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
