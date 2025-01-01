use super::error::Error;
use super::r#type::{Response, ResponseResult};
use std::{io::Write, net::TcpStream};

impl Default for Response {
    fn default() -> Self {
        Self {
            data: Some(Vec::new()),
        }
    }
}

impl Response {
    pub fn data<T>(&mut self, data: T) -> &mut Self
    where
        T: Into<Vec<u8>>,
    {
        self.set_data(Some(data.into()));
        self
    }

    pub fn send(&mut self, mut stream: &TcpStream) -> ResponseResult {
        let send_res: ResponseResult = stream
            .write_all(&self.get_data().clone().unwrap())
            .map_err(|err| Error::ResponseError(err.to_string()))
            .and_then(|_| Ok(self.get_data()))
            .cloned();
        send_res
    }
}
