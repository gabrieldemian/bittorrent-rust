use std::io;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Failed to send a connect request to the tracker")]
    ConnectSendFailed,
    #[error("Failed to resolve socket address")]
    TrackerSocketAddrs(#[from] io::Error),
    #[error("Tracker resolved to no unusable addresses")]
    TrackerNoHosts,
    #[error("The response received from the connect handshake was wrong")]
    TrackerResponse,
    #[error(transparent)]
    Bincode(#[from] bincode::Error),
}