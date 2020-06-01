mod behaviour;
mod compress;
mod config;
pub mod errors;
pub mod network;
mod network_group;
mod peer;
pub mod peer_registry;
pub mod peer_store;
mod protocols;
mod services;

#[cfg(test)]
mod tests;

pub use crate::{
    behaviour::Behaviour,
    config::NetworkConfig,
    errors::Error,
    network::{NetworkController, NetworkService, NetworkState},
    peer::{Peer, PeerIdentifyInfo},
    peer_registry::PeerRegistry,
    peer_store::{types::MultiaddrExt, Score},
    protocols::{
        support_protocols::SupportProtocols, CKBProtocol, CKBProtocolContext, CKBProtocolHandler,
        PeerIndex,
    },
};
pub use p2p::{
    bytes, multiaddr,
    secio::{PeerId, PublicKey},
    service::{BlockingFlag, ServiceControl, SessionType, TargetSession},
    traits::ServiceProtocol,
    ProtocolId,
};
pub use tokio;

// Max data size in send buffer: 24MB (a little larger than max frame length)
pub const DEFAULT_SEND_BUFFER: usize = 24 * 1024 * 1024;

pub type ProtocolVersion = String;
