use std::sync::Mutex;

use crate::LbaseConnector;


pub struct AppState {
    pub db: Mutex<LbaseConnector>,
}
