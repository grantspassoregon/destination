use crate::{
    deserialize_arcgis_data, from_bin, from_csv, to_bin, to_csv, AddressError, AddressErrorKind,
    AddressStatus, CommonAddress, CommonAddresses, GeoAddress, GeoAddresses, IntoBin, IntoCsv, Io,
    SpatialAddress, SpatialAddresses, State, StreetNamePostType, StreetNamePreDirectional,
    StreetNamePreModifier, StreetNamePreType, StreetSeparator, SubaddressType,
};
/// The `SpatialAddressRaw` struct defines the fields of a valid address, following the FGDC standard,
/// with the inclusion of NENA-required fields for emergency response.
#[derive(Debug, Default, Clone, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize)]
pub struct SpatialAddressRaw {
    /// The `number` field represents the address number component of the complete address
    /// number.
    pub number: i64,
    /// The `number_suffix` field represents the address number suffix component of the complete
    /// address number.
    #[serde(deserialize_with = "deserialize_arcgis_data")]
    pub number_suffix: Option<String>,
    /// The `directional` field represents the street name pre directional component of the
    /// complete street name.
    #[serde(deserialize_with = "StreetNamePreDirectional::deserialize_mixed")]
    pub directional: Option<StreetNamePreDirectional>,
    /// The `pre_modifier` field represents the street name pre modifier component of the complete
    /// street name.
    #[serde(deserialize_with = "StreetNamePreModifier::deserialize_mixed")]
    pub pre_modifier: Option<StreetNamePreModifier>,
    /// The `pre_type` field represents the street name pre type component of the complete street
    /// name.
    #[serde(deserialize_with = "StreetNamePreType::deserialize_mixed")]
    pub pre_type: Option<StreetNamePreType>,
    /// The `separator` field represents the separator element component of the complete street
    /// name.
    #[serde(deserialize_with = "StreetSeparator::deserialize_mixed")]
    pub separator: Option<StreetSeparator>,
    /// The `street_name` field represents the street name component of the complete street name.
    pub street_name: String,
    /// The `street_type` field represents the street name post type component of the complete street
    /// name.
    #[serde(deserialize_with = "StreetNamePostType::deserialize_mixed")]
    pub street_type: Option<StreetNamePostType>,
    /// The `subaddress_type` field represents the subaddress type component of the complete
    /// subaddress.
    #[serde(deserialize_with = "SubaddressType::deserialize_mixed")]
    pub subaddress_type: Option<SubaddressType>,
    /// The `subaddress_id` field represents the subaddress identifier component of the complete
    /// subaddress.
    #[serde(deserialize_with = "deserialize_arcgis_data")]
    pub subaddress_id: Option<String>,
    /// The `floor` field represents the floor identifier, corresponding to the `Floor` field from the NENA standard.
    #[serde(deserialize_with = "csv::invalid_option")]
    pub floor: Option<i64>,
    /// The `building` field represents the building identifier, corresponding to the `Building` field from the NENA standard.
    #[serde(deserialize_with = "deserialize_arcgis_data")]
    pub building: Option<String>,
    /// The `zip` field represents the postal zip code of the address.
    pub zip: i64,
    /// The `postal_community` field represents the postal community component of the address,
    /// being either the unincorporated or incorporated municipality name.
    pub postal_community: String,
    /// The `state` field represents the state name component of the address.
    #[serde(deserialize_with = "State::deserialize_mixed")]
    pub state: State,
    /// The `status` field represents the local status of the address as determined by the relevant
    /// addressing authority.
    pub status: AddressStatus,
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

impl From<SpatialAddressRaw> for CommonAddress {
    fn from(value: SpatialAddressRaw) -> Self {
        Self {
            number: value.number,
            number_suffix: value.number_suffix,
            directional: value.directional,
            pre_modifier: value.pre_modifier,
            pre_type: value.pre_type,
            separator: value.separator,
            street_name: value.street_name,
            street_type: value.street_type,
            subaddress_type: value.subaddress_type,
            subaddress_id: value.subaddress_id,
            floor: value.floor,
            building: value.building,
            zip: value.zip,
            postal_community: value.postal_community,
            state: value.state,
            status: value.status,
        }
    }
}

impl From<SpatialAddressRaw> for GeoAddress {
    fn from(value: SpatialAddressRaw) -> Self {
        let address = CommonAddress::from(value.clone());
        Self {
            address,
            longitude: value.longitude,
            latitude: value.latitude,
        }
    }
}

impl From<SpatialAddressRaw> for SpatialAddress {
    fn from(value: SpatialAddressRaw) -> Self {
        let address = CommonAddress::from(value.clone());
        Self {
            address,
            longitude: value.longitude,
            latitude: value.latitude,
            x: value.x,
            y: value.y,
        }
    }
}

/// The `SpatialAddressesRaw` struct holds a vector of type [`SpatialAddressRaw`].
#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    PartialOrd,
    serde::Serialize,
    serde::Deserialize,
    derive_more::Deref,
    derive_more::DerefMut,
)]
pub struct SpatialAddressesRaw(Vec<SpatialAddressRaw>);

impl From<SpatialAddressesRaw> for CommonAddresses {
    fn from(value: SpatialAddressesRaw) -> Self {
        let raw = value
            .iter()
            .map(|x| CommonAddress::from(x.clone()))
            .collect::<Vec<CommonAddress>>();
        Self::new(raw)
    }
}

impl From<SpatialAddressesRaw> for GeoAddresses {
    fn from(value: SpatialAddressesRaw) -> Self {
        let raw = value
            .iter()
            .map(|x| GeoAddress::from(x.clone()))
            .collect::<Vec<GeoAddress>>();
        Self::new(raw)
    }
}

impl From<SpatialAddressesRaw> for SpatialAddresses {
    fn from(value: SpatialAddressesRaw) -> Self {
        let raw = value
            .iter()
            .map(|x| SpatialAddress::from(x.clone()))
            .collect::<Vec<SpatialAddress>>();
        Self::new(raw)
    }
}

impl IntoBin<SpatialAddressesRaw> for SpatialAddressesRaw {
    fn load<P: AsRef<std::path::Path>>(path: P) -> Result<Self, AddressError> {
        match from_bin(path) {
            Ok(records) => {
                let decode: Self = bincode::deserialize(&records)?;
                Ok(decode)
            }
            Err(source) => Err(AddressErrorKind::from(source).into()),
        }
    }

    fn save<P: AsRef<std::path::Path>>(&self, path: P) -> Result<(), AddressError> {
        to_bin(self, path)
    }
}

impl IntoCsv<SpatialAddressesRaw> for SpatialAddressesRaw {
    fn from_csv<P: AsRef<std::path::Path>>(path: P) -> Result<Self, Io> {
        let records = from_csv(path)?;
        Ok(Self(records))
    }

    fn to_csv<P: AsRef<std::path::Path>>(&mut self, path: P) -> Result<(), AddressErrorKind> {
        to_csv(&mut self.0, path.as_ref().into())
    }
}
