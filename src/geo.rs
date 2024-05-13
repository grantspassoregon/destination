use crate::prelude::{
    from_csv, load_bin, save, to_csv, Address, AddressDelta, AddressDeltas, AddressStatus,
    Addresses, CommonAddress, Portable, StreetNamePostType, StreetNamePreDirectional,
    SubaddressType, Vectorized,
};
use aid::prelude::Clean;
use galileo::galileo_types::cartesian::CartesianPoint2d;
use galileo::galileo_types::geo::GeoPoint;
use galileo::galileo_types::geometry_type::{
    AmbiguousSpace, CartesianSpace2d, GeoSpace2d, GeometryType, PointGeometryType,
};
use indicatif::ParallelProgressIterator;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::path::Path;

pub trait Point {
    fn x(&self) -> f64;
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
        AddressDeltas { records }
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
            .map(|v| records.append(&mut v.values().clone()))
            .for_each(drop);
        AddressDeltas { records }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, PartialOrd)]
pub struct GeoAddress {
    pub address: CommonAddress,
    pub latitude: f64,
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

    fn street_name_pre_modifier(&self) -> &Option<String> {
        &self.address.pre_modifier
    }

    fn street_name_pre_modifier_mut(&mut self) -> &mut Option<String> {
        &mut self.address.pre_modifier
    }

    fn street_name_pre_type(&self) -> &Option<String> {
        &self.address.pre_type
    }

    fn street_name_pre_type_mut(&mut self) -> &mut Option<String> {
        &mut self.address.pre_type
    }

    fn street_name_separator(&self) -> &Option<String> {
        &self.address.separator
    }

    fn street_name_separator_mut(&mut self) -> &mut Option<String> {
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

    fn state(&self) -> &String {
        &self.address.state
    }

    fn state_mut(&mut self) -> &mut String {
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

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, PartialOrd)]
pub struct GeoAddresses {
    pub records: Vec<GeoAddress>,
}

impl Addresses<GeoAddress> for GeoAddresses {}

impl Vectorized<GeoAddress> for GeoAddresses {
    fn values(&self) -> &Vec<GeoAddress> {
        &self.records
    }

    fn values_mut(&mut self) -> &mut Vec<GeoAddress> {
        &mut self.records
    }

    fn into_values(self) -> Vec<GeoAddress> {
        self.records
    }
}

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
        Ok(Self { records })
    }

    fn to_csv<P: AsRef<Path>>(&mut self, path: P) -> Clean<()> {
        Ok(to_csv(&mut self.records, path.as_ref().into())?)
    }
}

impl<T: Address + GeoPoint<Num = f64> + Clone + Sized> From<&[T]> for GeoAddresses {
    fn from(addresses: &[T]) -> Self {
        let records = addresses
            .iter()
            .map(GeoAddress::from)
            .collect::<Vec<GeoAddress>>();
        Self { records }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, PartialOrd)]
pub struct AddressPoint {
    pub address: CommonAddress,
    pub x: f64,
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

    fn street_name_pre_modifier(&self) -> &Option<String> {
        &self.address.pre_modifier
    }

    fn street_name_pre_modifier_mut(&mut self) -> &mut Option<String> {
        &mut self.address.pre_modifier
    }

    fn street_name_pre_type(&self) -> &Option<String> {
        &self.address.pre_type
    }

    fn street_name_pre_type_mut(&mut self) -> &mut Option<String> {
        &mut self.address.pre_type
    }

    fn street_name_separator(&self) -> &Option<String> {
        &self.address.separator
    }

    fn street_name_separator_mut(&mut self) -> &mut Option<String> {
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

    fn state(&self) -> &String {
        &self.address.state
    }

    fn state_mut(&mut self) -> &mut String {
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

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, PartialOrd)]
pub struct AddressPoints {
    pub records: Vec<AddressPoint>,
}

impl Addresses<AddressPoint> for AddressPoints {}

impl Vectorized<AddressPoint> for AddressPoints {
    fn values(&self) -> &Vec<AddressPoint> {
        &self.records
    }

    fn values_mut(&mut self) -> &mut Vec<AddressPoint> {
        &mut self.records
    }

    fn into_values(self) -> Vec<AddressPoint> {
        self.records
    }
}

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
        Ok(Self { records })
    }

    fn to_csv<P: AsRef<Path>>(&mut self, path: P) -> Clean<()> {
        Ok(to_csv(&mut self.records, path.as_ref().into())?)
    }
}

impl<T: Address + Point + Clone + Sized> From<&[T]> for AddressPoints {
    fn from(addresses: &[T]) -> Self {
        let records = addresses
            .iter()
            .map(AddressPoint::from)
            .collect::<Vec<AddressPoint>>();
        Self { records }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, PartialOrd)]
pub struct SpatialAddress {
    pub address: CommonAddress,
    pub latitude: f64,
    pub longitude: f64,
    pub x: f64,
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

    fn street_name_pre_modifier(&self) -> &Option<String> {
        &self.address.pre_modifier
    }

    fn street_name_pre_modifier_mut(&mut self) -> &mut Option<String> {
        &mut self.address.pre_modifier
    }

    fn street_name_pre_type(&self) -> &Option<String> {
        &self.address.pre_type
    }

    fn street_name_pre_type_mut(&mut self) -> &mut Option<String> {
        &mut self.address.pre_type
    }

    fn street_name_separator(&self) -> &Option<String> {
        &self.address.separator
    }

    fn street_name_separator_mut(&mut self) -> &mut Option<String> {
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

    fn state(&self) -> &String {
        &self.address.state
    }

    fn state_mut(&mut self) -> &mut String {
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

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, PartialOrd)]
pub struct SpatialAddresses {
    pub records: Vec<SpatialAddress>,
}

impl Addresses<SpatialAddress> for SpatialAddresses {}

impl Vectorized<SpatialAddress> for SpatialAddresses {
    fn values(&self) -> &Vec<SpatialAddress> {
        &self.records
    }

    fn values_mut(&mut self) -> &mut Vec<SpatialAddress> {
        &mut self.records
    }

    fn into_values(self) -> Vec<SpatialAddress> {
        self.records
    }
}

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
        Ok(Self { records })
    }

    fn to_csv<P: AsRef<Path>>(&mut self, path: P) -> Clean<()> {
        Ok(to_csv(&mut self.records, path.as_ref().into())?)
    }
}

impl<T: Address + Point + GeoPoint<Num = f64> + Clone + Sized> From<&[T]> for SpatialAddresses {
    fn from(addresses: &[T]) -> Self {
        let records = addresses
            .iter()
            .map(SpatialAddress::from)
            .collect::<Vec<SpatialAddress>>();
        Self { records }
    }
}
