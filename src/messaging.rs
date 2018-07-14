use super::compact::Compact;
use super::id::RawID;

/// Return type of message handling functions, signifying if
/// an `Actor`/`Actor` should live on after receiving a certain message type.
///
/// Note: so far only has an effect on `Actor`s in `Swarm`s
pub enum Fate {
    /// Means: the `Actor`/`Actor` should live on
    Live,
    /// Means: the `Actor`/`Actor` should be stopped, its state can be deallocated
    Die,
}

/// Trait that a datastructure must implement in order
/// to be sent and received as a message.
///
/// Automatically implemented for everything that is [`Compact`](../../compact)
pub trait Message: Compact + 'static {}
impl<T: Compact + 'static> Message for T {}

/// Combination of a message and its destination recipient id
#[derive(Compact, Clone)]
#[repr(C)]
pub struct Packet<M: Message> {
    /// RawID of the `Actor`/`Actor` that should receive this message
    pub recipient_id: RawID,
    /// The message itself
    pub message: M,
}
