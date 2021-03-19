use std::env;
use std::sync::Mutex;

use bigi_ecc::schemas::load_secp256k1;
use actix_web::{web, App, HttpServer};
use actix_web::http::header;
use actix_cors::Cors;

use hashstorage::db::LbaseConnector;
use hashstorage::appstate::AppState;
use hashstorage::views::{version, groups, keys, info, data_get, data_post};


const DB_PATH_DEFAULT: &str = "db";
const HASHSTORAGE_HOST_DEFAULT: &str = "127.0.0.1";
const HASHSTORAGE_PORT_DEFAULT: u16 = 8080;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let hashstorage_host: &str = &env::var("HASHSTORAGE_HOST")
        .unwrap_or(HASHSTORAGE_HOST_DEFAULT.to_string())[..];

    let hashstorage_port: u16 = env::var("HASHSTORAGE_PORT")
        .unwrap_or(HASHSTORAGE_PORT_DEFAULT.to_string())
        .parse::<u16>().unwrap();

    let lbase_path: &str = &env::var("DB_PATH")
        .unwrap_or(DB_PATH_DEFAULT.to_string())[..];

    let state = web::Data::new(AppState {
        db: Mutex::new(LbaseConnector::new(lbase_path)),
        schema: load_secp256k1(),
    });

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allowed_methods(vec!["GET", "POST"])
            .allowed_header(header::CONTENT_TYPE);

        App::new()
            .wrap(cors)
            .app_data(state.clone())
            .service(version)
            .service(groups)
            .service(keys)
            .service(info)
            .service(data_get)
            .service(data_post)
    })
        .bind((hashstorage_host, hashstorage_port))?
        .run()
        .await
}
