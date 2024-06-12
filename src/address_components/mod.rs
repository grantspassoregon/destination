//! The `address_components` module holds type definitions and methods for address component
//! elements, as defined by FGDC guidelines.
mod address_status;
mod floor;
mod street_name_post_type;
mod street_name_pre_directional;
mod subaddress_type;

pub use address_status::AddressStatus;
pub use floor::zero_floor;
pub use street_name_post_type::{
    deserialize_abbreviated_post_type, deserialize_mixed_post_type, match_mixed_post_type,
    StreetNamePostType,
};
pub use street_name_pre_directional::{
    deserialize_abbreviated_pre_directional, deserialize_mixed_pre_directional,
    match_mixed_pre_directional, StreetNamePreDirectional,
};
pub use subaddress_type::{
    deserialize_abbreviated_subaddress_type, deserialize_mixed_subaddress_type,
    match_mixed_subaddress_type, SubaddressType,
};
