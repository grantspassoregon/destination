use crate::prelude::*;
use aid::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct FireInspectionRaw {
    name: String,
    address: String,
    class: Option<String>,
    subclass: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FireInspectionsRaw {
    pub records: Vec<FireInspectionRaw>,
}

impl FireInspectionsRaw {
    pub fn from_csv<P: AsRef<std::path::Path>>(path: P) -> Result<Self, std::io::Error> {
        let records = from_csv(path)?;
        Ok(FireInspectionsRaw { records })
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct FireInspection {
    name: String,
    address: PartialAddress,
    class: Option<String>,
    subclass: Option<String>,
}

impl FireInspection {
    pub fn name(&self) -> String {
        self.name.to_owned()
    }

    pub fn address(&self) -> PartialAddress {
        self.address.clone()
    }

    pub fn class(&self) -> Option<String> {
        self.class.clone()
    }

    pub fn subclass(&self) -> Option<String> {
        self.subclass.clone()
    }
}

impl TryFrom<FireInspectionRaw> for FireInspection {
    type Error = Bandage;

    fn try_from(raw: FireInspectionRaw) -> Result<Self, Self::Error> {
        match parse_address(&raw.address) {
            Ok((_, address)) => {
                let mut upper_address = address.clone();
                if let Some(identifier) = address.subaddress_identifier() {
                    upper_address.set_subaddress_identifier(&identifier.to_uppercase())
                };
                Ok(FireInspection {
                    name: raw.name,
                    address: upper_address,
                    class: raw.class,
                    subclass: raw.subclass,
                })
            }
            Err(_) => Err(Bandage::Parse),
        }
    }
}

#[derive(Debug, Clone)]
pub struct FireInspections {
    records: Vec<FireInspection>,
}

impl FireInspections {
    pub fn from_csv<P: AsRef<std::path::Path>>(path: P) -> Clean<Self> {
        let raw = FireInspectionsRaw::from_csv(path)?;
        let mut records = Vec::new();
        for record in raw.records {
            records.push(FireInspection::try_from(record)?);
        }
        Ok(FireInspections { records })
    }

    pub fn records(&self) -> Vec<FireInspection> {
        self.records.clone()
    }
}
