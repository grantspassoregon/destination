//! The `business` module matches addresses associated with business licenses against a set of known [`Addresses`], producing a record of
//! matching, divergent and missing addresses.
use crate::{
    deserialize_phone_number, from_csv, to_csv, Address, AddressErrorKind, Geographic, IntoCsv, Io,
    MatchStatus, Nom, Parse, StreetNamePostType, StreetNamePreDirectional,
};
use derive_more::{Deref, DerefMut};
// use galileo::galileo_types::geo::GeoPoint;
use indicatif::ParallelProgressIterator;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use tracing::info;

/// The `BusinessMatchRecord` struct holds match data for a licensed business.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd)]
pub struct BusinessMatchRecord {
    match_status: MatchStatus,
    business_address_label: String,
    company_name: Option<String>,
    contact_name: Option<String>,
    business_type: String,
    dba: Option<String>,
    license: String,
    expires: String,
    industry_code: i64,
    community: String,
    other_address_label: Option<String>,
    address_latitude: Option<f64>,
    address_longitude: Option<f64>,
}

impl BusinessMatchRecord {
    /// The `company_name` field represents the registered business name associated with the
    /// active business license.
    pub fn company_name(&self) -> Option<String> {
        self.company_name.clone()
    }

    /// The `contact_name` field represents the contact name associated with the
    /// active business license.
    pub fn contact_name(&self) -> Option<String> {
        self.contact_name.clone()
    }

    /// The `dba` field represents the business name alias associated with the
    /// active business license.
    pub fn dba(&self) -> Option<String> {
        self.dba.clone()
    }

    /// The `business_address_label` field represents the submitted address associated with the
    /// active business license.
    pub fn business_address_label(&self) -> String {
        self.business_address_label.clone()
    }

    /// The `other_address_label` field represents the City address associated with the
    /// active business license.
    pub fn other_address_label(&self) -> Option<String> {
        self.other_address_label.clone()
    }

    /// The `license` field represents the business license number associated with the
    /// active business license.
    pub fn license(&self) -> String {
        self.license.clone()
    }

    /// The `industry_code` field represents the tax code associated with the
    /// active business license.
    pub fn industry_code(&self) -> i64 {
        self.industry_code
    }

    /// The `latitude` method returns the latitude of the address for the
    /// active business license.
    pub fn latitude(&self) -> Option<f64> {
        self.address_latitude
    }

    /// The `longitude` method returns the longitude of the address for the
    /// active business license.
    pub fn longitude(&self) -> Option<f64> {
        self.address_longitude
    }
}

/// The `BusinessMatchRecords` struct holds a vector of [`BusinessMatchRecord`] objects.
#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize, Serialize, Deref, DerefMut)]
pub struct BusinessMatchRecords(Vec<BusinessMatchRecord>);

impl BusinessMatchRecords {
    /// Matches the provided address associated with a business license against the addresses in
    /// `addresses`, creating a new `BusinessMatchRecords` struct containing the results.
    pub fn new<T: Address + Geographic>(business: &BusinessLicense, addresses: &[T]) -> Self {
        let mut records = Vec::new();
        for address in addresses {
            let business_match = business.coincident(address);
            if let Some(record) = business_match {
                records.push(record);
            }
        }
        if records.is_empty() {
            records.push(BusinessMatchRecord {
                match_status: MatchStatus::Missing,
                business_address_label: business.label(),
                company_name: business.company_name(),
                contact_name: business.contact_name(),
                business_type: business.business_type(),
                dba: business.dba(),
                license: business.license(),
                expires: business.expires(),
                industry_code: business.industry_code(),
                community: business.community(),
                other_address_label: None,
                address_latitude: None,
                address_longitude: None,
            });
        }
        let business_record = BusinessMatchRecords(records);
        let matched = business_record.clone().filter("matching");
        let divergent = business_record.clone().filter("divergent");
        if !matched.is_empty() {
            let trim_match = matched[0].clone();
            BusinessMatchRecords(vec![trim_match])
        } else if !divergent.is_empty() {
            divergent
        } else {
            business_record
        }
    }

    /// This method compares the address of `business` against the addresses in `address_list`,
    /// returning a `BusinessMatchRecords` struct containing an exact match if found, otherwise a
    /// list of partial (divergent) matches if found, otherwise a missing record.  Since divergent
    /// addresses are unnecessary to inspect if an exact match is found, this is a more efficient
    /// matching method compared to [`BusinessMatchRecords::compare()`].
    pub fn chain<T: Address + Geographic>(
        business: &BusinessLicense,
        address_list: &[&[T]],
    ) -> Self {
        let mut matching = Vec::new();
        let mut divergent = Vec::new();
        let mut missing = Vec::new();
        for addresses in address_list {
            let record = BusinessMatchRecords::new(business, addresses);
            let matched = record.clone().filter("matching");
            let diverged = record.clone().filter("divergent");
            let missed = record.clone().filter("missing");
            if !matched.is_empty() && matching.is_empty() {
                matching = matched.0;
            } else if !diverged.is_empty() && divergent.is_empty() {
                divergent = diverged.0;
            } else if !missed.is_empty() && missing.is_empty() {
                missing = missed.0;
            }
        }
        if !matching.is_empty() {
            BusinessMatchRecords(matching)
        } else if !divergent.is_empty() {
            BusinessMatchRecords(divergent)
        } else {
            BusinessMatchRecords(missing)
        }
    }

    /// For each [`BusinessLicense`] object in `businesses`, this method creates a
    /// `BusinessMatchRecords` using the [`BusinessMatchRecords::new()`] method.  Match records
    /// will include matching, divergent and missing records.
    pub fn compare<T: Address + Geographic + Send + Sync>(
        businesses: &BusinessLicenses,
        addresses: &[T],
    ) -> Self {
        let style = indicatif::ProgressStyle::with_template(
            "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {'Comparing addresses.'}",
        )
        .unwrap();
        let record = businesses
            .par_iter()
            .map(|address| BusinessMatchRecords::new(address, addresses))
            .progress_with_style(style)
            .collect::<Vec<BusinessMatchRecords>>();
        let mut records = Vec::new();
        for mut item in record {
            records.append(&mut item);
        }
        BusinessMatchRecords(records)
    }

    /// Compares each address in `businesses` against the addresses in `addresses` using the
    /// [`BusinessMatchRecords::chain()`] method, which returns only an exact match if available,
    /// otherwise returning a list of partial matches or a missing record.
    pub fn compare_chain<T: Address + Geographic + Send + Sync>(
        businesses: &BusinessLicenses,
        addresses: &[&[T]],
    ) -> Self {
        let style = indicatif::ProgressStyle::with_template(
            "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {'Comparing addresses.'}",
        )
        .unwrap();
        let record = businesses
            .par_iter()
            .map(|address| BusinessMatchRecords::chain(address, addresses))
            .progress_with_style(style)
            .collect::<Vec<BusinessMatchRecords>>();
        let mut records = Vec::new();
        for mut item in record {
            records.append(&mut item);
        }
        BusinessMatchRecords(records)
    }

    /// The `filter` method filters the [`BusinessMatchRecord`] objects in the `records` field
    /// based upon the match status of the record.  The `filter` field accepts the values
    /// "missing", "nonmissing", "divergent", "matching", "unique" and "multiple". The "unique"
    /// option returns records where the business name is unique.  The "multiple" options returns
    /// records where multiple licenses exist registered under the same business name. The "local"
    /// option returns records within Grants Pass or Merlin.
    ///
    /// As a filter, the method must either copy the data in Self to create a subset using the
    /// filter, or it must mutate the data of Self in place.  Here we take ownership of Self and
    /// mutate it in place, because it is the more efficient option, and leaves the option to the
    /// caller whether to mutate the source data or clone it before passing in.  Whereas if we take
    /// a reference, we must clone inside the function, removing the choice from the caller.
    pub fn filter(mut self, filter: &str) -> Self {
        match filter {
            "missing" => self.retain(|r| r.match_status == MatchStatus::Missing),
            "nonmissing" => self.retain(|r| r.match_status != MatchStatus::Missing),
            "divergent" => self.retain(|r| r.match_status == MatchStatus::Divergent),
            "matching" => self.retain(|r| r.match_status == MatchStatus::Matching),
            "unique" => {
                let mut names = HashSet::new();
                let mut records = Vec::new();
                for record in self.iter() {
                    if let Some(name) = record.company_name() {
                        if !names.contains(&name) {
                            names.insert(name.clone());
                            let subset = self.clone().filter_field("name", &name);
                            if subset.len() == 1 {
                                records.push(subset[0].clone());
                            }
                        }
                    }
                }
                self.0 = records;
            }
            "multiple" => {
                let mut names = HashSet::new();
                let mut records = Vec::new();
                for record in self.iter() {
                    if let Some(name) = record.company_name() {
                        if !names.contains(&name) {
                            names.insert(name.clone());
                            let mut subset = self.clone().filter_field("name", &name);
                            if subset.len() > 1 {
                                records.append(&mut subset);
                            }
                        }
                    }
                }
                self.0 = records;
            }
            "local" => self.retain(|r| r.community == "GRANTS PASS" || r.community == "MERLIN"),
            _ => info!("Invalid filter provided."),
        }
        self
    }

    /// The `filter_field` method filters [`BusinessMatchRecord`] objects in the `records` field
    /// by comparing the value of the field specified in `filter` to the value of `field`.  The
    /// `filter` field accepts the value "name", and matches the value of `field` against the company
    /// name associated with the record.
    ///
    /// Like the filter method, we move the responsibility to clone to the caller.
    pub fn filter_field(mut self, filter: &str, field: &str) -> Self {
        match filter {
            "name" => self.retain(|r| r.company_name() == Some(field.to_string())),
            _ => info!("Invalid filter provided."),
        }
        self
    }

    // /// Writes the contents of `BusinessMatchRecords` to a CSV file at location `title`.  Each element in
    // /// the vector of type [`BusinessMatchRecord`] maps to a row of data on the CSV.
    // pub fn to_csv(&mut self, title: std::path::PathBuf) -> Result<(), std::io::Error> {
    //     to_csv(self, title)?;
    //     Ok(())
    // }
    //
    // /// Creates a new `BusinessMatchRecords` struct from a CSV file located at `path`.  This method
    // /// does not parse raw business license data, and should only be used to read files output from
    // /// [`BusinessMatchRecords::to_csv()`].
    // pub fn from_csv<P: AsRef<std::path::Path>>(path: P) -> Result<Self, std::io::Error> {
    //     let records = from_csv(path)?;
    //     Ok(BusinessMatchRecords(records))
    // }
}

impl IntoCsv<BusinessMatchRecords> for BusinessMatchRecords {
    fn from_csv<P: AsRef<std::path::Path>>(path: P) -> Result<Self, Io> {
        let records = from_csv(path)?;
        Ok(Self(records))
    }

    fn to_csv<P: AsRef<std::path::Path>>(&mut self, path: P) -> Result<(), AddressErrorKind> {
        to_csv(&mut self.0, path.as_ref().into())
    }
}

/// The `BusinessLicense` struct is designed to deserialize CSV data produced by querying the
/// EnerGov SQL database for active business licenses.  If the structure of the SQL query changes,
/// this function will need to change to match the resulting fields in the CSV.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct BusinessLicense {
    company_name: Option<String>,
    contact_name: Option<String>,
    business_type: String,
    #[serde(rename(deserialize = "dba"))]
    dba: Option<String>,
    #[serde(deserialize_with = "deserialize_phone_number")]
    business_phone: Option<i64>,
    #[serde(rename(deserialize = "LICENSENUMBER"))]
    license: String,
    #[serde(rename(deserialize = "EXPIRATIONDATE"))]
    expires: String,
    #[serde(rename = "CodeNumber")]
    industry_code: i64,
    #[serde(rename(deserialize = "ADDRESSLINE1"))]
    address_number: String,
    #[serde(rename(deserialize = "ADDRESSLINE2"))]
    street_name: String,
    #[serde(
        rename(deserialize = "PREDIRECTION"),
        deserialize_with = "StreetNamePreDirectional::deserialize_mixed"
    )]
    street_name_pre_directional: Option<StreetNamePreDirectional>,
    #[serde(
        rename(deserialize = "STREETTYPE"),
        deserialize_with = "StreetNamePostType::deserialize_mixed"
    )]
    street_name_post_type: Option<StreetNamePostType>,
    #[serde(
        rename(deserialize = "UNITORSUITE"),
        deserialize_with = "csv::invalid_option"
    )]
    subaddress_identifier: Option<String>,
    #[serde(rename(deserialize = "CITY"))]
    postal_community: String,
    #[serde(rename(deserialize = "STATE"))]
    state_name: String,
    #[serde(rename(deserialize = "POSTALCODE"))]
    zip_code: i64,
}

impl BusinessLicense {
    /// Compares the address of `BusinessLicense` to `address`, producing either a matching
    /// [`BusinessMatchRecord`], any divergent [`BusinessMatchRecord`], or `None` if missing.
    pub fn coincident<T: Address + Geographic>(&self, address: &T) -> Option<BusinessMatchRecord> {
        let mut match_status = MatchStatus::Missing;
        let mut business_match = None;
        let mut subaddress_id = None;
        if let Some(val) = self.subaddress_identifier.clone() {
            if !val.is_empty() {
                // info!("Subaddress not empty: {}", &val);
                let trim_val = val.trim();
                if !trim_val.is_empty() {
                    // info!("Writing subaddress: {}", trim_val);
                    subaddress_id = Some(trim_val.to_string());
                }
            }
        }
        let street_name = self.street_name.trim().to_string();
        if self.address_number == address.complete_address_number()
            && self.street_name_pre_directional == *address.directional()
            && street_name == *address.street_name()
            && self.street_name_post_type == *address.street_type()
        // && self.postal_community == address.postal_community()
        // && self.state_name == address.state_name()
        {
            if subaddress_id != *address.subaddress_id() {
                match_status = MatchStatus::Divergent;
            }
            // robust against +4 codes?
            // if self.zip_code != address.zip() {
            //     match_status = MatchStatus::Divergent;
            // }
            if match_status != MatchStatus::Divergent {
                match_status = MatchStatus::Matching;
            }
            business_match = Some(BusinessMatchRecord {
                match_status,
                business_address_label: self.label(),
                company_name: self.company_name(),
                contact_name: self.contact_name(),
                business_type: self.business_type(),
                dba: self.dba(),
                license: self.license(),
                expires: self.expires(),
                industry_code: self.industry_code(),
                community: self.community(),
                other_address_label: Some(address.label()),
                address_latitude: Some(address.latitude()),
                address_longitude: Some(address.longitude()),
            });
        }
        business_match
    }

    /// The `community` method returns the postal community name from the `postal_community` field.
    pub fn community(&self) -> String {
        self.postal_community.to_owned()
    }

    /// The `company_name` field represents the registered name of the business.  This method
    /// returns the cloned value of the field.
    pub fn company_name(&self) -> Option<String> {
        match self.company_name.clone() {
            Some(name) => Some(name.trim().to_string()),
            None => None.to_owned(),
        }
    }

    /// The `contact_name` field represents the business owner name.  This method clones the
    /// value of the field.
    pub fn contact_name(&self) -> Option<String> {
        self.contact_name.to_owned()
    }

    /// The `business_type` field represents the tax classification associated with a business
    /// license.  This method clones the value of the field.
    pub fn business_type(&self) -> String {
        self.business_type.to_owned()
    }

    /// The `dba` field represents the alias name associated with a business license.  This method
    /// clones the value of the field.
    pub fn dba(&self) -> Option<String> {
        self.dba.to_owned()
    }

    /// The `license` field represents the license ID associated with the business.  This method
    /// clones the value of the field.
    pub fn license(&self) -> String {
        self.license.to_owned()
    }

    /// The `expires` field represents the time of expiration for the active business license.
    /// This method clones the value of the field.
    pub fn expires(&self) -> String {
        self.expires.to_owned()
    }

    /// The `industry_code` method returns the value of the `industry_code` field.
    pub fn industry_code(&self) -> i64 {
        self.industry_code
    }

    /// The `pre_directional` field represents the street pre-directional designation associated
    /// with a business license.  This method clones the value of the field.
    pub fn pre_directional(&self) -> Option<StreetNamePreDirectional> {
        self.street_name_pre_directional
    }

    /// The `post_type` field represents the street post type designation of the business address.
    /// This method returns the cloned value of the field.
    pub fn post_type(&self) -> Option<StreetNamePostType> {
        self.street_name_post_type
    }

    /// The `subaddress_identifier` field represents the subaddress unit identifier associated with
    /// a business address.  This method clones the value of the field.
    pub fn subaddress_identifier(&self) -> Option<String> {
        self.subaddress_identifier.to_owned()
    }

    /// The `label` method creates a string representation of the complete street address
    /// associated with a business license.
    fn label(&self) -> String {
        let street_name = match self.post_type() {
            Some(post_type) => format!("{} {}", self.street_name, post_type),
            None => self.street_name.to_string(),
        };
        let complete_street_name = match self.street_name_pre_directional {
            Some(pre_directional) => format!("{} {}", pre_directional.abbreviate(), street_name),
            None => street_name,
        };
        match self.subaddress_identifier() {
            Some(subaddress) => format!(
                "{} {} {}",
                self.address_number, complete_street_name, subaddress
            ),
            None => format!("{} {}", self.address_number, complete_street_name),
        }
    }

    /// EnerGov has a single field for entering a subaddress id, and staff sometimes include the
    /// subaddress type.  This method strips the type information from the id, so we can compare
    /// the id to addresses in the city.
    pub fn detype_subaddress(&mut self) -> Result<(), Nom> {
        if let Some(val) = &self.subaddress_identifier {
            match Parse::subaddress_type(val) {
                Ok((rem, _)) => match Parse::subaddress_id(rem) {
                    Ok((_, element)) => {
                        if let Some(id) = element {
                            self.subaddress_identifier = Some(id.to_string());
                        }
                    }
                    Err(source) => {
                        let description = format!("parsing subaddress id from {}", rem);
                        let source = Nom::new(description, source, line!(), file!().to_string());
                        return Err(source);
                    }
                },
                Err(source) => {
                    let description = format!("parsing subaddress type from {}", val);
                    let source = Nom::new(description, source, line!(), file!().to_string());
                    return Err(source);
                }
            }
        }
        Ok(())
    }
}

/// The `BusinessLicenses` struct holds a `records` field containing a vector of type
/// [`BusinessLicense`].
#[derive(Debug, Clone, Deserialize, Serialize, Deref, DerefMut)]
pub struct BusinessLicenses(Vec<BusinessLicense>);

impl BusinessLicenses {
    /// Creates a new `BusinessLicenses` struct from a CSV file located at `path`.
    pub fn from_csv<P: AsRef<std::path::Path>>(path: P) -> Result<Self, Io> {
        let records = from_csv(path)?;
        Ok(BusinessLicenses(records))
    }

    /// Returns the subset of `BusinessLicenses` where the value of the `filter` field is equal to
    /// the test value in `field`.  Currently `filter` can take the value `name`, referring to the
    /// company name.
    pub fn filter(mut self, filter: &str, field: &str) -> Self {
        match filter {
            "name" => self.retain(|r| r.company_name() == Some(field.to_string())),
            "license" => self.retain(|r| r.license() == *field),
            _ => info!("Invalid filter provided."),
        }
        self
    }

    /// Retains one record from each license in `BusinessLicenses`, keeping the first encountered,
    /// intended to remove duplicate licenses from a record.
    pub fn deduplicate(&self) -> Self {
        let mut records = Vec::new();
        let mut licenses = HashSet::new();
        for record in self.iter() {
            let license = record.license();
            if !licenses.contains(&license) {
                licenses.insert(license);
                records.push(record.clone());
            }
        }
        BusinessLicenses(records)
    }

    /// The `detype_subaddresses` method calls the [`BusinessLicense::detype_subaddress`] method on each record in
    /// `records`.
    pub fn detype_subaddresses(&mut self) -> Result<(), Nom> {
        self.iter_mut()
            .map(BusinessLicense::detype_subaddress)
            .for_each(drop);
        Ok(())
    }
}

impl IntoCsv<BusinessLicenses> for BusinessLicenses {
    fn from_csv<P: AsRef<std::path::Path>>(path: P) -> Result<Self, Io> {
        let records = from_csv(path)?;
        Ok(Self(records))
    }

    fn to_csv<P: AsRef<std::path::Path>>(&mut self, path: P) -> Result<(), AddressErrorKind> {
        to_csv(&mut self.0, path.as_ref().into())
    }
}
