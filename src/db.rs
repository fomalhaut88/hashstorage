use lbase::{Table, Index, Heap};

use crate::block::Block;


#[derive(Debug)]
pub struct LbaseConnector {
    pub block_table: Table,
    pub block_index_public: Table,
    pub block_index_public_group: Table,
    pub block_index_public_group_key: Table,
    pub block_heap_data: Heap,
}


impl LbaseConnector {
    pub fn new(path: &str) -> Self {
        Self {
            block_table: Table::new::<Block>(
                format!("{}/block-table.tbl", path).as_str()
            ),
            block_index_public: Table::new::<Index>(
                format!("{}/block-index-public.idx", path).as_str()
            ),
            block_index_public_group:
                Table::new::<Index>(
                    format!("{}/block-index-public-group.idx", path).as_str()
                ),
            block_index_public_group_key:
                Table::new::<Index>(
                    format!(
                        "{}/block-index-public-group-key.idx", path
                    ).as_str()
                ),
            block_heap_data: Heap::new(
                format!("{}/block-heap-data.heap", path).as_str()
            ),
        }
    }
}
