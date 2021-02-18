use lbase::TableTrait;


#[derive(Debug, Clone, Copy)]
pub struct Block {
    pub id: usize,
    pub version: u64,
    pub public: [u8; 64],
    pub group: [u8; 64],
    pub key: [u8; 64],
    pub signature: [u8; 64],
    pub data: usize,
}


impl TableTrait for Block {}


impl Block {}
