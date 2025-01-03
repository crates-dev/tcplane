use super::r#type::ControllerData;
use crate::server::{request::r#type::Request, response::r#type::Response};
use hyperlane_log::*;

impl ControllerData {
    pub fn new() -> Self {
        ControllerData {
            stream: None,
            request: Request::new(),
            response: Response::default(),
            log: Log::default(),
        }
    }
}
