use sha2::{Sha256, Digest};
use bigi::Bigi;
use bigi_ecc::{CurveTrait, Point};
use bigi_ecc::schemas::Schema;
use bigi_ecc::ecdsa::check_signature as ecdsa_check_signature;


pub fn check_signature<T: CurveTrait>(
            schema: &Schema<T>, signature: &[u8; 64],
            public: &[u8; 64], group: &[u8; 32], key: &[u8; 32],
            version: u64, data: &Vec<u8>
        ) -> bool {
    let signature_pair = (
        Bigi::from_bytes(&signature[..32]),
        Bigi::from_bytes(&signature[32..]),
    );
    let public_point = Point::from_bytes(public);
    let bytes = [group, key, &version.to_le_bytes()[..], &data[..]].concat();
    let hash = sha256_hash(&bytes);

    ecdsa_check_signature(schema, &public_point, &hash, &signature_pair)
}


fn sha256_hash(bytes: &[u8]) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.reset();
    hasher.input(bytes);
    hasher.result().to_vec()
}
