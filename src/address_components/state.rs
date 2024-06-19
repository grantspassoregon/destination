use derive_more::Display;
use serde::de::Deserializer;
use serde::{Deserialize, Serialize};

/// The `State` enum holds variants for state and territory names in the US used by the FAA.
/// https://www.faa.gov/air_traffic/publications/atpubs/cnt_html/appendix_a.html
#[allow(missing_docs)]
#[derive(
    Debug,
    Default,
    Copy,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Deserialize,
    Serialize,
    Display,
)]
pub enum State {
    Alabama,
    Alaska,
    Arizona,
    Arkansas,
    AmericanSomoa,
    California,
    Colorado,
    Connecticut,
    Delaware,
    DistrictOfColumbia,
    Florida,
    Georgia,
    Guam,
    Hawaii,
    Idaho,
    Illinois,
    Indiana,
    Iowa,
    Kansas,
    Kentucky,
    Louisiana,
    Maine,
    Maryland,
    Massachusetts,
    Michigan,
    Minnesota,
    Mississippi,
    Missouri,
    Montana,
    Nebraska,
    Nevada,
    NewHampshire,
    NewJersey,
    NewMexico,
    NewYork,
    NorthCarolina,
    NorthDakota,
    NorthernMarianaIslands,
    Ohio,
    Oklahoma,
    #[default]
    Oregon,
    Pennsylvania,
    PuertoRico,
    RhodeIsland,
    SouthCarolina,
    SouthDakota,
    Tennessee,
    Texas,
    TrustTerritories,
    Utah,
    Vermont,
    Virginia,
    VirginIslands,
    Washington,
    WestVirginia,
    Wisconsin,
    Wyoming,
}

impl State {
    /// The `abbreviate` method returns the two-character standard postal abbreviation for a state
    /// or territory.
    pub fn abbreviate(&self) -> String {
        let abbr = match self {
            Self::Alabama => "AL",
            Self::Alaska => "AS",
            Self::Arizona => "AZ",
            Self::Arkansas => "AR",
            Self::AmericanSomoa => "AS",
            Self::California => "CA",
            Self::Colorado => "CO",
            Self::Connecticut => "CT",
            Self::Delaware => "DE",
            Self::DistrictOfColumbia => "DC",
            Self::Florida => "FL",
            Self::Georgia => "GA",
            Self::Guam => "GU",
            Self::Hawaii => "HI",
            Self::Idaho => "ID",
            Self::Illinois => "IL",
            Self::Indiana => "IN",
            Self::Iowa => "IA",
            Self::Kansas => "KS",
            Self::Kentucky => "KY",
            Self::Louisiana => "LA",
            Self::Maine => "ME",
            Self::Maryland => "MD",
            Self::Massachusetts => "MA",
            Self::Michigan => "MI",
            Self::Minnesota => "MN",
            Self::Mississippi => "MS",
            Self::Missouri => "MO",
            Self::Montana => "MT",
            Self::Nebraska => "NE",
            Self::Nevada => "NV",
            Self::NewHampshire => "NH",
            Self::NewJersey => "NJ",
            Self::NewMexico => "NM",
            Self::NewYork => "NY",
            Self::NorthCarolina => "NC",
            Self::NorthDakota => "ND",
            Self::NorthernMarianaIslands => "MP",
            Self::Ohio => "OH",
            Self::Oklahoma => "OK",
            Self::Oregon => "OR",
            Self::Pennsylvania => "PA",
            Self::PuertoRico => "PR",
            Self::RhodeIsland => "RI",
            Self::SouthCarolina => "SC",
            Self::SouthDakota => "SD",
            Self::Tennessee => "TN",
            Self::Texas => "TX",
            Self::TrustTerritories => "TT",
            Self::Utah => "UT",
            Self::Vermont => "VT",
            Self::Virginia => "VA",
            Self::VirginIslands => "VI",
            Self::Washington => "WA",
            Self::WestVirginia => "WV",
            Self::Wisconsin => "WI",
            Self::Wyoming => "WY",
        };
        abbr.to_string()
    }

    /// The `match_abbreviated` method matches the `input` str against valid abbreviations for a US
    /// state or territory.
    pub fn match_abbreviated(input: &str) -> Option<Self> {
        match input.to_lowercase().as_str() {
            "al" => Some(Self::Alabama),
            "ak" => Some(Self::Alaska),
            "az" => Some(Self::Arizona),
            "ar" => Some(Self::Arkansas),
            "as" => Some(Self::AmericanSomoa),
            "ca" => Some(Self::California),
            "co" => Some(Self::Colorado),
            "ct" => Some(Self::Connecticut),
            "de" => Some(Self::Delaware),
            "dc" => Some(Self::DistrictOfColumbia),
            "fl" => Some(Self::Florida),
            "ga" => Some(Self::Georgia),
            "gu" => Some(Self::Guam),
            "hi" => Some(Self::Hawaii),
            "id" => Some(Self::Idaho),
            "il" => Some(Self::Illinois),
            "in" => Some(Self::Indiana),
            "ia" => Some(Self::Iowa),
            "ks" => Some(Self::Kansas),
            "ky" => Some(Self::Kentucky),
            "la" => Some(Self::Louisiana),
            "me" => Some(Self::Maine),
            "md" => Some(Self::Maryland),
            "ma" => Some(Self::Massachusetts),
            "mi" => Some(Self::Michigan),
            "mn" => Some(Self::Minnesota),
            "ms" => Some(Self::Mississippi),
            "mo" => Some(Self::Missouri),
            "mt" => Some(Self::Montana),
            "ne" => Some(Self::Nebraska),
            "nv" => Some(Self::Nevada),
            "nh" => Some(Self::NewHampshire),
            "nj" => Some(Self::NewJersey),
            "nm" => Some(Self::NewMexico),
            "ny" => Some(Self::NewYork),
            "nc" => Some(Self::NorthCarolina),
            "nd" => Some(Self::NorthDakota),
            "mp" => Some(Self::NorthernMarianaIslands),
            "oh" => Some(Self::Ohio),
            "ok" => Some(Self::Oklahoma),
            "or" => Some(Self::Oregon),
            "pa" => Some(Self::Pennsylvania),
            "pr" => Some(Self::PuertoRico),
            "ri" => Some(Self::RhodeIsland),
            "sc" => Some(Self::SouthCarolina),
            "sd" => Some(Self::SouthDakota),
            "tn" => Some(Self::Tennessee),
            "tx" => Some(Self::Texas),
            "tt" => Some(Self::TrustTerritories),
            "ut" => Some(Self::Utah),
            "vt" => Some(Self::Vermont),
            "va" => Some(Self::Virginia),
            "vi" => Some(Self::VirginIslands),
            "wa" => Some(Self::Washington),
            "wv" => Some(Self::WestVirginia),
            "wi" => Some(Self::Wisconsin),
            "wy" => Some(Self::Wyoming),
            _ => None,
        }
    }

    /// The `deserialize_abbreviated` method attempts to convert an input from the postal
    /// abbreviation for a US state or territory to a variant of the `State` enum.
    /// Use this method when the input field for state is sanitized and failure to map to a variant
    /// is an error.
    pub fn deserialize_abbreviated<'de, D: Deserializer<'de>>(
        de: D,
    ) -> Result<Option<Self>, D::Error> {
        let intermediate = Deserialize::deserialize(de)?;
        Ok(Self::match_abbreviated(intermediate))
    }

    /// The `match_mixed` method attempts to convert the str in `input` to a variant of the
    /// `State` enum.  As we encounter additional mappings of non-standard spellings to valid variants, we add them to the match statement here.
    pub fn match_mixed(input: &str) -> Option<Self> {
        if let Some(state) = Self::match_abbreviated(input) {
            Some(state)
        } else {
            match input.to_lowercase().as_str() {
                "california" => Some(Self::California),
                "oregon" => Some(Self::Oregon),
                "washington" => Some(Self::Washington),
                _ => None,
            }
        }
    }

    /// The `deserialize_abbreviated` method attempts to convert an input from the postal
    /// abbreviation for a US state or territory to a variant of the `State` enum.
    /// Use this method when the input field for state is not sanitized and failure to map to a variant
    /// is an acceptable or anticipated outcome.
    /// TODO: Eliminate unwrap.
    pub fn deserialize_mixed<'de, D: Deserializer<'de>>(de: D) -> Result<Self, D::Error> {
        let intermediate = Deserialize::deserialize(de)?;
        let result = Self::match_mixed(intermediate).unwrap();
        Ok(result)
    }
}
