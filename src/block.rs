use std::io;

use lbase::{TableTrait, Index};

use crate::db::LbaseConnector;


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
                db: &LbaseConnector, signature: &[u8; 64],
                public: &[u8; 64], group: &[u8; 32], key: &[u8; 32],
                version: u64, data: &[u8]
            ) -> Self {
        let data_pos = db.block_heap_data.append(data).unwrap();

        let block = Self {
            signature: *signature,
            public: *public,
            group: *group,
            key: *key,
            version: version,
            data: data_pos as u64,
        };
        block.insert(&db.block_table).unwrap();

        Index::<[u8; 64]>::add(&db.block_index_public, public).unwrap();
        Index::<([u8; 64], [u8; 32])>::add(
            &db.block_index_public_group, &(*public, *group)
        ).unwrap();
        Index::<([u8; 64], [u8; 32], [u8; 32])>::add(
            &db.block_index_public_group_key, &(*public, *group, *key)
        ).unwrap();

        block
    }

    pub fn exists(
                db: &LbaseConnector,
                public: &[u8; 64], group: &[u8; 32], key: &[u8; 32]
            ) -> bool {
        Self::get_by_public_group_key(db, public, group, key).is_some()
    }

    pub fn get_data(&self, db: &LbaseConnector) -> Result<Vec<u8>, io::Error> {
        db.block_heap_data.get(self.data as usize)
    }

    pub fn update_data(
                &mut self, db: &LbaseConnector,
                id: usize, signature: &[u8; 64], version: u64, data: &[u8]
            ) {
        self.signature = *signature;
        self.version = version;
        self.data = db.block_heap_data.update(
            data, self.data as usize
        ).unwrap() as u64;
        self.update(&db.block_table, id).unwrap();
    }

    pub fn get_by_public(db: &LbaseConnector, public: &[u8; 64]) -> Vec<Self> {
        Index::<[u8; 64]>::search_many(
            &db.block_index_public, &public
        ).map(
            |id| Block::get(&db.block_table, id).unwrap()
        ).collect()
    }

    pub fn get_by_public_group(
                db: &LbaseConnector, public: &[u8; 64], group: &[u8; 32]
            ) -> Vec<Self> {
        Index::<([u8; 64], [u8; 32])>::search_many(
            &db.block_index_public_group, &(*public, *group)
        ).map(
            |id| Block::get(&db.block_table, id).unwrap()
        ).collect()
    }

    pub fn get_by_public_group_key(
                db: &LbaseConnector,
                public: &[u8; 64], group: &[u8; 32], key: &[u8; 32]
            ) -> Option<(usize, Self)> {
        Index::<([u8; 64], [u8; 32], [u8; 32])>::search_many(
            &db.block_index_public_group_key, &(*public, *group, *key)
        ).map(
            |id| (id, Block::get(&db.block_table, id).unwrap())
        ).nth(0)
    }
}
