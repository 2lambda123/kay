use super::type_registry::ShortTypeId;

/// Identifies a machine in the network
#[cfg_attr(feature = "serde-serialization", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, Eq, PartialEq, PartialOrd, Ord, Hash, Debug)]
pub struct MachineID(pub u8);

/// A `RawID` uniquely identifies an `Actor`, or even a `Actor` within a `Swarm`
#[cfg_attr(feature = "serde-serialization", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct RawID {
    /// Used to identify instances within a top-level `Actor`. The main use-case is
    /// `Swarm` identifying and dispatching to its `Instances` using this field
    pub instance_id: u32,
    /// An ID for the type of the identified `Actor`, used to dispatch messages
    /// to the message handling functions registered for this type
    pub type_id: ShortTypeId,
    /// ID of the machine (in a computing cluster or multiplayer environment)
    /// that the identified `Actor` lives on
    pub machine: MachineID,
    /// Allows safe reuse of a `RawID` after `Actor`/`Actor` death.
    /// The version is incremented to make the new (otherwise identical) `RawID`
    /// distinguishable from erroneous references to the `Actor`/`Actor` previously identified
    pub version: u8,
}

pub fn broadcast_instance_id() -> u32 {
    u32::max_value()
}

pub fn broadcast_machine_id() -> MachineID {
    MachineID(u8::max_value())
}

impl RawID {
    /// Create a new `RawID`
    pub fn new(type_id: ShortTypeId, instance_id: u32, machine: MachineID, version: u8) -> Self {
        RawID {
            type_id,
            machine,
            version,
            instance_id,
        }
    }

    /// Get a version of an actor `RawID` that signals that a message
    /// should be delivered to all machine-local instances.
    pub fn local_broadcast(&self) -> RawID {
        RawID {
            instance_id: broadcast_instance_id(),
            ..*self
        }
    }

    /// Get a version of an actor `RawID` that signals that a message
    /// should be delivered globally (to all instances on all machines).
    pub fn global_broadcast(&self) -> RawID {
        RawID {
            machine: broadcast_machine_id(),
            ..self.local_broadcast()
        }
    }

    /// Check whether this `RawID` signals a local or global broadcast.
    pub fn is_broadcast(&self) -> bool {
        self.instance_id == broadcast_instance_id()
    }

    /// Check whether this `RawID` signals specifically a global broadcast.
    pub fn is_global_broadcast(&self) -> bool {
        self.machine == broadcast_machine_id()
    }
}

impl ::std::fmt::Debug for RawID {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(
            f,
            "{}_{}.{}@{}",
            u16::from(self.type_id),
            self.instance_id,
            self.version,
            self.machine.0,
        )
    }
}

/// `TypedID` is a construct on top of a `RawID` that can refer
/// to a specific kind of actor, or actor trait at compile time
pub trait TypedID: Copy + Clone + Sized + ::std::fmt::Debug + ::std::hash::Hash {
    /// Get the underlying `RawID`
    fn as_raw(&self) -> RawID;
    /// Construct a new `TypedID` from a `RawID` - this implies knowledge
    /// about the type of actor referenced by the `RawID`
    unsafe fn from_raw(raw: RawID) -> Self;
}
