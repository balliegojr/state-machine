use std::{error::Error, marker::PhantomData, net::IpAddr};

use crate::NodeConnection;

/// Benchmark function
pub fn run_full_state_machine() {
    DiscoverNodes {}
        .and_then(ConnectNodes::new)
        .and_then(Consensus::new)
        .and_then(|(leader, connections)| LeaderOrFollower::new(leader, connections))
        .execute()
        .unwrap()
}

/// Represent a task or state to be executed
pub trait State {
    type Output;

    fn execute(self) -> Result<Self::Output, Box<dyn Error>>;
}

/// Composer trait.
///
/// This will make it possible to chain states together
pub trait StateComposer {
    fn and_then<T, F>(self, map_fn: F) -> AndThen<Self, T, F>
    where
        Self: State + Sized,
        T: State,
        F: FnOnce(Self::Output) -> T,
    {
        AndThen {
            previous: self,
            map_fn,
            _marker: Default::default(),
        }
    }
}

impl<T> StateComposer for T where T: State {}

/// And Then chainable state
pub struct AndThen<T, U, F> {
    previous: T,
    map_fn: F,
    _marker: PhantomData<U>,
}

impl<T, U, F> State for AndThen<T, U, F>
where
    T: State,
    U: State,
    F: FnOnce(T::Output) -> U,
{
    type Output = U::Output;

    fn execute(self) -> Result<Self::Output, Box<dyn Error>>
    where
        Self: Sized,
    {
        let previous_output = self.previous.execute()?;
        let next_task = (self.map_fn)(previous_output);
        next_task.execute()
    }
}

// Mock States
// 1. Discover all nodes in the network
// 2. Connect to all nodes
// 3. Elect a leader
// 4. Start a sync process
//     1. Followers will wait for events
//     2. The Leader will only send events

pub struct DiscoverNodes {}
impl State for DiscoverNodes {
    type Output = Vec<IpAddr>;

    fn execute(self) -> Result<Self::Output, Box<dyn Error>> {
        Ok(crate::get_service_nodes())
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
    type Output = Vec<NodeConnection>;

    fn execute(self) -> Result<Self::Output, Box<dyn Error>> {
        Ok(crate::connect_to_nodes(&self.nodes))
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
    type Output = (bool, Vec<NodeConnection>);

    fn execute(self) -> Result<Self::Output, Box<dyn Error>> {
        Ok((true, self.connections))
    }
}

pub struct LeaderOrFollower {
    is_leader: bool,
    connections: Vec<NodeConnection>,
}

impl LeaderOrFollower {
    pub fn new(is_leader: bool, connections: Vec<NodeConnection>) -> Self {
        Self {
            is_leader,
            connections,
        }
    }
}

impl State for LeaderOrFollower {
    type Output = ();

    fn execute(self) -> Result<Self::Output, Box<dyn Error>> {
        if self.is_leader {
            Leader::new(self.connections).execute()
        } else {
            Follower::new(self.connections).execute()
        }
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
    type Output = ();

    fn execute(self) -> Result<Self::Output, Box<dyn Error>> {
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
}

impl State for Follower {
    type Output = ();

    fn execute(self) -> Result<Self::Output, Box<dyn Error>> {
        Ok(())
    }
}
