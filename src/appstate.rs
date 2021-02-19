use std::sync::Mutex;

use crate::Lbase;


pub struct AppState {
    pub db: Mutex<Lbase>,
}
