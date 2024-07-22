//! The `lexisnexis` module produces address range reports for the LexisNexis dispatch service.
use crate::prelude::{from_csv, load_bin, save, to_csv, Address, Addresses, Portable};
use aid::prelude::*;
use derive_more::{Deref, DerefMut};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::path::Path;
use tracing::warn;

/// The `LexisNexisItemBuilder` struct provides a framework to create and modify the required fields in the LexisNexis spreadsheet.
#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub struct LexisNexisItemBuilder {
    /// The `address_number_from` field represents the lower bound on the address number range for
    /// the row.
    pub address_number_from: Option<i64>,
    /// The `address_number_to` field represents the upper bound on the address number range for
    /// the row.
    pub address_number_to: Option<i64>,
    /// The `street_name_pre_directional` represents the street name pre-directional using the
    /// standard postal abbreviation.
    pub street_name_pre_directional: Option<String>,
    /// The `street_name` field represents the street name.
    pub street_name: Option<String>,
    /// The `street_name_post_type` field represents the street name post type.
    pub street_name_post_type: Option<String>,
    /// The `street_name_post_directional` field represents the street name post-directional.
    /// Grants Pass does not use street name post directional designations.
    pub street_name_post_directional: Option<String>,
    /// The `postal_community` field represents the city or postal community in an address.
    pub postal_community: Option<String>,
    /// The `beat` field is a required field in LexisNexis, but not used by the city.
    pub beat: Option<String>,
    /// The `area` field is a required field in LexisNexis, but not used by the city.
    pub area: Option<String>,
    /// The `district` field is a required field in LexisNexis, but not used by the city.
    pub district: Option<String>,
    /// The `zone` field is a required field in LexisNexis, but not used by the city.
    pub zone: Option<String>,
    /// The `zip_code` field represents the 5-digit postal zip code for addresses.
    pub zip_code: Option<i64>,
    /// The `commonplace` field is a required field in LexisNexis, but not used by the city.
    pub commonplace: Option<String>,
    /// The `address_number` field is a required field in LexisNexis, but not used by the city.
    pub address_number: Option<i64>,
}

impl LexisNexisItemBuilder {
    /// Creates a new `LexisNexisItemBuilder`, with fields initialized to default values.  Because
    /// of the number of fields in the [`LexisNexisItem`] struct, we use a builder to initialize a
    /// struct with default values, and then modify the values of the fields before calling
    /// *build*.
    pub fn new() -> Self {
        Self::default()
    }

    /// The `build` method converts a `LexisNexisItemBuilder` into a [`LexisNexisItem`].  Returns
    /// an error if a required field is missing, or set to None when a value is required.
    pub fn build(self) -> Clean<LexisNexisItem> {
        if let Some(address_number_from) = self.address_number_from {
            if let Some(address_number_to) = self.address_number_to {
                if let Some(street_name) = self.street_name {
                    if let Some(street_name_post_type) = self.street_name_post_type {
                        if let Some(postal_community) = self.postal_community {
                            if let Some(zip_code) = self.zip_code {
                                Ok(LexisNexisItem {
                                    address_number_from,
                                    address_number_to,
                                    street_name_pre_directional: self.street_name_pre_directional,
                                    street_name,
                                    street_name_post_type,
                                    street_name_post_directional: self.street_name_post_directional,
                                    postal_community,
                                    beat: self.beat,
                                    area: self.area,
                                    district: self.district,
                                    zone: self.zone,
                                    zip_code,
                                    commonplace: self.commonplace,
                                    address_number: self.address_number,
                                    id: uuid::Uuid::new_v4(),
                                })
                            } else {
                                warn!("Zip code missing.");
                                Err(Bandage::Unknown)
                            }
                        } else {
                            warn!("Postal community missing.");
                            Err(Bandage::Unknown)
                        }
                    } else {
                        warn!("Street name post type missing.");
                        Err(Bandage::Unknown)
                    }
                } else {
                    warn!("Missing street name.");
                    Err(Bandage::Unknown)
                }
            } else {
                warn!("Missing address number to.");
                Err(Bandage::Unknown)
            }
        } else {
            warn!("Missing address number from.");
            Err(Bandage::Unknown)
        }
    }
}

/// The `LexisNexisItem` struct contains the required fields in the LexisNexis spreadsheet.
#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub struct LexisNexisItem {
    /// The `address_number_from` field represents the lower range of address numbers associated
    /// with the service area.
    #[serde(rename(serialize = "StNumFrom"))]
    pub address_number_from: i64,
    /// The `address_number_to` field represents the upper range of address numbers associated
    /// with the service area.
    #[serde(rename(serialize = "StNumTo"))]
    pub address_number_to: i64,
    /// The `street_name_pre_directional` field represents the street name pre directional
    /// associated with the service area.
    #[serde(rename(serialize = "StPreDirection"))]
    pub street_name_pre_directional: Option<String>,
    /// The `street_name` field represents the street name component of the complete street name
    /// associated with the service area.
    #[serde(rename(serialize = "StName"))]
    pub street_name: String,
    /// The `street_name_post_type` field represents the street name post type component of the
    /// complete street name associated with the service area.
    #[serde(rename(serialize = "StType"))]
    pub street_name_post_type: String,
    /// The `street_name_post_directional` field represents the street name post directional component of
    /// the complete street name.  The City of Grants Pass does not issue addresses using a street
    /// name post directional component, but Josephine County does have some examples in their
    /// records.
    #[serde(rename(serialize = "StPostDirection"))]
    pub street_name_post_directional: Option<String>,
    /// The `postal_community` field represents either the unincorporated or incorporated
    /// municipality name associated with the service area.
    #[serde(rename(serialize = "City"))]
    pub postal_community: String,
    /// The `beat` field represents the police response jurisdiction associated with the service
    /// area.  The City of Grants Pass does not use this field directly, but its presence is a
    /// requirement of the LexisNexis schema.
    #[serde(rename(serialize = "Beat"))]
    pub beat: Option<String>,
    /// The `area` field represents the service
    /// area.  The City of Grants Pass does not use this field directly, but its presence is a
    /// requirement of the LexisNexis schema.
    #[serde(rename(serialize = "Area"))]
    pub area: Option<String>,
    /// The `district` field represents the service
    /// district.  The City of Grants Pass does not use this field directly, but its presence is a
    /// requirement of the LexisNexis schema.
    #[serde(rename(serialize = "District"))]
    pub district: Option<String>,
    /// The `zone` field represents the service
    /// zone.  The City of Grants Pass does not use this field directly, but its presence is a
    /// requirement of the LexisNexis schema.
    #[serde(rename(serialize = "Zone"))]
    pub zone: Option<String>,
    /// The `zip_code` field represents the postal zip code associated with the service area.
    #[serde(rename(serialize = "Zipcode"))]
    pub zip_code: i64,
    /// The `commonplace` field represents a common name associated with the service area.  The
    /// City of Grants Pass does not use this field directly, but its presence is a requirement of
    /// the LexisNexis schema.
    #[serde(rename(serialize = "CommonPlace"))]
    pub commonplace: Option<String>,
    /// The `address_number` field may possibly serve to represent a service area with an address
    /// range of one, but the City of Grants Pass reports these ranges using a single value for the
    /// _from and _to fields, so this field is currently unused.  Its presence is a requirement of
    /// the LexisNexis schema.
    #[serde(rename(serialize = "StNum"))]
    pub address_number: Option<i64>,
    /// The `id` field is an internal unique id.
    #[serde(skip_serializing)]
    pub id: uuid::Uuid,
}

/// The `LexisNexis` struct holds a vector of [`LexisNexisItem`] objects, for serialization into a
/// .csv file.
#[derive(
    Default,
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Deserialize,
    Serialize,
    Deref,
    DerefMut,
)]
pub struct LexisNexis(Vec<LexisNexisItem>);

impl LexisNexis {
    /// The `from_addresses` method creates a [`LexisNexis`] struct from a set of addresses to
    /// include in the range selection `include`, and a set of addresses to exclude from the range
    /// selection `exclude`.
    pub fn from_addresses<T: Address + Clone + Send + Sync, U: Addresses<T>>(
        include: &U,
        exclude: &U,
    ) -> Clean<LexisNexis> {
        // List of unique street names processed so far.
        let mut seen = HashSet::new();
        // Vector to hold Lexis Nexis results.
        let mut records = Vec::new();
        // For each address in the inclusion list...
        for address in include.iter() {
            // Get the complete street name.
            let comp_street = address.complete_street_name(false);
            // Get the street name element.
            let street = address.street_name().clone();
            // Get the street name post type element.
            let post_type = address.street_type();
            // If comp_street is a new street name...
            if !seen.contains(&comp_street) {
                // Add the new name to the list of seen names.
                seen.insert(comp_street.clone());
                // Obtain mutable clone of include group.
                let mut inc = include.clone();
                // Filter include group by current street name.
                inc.filter_field("street_name", &street);
                // Obtain mutable clone of exclude group.
                let mut exl = exclude.clone();
                // Filter exclude group by current street name.
                exl.filter_field("street_name", &street);
                tracing::trace!(
                    "After street name filter, inc: {}, exl: {}",
                    inc.len(),
                    exl.len()
                );
                if let Some(post) = post_type {
                    inc.filter_field("post_type", &post.to_string());
                    exl.filter_field("post_type", &post.to_string());
                } else {
                    inc.filter_field("post_type", "None");
                    exl.filter_field("post_type", "None");
                }
                tracing::trace!(
                    "After post type filter, inc: {}, exl: {}",
                    inc.len(),
                    exl.len()
                );
                if let Some(directional) = address.directional() {
                    inc.filter_field("pre_directional", &directional.to_string());
                    exl.filter_field("pre_directional", &directional.to_string());
                } else {
                    inc.filter_field("pre_directional", "None");
                    exl.filter_field("pre_directional", "None");
                }
                tracing::trace!(
                    "After post directional filter, inc: {}, exl: {}",
                    inc.len(),
                    exl.len()
                );
                let items = LexisNexisRange::from_addresses(&inc, &exl);
                let ranges = items.ranges();
                for rng in ranges {
                    let mut builder = LexisNexisItemBuilder::new();
                    builder.address_number_from = Some(rng.0);
                    builder.address_number_to = Some(rng.1);
                    builder.street_name_pre_directional = address.directional_abbreviated();
                    builder.street_name = Some(address.street_name().clone());
                    if let Some(street_type) = address.street_type() {
                        builder.street_name_post_type = Some(street_type.abbreviate());
                    }
                    builder.postal_community = Some(address.postal_community().clone());
                    builder.zip_code = Some(address.zip());
                    if let Ok(built) = builder.build() {
                        records.push(built);
                    }
                }
            }
        }
        Ok(LexisNexis(records))
    }
}

impl Portable<LexisNexis> for LexisNexis {
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

/// The `LexisNexisRangeItem` represents an address number `num`, and whether to include the number
/// in the range selection.
#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub struct LexisNexisRangeItem {
    /// The `num` field represents an address number observation.
    pub num: i64,
    /// The `include` field represents whether to include the number in the range selection.
    pub include: bool,
}

impl LexisNexisRangeItem {
    /// Creates a new `LexisNexisRangeItem` from an address number `num` and a boolean `include` indicating
    /// whether to include the address number in the range.
    pub fn new(num: i64, include: bool) -> Self {
        Self { num, include }
    }
}

/// The `LexisNexisRange` struct holds a vector of address number observations associated with a given complete
/// street name.  The `include` field is *true* for addresses within the city limits or with a public
/// safety agreement, and *false* for addresses outside of city limits or without a public safety
/// agreement.  Used to produce valid ranges of addresses in the city service area.
#[derive(
    Default,
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Deserialize,
    Serialize,
    Deref,
    DerefMut,
)]
pub struct LexisNexisRange(Vec<LexisNexisRangeItem>);

impl LexisNexisRange {
    /// The `from_addresses` method creates a [`LexisNexisRange`] from a set of addresses to
    /// include in the range selection `include`, and a set of addresses to exclude from the range
    /// selection `exclude`.
    pub fn from_addresses<T: Address + Clone + Send + Sync, U: Addresses<T>>(
        include: &U,
        exclude: &U,
    ) -> Self {
        let mut records = include
            .iter()
            .map(|v| LexisNexisRangeItem::new(v.number(), true))
            .collect::<Vec<LexisNexisRangeItem>>();
        records.extend(
            exclude
                .iter()
                .map(|v| LexisNexisRangeItem::new(v.number(), false))
                .collect::<Vec<LexisNexisRangeItem>>(),
        );
        records.sort_by_key(|v| v.num);
        // tracing::info!("Record: {:#?}", &records);
        Self(records)
    }

    /// The `ranges` method returns the ranges of addresses within the service area, as marked by
    /// the `include` field.
    pub fn ranges(&self) -> Vec<(i64, i64)> {
        let mut rngs = Vec::new();
        let mut min = 0;
        let mut max = 0;
        let mut open = false;
        for item in self.iter() {
            if item.include {
                if !open {
                    open = true;
                    min = item.num;
                }
                max = item.num;
            } else if open {
                open = false;
                rngs.push((min, max));
            }
        }
        if open {
            rngs.push((min, max));
        }
        rngs
    }
}
