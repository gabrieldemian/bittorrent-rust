pub mod bitfield;
pub mod cli;
pub mod disk;
pub mod error;
pub mod extension;
pub mod frontend;
pub mod magnet_parser;
pub mod metainfo;
pub mod peer;
pub mod tcp_wire;
pub mod torrent;
pub mod torrent_list;
pub mod tracker;

//
//  Torrent <--> Peers --> Disk IO
//     |         |
//    \/         |
//  Tracker  <---|
//
