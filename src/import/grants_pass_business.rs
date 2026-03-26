//! The `grants_pass_business` module contains data types for importing business license reports
//! for the City of Grants Pass.
use crate::{
    AddressError, AddressErrorKind, BusinessMatchRecord, BusinessMatchRecords, Decode, IntoBin,
    IntoCsv, Io, NaicsMissing, Nom, Parse, PartialAddress, error::ParseInt, from_bin, from_csv,
    to_bin, to_csv,
};
use derive_more::{Deref, DerefMut, From};
use elicitation::Elicit;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// The `BusinessRaw` struct contains business license records. Serves as an intermediary for
/// creating a [`Business`] struct when reading the data in from a csv.  Mainly this involves
/// parsing the `street_address_label` from a String into a `PartialAddress`.
/// The fields correspond to the export format from the GIS layer.
#[derive(
    Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize, JsonSchema, Elicit,
)]
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

/// The `BusinessesRaw` struct is a wrapper for a vector of type [`BusinessRaw`].
#[derive(
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
    JsonSchema,
    Elicit,
)]
pub struct BusinessesRaw(Vec<BusinessRaw>);

impl BusinessesRaw {
    /// Writes the contents of the struct to a csv file at location `path`.
    pub fn from_csv<P: AsRef<std::path::Path>>(path: P) -> Result<Self, Io> {
        let records = from_csv(path)?;
        Ok(Self(records))
    }
}

/// The `Business` struct holds query information for active business licenses, for access in GIS.
#[derive(
    Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize, JsonSchema, Elicit,
)]
pub struct Business {
    // The official name of the company.
    company_name: String,
    // The contact for the company.
    contact_name: Option<String>,
    // The business alias of the company.
    dba: Option<String>,
    // The situs address of the business.
    #[serde(flatten)]
    address: PartialAddress,
    // The license identifier.
    license: String,
    // The NAICS industry code of the business.
    industry_code: i32,
    // The NAICS industry code description.
    industry_name: String,
    // The NAICS sector code of the business.
    sector_code: i32,
    // The NAICS sector code description.
    sector_name: String,
    // The NAICS subsector code.
    subsector_code: i32,
    // The NAICS subsector code description.
    subsector_name: Option<String>,
    // Broad business categories used to drive symbolization in a GIS map.
    tourism: Option<String>,
    // The business district name of the GC zone, if in a GC zone.
    district: Option<String>,
}

impl Business {
    /// The `company_name` method returns the cloned value of the `company_name` field, which
    /// contains the company name.
    pub fn company_name(&self) -> String {
        self.company_name.to_owned()
    }

    /// The `contact_name` method returns the cloned value of the `contact_name` field, which
    /// contains the contact name for the business.
    pub fn contact_name(&self) -> Option<String> {
        self.contact_name.clone()
    }

    /// The `dba` method returns the cloned value of the `dba` field, which contains the business
    /// alias name.
    pub fn dba(&self) -> Option<String> {
        self.dba.clone()
    }

    /// The `address` method returns the cloned value of the `address` field, which contains a
    /// [`PartialAddress`] constructed from the provided business address.
    pub fn address(&self) -> PartialAddress {
        self.address.clone()
    }

    /// The `license` method returns the cloned value of the `license` field, which contains the
    /// license identifier assigned to the business.
    pub fn license(&self) -> String {
        self.license.to_owned()
    }

    /// The `industry_code` method returns the NAICS industry code.
    pub fn industry_code(&self) -> i32 {
        self.industry_code
    }

    /// The `industry_name` method returns the NAICS industry code description.
    pub fn industry_name(&self) -> String {
        self.industry_name.to_owned()
    }

    /// The `sector_code` method returns the NAICS sector code.
    pub fn sector_code(&self) -> i32 {
        self.sector_code
    }

    /// The `sector_name` method returns the NAICS sector code description.
    pub fn sector_name(&self) -> String {
        self.sector_name.to_owned()
    }

    /// The `subsector_code` method returns the NAICS subsector code.
    pub fn subsector_code(&self) -> i32 {
        self.subsector_code
    }

    /// The `subsector_name` method returns the NAICS subsector code description.
    pub fn subsector_name(&self) -> Option<String> {
        self.subsector_name.clone()
    }

    /// The `tourism` method clones the value of the `tourism` field, which contains broad business categories used to drive symbology in a GIS map.
    pub fn tourism(&self) -> Option<String> {
        self.tourism.clone()
    }

    /// The `district` method returns the cloned value of the `district` field, which contains the
    /// district name associated with the GC zone, if located in GC.
    pub fn district(&self) -> Option<String> {
        self.district.clone()
    }
}

impl TryFrom<BusinessRaw> for Business {
    type Error = Nom;

    // The `try_from` method does the heavy lifting converting a [`BusinessRaw`] struct to a
    // [`Business`] type.  Errors if the address parsing fails.
    fn try_from(raw: BusinessRaw) -> Result<Self, Self::Error> {
        // Attempt to parse the address label to a [`PartialAddress`].
        match Parse::address(&raw.street_address_label) {
            // Return the conversion on success.
            Ok((_, address)) => Ok(Business {
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
            }),
            // Throw an error if parsing fails.
            Err(source) => Err(Nom::new(
                raw.street_address_label.clone(),
                source,
                line!(),
                file!().into(),
            )),
        }
    }
}

impl TryFrom<&BusinessMatchRecord> for Business {
    type Error = AddressError;

    fn try_from(value: &BusinessMatchRecord) -> Result<Self, Self::Error> {
        let company_name = value.company_name().unwrap_or_default();
        let contact_name = value.contact_name();
        let dba = value.dba();
        let address = value.business_address_label();
        let (_, address) = Parse::address(&address).map_err(|e| {
            Nom::new(
                "situs address from business match".to_string(),
                e,
                line!(),
                file!().to_string(),
            )
        })?;
        let license = value.license();
        let industry_code = value.industry_code();
        let codestring = industry_code.to_string();
        let naics = bears_species::Naics::from_code(&codestring).ok_or(NaicsMissing::new(
            codestring.clone(),
            line!(),
            file!().to_string(),
        ))?;
        let industry_name = naics.description().to_owned();
        let sector_code = codestring.chars().take(2).collect::<String>();
        let sector = bears_species::NaicsSector::from_code(&sector_code).ok_or(
            NaicsMissing::new(sector_code.clone(), line!(), file!().to_string()),
        )?;
        let sector_code = sector_code.parse::<i32>().map_err(|e| {
            ParseInt::new(
                "sector code for business match".to_string(),
                e,
                line!(),
                file!().to_string(),
            )
        })?;
        let sector_name = sector.description().to_owned();
        let subsector_code = codestring.chars().take(3).collect::<String>();
        let subsector = bears_species::NaicsSubsector::from_code(&subsector_code).ok_or(
            NaicsMissing::new(subsector_code.clone(), line!(), file!().to_string()),
        )?;
        let subsector_code = subsector_code.parse::<i32>().map_err(|e| {
            ParseInt::new(
                "subsector code for business match".to_string(),
                e,
                line!(),
                file!().to_string(),
            )
        })?;
        let subsector_name = Some(subsector.description().to_owned());
        let tourism = Default::default();
        let district = Default::default();
        let industry_code = industry_code as i32;
        Ok(Self {
            company_name,
            contact_name,
            dba,
            address,
            license,
            industry_code,
            industry_name,
            sector_code,
            sector_name,
            subsector_code,
            subsector_name,
            tourism,
            district,
        })
    }
}

/// The `Businesses` struct is a wrapper around a vector of type [`Business`].
/// This struct contains business licenses that have mapped to valid addresses.
#[derive(
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
    From,
    JsonSchema,
    Elicit,
)]
pub struct Businesses(Vec<Business>);

impl Businesses {
    /// Writes the contents to a csv file at location `path`.
    pub fn from_raw_csv<P: AsRef<std::path::Path>>(path: P) -> Result<Self, AddressErrorKind> {
        let raw = BusinessesRaw::from_csv(path)?;
        let mut records = Vec::new();
        for record in raw.iter() {
            records.push(Business::try_from(record.clone())?);
        }
        Ok(Businesses(records))
    }
}

impl TryFrom<BusinessMatchRecords> for Businesses {
    type Error = AddressError;

    fn try_from(value: BusinessMatchRecords) -> Result<Self, Self::Error> {
        let features = value
            .iter()
            .map(Business::try_from)
            .collect::<Result<Vec<Business>, AddressError>>()?;
        Ok(Businesses::from(features))
    }
}

impl IntoBin<Businesses> for Businesses {
    fn load<P: AsRef<std::path::Path>>(path: P) -> Result<Self, AddressError> {
        let config = bincode::config::standard();
        match from_bin(path) {
            Ok(records) => {
                let (results, _) = bincode::serde::decode_from_slice::<
                    Self,
                    bincode::config::Configuration,
                >(&records, config)
                .map_err(|source| Decode::new(source, line!(), file!().into()))?;
                Ok(results)
            }
            Err(source) => Err(AddressErrorKind::from(source).into()),
        }
    }

    fn save<P: AsRef<std::path::Path>>(&self, path: P) -> Result<(), AddressError> {
        to_bin(self, path)
    }
}
