//! The `grants_pass_business` module contains data types for importing business license reports
//! for the City of Grants Pass.
use crate::{
    Address, AddressError, AddressErrorKind, BusinessMatchRecord, BusinessMatchRecords, Decode,
    Geographic, IntoBin, Io, MatchStatus, NaicsMissing, Nom, Parse, PartialAddress,
    PostalCommunity, State, StreetNamePostType, StreetNamePreDirectional, deserialize_phone_number,
    error::{Jiff, ParseInt},
    from_bin, from_csv, to_bin,
};
use derive_getters::Getters;
use derive_more::{Deref, DerefMut, From};
use elicitation::Elicit;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// The `BusinessRaw` struct contains business license records. Serves as an intermediary for
/// creating a [`Business`] struct when reading the data in from a csv.  Mainly this involves
/// parsing the `street_address_label` from a String into a `PartialAddress`.
/// The fields correspond to the export format from the GIS layer.
#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Deserialize,
    Serialize,
    JsonSchema,
    Elicit,
    Getters,
)]
#[serde(rename_all = "PascalCase")]
pub struct BusinessRaw {
    company_name: Option<String>,
    contact_name: Option<String>,
    business_type: String,
    #[serde(rename(deserialize = "DBA"))]
    dba: Option<String>,
    #[serde(rename(deserialize = "NumberOfEmployees"))]
    employees: i64,
    #[serde(
        rename(deserialize = "BUSINESSPHONE"),
        deserialize_with = "deserialize_phone_number"
    )]
    business_phone: Option<i64>,
    #[serde(rename(deserialize = "LICENSENUMBER"))]
    license: String,
    #[serde(rename(deserialize = "ISSUEDDATE"))]
    issued: String,
    #[serde(rename(deserialize = "EXPIRATIONDATE"))]
    expires: String,
    #[serde(rename = "CodeNumber")]
    industry_code: i32,
    #[serde(rename(deserialize = "situs_addressline1"))]
    situs_address_number: Option<String>,
    #[serde(rename(deserialize = "situs_addressline2"))]
    situs_street_name: Option<String>,
    #[serde(
        rename(deserialize = "situs_predirection"),
        deserialize_with = "StreetNamePreDirectional::deserialize_mixed"
    )]
    situs_street_name_pre_directional: Option<StreetNamePreDirectional>,
    #[serde(
        rename(deserialize = "situs_streettype"),
        deserialize_with = "StreetNamePostType::deserialize_mixed"
    )]
    situs_street_name_post_type: Option<StreetNamePostType>,
    #[serde(
        rename(deserialize = "situs_unitorsuite"),
        deserialize_with = "csv::invalid_option"
    )]
    situs_subaddress_identifier: Option<String>,
    #[serde(rename(deserialize = "situs_city"))]
    situs_postal_community: Option<String>,
    #[serde(rename(deserialize = "situs_state"))]
    situs_state_name: Option<String>,
    #[serde(rename(deserialize = "situs_postalcode"))]
    situs_zip_code: Option<i64>,
    #[serde(rename(deserialize = "mailing_addressline1"))]
    mailing_address_number: Option<String>,
    #[serde(rename(deserialize = "mailing_addressline2"))]
    mailing_street_name: Option<String>,
    #[serde(
        rename(deserialize = "mailing_predirection"),
        deserialize_with = "StreetNamePreDirectional::deserialize_mixed"
    )]
    mailing_street_name_pre_directional: Option<StreetNamePreDirectional>,
    #[serde(
        rename(deserialize = "mailing_streettype"),
        deserialize_with = "StreetNamePostType::deserialize_mixed"
    )]
    mailing_street_name_post_type: Option<StreetNamePostType>,
    #[serde(
        rename(deserialize = "mailing_unitorsuite"),
        deserialize_with = "csv::invalid_option"
    )]
    mailing_subaddress_identifier: Option<String>,
    #[serde(rename(deserialize = "mailing_city"))]
    mailing_postal_community: Option<String>,
    #[serde(rename(deserialize = "mailing_state"))]
    mailing_state_name: Option<String>,
    #[serde(rename(deserialize = "mailing_postalcode"))]
    mailing_zip_code: Option<i64>,
}

impl BusinessRaw {
    /// Converts situs address information to a [`PartialAddress`].
    pub fn situs(&self) -> PartialAddress {
        let mut situs = PartialAddress::new();
        if let Some(number) = &self.situs_address_number
            && let Ok(num) = str::parse::<i64>(number)
        {
            situs.with_address_number(num);
        }
        if let Some(predirectional) = &self.situs_street_name_pre_directional {
            situs.with_street_name_pre_directional(*predirectional);
        }
        if let Some(street) = &self.situs_street_name {
            situs.with_street_name(street.clone());
        }
        if let Some(post_type) = &self.situs_street_name_post_type {
            situs.with_street_name_post_type(*post_type);
        }
        if let Some(val) = &self.situs_subaddress_identifier {
            if let Ok((rem, Some(subtype))) = Parse::subaddress_type(val) {
                situs.with_subaddress_type(subtype);
                if let Ok((_, Some(id))) = Parse::subaddress_id(rem) {
                    situs.with_subaddress_identifier(id);
                }
            } else {
                situs.with_subaddress_identifier(val.clone());
            }
        }
        if let Some(subaddress) = &self.situs_subaddress_identifier {
            situs.with_subaddress_identifier(subaddress.clone());
        }
        if let Some(city) = &self.situs_postal_community
            && let Some(comm) = PostalCommunity::match_mixed(city)
        {
            situs.with_postal_community(comm);
        }
        if let Some(state) = &self.situs_state_name
            && let Some(s) = State::match_mixed(state)
        {
            situs.with_state_name(s);
        }
        if let Some(zip) = &self.situs_zip_code {
            situs.with_zip_code(*zip);
        }
        situs
    }

    /// The mailing address for the business.
    pub fn mailing(&self) -> PartialAddress {
        let mut mailing = PartialAddress::new();
        if let Some(number) = &self.mailing_address_number
            && let Ok(num) = str::parse::<i64>(number)
        {
            mailing.with_address_number(num);
        }
        if let Some(predirectional) = &self.mailing_street_name_pre_directional {
            mailing.with_street_name_pre_directional(*predirectional);
        }
        if let Some(street) = &self.mailing_street_name {
            mailing.with_street_name(street.clone());
        }
        if let Some(post_type) = &self.mailing_street_name_post_type {
            mailing.with_street_name_post_type(*post_type);
        }
        if let Some(val) = &self.mailing_subaddress_identifier {
            if let Ok((rem, Some(subtype))) = Parse::subaddress_type(val) {
                mailing.with_subaddress_type(subtype);
                if let Ok((_, Some(id))) = Parse::subaddress_id(rem) {
                    mailing.with_subaddress_identifier(id);
                }
            } else {
                mailing.with_subaddress_identifier(val.clone());
            }
        }
        if let Some(city) = &self.mailing_postal_community
            && let Some(comm) = PostalCommunity::match_mixed(city)
        {
            mailing.with_postal_community(comm);
        }
        if let Some(state) = &self.mailing_state_name
            && let Some(s) = State::match_mixed(state)
        {
            mailing.with_state_name(s);
        }
        if let Some(zip) = &self.mailing_zip_code {
            mailing.with_zip_code(*zip);
        }
        mailing
    }
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
    Clone,
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Deserialize,
    Serialize,
    JsonSchema,
    Elicit,
    Getters,
)]
pub struct Business {
    // The official name of the company.
    company_name: Option<String>,
    // The contact for the company.
    contact_name: Option<String>,
    // The business alias of the company.
    dba: Option<String>,
    // The business type.
    business_type: String,
    // The situs address of the business.
    #[serde(flatten)]
    situs_address: PartialAddress,
    // The mailing address of the business.
    #[serde(flatten)]
    mailing_address: PartialAddress,
    // The license identifier.
    license: String,
    // Date of first issue
    issued: jiff::civil::DateTime,
    // Date of expiration
    expires: jiff::civil::DateTime,
    // Number of employees
    employees: i64,
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
    /// Compares the address of `Business` to `address`, producing either a matching
    /// [`BusinessMatchRecord`], any divergent [`BusinessMatchRecord`], or `None` if missing.
    pub fn coincident<T: Address + Geographic>(&self, address: &T) -> Option<BusinessMatchRecord> {
        let mut match_status = MatchStatus::Missing;
        let mut business_match = None;
        let mut subaddress_id = None;
        if let Some(val) = self.situs_address().subaddress_identifier().clone()
            && !val.is_empty()
        {
            // info!("Subaddress not empty: {}", &val);
            let trim_val = val.trim();
            if !trim_val.is_empty() {
                // info!("Writing subaddress: {}", trim_val);
                subaddress_id = Some(trim_val.to_string());
            }
        }
        #[allow(renamed_and_removed_lints)]
        #[allow(question_mark)]
        let street_name = match self.situs_address().street_name() {
            Some(street) => street.trim().to_string(),
            None => return None,
        };
        if *self.situs_address().address_number() == Some(address.number())
            && *self.situs_address().street_name_pre_directional() == *address.directional()
            && street_name == *address.street_name()
            && *self.situs_address().street_name_post_type() == *address.street_type()
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
            let record = BusinessMatchRecord::builder()
                .match_status(match_status)
                .situs(self.situs_address().label())
                .mailing(self.mailing_address().label())
                .company_name(self.company_name().clone())
                .contact_name(self.contact_name().clone())
                .business_type(self.business_type().clone())
                .dba(self.dba().clone())
                .license(self.license().clone())
                .issued(*self.issued())
                .expires(*self.expires())
                .employees(*self.employees())
                .industry_code(*self.industry_code() as i64)
                .community(*self.situs_address().postal_community())
                .other_address_label(Some(address.label()))
                .address_latitude(Some(address.latitude()))
                .address_longitude(Some(address.longitude()))
                .build()
                .expect("valid build");
            business_match = Some(record);
        }
        business_match
    }
}

impl TryFrom<&BusinessRaw> for Business {
    type Error = AddressErrorKind;

    // The `try_from` method does the heavy lifting converting a [`BusinessRaw`] struct to a
    // [`Business`] type.  Errors if the address parsing fails.
    fn try_from(raw: &BusinessRaw) -> Result<Self, Self::Error> {
        let situs_address = raw.situs();
        let mailing_address = raw.mailing();
        // let issued = jiff::civil::DateTime::strptime("%m/%d/%Y %H:%M", raw.issued())
        let issued =
            if let Ok(time) = jiff::civil::DateTime::strptime("%m/%d/%Y %H:%M", raw.issued()) {
                time
            } else {
                match jiff::civil::DateTime::strptime("%Y-%m-%d %H:%M:%S", raw.issued()) {
                    Ok(time) => time,
                    Err(e) => return Err(Jiff::new(raw.issued().to_owned(), e).into()),
                }
            };
        let expires =
            if let Ok(time) = jiff::civil::DateTime::strptime("%m/%d/%Y %H:%M", raw.issued()) {
                time
            } else {
                match jiff::civil::DateTime::strptime("%Y-%m-%d %H:%M:%S", raw.issued()) {
                    Ok(time) => time,
                    Err(e) => return Err(Jiff::new(raw.issued().to_owned(), e).into()),
                }
            };

        let codestring = raw.industry_code.to_string();
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
        Ok(Self {
            company_name: raw.company_name.clone(),
            contact_name: raw.contact_name.clone(),
            dba: raw.dba.clone(),
            business_type: raw.business_type.clone(),
            situs_address,
            mailing_address,
            license: raw.license.clone(),
            issued,
            expires,
            employees: raw.employees,
            industry_code: raw.industry_code,
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

impl TryFrom<&BusinessMatchRecord> for Business {
    type Error = AddressErrorKind;

    fn try_from(value: &BusinessMatchRecord) -> Result<Self, Self::Error> {
        let company_name = value.company_name().clone();
        let contact_name = value.contact_name().clone();
        let dba = value.dba().clone();
        let business_type = value.business_type().clone();
        let situs_address = value.situs();
        let (_, situs_address) = Parse::address(situs_address).map_err(|e| {
            Nom::new(
                "situs address from business match".to_string(),
                e,
                line!(),
                file!().to_string(),
            )
        })?;
        let mailing_address = value.mailing();
        let (_, mailing_address) = Parse::address(mailing_address).map_err(|e| {
            Nom::new(
                "mailing address from business match".to_string(),
                e,
                line!(),
                file!().to_string(),
            )
        })?;
        let license = value.license().clone();
        let issued = *value.issued();
        let expires = *value.expires();
        let employees = *value.employees();
        let industry_code = *value.industry_code();
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
            business_type,
            situs_address,
            mailing_address,
            license,
            issued,
            expires,
            employees,
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
            records.push(Business::try_from(record)?);
        }
        Ok(Businesses(records))
    }
}

impl TryFrom<&BusinessesRaw> for Businesses {
    type Error = AddressErrorKind;

    fn try_from(value: &BusinessesRaw) -> Result<Self, Self::Error> {
        let features = value
            .iter()
            .map(Business::try_from)
            .collect::<Result<Vec<Business>, AddressErrorKind>>()?;
        Ok(Businesses::from(features))
    }
}

impl TryFrom<BusinessMatchRecords> for Businesses {
    type Error = AddressErrorKind;

    fn try_from(value: BusinessMatchRecords) -> Result<Self, Self::Error> {
        let features = value
            .iter()
            .map(Business::try_from)
            .collect::<Result<Vec<Business>, AddressErrorKind>>()?;
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
