use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use libp2p::PeerId;
use kv_trie::Trie;
use command::RecordConfig;
use blake2::Blake2b512;
use logs::Log;

enum NodeType {
    Leader,
    Candidate,
    Follower
}

/// The node that the current instance uses. Self.
/// Generic over a record config which is used for both the command and the persistant state.
struct Node<T: RecordConfig> {
    term: Arc<AtomicUsize>,
    node_type: NodeType,
    voted_for: libp2p::PeerId,
    state: RwLock<Trie<T::Value, T::Key>>
    logs
}

