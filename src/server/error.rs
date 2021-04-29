use crate::net::error::NetError;

#[derive(Debug)]
pub enum ServerError {
    NetError(NetError)
}
