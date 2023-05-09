use std::{error::Error, net::IpAddr, sync::mpsc::Receiver};

use crate::NodeConnection;

pub trait ExternallyDrivenTransition {
    type EventType;

    fn execute(&mut self, input: Self::EventType) -> Result<(), Box<dyn Error>>;
    fn is_terminal_state(&self) -> bool;
    fn transition(self) -> Self;
}

pub fn externally_driven_executor<T: ExternallyDrivenTransition>(
    initial_state: T,
    events: Receiver<T::EventType>,
) -> Result<(), Box<dyn Error>> {
    let mut current_state = initial_state;

    while let Ok(input) = events.recv() {
        current_state.execute(input)?;

        current_state = current_state.transition();
        if current_state.is_terminal_state() {
            break;
        }
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

pub enum ExternalEvent {}

impl ExternallyDrivenTransition for FullStateMachine {
    type EventType = ExternalEvent;

    fn execute(&mut self, input: Self::EventType) -> Result<(), Box<dyn Error>> {
        match self {
            FullStateMachine::DiscoverNodes(state) => state.execute(input),
            FullStateMachine::ConnectNodes(state) => state.execute(input),
            FullStateMachine::Consensus(state) => state.execute(input),
            FullStateMachine::Leader(state) => state.execute(input),
            FullStateMachine::Follower(state) => state.execute(input),
            FullStateMachine::Terminate => unreachable!(),
        }
    }

    fn is_terminal_state(&self) -> bool {
        matches!(self, Self::Terminate)
    }

    fn transition(self) -> Self {
        match self {
            FullStateMachine::DiscoverNodes(state) => {
                FullStateMachine::ConnectNodes(ConnectNodes::new(state.nodes))
            }
            FullStateMachine::ConnectNodes(state) => {
                FullStateMachine::Consensus(Consensus::new(state.connections))
            }
            FullStateMachine::Consensus(state) => {
                if state.is_leader {
                    FullStateMachine::Leader(Leader::new(state.connections))
                } else {
                    FullStateMachine::Follower(Follower::new(state.connections))
                }
            }
            FullStateMachine::Leader(_) => FullStateMachine::Terminate,
            FullStateMachine::Follower(_) => FullStateMachine::Terminate,
            FullStateMachine::Terminate => unreachable!(),
        }
    }
}

// Mock States
// 1. Discover all nodes in the network
// 2. Connect to all nodes
// 3. Elect a leader
// 4. Start a sync process
//     1. Followers will wait for events
//     2. The Leader will only send events

pub struct DiscoverNodes {
    nodes: Vec<IpAddr>,
}
impl DiscoverNodes {
    pub fn execute(&mut self, _input: ExternalEvent) -> Result<(), Box<dyn Error>> {
        self.nodes = crate::get_service_nodes();
        Ok(())
    }
}

pub struct ConnectNodes {
    nodes: Vec<IpAddr>,
    connections: Vec<NodeConnection>,
}

impl ConnectNodes {
    pub fn new(nodes: Vec<IpAddr>) -> Self {
        Self {
            nodes,
            connections: Vec::new(),
        }
    }

    pub fn execute(&mut self, _input: ExternalEvent) -> Result<(), Box<dyn Error>> {
        self.connections = crate::connect_to_nodes(&self.nodes);
        Ok(())
    }
}

pub struct Consensus {
    connections: Vec<NodeConnection>,
    is_leader: bool,
}

impl Consensus {
    pub fn new(connections: Vec<NodeConnection>) -> Self {
        Self {
            connections,
            is_leader: false,
        }
    }

    pub fn execute(&mut self, _input: ExternalEvent) -> Result<(), Box<dyn Error>> {
        self.is_leader = true;
        Ok(())
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

    pub fn execute(&mut self, _input: ExternalEvent) -> Result<(), Box<dyn Error>> {
        Ok(())
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

    pub fn execute(&mut self, _input: ExternalEvent) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}
