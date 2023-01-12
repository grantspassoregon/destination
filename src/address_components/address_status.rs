use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub enum AddressStatus {
    Current,
    Pending,
    Retired,
    Temporary,
    Other,
}
