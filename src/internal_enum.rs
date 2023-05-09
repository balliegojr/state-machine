use std::{error::Error, net::IpAddr};

use crate::NodeConnection;

/// Benchmark function
pub fn run_full_state_machine() {
    internally_driven_executor(FullStateMachine::DiscoverNodes(DiscoverNodes::default())).unwrap();
}

/// Trait to be implemented by the state machine enum, so we can have a generic executor
///
/// The states in this implementation don't need to implement any trait
pub trait InternallyDrivenTransition {
    fn execute(self) -> Result<Self, Box<dyn Error>>
    where
        Self: Sized;

    fn is_terminal_state(&self) -> bool;
}

/// State machine executor function
pub fn internally_driven_executor<T: InternallyDrivenTransition>(
    initial_state: T,
) -> Result<(), Box<dyn Error>> {
    let mut current_state = initial_state;

    while !current_state.is_terminal_state() {
        current_state = current_state.execute()?;
    }

    Ok(())
}

/// Represent all possible states
pub enum FullStateMachine {
    DiscoverNodes(DiscoverNodes),
    ConnectNodes(ConnectNodes),
    Consensus(Consensus),
    Leader(Leader),
    Follower(Follower),
    Terminate,
}

impl InternallyDrivenTransition for FullStateMachine {
    fn execute(self) -> Result<Self, Box<dyn Error>>
    where
        Self: Sized,
    {
        match self {
            FullStateMachine::DiscoverNodes(discover_nodes) => {
                let nodes = discover_nodes.execute();
                Ok(FullStateMachine::ConnectNodes(ConnectNodes::new(nodes)))
            }
            FullStateMachine::ConnectNodes(connect_nodes) => {
                let connections = connect_nodes.execute();
                Ok(FullStateMachine::Consensus(Consensus::new(connections)))
            }
            FullStateMachine::Consensus(consensus) => {
                let (is_leader, connections) = consensus.execute();
                if is_leader {
                    Ok(FullStateMachine::Leader(Leader::new(connections)))
                } else {
                    Ok(FullStateMachine::Follower(Follower::new(connections)))
                }
            }
            FullStateMachine::Leader(leader) => {
                leader.execute();
                Ok(FullStateMachine::Terminate)
            }
            FullStateMachine::Follower(follower) => {
                follower.execute();
                Ok(FullStateMachine::Terminate)
            }
            FullStateMachine::Terminate => {
                unreachable!()
            }
        }
    }

    fn is_terminal_state(&self) -> bool {
        matches!(self, Self::Terminate)
    }
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
impl DiscoverNodes {
    pub fn execute(self) -> Vec<IpAddr> {
        crate::get_service_nodes()
    }
}

pub struct ConnectNodes {
    nodes: Vec<IpAddr>,
}

impl ConnectNodes {
    pub fn new(nodes: Vec<IpAddr>) -> Self {
        Self { nodes }
    }

    pub fn execute(self) -> Vec<crate::NodeConnection> {
        crate::connect_to_nodes(&self.nodes)
    }
}

pub struct Consensus {
    connections: Vec<NodeConnection>,
}

impl Consensus {
    pub fn new(connections: Vec<NodeConnection>) -> Self {
        Self { connections }
    }

    pub fn execute(self) -> (bool, Vec<NodeConnection>) {
        (true, self.connections)
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

    pub fn execute(self) {}
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

    pub fn execute(self) {}
}
