//! The `import` module contains data types for importing addresses from different agencies.
mod fire_inspection;
mod grants_pass;
mod grants_pass_business;
mod josephine_county;

pub use fire_inspection::{FireInspection, FireInspectionRaw, FireInspections};
pub use grants_pass::{
    GrantsPassAddress, GrantsPassAddresses, GrantsPassSpatialAddress, GrantsPassSpatialAddresses,
};
pub use grants_pass_business::{Business, Businesses};
pub use josephine_county::{
    JosephineCountyAddress, JosephineCountyAddress2024, JosephineCountyAddresses,
    JosephineCountyAddresses2024, JosephineCountySpatialAddress, JosephineCountySpatialAddress2024,
    JosephineCountySpatialAddresses, JosephineCountySpatialAddresses2024,
};
