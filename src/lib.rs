#![feature(async_fn_in_trait)]

use std::net::IpAddr;

pub mod compose_trait;
pub mod dyn_trait;
pub mod external_enum;
pub mod internal_enum;

pub fn get_service_nodes() -> Vec<IpAddr> {
    Vec::new()
}

pub fn connect_to_nodes(nodes: &[IpAddr]) -> Vec<NodeConnection> {
    let mut connections = Vec::with_capacity(nodes.len());

    for node in nodes {
        connections.push(NodeConnection::connect(*node));
    }

    connections
}

pub struct NodeConnection {
    _addr: IpAddr,
}
impl NodeConnection {
    pub fn connect(addr: IpAddr) -> Self {
        Self { _addr: addr }
    }
}
