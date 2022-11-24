
use common::{
    Term,
    RaftPeerId,
    Index,
};

pub trait RaftRpc: Send + Sync {
    /// Invoked by leader to replicate log entries.
    /// Arguments: 
    /// current_term: the current term being held by the leader.
    /// leader_id: the peer id of the leader as defined by the p2p network.
    /// prev_log_index: the latest log index preceeding all the incoming logs.
    /// prev_log_term: the latest log term preceeding all incoming logs. (empty for heartbeat)
    /// leader_commit_index: leaders commit index.
    fn append(
        current_term: Term,
        leader_id: RaftPeerId,
        prev_log_index: Index,
        prev_log_term: Term,
        leader_commit_index: Index)
    -> Result<(), ()>;


    /// Invoked by candidates to gather votes.
    /// Arguments:
    /// term: candidates term
    /// candidate_id: the Raft PeerId of the candidate.
    /// last_log_index: The latest known log index.
    /// last_log_term: The latest known log term.
    fn request_vote(term: Term,
        candidate_id: RaftPeerId, 
        last_log_index: Index, 
        last_log_term: Term) 
    -> Result<(), ()>
}