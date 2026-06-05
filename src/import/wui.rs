use crate::{AddressErrorKind, Io, Nom, Parse, PartialAddress};

/// The `WuiRaw` struct contains address, contact information and metadata for fire suppression
/// applicatants within the Wilderness Urban Interface (WUI).
#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    serde::Deserialize,
    serde::Serialize,
    schemars::JsonSchema,
    elicitation::Elicit,
)]
#[serde(rename_all = "UPPERCASE")]
pub struct WuiRaw {
    // Address number
    address_number: i64,
    // Date of application
    date: String,
    // Street name predirectional
    #[serde(rename = "DIR")]
    directional: Option<String>,
    // Contact email address
    email: Option<String>,
    // Contact name
    name: Option<String>,
    // Contact phone number
    phone: Option<String>,
    // Complete street name
    street: String,
    // Hour and minute of application
    time: i64,
}

impl WuiRaw {
    /// Construct the address string from parts
    pub fn address(&self) -> String {
        let mut address = self.address_number.to_string();
        address.push(' ');
        if let Some(dir) = &self.directional {
            address.push_str(dir);
            address.push(' ');
        }
        address.push_str(&self.street);
        address
    }
}

/// The `WuisRaw` struct is a wrapper around a vector of type [`WuiRaw`].
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    serde::Deserialize,
    serde::Serialize,
    derive_more::Deref,
    derive_more::DerefMut,
    schemars::JsonSchema,
    elicitation::Elicit,
)]
pub struct WuisRaw(Vec<WuiRaw>);

impl WuisRaw {
    /// Used to read fire inspection data in from the csv source file.
    #[tracing::instrument(skip_all)]
    pub fn from_csv<P: AsRef<std::path::Path>>(path: P) -> Result<Self, Io> {
        let records = crate::from_csv(path)?;
        Ok(WuisRaw(records))
    }
}

/// The `Wui` struct contains a parsed address, contact information and metadata for fire suppression
/// applicatants within the Wilderness Urban Interface (WUI).
#[derive(
    Clone,
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    serde::Deserialize,
    serde::Serialize,
    schemars::JsonSchema,
    elicitation::Elicit,
    derive_getters::Getters,
)]
#[serde(rename_all = "PascalCase")]
pub struct Wui {
    // Address
    address: PartialAddress,
    // Date of application
    date: String,
    // Contact email address
    email: Option<String>,
    // Contact name
    name: Option<String>,
    // Contact phone number
    phone: Option<String>,
    // Hour and minute of application
    time: i64,
}

impl TryFrom<WuiRaw> for Wui {
    type Error = Nom;

    fn try_from(raw: WuiRaw) -> Result<Self, Self::Error> {
        let raw_address = raw.address();
        match Parse::address(&raw_address) {
            Ok((_, address)) => Ok(Self {
                address,
                date: raw.date,
                email: raw.email,
                name: raw.name,
                phone: raw.phone,
                time: raw.time,
            }),
            Err(source) => Err(Nom::new(
                raw_address.clone(),
                source,
                line!(),
                file!().to_string(),
            )),
        }
    }
}

/// The `Wuis` struct is a wrapper around a vector of type [`Wui`].
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    serde::Deserialize,
    serde::Serialize,
    derive_more::Deref,
    derive_more::DerefMut,
    schemars::JsonSchema,
    elicitation::Elicit,
)]
pub struct Wuis(Vec<Wui>);

impl Wuis {
    /// Reads in the data as a raw wui applicant, attempts to parse each address, returning a
    /// `Wuis` if successful.
    #[tracing::instrument(skip_all)]
    pub fn from_csv<P: AsRef<std::path::Path>>(path: P) -> Result<Self, AddressErrorKind> {
        // Try to read in as raw.
        let raw = WuisRaw::from_csv(path)?;
        let mut records = Vec::new();
        for record in raw.iter() {
            // Parse the raw address.
            records.push(Wui::try_from(record.clone())?);
        }
        Ok(Wuis(records))
    }
}
