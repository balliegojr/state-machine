use std::{error::Error, net::IpAddr};

use crate::NodeConnection;

/// Benchmark function
pub fn run_full_state_machine() {
    executor(Box::<DiscoverNodes>::default()).unwrap();
}

/// Trait to be implemented by each state
pub trait State {
    fn execute(self: Box<Self>) -> Result<Option<Box<dyn State>>, Box<dyn Error>>;
}

/// State machine executor function
pub fn executor(initial_state: Box<dyn State>) -> Result<(), Box<dyn Error>> {
    let mut current_state = Some(initial_state);

    while let Some(state) = current_state {
        current_state = state.execute()?;
    }

    Ok(())
}

// Mock States
// 1. Discover all nodes in the network
// 2. Connect to all nodes
// 3. Elect a leader
// 4. Start a sync process
//     1. Followers will wait for events
//     2. The Leader will only send events

#[derive(Default)]
pub struct DiscoverNodes {}

impl State for DiscoverNodes {
    fn execute(self: Box<Self>) -> Result<Option<Box<dyn State>>, Box<dyn Error>> {
        let nodes = crate::get_service_nodes();
        Ok(Some(Box::new(ConnectNodes::new(nodes))))
    }
}

pub struct ConnectNodes {
    nodes: Vec<IpAddr>,
}

impl ConnectNodes {
    pub fn new(nodes: Vec<IpAddr>) -> Self {
        Self { nodes }
    }
}

impl State for ConnectNodes {
    fn execute(self: Box<Self>) -> Result<Option<Box<dyn State>>, Box<dyn Error>> {
        let nodes = crate::connect_to_nodes(&self.nodes);

        Ok(Some(Box::new(Consensus::new(nodes))))
    }
}

pub struct Consensus {
    connections: Vec<NodeConnection>,
}

impl Consensus {
    pub fn new(connections: Vec<NodeConnection>) -> Self {
        Self { connections }
    }
}
impl State for Consensus {
    fn execute(self: Box<Self>) -> Result<Option<Box<dyn State>>, Box<dyn Error>> {
        let consensus_result = true;
        let next: Box<dyn State> = if consensus_result {
            Box::new(Leader::new(self.connections))
        } else {
            Box::new(Follower::new(self.connections))
        };

        Ok(Some(next))
    }
}

pub struct Leader {
    _connections: Vec<NodeConnection>,
}

impl Leader {
    pub fn new(connections: Vec<NodeConnection>) -> Self {
        Self {
            _connections: connections,
        }
    }
}

impl State for Leader {
    fn execute(self: Box<Self>) -> Result<Option<Box<dyn State>>, Box<dyn Error>> {
        Ok(None)
    }
}

pub struct Follower {
    _connections: Vec<NodeConnection>,
}

impl Follower {
    pub fn new(connections: Vec<NodeConnection>) -> Self {
        Self {
            _connections: connections,
        }
    }
}

impl State for Follower {
    fn execute(self: Box<Self>) -> Result<Option<Box<dyn State>>, Box<dyn Error>> {
        Ok(None)
    }
}
