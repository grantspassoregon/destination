//! The `lexisnexis` module produces address range reports for the LexisNexis dispatch service.
use crate::prelude::*;
use aid::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use tracing::warn;

/// The `LexisNexisItemBuilder` struct provides a framework to create and modify the required fields in the LexisNexis spreadsheet.
#[derive(Default, Debug, Clone)]
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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LexisNexisItem {
    #[serde(rename(serialize = "StNumFrom"))]
    address_number_from: i64,
    #[serde(rename(serialize = "StNumTo"))]
    address_number_to: i64,
    #[serde(rename(serialize = "StPreDirection"))]
    street_name_pre_directional: Option<String>,
    #[serde(rename(serialize = "StName"))]
    street_name: String,
    #[serde(rename(serialize = "StType"))]
    street_name_post_type: String,
    #[serde(rename(serialize = "StPostDirection"))]
    street_name_post_directional: Option<String>,
    #[serde(rename(serialize = "City"))]
    postal_community: String,
    #[serde(rename(serialize = "Beat"))]
    beat: Option<String>,
    #[serde(rename(serialize = "Area"))]
    area: Option<String>,
    #[serde(rename(serialize = "District"))]
    district: Option<String>,
    #[serde(rename(serialize = "Zone"))]
    zone: Option<String>,
    #[serde(rename(serialize = "Zipcode"))]
    zip_code: i64,
    #[serde(rename(serialize = "CommonPlace"))]
    commonplace: Option<String>,
    #[serde(rename(serialize = "StNum"))]
    address_number: Option<i64>,
}

/// The `LexisNexis` struct holds a vector of [`LexisNexisItem`] objects, for serialization into a
/// .csv file.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LexisNexis {
    /// The `records` field holds a vector of type [`LexisNexisItem`].
    pub records: Vec<LexisNexisItem>,
}

impl LexisNexis {
    /// The `from_addresses` method creates a [`LexisNexis`] struct from a set of addresses to
    /// include in the range selection `include`, and a set of addresses to exclude from the range
    /// selection `exclude`.
    pub fn from_addresses(
        include: &CommonAddresses,
        exclude: &CommonAddresses,
    ) -> Clean<LexisNexis> {
        let mut seen = HashSet::new();
        let mut records = Vec::new();
        for address in include.records_ref() {
            let comp_street = address.complete_street_name();
            let street = address.street_name.clone();
            let post_type = address.street_type;
            if !seen.contains(&comp_street) {
                seen.insert(comp_street.clone());
                let mut inc = include.filter_field("street_name", &street);
                let mut exl = exclude.filter_field("street_name", &street);
                inc = inc.filter_field("post_type", &format!("{:?}", post_type));
                exl = exl.filter_field("post_type", &format!("{:?}", post_type));
                inc = inc.filter_field(
                    "pre_directional",
                    &format!("{:?}", address.pre_directional()),
                );
                exl = exl.filter_field(
                    "pre_directional",
                    &format!("{:?}", address.pre_directional()),
                );
                let items = LexisNexisRange::from_addresses(&inc, &exl);
                let ranges = items.ranges();
                for rng in ranges {
                    let mut builder = LexisNexisItemBuilder::new();
                    builder.address_number_from = Some(rng.0);
                    builder.address_number_to = Some(rng.1);
                    builder.street_name_pre_directional = address.pre_directional_abbreviated();
                    builder.street_name = Some(address.street_name.clone());
                    if let Some(street_type) = address.street_type {
                        builder.street_name_post_type = Some(street_type.abbreviate());
                    }
                    builder.postal_community = Some(address.postal_community());
                    builder.zip_code = Some(address.zip_code());
                    if let Ok(built) = builder.build() {
                        records.push(built);
                    }
                }
            }
        }
        Ok(LexisNexis { records })
    }

    /// Writes the contents of `LexisNexis` to a CSV file output to path `title`.  Each element
    /// of the vector in `records` writes to a row on the CSV file.
    pub fn to_csv(&mut self, title: std::path::PathBuf) -> Result<(), std::io::Error> {
        to_csv(&mut self.records, title)?;
        Ok(())
    }
}

/// The `LexisNexisRangeItem` represents an address number `num`, and whether to include the number
/// in the range selection.
#[derive(Debug, Clone)]
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
#[derive(Debug, Clone)]
pub struct LexisNexisRange {
    /// The `records` field holds a vector of type [`LexisNexisRangeItem`].
    pub records: Vec<LexisNexisRangeItem>,
}

impl LexisNexisRange {
    /// The `from_addresses` method creates a [`LexisNexisRange`] from a set of addresses to
    /// include in the range selection `include`, and a set of addresses to exclude from the range
    /// selection `exclude`.
    pub fn from_addresses(include: &CommonAddresses, exclude: &CommonAddresses) -> Self {
        let mut records = include
            .records_ref()
            .iter()
            .map(|v| LexisNexisRangeItem::new(v.number, true))
            .collect::<Vec<LexisNexisRangeItem>>();
        records.extend(
            exclude
                .records_ref()
                .iter()
                .map(|v| LexisNexisRangeItem::new(v.number, false))
                .collect::<Vec<LexisNexisRangeItem>>(),
        );
        records.sort_by_key(|v| v.num);
        // tracing::info!("Record: {:#?}", &records);
        Self { records }
    }

    /// The `ranges` method returns the ranges of addresses within the service area, as marked by
    /// the `include` field.
    pub fn ranges(&self) -> Vec<(i64, i64)> {
        let mut rngs = Vec::new();
        let mut min = 0;
        let mut max = 0;
        let mut open = false;
        for item in &self.records {
            if item.include {
                if !open {
                    open = true;
                    min = item.num;
                }
                max = item.num;
            } else {
                if open {
                    open = false;
                    rngs.push((min, max));
                }
            }
        }
        if open {
            rngs.push((min, max));
        }
        rngs
    }
}
