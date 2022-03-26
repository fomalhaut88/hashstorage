use itertools::Itertools;

use actix_web::{get, post};
use actix_web::{web, Result, HttpResponse};
use actix_web::error::{ErrorNotFound, ErrorPreconditionFailed, ErrorForbidden};
use serde::{Serialize, Deserialize};
use hashstorage_utils::convert::*;
use hashstorage_utils::crypto::check_signature;

use crate::block::Block;
use crate::appstate::AppState;


/* JSON structs */

#[derive(Serialize, Deserialize, Debug)]
struct VersionJson {
    version: String,
}


#[derive(Serialize, Deserialize, Debug)]
struct InputJson {
    version: u64,
    data: String,
    signature: String,
}


#[derive(Serialize, Deserialize, Debug)]
struct InfoJson {
    signature: String,
    public: String,
    group: String,
    key: String,
    version: u64,
}


#[derive(Serialize, Deserialize, Debug)]
struct BlockJson {
    signature: String,
    public: String,
    group: String,
    key: String,
    version: u64,
    data: String,
}


#[derive(Deserialize, Debug)]
struct URLFullParams {
    public: String,
    group: String,
    key: String,
}


/* Views */

#[get("/version")]
async fn version() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(
        VersionJson { version: env!("CARGO_PKG_VERSION").to_string() }
    ))
}


#[get("/groups/{public}")]
async fn groups(
            public: web::Path<String>,
            state: web::Data<AppState>
        ) -> Result<HttpResponse> {
    let public_bytes: [u8; 64] = hex_to_bytes(&public);

    let result = {
        let db = state.db.lock().unwrap();
        Block::get_by_public(&db, &public_bytes).iter().map(
            |block| str_from_bytes(&block.group)
        ).unique().collect::<Vec<String>>()
    };

    Ok(HttpResponse::Ok().json(result))
}


#[get("/keys/{public}/{group}")]
async fn keys(
            params: web::Path<(String, String)>,
            state: web::Data<AppState>
        ) -> Result<HttpResponse> {
    let (public, group) = params.into_inner();

    let public_bytes: [u8; 64] = hex_to_bytes(&public);
    let group_bytes: [u8; 32] = str_to_bytes_sized(&group);

    let result = {
        let db = state.db.lock().unwrap();
        Block::get_by_public_group(
            &db, &public_bytes, &group_bytes
        ).iter().map(
            |block| str_from_bytes(&block.key)
        ).collect::<Vec<String>>()
    };

    Ok(HttpResponse::Ok().json(result))
}


#[get("/info/{public}/{group}/{key}")]
async fn info(
            params: web::Path<URLFullParams>,
            state: web::Data<AppState>
        ) -> Result<HttpResponse> {
    let public_bytes: [u8; 64] = hex_to_bytes(&params.public);
    let group_bytes: [u8; 32] = str_to_bytes_sized(&params.group);
    let key_bytes: [u8; 32] = str_to_bytes_sized(&params.key);

    // Get block
    let pair_id_block = {
        let db = state.db.lock().unwrap();
        Block::get_by_public_group_key(
            &db, &public_bytes, &group_bytes, &key_bytes
        )
    };

    // If block is not found
    if pair_id_block.is_none() {
        return Err(ErrorNotFound("not found"));
    }

    // Unpack pair_id_block
    let block = pair_id_block.unwrap().1;

    Ok(HttpResponse::Ok().json(InfoJson {
        signature: hex_from_bytes(&block.signature),
        public: params.public.clone(),
        group: params.group.clone(),
        key: params.key.clone(),
        version: block.version,
    }))
}


#[get("/data/{public}/{group}/{key}")]
async fn data_get(
            params: web::Path<URLFullParams>,
            state: web::Data<AppState>
        ) -> Result<HttpResponse> {
    let public_bytes: [u8; 64] = hex_to_bytes(&params.public);
    let group_bytes: [u8; 32] = str_to_bytes_sized(&params.group);
    let key_bytes: [u8; 32] = str_to_bytes_sized(&params.key);

    let result = {
        let db = state.db.lock().unwrap();

        // Get block
        let pair_id_block = Block::get_by_public_group_key(
            &db, &public_bytes, &group_bytes, &key_bytes
        );

        // If block is not found
        if pair_id_block.is_none() {
            return Err(ErrorNotFound("not found"));
        }

        // Unpack pair_id_block and get bytes
        let block = pair_id_block.unwrap().1;
        let bytes = block.get_data(&db).unwrap();

        Ok((block, bytes))
    };

    match result {
        Ok((block, bytes)) => Ok(HttpResponse::Ok().json(BlockJson {
            signature: hex_from_bytes(&block.signature),
            public: params.public.clone(),
            group: params.group.clone(),
            key: params.key.clone(),
            version: block.version,
            data: String::from_utf8(bytes).unwrap(),
        })),
        Err(err) => Err(err)
    }
}


#[post("/data/{public}/{group}/{key}")]
async fn data_post(
            req_json: web::Json<InputJson>,
            params: web::Path<URLFullParams>,
            state: web::Data<AppState>
        ) -> Result<HttpResponse> {
    let public_bytes: [u8; 64] = hex_to_bytes(&params.public);
    let group_bytes: [u8; 32] = str_to_bytes_sized(&params.group);
    let key_bytes: [u8; 32] = str_to_bytes_sized(&params.key);
    let data_bytes = req_json.data.as_bytes();
    let signature_bytes: [u8; 64] = hex_to_bytes(&req_json.signature);

    let result = {
        // Check signature
        if !check_signature(
                    &state.schema, &signature_bytes,
                    &public_bytes, &group_bytes, &key_bytes,
                    req_json.version, &data_bytes
                ) {
            return Err(ErrorForbidden("invalid signature"));
        }

        // Get DB connection
        let db = state.db.lock().unwrap();

        // Get block
        let pair_id_block = Block::get_by_public_group_key(
            &db, &public_bytes, &group_bytes, &key_bytes
        );

        // Insert or update block
        match pair_id_block {
            Some((block_id, mut block)) => {
                // If version is not the next version of the block
                if req_json.version != block.version + 1 {
                    return Err(ErrorPreconditionFailed("invalid version"));
                }

                // Update block
                block.update_data(
                    &db, block_id, &signature_bytes, req_json.version,
                    &data_bytes
                );
            },
            None => {
                // If version is not equal to 1
                if req_json.version != 1 {
                    return Err(ErrorPreconditionFailed("invalid version"));
                }

                // Insert block
                Block::create(
                    &db, &signature_bytes,
                    &public_bytes, &group_bytes, &key_bytes,
                    req_json.version, &data_bytes
                );
            }
        };

        Ok(())
    };

    match result {
        Ok(()) => Ok(HttpResponse::NoContent().finish()),
        Err(err) => Err(err)
    }
}
