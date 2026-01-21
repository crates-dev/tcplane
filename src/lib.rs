//! tcplane
//!
//! tcplane is a lightweight and high-performance Rust TCP server
//! library designed to simplify network service development.
//! It supports TCP communication, data stream management,
//! and connection handling, focusing on providing efficient
//! low-level network connections and data transmission capabilities,
//! making it ideal for building modern network services.

pub(crate) mod common;
pub(crate) mod config;
pub(crate) mod context;
pub(crate) mod handler;
pub(crate) mod middleware;
pub(crate) mod request;
pub(crate) mod response;
pub(crate) mod server;
pub(crate) mod stream;
pub(crate) mod utils;

pub use {config::*, context::*, request::*, response::*, server::*, stream::*, utils::*};

pub use tokio;

pub(crate) use {common::*, handler::*, middleware::*};

pub(crate) use std::{
    any::Any,
    collections::HashMap,
    error::Error as StdError,
    fmt::{self, Display},
    future::Future,
    net::{IpAddr, Ipv4Addr, SocketAddr, SocketAddrV4},
    panic::set_hook,
    pin::Pin,
    sync::Arc,
};

pub(crate) use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
    sync::{MutexGuard, RwLock, RwLockReadGuard, RwLockWriteGuard},
};
