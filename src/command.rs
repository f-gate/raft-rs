use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
/// A command is what is serialized and gossiped when the leader wants to commit 
/// certain logs.
enum Command<T: RecordConfig> 
{
    Insert{key: T::Key, value: T::Value},
    Get(T::Key),
    Remove(T::Key),
}

/// The config used in the stoage item.
trait RecordConfig {
    type Key = Sized + Clone 
    type Value = Sized + Clone + Hash + Eq
}



