//! The `utils` module contains utility functions accessed by multiple data types, where declaring
//! a stand-alone function eliminates code duplication in different methods.
use crate::{AddressError, AddressErrorKind, Bincode, Csv, Io};
use indicatif::{ProgressBar, ProgressStyle};
use serde::Serialize;
use serde::de::{Deserialize, DeserializeOwned, Deserializer};
use std::fs;
use std::path::{Path, PathBuf};
use std::time::Duration;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

/// Function for deserailizing ArcGIS data that may contain either empty (Null) fields, or fields
/// with string value "\<Null\>", either of which should translate to `None`.
///
/// Used to deserialize fields in import data from ArcGIS, e.g.
/// [`GrantsPassAddress`](crate::GrantsPassAddress).
pub fn deserialize_arcgis_data<'de, D: Deserializer<'de>>(
    de: D,
) -> Result<Option<String>, D::Error> {
    let intermediate = Deserialize::deserialize(de)?;

    match intermediate {
        None => Ok(None),
        Some("<Null>") => Ok(None),
        Some(other_value) => Ok(Some(other_value.to_string())),
    }
}

/// Generic function to serialize data types into a CSV file.  Called by methods to avoid code
/// duplication.
///
/// See
/// [`AddressDeltas::to_csv`](crate::AddressDeltas::to_csv),
/// [`Businesses::to_csv`](crate::Businesses::to_csv),
/// [`BusinessLicenses::to_csv`](crate::BusinessLicenses::to_csv),
/// [`BusinessMatchRecords::to_csv`](crate::BusinessMatchRecords::to_csv),
/// [`CommonAddresses::to_csv`](crate::CommonAddresses::to_csv),
/// [`FireInspectionMatchRecords::to_csv`](crate::FireInspectionMatchRecords::to_csv),
/// [`GrantsPassAddresses::to_csv`](crate::GrantsPassAddresses::to_csv),
/// [`GrantsPassSpatialAddresses::to_csv`](crate::GrantsPassSpatialAddresses::to_csv),
/// [`JosephineCountyAddresses2024::to_csv`](crate::JosephineCountyAddresses2024::to_csv),
/// [`JosephineCountySpatialAddresses2024::to_csv`](crate::JosephineCountySpatialAddresses2024::to_csv),
/// [`JosephineCountyAddresses::to_csv`](crate::JosephineCountyAddresses::to_csv),
/// [`JosephineCountySpatialAddresses::to_csv`](crate::JosephineCountySpatialAddresses::to_csv),
/// [`LexisNexis::to_csv`](crate::LexisNexis::to_csv).
/// [`MatchPartialRecords::to_csv`](crate::MatchPartialRecords::to_csv),
/// [`MatchRecords::to_csv`](crate::MatchRecords::to_csv),
/// [`PartialAddresses::to_csv`](crate::PartialAddresses::to_csv),
/// [`SpatialAddressesRaw::to_csv`](crate::SpatialAddressesRaw::to_csv),
pub fn to_csv<T: Serialize + Clone>(item: &mut [T], path: PathBuf) -> Result<(), AddressErrorKind> {
    match csv::Writer::from_path(&path) {
        Ok(mut wtr) => {
            for i in item {
                wtr.serialize(i)
                    .map_err(|source| Csv::new(path.clone(), source, line!(), file!().into()))?;
            }
            wtr.flush()
                .map_err(|source| Io::new(path.clone(), source, line!(), file!().into()))?;
            Ok(())
        }
        Err(source) => Err(Csv::new(path, source, line!(), file!().to_string()).into()),
    }
}

/// Generic function to deserialize data types from a CSV file.  Called by methods to avoid code
/// duplication.
///
/// See
/// [`AddressDeltas::from_csv`](crate::AddressDeltas::from_csv),
/// [`Businesses::from_csv`](crate::Businesses::from_csv),
/// [`BusinessLicenses::from_csv`](crate::BusinessLicenses::from_csv),
/// [`BusinessMatchRecords::from_csv`](crate::BusinessMatchRecords::from_csv),
/// [`CommonAddresses::from_csv`](crate::CommonAddresses::from_csv),
/// [`FireInspectionMatchRecords::from_csv`](crate::FireInspectionMatchRecords::from_csv),
/// [`GrantsPassAddresses::from_csv`](crate::GrantsPassAddresses::from_csv),
/// [`GrantsPassSpatialAddresses::from_csv`](crate::GrantsPassSpatialAddresses::from_csv),
/// [`JosephineCountyAddresses2024::from_csv`](crate::JosephineCountyAddresses2024::from_csv),
/// [`JosephineCountySpatialAddresses2024::from_csv`](crate::JosephineCountySpatialAddresses2024::from_csv),
/// [`JosephineCountyAddresses::from_csv`](crate::JosephineCountyAddresses::from_csv),
/// [`JosephineCountySpatialAddresses::from_csv`](crate::JosephineCountySpatialAddresses::from_csv),
/// [`LexisNexis::from_csv`](crate::LexisNexis::from_csv).
/// [`MatchPartialRecords::from_csv`](crate::MatchPartialRecords::from_csv),
/// [`MatchRecords::from_csv`](crate::MatchRecords::from_csv),
/// [`PartialAddresses::from_csv`](crate::PartialAddresses::from_csv),
/// [`SpatialAddressesRaw::from_csv`](crate::SpatialAddressesRaw::from_csv),
pub fn from_csv<T: DeserializeOwned + Clone, P: AsRef<std::path::Path>>(
    path: P,
) -> Result<Vec<T>, Io> {
    let mut records = Vec::new();
    match std::fs::File::open(&path) {
        Ok(file) => {
            let mut rdr = csv::Reader::from_reader(file);

            let mut dropped = 0;
            for result in rdr.deserialize() {
                match result {
                    Ok(record) => records.push(record),
                    Err(e) => {
                        tracing::trace!("Dropping: {}", e.to_string());
                        dropped += 1;
                    }
                }
            }
            tracing::info!("{} records dropped.", dropped);

            Ok(records)
        }
        Err(source) => Err(Io::new(
            path.as_ref().into(),
            source,
            line!(),
            file!().into(),
        )),
    }
}

/// The `save` method serializes the contents of self into binary and writes to a file at
/// location `path`.  Errors bubble up from serialization in [`bincode`] or file system access during write.
///
/// See
/// [`AddressDeltas::save`](crate::AddressDeltas::save),
/// [`AddressPoints::save`](crate::AddressPoints::save),
/// [`Businesses::save`](crate::Businesses::save),
/// [`CommonAddresses::save`](crate::CommonAddresses::save),
/// [`GeoAddresses::save`](crate::GeoAddresses::save),
/// [`GrantsPassAddresses::save`](crate::GrantsPassAddresses::save),
/// [`GrantsPassSpatialAddresses::save`](crate::GrantsPassSpatialAddresses::save),
/// [`JosephineCountyAddresses2024::save`](crate::JosephineCountyAddresses2024::save),
/// [`JosephineCountySpatialAddresses2024::save`](crate::JosephineCountySpatialAddresses2024::save),
/// [`JosephineCountyAddresses::save`](crate::JosephineCountyAddresses::save),
/// [`JosephineCountySpatialAddresses::save`](crate::JosephineCountySpatialAddresses::save),
/// [`LexisNexis::save`](crate::LexisNexis::save).
/// [`PartialAddresses::save`](crate::PartialAddresses::save),
/// [`SpatialAddresses::save`](crate::SpatialAddresses::save),
/// [`SpatialAddressesRaw::save`](crate::SpatialAddressesRaw::save),
pub fn to_bin<T: Serialize, P: AsRef<Path>>(data: &T, path: P) -> Result<(), AddressError> {
    info!("Serializing to binary.");
    let encode =
        bincode::serialize(data).map_err(|source| Bincode::new(source, line!(), file!().into()))?;
    info!("Writing to file.");
    std::fs::write(&path, encode)
        .map_err(|source| Io::new(path.as_ref().into(), source, line!(), file!().into()))?;
    Ok(())
}

/// The `from_bin` function loads the contents of a file at location `path` into a `Vec<u8>`.
/// May error reading the file, for example if the location is invalid, or when deserializing
/// the binary if the format is invalid.
///
/// See
/// [`AddressDeltas::load`](crate::AddressDeltas::load),
/// [`AddressPoints::load`](crate::AddressPoints::load),
/// [`Businesses::load`](crate::Businesses::load),
/// [`CommonAddresses::load`](crate::CommonAddresses::load),
/// [`GeoAddresses::load`](crate::GeoAddresses::load),
/// [`GrantsPassAddresses::load`](crate::GrantsPassAddresses::load),
/// [`GrantsPassSpatialAddresses::load`](crate::GrantsPassSpatialAddresses::load),
/// [`JosephineCountyAddresses2024::load`](crate::JosephineCountyAddresses2024::load),
/// [`JosephineCountySpatialAddresses2024::load`](crate::JosephineCountySpatialAddresses2024::load),
/// [`JosephineCountyAddresses::load`](crate::JosephineCountyAddresses::load),
/// [`JosephineCountySpatialAddresses::load`](crate::JosephineCountySpatialAddresses::load),
/// [`LexisNexis::load`](crate::LexisNexis::load).
/// [`PartialAddresses::load`](crate::PartialAddresses::load),
/// [`SpatialAddresses::load`](crate::SpatialAddresses::load),
/// [`SpatialAddressesRaw::load`](crate::SpatialAddressesRaw::load),
pub fn from_bin<P: AsRef<Path>>(path: P) -> Result<Vec<u8>, Io> {
    info!("Loading from binary.");
    let bar = ProgressBar::new_spinner();
    bar.enable_steady_tick(Duration::from_millis(120));
    bar.set_style(
        ProgressStyle::with_template("{spinner:.blue} {msg}")
            .unwrap()
            .tick_strings(&[
                "▹▹▹▹▹",
                "▸▹▹▹▹",
                "▹▸▹▹▹",
                "▹▹▸▹▹",
                "▹▹▹▸▹",
                "▹▹▹▹▸",
                "▪▪▪▪▪",
            ]),
    );
    bar.set_message("Loading...");
    match fs::read(path.as_ref()) {
        Ok(vec) => {
            bar.finish_with_message("Loaded!");
            Ok(vec)
        }
        Err(source) => Err(Io::new(
            path.as_ref().into(),
            source,
            line!(),
            file!().into(),
        )),
    }
}

/// The `IntoCsv` trait indicates the type can be read from and to a csv file.
pub trait IntoCsv<T> {
    /// The `from_csv` method attempts to deserialize the data from a `csv` file located at `path`.
    fn from_csv<P: AsRef<Path>>(path: P) -> Result<T, Io>;
    /// The `to_csv` method attempts to serialize the data to a `csv` file at location `path`.
    fn to_csv<P: AsRef<Path>>(&mut self, path: P) -> Result<(), AddressErrorKind>;
}

/// The `IntoBin` trait indicates the type can be read from and to a binary file.
pub trait IntoBin<T> {
    /// The `load` method attempts to deserialize the data from a binary file located at `path`.
    fn load<P: AsRef<Path>>(path: P) -> Result<T, AddressError>;
    /// The `save` method attempts to serialize the data to a binary file at location `path`.
    fn save<P: AsRef<Path>>(&self, path: P) -> Result<(), AddressError>;
}

/// The `trace_init` function initializing the tracing subscriber.
pub fn trace_init() {
    if tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "destination=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .try_init()
        .is_ok()
    {};
    info!("Subscriber initialized.");
}
