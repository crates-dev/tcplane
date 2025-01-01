use crate::server::response::r#type::Response;

use super::r#type::ControllerData;
use hyperlane_log::*;

impl ControllerData {
    pub fn new() -> Self {
        ControllerData {
            stream: None,
            request: None,
            response: Response::default(),
            log: Log::default(),
        }
    }
}
