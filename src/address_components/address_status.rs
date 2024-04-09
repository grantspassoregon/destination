use serde::{Deserialize, Serialize};
use std::fmt;

/// The `AddressStatus` enum represents the address status, used by City of Grants Pass staff.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, PartialEq, Eq, Default, PartialOrd, Ord)]
pub enum AddressStatus {
    /// Current active valid address.
    Current,
    /// Pending assignment that is not active.
    Pending,
    /// Former assignment that has been retired.
    Retired,
    /// Temporary assignment for development.
    Temporary,
    /// Physical location associated with a virtual business office.
    Virtual,
    /// A valid address that has not been classified.
    #[default]
    Other,
}

impl fmt::Display for AddressStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Current => write!(f, "Current"),
            Self::Pending => write!(f, "Pending"),
            Self::Retired => write!(f, "Retired"),
            Self::Temporary => write!(f, "Temporary"),
            Self::Virtual => write!(f, "Virtual"),
            Self::Other => write!(f, "Other"),
        }
    }
}
