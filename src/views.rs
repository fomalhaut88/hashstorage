use actix_web::{get, post, put};
use actix_web::{web, Result, HttpResponse};
use actix_web::error::{ErrorNotFound, ErrorPreconditionFailed, ErrorForbidden};
use serde::{Serialize, Deserialize};

use crate::block::Block;
use crate::utils::*;
use crate::crypto::check_signature;
use crate::appstate::AppState;


/* JSON structs */

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


/* Views */

#[get("/version")]
async fn version() -> Result<String> {
    Ok(env!("CARGO_PKG_VERSION").to_string())
}


#[get("/groups/{public}")]
async fn groups(
            web::Path(public): web::Path<String>,
            state: web::Data<AppState>
        ) -> Result<HttpResponse> {
    let public_bytes: [u8; 64] = hex_to_bytes(&public);

    let result = {
        let db = state.db.lock().unwrap();
        Block::get_by_public(&db, &public_bytes).iter().map(
            |block| String::from_utf8(block.group.to_vec()).unwrap()
        ).collect::<Vec<String>>()
    };

    Ok(HttpResponse::Ok().json(result))
}


#[get("/keys/{public}/{group}")]
async fn keys(
            web::Path((public, group)): web::Path<(String, String)>,
            state: web::Data<AppState>
        ) -> Result<HttpResponse> {
    let public_bytes: [u8; 64] = hex_to_bytes(&public);
    let group_bytes: [u8; 32] = str_to_bytes_sized(&group);

    let result = {
        let db = state.db.lock().unwrap();
        Block::get_by_public_group(
            &db, &public_bytes, &group_bytes
        ).iter().map(
            |block| String::from_utf8(block.key.to_vec()).unwrap()
        ).collect::<Vec<String>>()
    };

    Ok(HttpResponse::Ok().json(result))
}


#[get("/info/{public}/{group}/{key}")]
async fn info(
            web::Path((public, group, key)):
                web::Path<(String, String, String)>,
            state: web::Data<AppState>
        ) -> Result<HttpResponse> {
    let public_bytes: [u8; 64] = hex_to_bytes(&public);
    let group_bytes: [u8; 32] = str_to_bytes_sized(&group);
    let key_bytes: [u8; 32] = str_to_bytes_sized(&key);

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
        public: public,
        group: group,
        key: key,
        version: block.version,
    }))
}


#[get("/data/{public}/{group}/{key}")]
async fn data_get(
            web::Path((public, group, key)):
                web::Path<(String, String, String)>,
            state: web::Data<AppState>
        ) -> Result<HttpResponse> {
    let public_bytes: [u8; 64] = hex_to_bytes(&public);
    let group_bytes: [u8; 32] = str_to_bytes_sized(&group);
    let key_bytes: [u8; 32] = str_to_bytes_sized(&key);

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
            public: public,
            group: group,
            key: key,
            version: block.version,
            data: hex_from_bytes(&bytes),
        })),
        Err(err) => Err(err)
    }
}


#[post("/data/{public}/{group}/{key}")]
async fn data_post(
            req_json: web::Json<InputJson>,
            web::Path((public, group, key)):
                web::Path<(String, String, String)>,
            state: web::Data<AppState>
        ) -> Result<HttpResponse> {
    let public_bytes: [u8; 64] = hex_to_bytes(&public);
    let group_bytes: [u8; 32] = str_to_bytes_sized(&group);
    let key_bytes: [u8; 32] = str_to_bytes_sized(&key);
    let data_bytes = hex_to_bytes_vec(&req_json.data);
    let signature_bytes: [u8; 64] = hex_to_bytes(&req_json.signature);

    let result = {
        let db = state.db.lock().unwrap();

        // Check block exists
        if Block::exists(&db, &public_bytes, &group_bytes, &key_bytes) {
            return Err(ErrorPreconditionFailed("block exists"));
        }

        // Check signature
        if !check_signature(
                    &state.schema, &signature_bytes,
                    &public_bytes, &group_bytes, &key_bytes,
                    req_json.version, &data_bytes
                ) {
            return Err(ErrorForbidden("invalid signature"));
        }

        // Insert block
        Block::create(
            &db, &signature_bytes,
            &public_bytes, &group_bytes, &key_bytes,
            req_json.version, &data_bytes
        );

        Ok(())
    };

    match result {
        Ok(()) => Ok(HttpResponse::Created().finish()),
        Err(err) => Err(err)
    }
}


#[put("/data/{public}/{group}/{key}")]
async fn data_put(
            req_json: web::Json<InputJson>,
            web::Path((public, group, key)):
                web::Path<(String, String, String)>,
            state: web::Data<AppState>
        ) -> Result<HttpResponse> {
    let public_bytes: [u8; 64] = hex_to_bytes(&public);
    let group_bytes: [u8; 32] = str_to_bytes_sized(&group);
    let key_bytes: [u8; 32] = str_to_bytes_sized(&key);
    let data_bytes = hex_to_bytes_vec(&req_json.data);
    let signature_bytes: [u8; 64] = hex_to_bytes(&req_json.signature);

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

        // Unpack pair_id_block
        let (block_id, mut block) = pair_id_block.unwrap();

        // Check signature
        if !check_signature(
                    &state.schema, &signature_bytes,
                    &public_bytes, &group_bytes, &key_bytes,
                    req_json.version, &data_bytes
                ) {
            return Err(ErrorForbidden("invalid signature"));
        }

        // Update block
        block.update_data(
            &db, block_id, &signature_bytes, req_json.version, &data_bytes
        );

        Ok(())
    };

    match result {
        Ok(()) => Ok(HttpResponse::Created().finish()),
        Err(err) => Err(err)
    }
}
