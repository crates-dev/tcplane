use super::{config::r#type::ServerConfig, func::r#type::FuncBox, tmp::r#type::Tmp};
use http_type::ArcRwLock;
use lombok_macros::Lombok;

#[derive(Clone, Lombok)]
pub struct Server {
    pub(crate) cfg: ArcRwLock<ServerConfig>,
    pub(crate) func: ArcRwLock<FuncBox>,
    pub(crate) middleware: ArcRwLock<Vec<FuncBox>>,
    pub(crate) tmp: ArcRwLock<Tmp>,
}
