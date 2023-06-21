use crate::{address::PartialAddress, parser::parse_address};
use crate::address_components::*;
use crate::error::AddressError;
use crate::utils::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BusinessRaw {
    company_name: String,
    contact_name: Option<String>,
    dba: Option<String>,
    street_address_label: String,
    license: String,
    industry_code: i32,
    industry_name: String,
    sector_code: i32,
    sector_name: String,
    subsector_code: i32,
    subsector_name: Option<String>,
    tourism: Option<String>,
    district: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BusinessesRaw {
    records: Vec<BusinessRaw>,
}

impl BusinessesRaw {
    pub fn from_csv<P: AsRef<std::path::Path>>(path: P) -> Result<Self, std::io::Error> {
        let mut records = Vec::new();
        let file = std::fs::File::open(path)?;
        let mut rdr = csv::Reader::from_reader(file);

        for result in rdr.deserialize() {
            let record: BusinessRaw = result?;
            records.push(record);
        }

        Ok(BusinessesRaw { records })
    }

    pub fn records(&self) -> Vec<BusinessRaw> {
        self.records.clone()
    }
}

#[derive(Clone, Debug)]
pub struct Business {
    company_name: String,
    contact_name: Option<String>,
    dba: Option<String>,
    address: PartialAddress,
    license: String,
    industry_code: i32,
    industry_name: String,
    sector_code: i32,
    sector_name: String,
    subsector_code: i32,
    subsector_name: Option<String>,
    tourism: Option<String>,
    district: Option<String>,
}

impl Business {
    pub fn company_name(&self) -> String {
        self.company_name.to_owned()
    }

    pub fn contact_name(&self) -> Option<String> {
        self.contact_name.clone()
    }

    pub fn dba(&self) -> Option<String> {
        self.dba.clone()
    }

    pub fn address(&self) -> PartialAddress {
        self.address.clone()
    }

    pub fn license(&self) -> String {
        self.license.to_owned()
    }

    pub fn industry_code(&self) -> i32 {
        self.industry_code
    }

    pub fn industry_name(&self) -> String {
        self.industry_name.to_owned()
    }

    pub fn sector_code(&self) -> i32 {
        self.sector_code
    }

    pub fn sector_name(&self) -> String {
        self.sector_name.to_owned()
    }

    pub fn subsector_code(&self) -> i32 {
        self.subsector_code
    }

    pub fn subsector_name(&self) -> Option<String> {
        self.subsector_name.clone()
    }

    pub fn tourism(&self) -> Option<String> {
        self.tourism.clone()
    }

    pub fn district(&self) -> Option<String> {
        self.district.clone()
    }
}

impl TryFrom<BusinessRaw> for Business {
    type Error = AddressError;

    fn try_from(raw: BusinessRaw) -> Result<Self, Self::Error> {
        match parse_address(&raw.street_address_label) {
            Ok((_, address)) => {
                Ok(Business {
                    company_name: raw.company_name,
                    contact_name: raw.contact_name,
                    dba: raw.dba,
                    address,
                    license: raw.license,
                    industry_code: raw.industry_code,
                    industry_name: raw.industry_name,
                    sector_code: raw.sector_code,
                    sector_name: raw.sector_name,
                    subsector_code: raw.subsector_code,
                    subsector_name: raw.subsector_name,
                    tourism: raw.tourism,
                    district: raw.district,
                })
            },
            Err(_) => Err(AddressError::ParseError),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Businesses {
    records: Vec<Business>,
}

impl Businesses {
    pub fn from_csv<P: AsRef<std::path::Path>>(path: P) -> Result<Self, AddressError> {
        let raw = BusinessesRaw::from_csv(path)?;
        let mut records = Vec::new();
        for record in raw.records {
            records.push(Business::try_from(record)?);
        }
        Ok(Businesses { records })
    }

    pub fn records(&self) -> Vec<Business> {
        self.records.clone()
    }
}

