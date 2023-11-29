use hex;
use rand::Rng;

pub fn generate_apikey() -> String {
    let mut rng = rand::thread_rng();
    let b: [u8; 16] = rng.gen();
    hex::encode(b)
}
