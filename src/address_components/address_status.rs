use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Deserialize, Serialize, PartialEq, Eq, Default)]
pub enum AddressStatus {
    Current,
    Pending,
    Retired,
    Temporary,
    #[default]
    Other,
}
