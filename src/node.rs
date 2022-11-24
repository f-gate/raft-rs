use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use kv_trie::Trie;
use command::RecordConfig;
use blake2::Blake2b512;
use logs::Log;

use common::{
    Term,
    RaftPeerId
};

enum NodeType {
    Leader,
    Candidate,
    Follower
}

/// The node that the current instance uses. Self.
/// Generic over a record config which is used for both the command and the persistant state.
struct Node<T: RecordConfig> {
    current_term: Arc<Term>,
    voted_for: libp2p::PeerId,
    // Using blake2 as a default for ease.
    logs: Log<T>,
    last_applied: Index,

    node_type: NodeType,
    state: RwLock<Trie<T::Value, T::Key, Blake2b512>>,



    
}

