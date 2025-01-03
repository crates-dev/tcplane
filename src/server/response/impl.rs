use super::error::Error;
use super::r#type::{Response, ResponseResult};
use std::{io::Write, net::TcpStream};

impl Default for Response {
    fn default() -> Self {
        Self { data: Vec::new() }
    }
}

impl Response {
    pub fn data<T>(&mut self, data: T) -> &mut Self
    where
        T: Into<Vec<u8>>,
    {
        self.set_data(data.into());
        self
    }

    pub fn send(&mut self, mut stream: &TcpStream) -> ResponseResult {
        let send_res: ResponseResult = stream
            .write_all(&self.get_data())
            .and_then(|_| stream.flush())
            .map_err(|err| Error::ResponseError(err.to_string()))
            .and_then(|_| Ok(self.get_data()))
            .cloned();
        send_res
    }
}
