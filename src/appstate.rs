use std::sync::Mutex;

use bigi_ecc::weierstrass::WeierstrassCurve;
use bigi_ecc::schemas::Schema;

use crate::LbaseConnector;


pub struct AppState {
    pub db: Mutex<LbaseConnector>,
    pub schema: Schema<WeierstrassCurve>,
}
