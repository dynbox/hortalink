use sqlx::{Pool, Postgres};
use crate::json::event::SocketRequest;

pub mod emitters;
pub mod receivers;