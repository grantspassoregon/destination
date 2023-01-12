use crate::address_components::*;
use crate::utils::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct CityAddress {
    #[serde(rename(deserialize = "OID_"))]
    object_id: i64,
    #[serde(rename(deserialize = "Add_Number"))]
    address_number: i64,
    #[serde(deserialize_with = "deserialize_arcgis_data", rename(deserialize = "AddNum_Suf"))]
    address_number_suffix: Option<String>,
    #[serde(deserialize_with = "csv::invalid_option", rename(deserialize = "St_PreDir"))]
    street_name_pre_directional: Option<StreetNamePreDirectional>,
    #[serde(rename(deserialize = "St_Name"))]
    street_name: String,
    #[serde(rename(deserialize = "St_PosTyp"))]
    street_name_post_type: StreetNamePostType,
    #[serde(deserialize_with = "csv::invalid_option")]
    subaddress_type: Option<SubaddressType>,
    #[serde(deserialize_with = "deserialize_arcgis_data")]
    subaddress_identifier: Option<String>,
    #[serde(deserialize_with = "csv::invalid_option")]
    floor: Option<i64>,
    #[serde(deserialize_with = "deserialize_arcgis_data")]
    building: Option<String>,
    #[serde(rename(deserialize = "Post_Code"))]
    zip_code: i64,
    #[serde(rename(deserialize = "STATUS"))]
    status: AddressStatus,
    #[serde(deserialize_with = "deserialize_arcgis_data", rename(deserialize = "NOTIFICATION"))]
    notification: Option<String>,
    #[serde(deserialize_with = "deserialize_arcgis_data", rename(deserialize = "NOTES"))]
    notes: Option<String>,
    #[serde(rename(deserialize = "GlobalID"))]
    global_id: String,
    #[serde(deserialize_with = "deserialize_arcgis_data", rename(deserialize = "created_user"))]
    created_user: Option<String>,
    #[serde(deserialize_with = "deserialize_arcgis_data", rename(deserialize = "created_date"))]
    created_date: Option<String>,
    #[serde(rename(deserialize = "last_edited_user"))]
    last_edited_user: String,
    #[serde(rename(deserialize = "last_edited_date"))]
    last_edited_date: String,
    complete_address_number: String,
    complete_street_name: String,
    #[serde(deserialize_with = "deserialize_arcgis_data")]
    complete_subaddress: Option<String>,
    complete_street_address: String,
    street_address_label: String,
    place_state_zip: String,
    #[serde(rename(deserialize = "Post_Comm"))]
    postal_community: String,
    state_name: String,
    #[serde(deserialize_with = "deserialize_arcgis_data", rename(deserialize = "Inc_Muni"))]
    incorporated_municipality: Option<String>,
    #[serde(deserialize_with = "deserialize_arcgis_data", rename(deserialize = "Uninc_Comm"))]
    unincorporated_community: Option<String>,
}


#[derive(Debug, Deserialize, Serialize)]
pub struct CityAddresses {
    pub records: Vec<CityAddress>,
}

impl CityAddresses {
    pub fn from_csv<P: AsRef<std::path::Path>>(path: P) -> Result<Self, std::io::Error> {
        let mut data = Vec::new();
        let file = std::fs::File::open(path)?;
        let mut rdr = csv::Reader::from_reader(file);

        for result in rdr.deserialize() {
            let record: CityAddress = result?;
            data.push(record);
        }

        Ok(CityAddresses { records: data})
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CountyAddress {
    #[serde(rename(deserialize = "OID_"))]
    object_id: i64,
    #[serde(deserialize_with = "deserialize_arcgis_data")]
    taxlot: Option<String>,
    #[serde(rename(deserialize = "stnum"))]
    address_number: i64,
    #[serde(deserialize_with = "deserialize_arcgis_data", rename(deserialize = "stnumsuf"))]
    address_number_suffix: Option<String>,
    #[serde(deserialize_with = "deserialize_abbreviated_pre_directional", rename(deserialize = "predir"))]
    street_name_pre_directional: Option<StreetNamePreDirectional>,
    #[serde(rename(deserialize = "name"))]
    street_name: String,
    #[serde(deserialize_with = "deserialize_abbreviated_post_type", rename(deserialize = "type"))]
    street_name_post_type: Option<StreetNamePostType>,
    #[serde(deserialize_with = "deserialize_abbreviated_subaddress_type", rename(deserialize = "unit_type"))]
    subaddress_type: Option<SubaddressType>,
    #[serde(deserialize_with = "deserialize_arcgis_data", rename(deserialize = "unit"))]
    subaddress_identifier: Option<String>,
    #[serde(deserialize_with = "csv::invalid_option")]
    floor: Option<i64>,
    #[serde(rename(deserialize = "address"))]
    complete_street_address: String,
    #[serde(rename(deserialize = "postcomm"))]
    postal_community: String,
    #[serde(rename(deserialize = "zip"))]
    zip_code: i64,
    status: AddressStatus,
    point_y: Option<f64>,
    point_x: Option<f64>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CountyAddresses {
    pub records: Vec<CountyAddress>,
}

impl CountyAddresses {
    pub fn from_csv<P: AsRef<std::path::Path>>(path: P) -> Result<Self, std::io::Error> {
        let mut data = Vec::new();
        let file = std::fs::File::open(path)?;
        let mut rdr = csv::Reader::from_reader(file);

        for result in rdr.deserialize() {
            let record: CountyAddress = result?;
            data.push(record);
        }

        Ok(CountyAddresses { records: data})
    }
}



