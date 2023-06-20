use crate::address::*;
use crate::address_components::*;
use crate::compare::*;
use crate::utils::*;
use indicatif::ParallelProgressIterator;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use tracing::info;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BusinessMatchRecord {
    match_status: MatchStatus,
    business_address_label: String,
    company_name: Option<String>,
    contact_name: Option<String>,
    business_type: String,
    dba: Option<String>,
    license: String,
    expires: String,
    id: Option<i64>,
    other_address_label: Option<String>,
    address_latitude: Option<f64>,
    address_longitude: Option<f64>,
}

impl BusinessMatchRecord {
    pub fn company_name(&self) -> Option<String> {
        match &self.company_name {
            Some(name) => Some(name.clone()),
            None => None.to_owned(),
        }
    }
}

#[derive(Clone)]
pub struct BusinessMatchRecords {
    pub records: Vec<BusinessMatchRecord>,
}

impl BusinessMatchRecords {
    fn new(business: &BusinessLicense, addresses: &Addresses) -> Self {
        let mut records = Vec::new();
        for address in &addresses.records {
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
                id: None,
                other_address_label: None,
                address_latitude: None,
                address_longitude: None,
            });
        }
        let business_record = BusinessMatchRecords { records };
        let matched = business_record.filter("matching");
        let divergent = business_record.filter("divergent");
        if !matched.records.is_empty() {
            let trim_match = matched.records[0].clone();
            BusinessMatchRecords {
                records: vec![trim_match],
            }
        } else if !divergent.records.is_empty() {
            divergent
        } else {
            business_record
        }
    }

    fn chain(business: &BusinessLicense, address_list: &[&Addresses]) -> Self {
        let mut matching = Vec::new();
        let mut divergent = Vec::new();
        let mut missing = Vec::new();
        for addresses in address_list {
            let record = BusinessMatchRecords::new(business, addresses);
            let matched = record.filter("matching");
            let diverged = record.filter("divergent");
            let missed = record.filter("missing");
            if !matched.records.is_empty() && matching.is_empty() {
                matching = matched.records;
            } else if !diverged.records.is_empty() && divergent.is_empty() {
                divergent = diverged.records;
            } else if !missed.records.is_empty() && missing.is_empty() {
                missing = missed.records;
            }
        }
        if !matching.is_empty() {
            BusinessMatchRecords { records: matching }
        } else if !divergent.is_empty() {
            BusinessMatchRecords { records: divergent }
        } else {
            BusinessMatchRecords { records: missing }
        }
    }

    pub fn compare(businesses: &BusinessLicenses, addresses: &Addresses) -> Self {
        let style = indicatif::ProgressStyle::with_template(
            "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {'Comparing addresses.'}",
        )
        .unwrap();
        let record = businesses
            .records
            .par_iter()
            .map(|address| BusinessMatchRecords::new(address, addresses))
            .progress_with_style(style)
            .collect::<Vec<BusinessMatchRecords>>();
        let mut records = Vec::new();
        for mut item in record {
            records.append(&mut item.records);
        }
        BusinessMatchRecords { records }
    }

    pub fn compare_chain(businesses: &BusinessLicenses, addresses: &[&Addresses]) -> Self {
        let style = indicatif::ProgressStyle::with_template(
            "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {'Comparing addresses.'}",
        )
        .unwrap();
        let record = businesses
            .records
            .par_iter()
            .map(|address| BusinessMatchRecords::chain(address, addresses))
            .progress_with_style(style)
            .collect::<Vec<BusinessMatchRecords>>();
        let mut records = Vec::new();
        for mut item in record {
            records.append(&mut item.records);
        }
        BusinessMatchRecords { records }
    }

    pub fn filter(&self, filter: &str) -> Self {
        let mut records = Vec::new();
        match filter {
            "missing" => records.append(
                &mut self
                    .records
                    .par_iter()
                    .cloned()
                    .filter(|record| record.match_status == MatchStatus::Missing)
                    .collect(),
            ),
            "nonmissing" => records.append(
                &mut self
                    .records
                    .par_iter()
                    .cloned()
                    .filter(|record| record.match_status == MatchStatus::Missing)
                    .collect(),
            ),
            "divergent" => records.append(
                &mut self
                    .records
                    .par_iter()
                    .cloned()
                    .filter(|record| record.match_status == MatchStatus::Divergent)
                    .collect(),
            ),
            "matching" => records.append(
                &mut self
                    .records
                    .par_iter()
                    .cloned()
                    .filter(|record| record.match_status == MatchStatus::Matching)
                    .collect(),
            ),
            "unique" => {
                let mut names = HashSet::new();
                for record in self.records.clone() {
                    if let Some(name) = record.company_name() {
                        if !names.contains(&name) {
                            names.insert(name.clone());
                            let subset = self.filter_field("name", &name);
                            if subset.records.len() == 1 {
                                records.push(subset.records[0].clone());
                            }
                        }
                    }
                }
            }
            "multiple" => {
                let mut names = HashSet::new();
                for record in self.records.clone() {
                    if let Some(name) = record.company_name() {
                        if !names.contains(&name) {
                            names.insert(name.clone());
                            let mut subset = self.filter_field("name", &name);
                            if subset.records.len() > 1 {
                                records.append(&mut subset.records);
                            }
                        }
                    }
                }
            }
            _ => info!("Invalid filter provided."),
        }
        BusinessMatchRecords { records }
    }

    pub fn filter_field(&self, filter: &str, field: &str) -> Self {
        let mut records = Vec::new();
        match filter {
            "name" => records.append(
                &mut self
                    .records
                    .par_iter()
                    .cloned()
                    .filter(|record| record.company_name() == Some(field.to_string()))
                    .collect(),
            ),
            _ => info!("Invalid filter provided."),
        }
        BusinessMatchRecords { records }
    }

    pub fn to_csv(&mut self, title: std::path::PathBuf) -> Result<(), std::io::Error> {
        let mut wtr = csv::Writer::from_path(title)?;
        for i in self.records.clone() {
            wtr.serialize(i)?;
        }
        wtr.flush()?;
        Ok(())
    }

    pub fn from_csv<P: AsRef<std::path::Path>>(path: P) -> Result<Self, std::io::Error> {
        let mut records = Vec::new();
        let file = std::fs::File::open(path)?;
        let mut rdr = csv::Reader::from_reader(file);

        for result in rdr.deserialize() {
            let record: BusinessMatchRecord = result?;
            records.push(record);
        }

        Ok(BusinessMatchRecords { records })
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GrantsPass2022Address {
    #[serde(rename(deserialize = "OID_"))]
    object_id: i64,
    #[serde(rename(deserialize = "ADDRNUM"))]
    address_number: i64,
    #[serde(
        deserialize_with = "csv::invalid_option",
        rename(deserialize = "ROADPREDIR")
    )]
    street_name_pre_directional: Option<StreetNamePreDirectional>,
    #[serde(rename(deserialize = "ROADNAME"))]
    street_name: String,
    #[serde(
        rename(deserialize = "ROADTYPE"),
        deserialize_with = "deserialize_mixed_post_type"
    )]
    street_name_post_type: Option<StreetNamePostType>,
    #[serde(
        rename(deserialize = "APARTMENT"),
        deserialize_with = "deserialize_arcgis_data"
    )]
    subaddress_identifier: Option<String>,
    #[serde(
        rename(deserialize = "FLOOR"),
        deserialize_with = "csv::invalid_option"
    )]
    floor: Option<i64>,
    #[serde(rename(deserialize = "ZIP"))]
    zip_code: i64,
    #[serde(rename(deserialize = "STATUS"))]
    status: AddressStatus,
    #[serde(rename(deserialize = "CITY"))]
    postal_community: String,
    #[serde(rename(deserialize = "STATE"))]
    state_name: String,
    #[serde(rename(deserialize = "latitude"))]
    address_latitude: f64,
    #[serde(rename(deserialize = "longitude"))]
    address_longitude: f64,
}

impl GrantsPass2022Address {
    pub fn address_number(&self) -> i64 {
        self.address_number
    }

    pub fn street_name(&self) -> String {
        self.street_name.to_owned()
    }

    pub fn pre_directional(&self) -> Option<StreetNamePreDirectional> {
        self.street_name_pre_directional
    }

    pub fn post_type(&self) -> Option<StreetNamePostType> {
        self.street_name_post_type
    }

    pub fn subaddress_identifier(&self) -> Option<String> {
        self.subaddress_identifier.to_owned()
    }

    pub fn floor(&self) -> Option<i64> {
        self.floor
    }

    pub fn zip_code(&self) -> i64 {
        self.zip_code
    }

    pub fn status(&self) -> AddressStatus {
        self.status
    }

    pub fn state_name(&self) -> String {
        self.state_name.to_owned()
    }

    pub fn postal_community(&self) -> String {
        self.postal_community.to_owned()
    }

    pub fn object_id(&self) -> i64 {
        self.object_id
    }

    pub fn address_latitude(&self) -> f64 {
        self.address_latitude
    }

    pub fn address_longitude(&self) -> f64 {
        self.address_longitude
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GrantsPass2022Addresses {
    pub records: Vec<GrantsPass2022Address>,
}

impl GrantsPass2022Addresses {
    pub fn from_csv<P: AsRef<std::path::Path>>(path: P) -> Result<Self, std::io::Error> {
        let mut data = Vec::new();
        let file = std::fs::File::open(path)?;
        let mut rdr = csv::Reader::from_reader(file);

        for result in rdr.deserialize() {
            let record: GrantsPass2022Address = result?;
            data.push(record);
        }

        Ok(GrantsPass2022Addresses { records: data })
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct BusinessLicense {
    company_name: Option<String>,
    contact_name: Option<String>,
    business_type: String,
    #[serde(rename(deserialize = "dba"))]
    dba: Option<String>,
    business_phone: Option<i64>,
    #[serde(rename(deserialize = "LICENSENUMBER"))]
    license: String,
    #[serde(rename(deserialize = "EXPIRATIONDATE"))]
    expires: String,
    #[serde(rename(deserialize = "ADDRESSLINE1"))]
    address_number: i64,
    #[serde(rename(deserialize = "ADDRESSLINE2"))]
    street_name: String,
    #[serde(
        rename(deserialize = "PREDIRECTION"),
        deserialize_with = "deserialize_mixed_pre_directional"
    )]
    street_name_pre_directional: Option<StreetNamePreDirectional>,
    #[serde(
        rename(deserialize = "STREETTYPE"),
        deserialize_with = "deserialize_mixed_post_type"
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
    pub fn coincident(&self, address: &Address) -> Option<BusinessMatchRecord> {
        let mut match_status = MatchStatus::Missing;
        let mut business_match = None;
        let street_name = self.street_name.trim().to_string();
        if self.address_number == address.address_number()
            && self.street_name_pre_directional == address.pre_directional()
            && street_name == address.street_name()
            && self.street_name_post_type == Some(address.post_type())
        // && self.postal_community == address.postal_community()
        // && self.state_name == address.state_name()
        {
            if self.subaddress_identifier != address.subaddress_identifier() {
                match_status = MatchStatus::Divergent;
            }
            if self.zip_code != address.zip_code() {
                match_status = MatchStatus::Divergent;
            }
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
                id: Some(address.object_id()),
                other_address_label: Some(address.label()),
                address_latitude: Some(address.address_latitude()),
                address_longitude: Some(address.address_longitude()),
            });
        }
        business_match
    }

    pub fn company_name(&self) -> Option<String> {
        match self.company_name.clone() {
            Some(name) => Some(name.trim().to_string()),
            None => None.to_owned(),
        }
    }

    pub fn contact_name(&self) -> Option<String> {
        self.contact_name.to_owned()
    }

    pub fn business_type(&self) -> String {
        self.business_type.to_owned()
    }

    pub fn dba(&self) -> Option<String> {
        self.dba.to_owned()
    }

    pub fn license(&self) -> String {
        self.license.to_owned()
    }

    pub fn expires(&self) -> String {
        self.expires.to_owned()
    }

    pub fn pre_directional(&self) -> Option<StreetNamePreDirectional> {
        self.street_name_pre_directional
    }

    pub fn post_type(&self) -> Option<StreetNamePostType> {
        self.street_name_post_type
    }

    pub fn subaddress_identifier(&self) -> Option<String> {
        self.subaddress_identifier.to_owned()
    }

    fn label(&self) -> String {
        let street_name = match self.post_type() {
            Some(post_type) => format!("{} {:?}", self.street_name, post_type),
            None => self.street_name.to_string(),
        };
        let complete_street_name = match self.street_name_pre_directional {
            Some(pre_directional) => format!("{:?} {}", pre_directional, street_name),
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
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BusinessLicenses {
    pub records: Vec<BusinessLicense>,
}

impl BusinessLicenses {
    pub fn from_csv<P: AsRef<std::path::Path>>(path: P) -> Result<Self, std::io::Error> {
        let mut data = Vec::new();
        let file = std::fs::File::open(path)?;
        let mut rdr = csv::Reader::from_reader(file);

        for result in rdr.deserialize() {
            let record: BusinessLicense = result?;
            data.push(record);
        }

        Ok(BusinessLicenses { records: data })
    }

    pub fn filter(&self, filter: &str, field: &str) -> Self {
        let mut records = Vec::new();
        match filter {
            "name" => records.append(
                &mut self
                    .records
                    .par_iter()
                    .cloned()
                    .filter(|record| record.company_name() == Some(field.to_string()))
                    .collect(),
            ),
            _ => info!("Invalid filter provided."),
        }
        BusinessLicenses { records }
    }

    pub fn deduplicate(&self) -> Self {
        let mut records = Vec::new();
        let mut licenses = HashSet::new();
        for record in self.records.clone() {
            let license = record.license();
            if !licenses.contains(&license) {
                licenses.insert(license);
                records.push(record);
            }
        }
        BusinessLicenses { records }
    }
}
