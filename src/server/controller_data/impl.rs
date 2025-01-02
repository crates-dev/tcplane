use super::r#type::ControllerData;
use crate::{server::response::r#type::Response, ControllerDataRequest};
use hyperlane_log::*;

impl ControllerData {
    pub fn new() -> Self {
        ControllerData {
            stream: None,
            request: ControllerDataRequest::new(),
            response: Response::default(),
            log: Log::default(),
        }
    }
}
