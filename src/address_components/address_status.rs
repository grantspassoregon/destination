use serde::{Deserialize, Serialize};

/// The `AddressStatus` enum represents the address status, used by City of Grants Pass staff.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, PartialEq, Eq, Default)]
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
