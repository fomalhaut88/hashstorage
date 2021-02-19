use std::convert::TryInto;

use lbase::TableTrait;
use bigi::Bigi;
use bigi_ecc::Point;

use crate::utils::string_to_bytes_fixed;
use crate::db::Lbase;


#[derive(Debug, Clone, Copy)]
pub struct Block {
    pub signature: [u8; 64],
    pub public: [u8; 64],
    pub group: [u8; 32],
    pub key: [u8; 32],
    pub version: u64,
    pub data: u64,
}


impl TableTrait for Block {}


impl Block {
    pub fn create(
                db: &Lbase, public_key: &Point, group: &str, key: &str,
                version: u64, data: &[u8], signature: &(Bigi, Bigi)
            ) -> (usize, Self) {
        let data_pos = db.block_heap_data.append(data).unwrap();
        let block = Self {
            signature: [signature.0.to_bytes(), signature.1.to_bytes()].concat()[..].try_into().unwrap(),
            public: public_key.to_bytes()[..].try_into().unwrap(),
            group: string_to_bytes_fixed(group, 32)[..].try_into().unwrap(),
            key: string_to_bytes_fixed(key, 32)[..].try_into().unwrap(),
            version: version,
            data: data_pos as u64,
        };
        let id = block.insert(&db.block_table).unwrap();
        (id, block)
    }

    pub fn exists(db: &Lbase, public_key: &Point, group: &str, key: &str) -> bool {
        false
    }
}
