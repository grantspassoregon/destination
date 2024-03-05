use crate::prelude::{Address, CityAddress, CityAddresses};

#[derive(Debug, Clone)]
pub struct GeoAddress {
    pub address: Address,
    pub x_coordinate: f64,
    pub y_coordinate: f64,
    pub latitude: f64,
    pub longitude: f64,
}

impl From<&CityAddress> for GeoAddress {
    fn from(address: &CityAddress) -> Self {
        let x_coordinate = address.address_x_coordinate();
        let y_coordinate = address.address_y_coordinate();
        let latitude = address.address_latitude();
        let longitude = address.address_longitude();
        let address = Address::from(address);
        Self {
            address,
            x_coordinate,
            y_coordinate,
            latitude,
            longitude,
        }
    }
}

#[derive(Debug, Clone)]
pub struct GeoAddresses {
    pub records: Vec<GeoAddress>,
}

impl From<&CityAddresses> for GeoAddresses {
    fn from(addresses: &CityAddresses) -> Self {
        let records = addresses
            .records
            .iter()
            .map(|v| GeoAddress::from(v))
            .collect::<Vec<GeoAddress>>();
        Self { records }
    }
}
