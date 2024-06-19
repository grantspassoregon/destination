//! The `geo` module defines spatial address types, and implements traits from the `galileo` crate for these types.
use crate::address_components::StreetSeparator;
use crate::prelude::{
    from_csv, load_bin, save, to_csv, Address, AddressDelta, AddressDeltas, AddressStatus,
    Addresses, CommonAddress, Portable, State, StreetNamePostType, StreetNamePreDirectional,
    StreetNamePreModifier, StreetNamePreType, SubaddressType,
};
use aid::prelude::Clean;
use derive_more::{Deref, DerefMut};
use galileo::galileo_types::cartesian::CartesianPoint2d;
use galileo::galileo_types::geo::GeoPoint;
use galileo::galileo_types::geometry_type::{
    AmbiguousSpace, CartesianSpace2d, GeoSpace2d, GeometryType, PointGeometryType,
};
use indicatif::ParallelProgressIterator;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::path::Path;

/// The `Point` trait is designed to facilitate working with spatial libraries in rust.  This is an
/// intermediary step to implementing traits from geo_types directly, and intended to be replaced
/// by direct impls of types in other crates.  Methods like [`distance`] have more robust
/// implementations in these core spatial libraries, and we should be leaning on those rather than
/// rolling our own.
pub trait Point {
    /// The `x` method returns the cartesian X portion of the projected coordinates of the address.
    fn x(&self) -> f64;
    /// The `y` method returns the cartesian Y portion of the projected coordinates of the address.
    fn y(&self) -> f64;

    /// The `distance` function returns the distance between a point `self` and another point
    /// `other` in the same unit as `self`.
    fn distance<T: Point + ?Sized>(&self, other: &T) -> f64 {
        ((self.y() - other.y()).powi(2) + (self.x() - other.x()).powi(2)).sqrt()
    }

    /// Distance between address and other addresses with matching label.
    /// Iterates through records of `others`, calculates the distance from self
    /// to matching addresses in others, collects the results into a vector and
    /// returns the results in the records field of a new `AddressDeltas` struct.
    fn delta<T: Address + Clone + Point + Sync + Send>(
        &self,
        others: &[T],
        min: f64,
    ) -> AddressDeltas
    where
        Self: Address + Point + Sized + Clone + Send + Sync,
    {
        let records = others
            .par_iter()
            .filter(|v| v.label() == self.label())
            .map(|v| AddressDelta::new(v, v.distance(self)))
            .filter(|d| d.delta > min)
            .collect::<Vec<AddressDelta>>();
        AddressDeltas::new(records)
    }

    /// Distance between addresses and other addresses with matching label.
    /// Iterates through records of `others`, calculates the distance from self
    /// to matching addresses in others, collects the results into a vector and
    /// returns the results in the records field of a new `AddressDeltas` struct. Calls
    /// [`Address::deltas()`].
    fn deltas<
        T: Point + Address + Clone + Sync + Send,
        U: Point + Address + Clone + Sync + Send,
    >(
        values: &[T],
        other: &[U],
        min: f64,
    ) -> AddressDeltas {
        let style = indicatif::ProgressStyle::with_template(
            "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {'Calculating deltas...'}",
        )
        .unwrap();
        let records_raw = values
            .par_iter()
            .progress_with_style(style)
            .map(|v| Point::delta(v, other, min))
            .collect::<Vec<AddressDeltas>>();
        let mut records = Vec::new();
        records_raw
            .iter()
            .map(|v| records.append(&mut v.clone()))
            .for_each(drop);
        AddressDeltas::new(records)
    }
}

/// The `GeoAddress` struct defines a common address that has associated geographic coordinates.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, PartialOrd)]
pub struct GeoAddress {
    /// The `address` field holds a [`CommonAddress`] struct, which defines the fields of a valid address, following the FGDC standard,
    /// with the inclusion of NENA-required fields for emergency response.
    pub address: CommonAddress,
    /// The `latitude` field represents the latitude of the geographic coordinates for the address.
    pub latitude: f64,
    /// The `longitude` field represents the longitude of the geographic coordinates for the address.
    pub longitude: f64,
}

impl Address for GeoAddress {
    fn number(&self) -> i64 {
        self.address.number
    }

    fn number_mut(&mut self) -> &mut i64 {
        &mut self.address.number
    }

    fn number_suffix(&self) -> &Option<String> {
        &self.address.number_suffix
    }

    fn number_suffix_mut(&mut self) -> &mut Option<String> {
        &mut self.address.number_suffix
    }

    fn directional(&self) -> &Option<StreetNamePreDirectional> {
        &self.address.directional
    }

    fn directional_mut(&mut self) -> &mut Option<StreetNamePreDirectional> {
        &mut self.address.directional
    }

    fn street_name_pre_modifier(&self) -> &Option<StreetNamePreModifier> {
        &self.address.pre_modifier
    }

    fn street_name_pre_modifier_mut(&mut self) -> &mut Option<StreetNamePreModifier> {
        &mut self.address.pre_modifier
    }

    fn street_name_pre_type(&self) -> &Option<StreetNamePreType> {
        &self.address.pre_type
    }

    fn street_name_pre_type_mut(&mut self) -> &mut Option<StreetNamePreType> {
        &mut self.address.pre_type
    }

    fn street_name_separator(&self) -> &Option<StreetSeparator> {
        &self.address.separator
    }

    fn street_name_separator_mut(&mut self) -> &mut Option<StreetSeparator> {
        &mut self.address.separator
    }

    fn street_name(&self) -> &String {
        &self.address.street_name
    }

    fn street_name_mut(&mut self) -> &mut String {
        &mut self.address.street_name
    }

    fn street_type(&self) -> &Option<StreetNamePostType> {
        &self.address.street_type
    }

    fn street_type_mut(&mut self) -> &mut Option<StreetNamePostType> {
        &mut self.address.street_type
    }

    fn subaddress_id(&self) -> &Option<String> {
        &self.address.subaddress_id
    }

    fn subaddress_id_mut(&mut self) -> &mut Option<String> {
        &mut self.address.subaddress_id
    }

    fn subaddress_type(&self) -> &Option<SubaddressType> {
        &self.address.subaddress_type
    }

    fn subaddress_type_mut(&mut self) -> &mut Option<SubaddressType> {
        &mut self.address.subaddress_type
    }

    fn floor(&self) -> &Option<i64> {
        &self.address.floor
    }

    fn floor_mut(&mut self) -> &mut Option<i64> {
        &mut self.address.floor
    }

    fn building(&self) -> &Option<String> {
        &self.address.building
    }

    fn building_mut(&mut self) -> &mut Option<String> {
        &mut self.address.building
    }

    fn zip(&self) -> i64 {
        self.address.zip
    }

    fn zip_mut(&mut self) -> &mut i64 {
        &mut self.address.zip
    }

    fn postal_community(&self) -> &String {
        &self.address.postal_community
    }

    fn postal_community_mut(&mut self) -> &mut String {
        &mut self.address.postal_community
    }

    fn state(&self) -> &State {
        &self.address.state
    }

    fn state_mut(&mut self) -> &mut State {
        &mut self.address.state
    }

    fn status(&self) -> &AddressStatus {
        &self.address.status
    }

    fn status_mut(&mut self) -> &mut AddressStatus {
        &mut self.address.status
    }
}

impl GeoPoint for GeoAddress {
    type Num = f64;

    fn lat(&self) -> Self::Num {
        self.latitude
    }

    fn lon(&self) -> Self::Num {
        self.longitude
    }
}

impl GeometryType for GeoAddress {
    type Type = PointGeometryType;
    type Space = GeoSpace2d;
}

impl<T: Address + GeoPoint<Num = f64> + Clone> From<&T> for GeoAddress {
    fn from(data: &T) -> Self {
        let latitude = data.lat();
        let longitude = data.lon();
        let address = CommonAddress::from(data);
        Self {
            address,
            latitude,
            longitude,
        }
    }
}

/// The `GeoAddresses` struct holds a vector of type [`GeoAddress`].
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, PartialOrd, Deref, DerefMut)]
pub struct GeoAddresses(Vec<GeoAddress>);

impl Addresses<GeoAddress> for GeoAddresses {}

impl Portable<GeoAddresses> for GeoAddresses {
    fn load<P: AsRef<Path>>(path: P) -> Clean<Self> {
        let records = load_bin(path)?;
        let decode: Self = bincode::deserialize(&records[..])?;
        Ok(decode)
    }

    fn save<P: AsRef<Path>>(&self, path: P) -> Clean<()> {
        save(self, path)
    }

    fn from_csv<P: AsRef<Path>>(path: P) -> Clean<Self> {
        let records = from_csv(path)?;
        Ok(Self(records))
    }

    fn to_csv<P: AsRef<Path>>(&mut self, path: P) -> Clean<()> {
        Ok(to_csv(&mut self.0, path.as_ref().into())?)
    }
}

impl<T: Address + GeoPoint<Num = f64> + Clone + Sized> From<&[T]> for GeoAddresses {
    fn from(addresses: &[T]) -> Self {
        let records = addresses
            .iter()
            .map(GeoAddress::from)
            .collect::<Vec<GeoAddress>>();
        Self(records)
    }
}

/// The `AddressPoint` struct defines a common address that has associated projected cartesian coordinates.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, PartialOrd)]
pub struct AddressPoint {
    /// The `address` field holds a [`CommonAddress`] struct, which defines the fields of a valid address, following the FGDC standard,
    /// with the inclusion of NENA-required fields for emergency response.
    pub address: CommonAddress,
    /// The `x` field represents the cartesian X portion of the projected coordinates of the
    /// address.
    pub x: f64,
    /// The `y` field represents the cartesian Y portion of the projected coordinates of the
    /// address.
    pub y: f64,
}

impl Address for AddressPoint {
    fn number(&self) -> i64 {
        self.address.number
    }

    fn number_mut(&mut self) -> &mut i64 {
        &mut self.address.number
    }

    fn number_suffix(&self) -> &Option<String> {
        &self.address.number_suffix
    }

    fn number_suffix_mut(&mut self) -> &mut Option<String> {
        &mut self.address.number_suffix
    }

    fn directional(&self) -> &Option<StreetNamePreDirectional> {
        &self.address.directional
    }

    fn directional_mut(&mut self) -> &mut Option<StreetNamePreDirectional> {
        &mut self.address.directional
    }

    fn street_name_pre_modifier(&self) -> &Option<StreetNamePreModifier> {
        &self.address.pre_modifier
    }

    fn street_name_pre_modifier_mut(&mut self) -> &mut Option<StreetNamePreModifier> {
        &mut self.address.pre_modifier
    }

    fn street_name_pre_type(&self) -> &Option<StreetNamePreType> {
        &self.address.pre_type
    }

    fn street_name_pre_type_mut(&mut self) -> &mut Option<StreetNamePreType> {
        &mut self.address.pre_type
    }

    fn street_name_separator(&self) -> &Option<StreetSeparator> {
        &self.address.separator
    }

    fn street_name_separator_mut(&mut self) -> &mut Option<StreetSeparator> {
        &mut self.address.separator
    }

    fn street_name(&self) -> &String {
        &self.address.street_name
    }

    fn street_name_mut(&mut self) -> &mut String {
        &mut self.address.street_name
    }

    fn street_type(&self) -> &Option<StreetNamePostType> {
        &self.address.street_type
    }

    fn street_type_mut(&mut self) -> &mut Option<StreetNamePostType> {
        &mut self.address.street_type
    }

    fn subaddress_id(&self) -> &Option<String> {
        &self.address.subaddress_id
    }

    fn subaddress_id_mut(&mut self) -> &mut Option<String> {
        &mut self.address.subaddress_id
    }

    fn subaddress_type(&self) -> &Option<SubaddressType> {
        &self.address.subaddress_type
    }

    fn subaddress_type_mut(&mut self) -> &mut Option<SubaddressType> {
        &mut self.address.subaddress_type
    }

    fn floor(&self) -> &Option<i64> {
        &self.address.floor
    }

    fn floor_mut(&mut self) -> &mut Option<i64> {
        &mut self.address.floor
    }

    fn building(&self) -> &Option<String> {
        &self.address.building
    }

    fn building_mut(&mut self) -> &mut Option<String> {
        &mut self.address.building
    }

    fn zip(&self) -> i64 {
        self.address.zip
    }

    fn zip_mut(&mut self) -> &mut i64 {
        &mut self.address.zip
    }

    fn postal_community(&self) -> &String {
        &self.address.postal_community
    }

    fn postal_community_mut(&mut self) -> &mut String {
        &mut self.address.postal_community
    }

    fn state(&self) -> &State {
        &self.address.state
    }

    fn state_mut(&mut self) -> &mut State {
        &mut self.address.state
    }

    fn status(&self) -> &AddressStatus {
        &self.address.status
    }

    fn status_mut(&mut self) -> &mut AddressStatus {
        &mut self.address.status
    }
}

impl Point for AddressPoint {
    fn x(&self) -> f64 {
        self.x
    }

    fn y(&self) -> f64 {
        self.y
    }
}

impl CartesianPoint2d for AddressPoint {
    type Num = f64;

    fn x(&self) -> Self::Num {
        self.x
    }

    fn y(&self) -> Self::Num {
        self.y
    }
}

impl GeometryType for AddressPoint {
    type Type = PointGeometryType;
    type Space = CartesianSpace2d;
}

impl<T: Address + Point + Clone> From<&T> for AddressPoint {
    fn from(data: &T) -> Self {
        let address = CommonAddress::from(data);
        let x = data.x();
        let y = data.y();
        Self { address, x, y }
    }
}

/// The `AddressPoints` struct holds a vector of type [`AddressPoint`].
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, PartialOrd, Deref, DerefMut)]
pub struct AddressPoints(Vec<AddressPoint>);

impl Addresses<AddressPoint> for AddressPoints {}

impl Portable<AddressPoints> for AddressPoints {
    fn load<P: AsRef<Path>>(path: P) -> Clean<Self> {
        let records = load_bin(path)?;
        let decode: Self = bincode::deserialize(&records[..])?;
        Ok(decode)
    }

    fn save<P: AsRef<Path>>(&self, path: P) -> Clean<()> {
        save(self, path)
    }

    fn from_csv<P: AsRef<Path>>(path: P) -> Clean<Self> {
        let records = from_csv(path)?;
        Ok(Self(records))
    }

    fn to_csv<P: AsRef<Path>>(&mut self, path: P) -> Clean<()> {
        Ok(to_csv(&mut self.0, path.as_ref().into())?)
    }
}

impl<T: Address + Point + Clone + Sized> From<&[T]> for AddressPoints {
    fn from(addresses: &[T]) -> Self {
        let records = addresses
            .iter()
            .map(AddressPoint::from)
            .collect::<Vec<AddressPoint>>();
        Self(records)
    }
}

/// The `SpatialAddress` struct defines a common address that has both associated geographic coordinates and projected cartesian coordinates.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, PartialOrd)]
pub struct SpatialAddress {
    /// The `address` field holds a [`CommonAddress`] struct, which defines the fields of a valid address, following the FGDC standard,
    /// with the inclusion of NENA-required fields for emergency response.
    pub address: CommonAddress,
    /// The `latitude` field represents the latitude of the geographic coordinates for the address.
    pub latitude: f64,
    /// The `longitude` field represents the longitude of the geographic coordinates for the address.
    pub longitude: f64,
    /// The `x` field represents the cartesian X portion of the projected coordinates of the
    /// address.
    pub x: f64,
    /// The `y` field represents the cartesian Y portion of the projected coordinates of the
    /// address.
    pub y: f64,
}

impl Address for SpatialAddress {
    fn number(&self) -> i64 {
        self.address.number
    }

    fn number_mut(&mut self) -> &mut i64 {
        &mut self.address.number
    }

    fn number_suffix(&self) -> &Option<String> {
        &self.address.number_suffix
    }

    fn number_suffix_mut(&mut self) -> &mut Option<String> {
        &mut self.address.number_suffix
    }

    fn directional(&self) -> &Option<StreetNamePreDirectional> {
        &self.address.directional
    }

    fn directional_mut(&mut self) -> &mut Option<StreetNamePreDirectional> {
        &mut self.address.directional
    }

    fn street_name_pre_modifier(&self) -> &Option<StreetNamePreModifier> {
        &self.address.pre_modifier
    }

    fn street_name_pre_modifier_mut(&mut self) -> &mut Option<StreetNamePreModifier> {
        &mut self.address.pre_modifier
    }

    fn street_name_pre_type(&self) -> &Option<StreetNamePreType> {
        &self.address.pre_type
    }

    fn street_name_pre_type_mut(&mut self) -> &mut Option<StreetNamePreType> {
        &mut self.address.pre_type
    }

    fn street_name_separator(&self) -> &Option<StreetSeparator> {
        &self.address.separator
    }

    fn street_name_separator_mut(&mut self) -> &mut Option<StreetSeparator> {
        &mut self.address.separator
    }

    fn street_name(&self) -> &String {
        &self.address.street_name
    }

    fn street_name_mut(&mut self) -> &mut String {
        &mut self.address.street_name
    }

    fn street_type(&self) -> &Option<StreetNamePostType> {
        &self.address.street_type
    }

    fn street_type_mut(&mut self) -> &mut Option<StreetNamePostType> {
        &mut self.address.street_type
    }

    fn subaddress_id(&self) -> &Option<String> {
        &self.address.subaddress_id
    }

    fn subaddress_id_mut(&mut self) -> &mut Option<String> {
        &mut self.address.subaddress_id
    }

    fn subaddress_type(&self) -> &Option<SubaddressType> {
        &self.address.subaddress_type
    }

    fn subaddress_type_mut(&mut self) -> &mut Option<SubaddressType> {
        &mut self.address.subaddress_type
    }

    fn floor(&self) -> &Option<i64> {
        &self.address.floor
    }

    fn floor_mut(&mut self) -> &mut Option<i64> {
        &mut self.address.floor
    }

    fn building(&self) -> &Option<String> {
        &self.address.building
    }

    fn building_mut(&mut self) -> &mut Option<String> {
        &mut self.address.building
    }

    fn zip(&self) -> i64 {
        self.address.zip
    }

    fn zip_mut(&mut self) -> &mut i64 {
        &mut self.address.zip
    }

    fn postal_community(&self) -> &String {
        &self.address.postal_community
    }

    fn postal_community_mut(&mut self) -> &mut String {
        &mut self.address.postal_community
    }

    fn state(&self) -> &State {
        &self.address.state
    }

    fn state_mut(&mut self) -> &mut State {
        &mut self.address.state
    }

    fn status(&self) -> &AddressStatus {
        &self.address.status
    }

    fn status_mut(&mut self) -> &mut AddressStatus {
        &mut self.address.status
    }
}

impl GeoPoint for SpatialAddress {
    type Num = f64;
    fn lat(&self) -> Self::Num {
        self.latitude
    }

    fn lon(&self) -> Self::Num {
        self.longitude
    }
}

impl Point for SpatialAddress {
    fn x(&self) -> f64 {
        self.x
    }

    fn y(&self) -> f64 {
        self.y
    }
}

impl CartesianPoint2d for SpatialAddress {
    type Num = f64;

    fn x(&self) -> Self::Num {
        self.x
    }

    fn y(&self) -> Self::Num {
        self.y
    }
}

impl GeometryType for SpatialAddress {
    type Type = PointGeometryType;
    type Space = AmbiguousSpace;
}

impl<T: Address + Point + GeoPoint<Num = f64> + Clone> From<&T> for SpatialAddress {
    fn from(data: &T) -> Self {
        let latitude = data.lat();
        let longitude = data.lon();
        let address = CommonAddress::from(data);
        let x = data.x();
        let y = data.y();
        Self {
            address,
            latitude,
            longitude,
            x,
            y,
        }
    }
}

/// The `SpatialAddresses` struct holds a vector of type [`SpatialAddress`].
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, PartialOrd, Deref, DerefMut)]
pub struct SpatialAddresses(Vec<SpatialAddress>);

impl Addresses<SpatialAddress> for SpatialAddresses {}

impl Portable<SpatialAddresses> for SpatialAddresses {
    fn load<P: AsRef<Path>>(path: P) -> Clean<Self> {
        let records = load_bin(path)?;
        let decode: Self = bincode::deserialize(&records[..])?;
        Ok(decode)
    }

    fn save<P: AsRef<Path>>(&self, path: P) -> Clean<()> {
        save(self, path)
    }

    fn from_csv<P: AsRef<Path>>(path: P) -> Clean<Self> {
        let records = from_csv(path)?;
        Ok(Self(records))
    }

    fn to_csv<P: AsRef<Path>>(&mut self, path: P) -> Clean<()> {
        Ok(to_csv(&mut self.0, path.as_ref().into())?)
    }
}

impl<T: Address + Point + GeoPoint<Num = f64> + Clone + Sized> From<&[T]> for SpatialAddresses {
    fn from(addresses: &[T]) -> Self {
        let records = addresses
            .iter()
            .map(SpatialAddress::from)
            .collect::<Vec<SpatialAddress>>();
        Self(records)
    }
}
