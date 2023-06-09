use rand::Rng;
use speedy::{BigEndian, Readable, Writable};

use crate::error::Error;

use super::action::Action;

#[derive(Debug, PartialEq, Readable, Writable)]
pub struct Request {
    pub connection_id: u64,
    pub action: u32,
    pub transaction_id: u32,
    pub infohash: [u8; 20],
    pub peer_id: [u8; 20],
    pub downloaded: u64,
    pub left: u64,
    pub uploaded: u64,
    pub event: u64,
    pub ip_address: u32,
    pub num_want: u32,
    pub port: u16,
}

impl Request {
    pub const LENGTH: usize = 98;

    pub fn new(connection_id: u64, infohash: [u8; 20], peer_id: [u8; 20], port: u16) -> Self {
        let mut rng = rand::thread_rng();
        Self {
            connection_id,
            action: Action::Announce.into(),
            transaction_id: rng.gen(),
            infohash,
            peer_id,
            downloaded: 0,
            left: u64::MAX,
            uploaded: 0,
            event: 0,
            ip_address: 0,
            num_want: u32::MAX,
            port,
        }
    }

    fn _deserialize(buf: &[u8]) -> Result<(Self, &[u8]), Error> {
        if buf.len() != Self::LENGTH {
            return Err(Error::TrackerResponseLength);
        }

        let res = Self::read_from_buffer_with_ctx(BigEndian {}, buf)?;

        Ok((res, &buf[Self::LENGTH..]))
    }

    pub fn serialize(&self) -> Vec<u8> {
        self.write_to_vec_with_ctx(BigEndian {}).unwrap()
    }
}

#[derive(Debug, PartialEq, Writable, Readable)]
pub struct Response {
    pub action: u32,
    pub transaction_id: u32,
    pub interval: u32,
    pub leechers: u32,
    pub seeders: u32,
}

impl Response {
    pub(crate) const LENGTH: usize = 20;

    pub fn deserialize(buf: &[u8]) -> Result<(Self, &[u8]), Error> {
        if buf.len() < Response::LENGTH {
            return Err(Error::TrackerResponseLength);
        }

        let res = Self::read_from_buffer_with_ctx(BigEndian {}, buf)?;

        Ok((res, &buf[Self::LENGTH..]))
    }
}
