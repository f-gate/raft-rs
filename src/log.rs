
use std::marker::PhantomData;

/// The logger is used to keep track of the logs within the node.
/// Must be updated when the persistant state is changed.
struct Logger<T: RecordConfig> {
    last_commit_index: usize,
    logs: Vec<Command<T>>
}