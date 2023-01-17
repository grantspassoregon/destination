use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub enum AddressStatus {
    Current,
    Pending,
    Retired,
    Temporary,
    Other,
}
