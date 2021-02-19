use actix_web::{get, post, put, delete};
use actix_web::{web, Result};
use actix_web::error::{ErrorPreconditionFailed, ErrorForbidden};
use serde::Deserialize;

use crate::utils::{hex_to_point, hex_to_bigi_pair, hex_to_bytes};
use crate::block::Block;
use crate::appstate::AppState;


#[derive(Deserialize, Debug)]
struct InputJson {
    version: Option<u64>,
    data: String,
    signature: String,
}


// async fn index(state: web::Data<AppState>) -> impl Responder {
//     let size;
//     {
//         let db = state.db.lock().unwrap();
//         size = db.block_table.size();
//     }
//     format!("{}\n", size)
// }


#[get("/version")]
async fn version() -> Result<String> {
    Ok("2.0.0".to_string())
}


#[get("/groups/{public}")]
async fn groups(
            web::Path(public): web::Path<String>,
            state: web::Data<AppState>
        ) -> Result<String> {
    let mut public_bytes: [u8; 64] = [0u8; 64];
    public_bytes[..public.len()].clone_from_slice(public.as_bytes());
    // println!("{:?}", public_bytes);
    Ok("success".to_string())
}


#[get("/keys/{public}/{group}")]
async fn keys(
            web::Path((public, group)): web::Path<(String, String)>,
            state: web::Data<AppState>
        ) -> Result<String> {
    Ok("success".to_string())
}


#[get("/info/{public}/{group}/{key}")]
async fn info(
            web::Path((public, group, key)): web::Path<(String, String, String)>,
            state: web::Data<AppState>
        ) -> Result<String> {
    Ok("success".to_string())
}


#[get("/data/{public}/{group}/{key}")]
async fn data_get(
            web::Path((public, group, key)): web::Path<(String, String, String)>,
            state: web::Data<AppState>
        ) -> Result<String> {
    Ok("success".to_string())
}


#[post("/data/{public}/{group}/{key}")]
async fn data_post(
            req_json: web::Json<InputJson>,
            web::Path((public, group, key)): web::Path<(String, String, String)>,
            state: web::Data<AppState>
        ) -> Result<String> {
    // Unpacking input
    let public_key = hex_to_point(&public);
    let signature = hex_to_bigi_pair(&req_json.signature);
    let data = hex_to_bytes(&req_json.data);

    // Check version
    if req_json.version.is_none() || (req_json.version.unwrap() == 0) {
        return Err(ErrorPreconditionFailed("invalid version"));
    }

    // Check block exists
    if {
        let db = state.db.lock().unwrap();
        Block::exists(&db, &public_key, &group, &key)
    } {
        return Err(ErrorPreconditionFailed("block exists"));
    }

    // Check signature
    if false {
        return Err(ErrorForbidden("invalid signature"));
    }

    // Insert block
    {
        let db = state.db.lock().unwrap();
        Block::create(&db, &public_key, &group, &key, req_json.version.unwrap(), &data, &signature);
    }

    Ok("ok".to_string())
}


#[put("/data/{public}/{group}/{key}")]
async fn data_put(
            req_body: String,
            web::Path((public, group, key)): web::Path<(String, String, String)>,
            state: web::Data<AppState>
        ) -> Result<String> {
    Ok("success".to_string())
}


#[delete("/data/{public}/{group}/{key}")]
async fn data_delete(
            req_body: String,
            web::Path((public, group, key)): web::Path<(String, String, String)>,
            state: web::Data<AppState>
        ) -> Result<String> {
    Ok("success".to_string())
}
