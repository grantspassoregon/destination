//! The `utils` module contains utility functions accessed by multiple data types, where declaring
//! a stand-alone function eliminates code duplication in different methods.
use crate::{AddressError, AddressErrorKind, Bincode, Csv, Io};
use indicatif::{ProgressBar, ProgressStyle};
use serde::de::{Deserialize, DeserializeOwned, Deserializer};
use serde::Serialize;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::Duration;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

/// Function for deserailizing ArcGIS data that may contain either empty (Null) fields, or fields
/// with string value "\<Null\>", either of which should translate to `None`.
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
pub fn to_csv<T: Serialize + Clone>(item: &mut [T], path: PathBuf) -> Result<(), AddressErrorKind> {
    match csv::Writer::from_path(&path) {
        Ok(mut wtr) => {
            for i in item {
                match wtr.serialize(i) {
                    Ok(_) => {}
                    Err(source) => {
                        return Err(Csv::new(path, source, line!(), file!().to_string()).into());
                    }
                }
            }
            match wtr.flush() {
                Ok(_) => {}
                Err(source) => {
                    return Err(Io::new(path, source, line!(), file!().to_string()).into());
                }
            }
            Ok(())
        }
        Err(source) => Err(Csv::new(path, source, line!(), file!().to_string()).into()),
    }
}

/// Generic function to deserialize data types from a CSV file.  Called by methods to avoid code
/// duplication.
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
        Err(source) => {
            let path = std::path::PathBuf::from(path.as_ref());
            Err(Io::new(path, source, line!(), file!().to_string()))
        }
    }
}

/// The `save` method serializes the contents of self into binary and writes to a file at
/// location `path`.  Errors bubble up from serialization in [`bincode`] or file system access during write.
pub fn to_bin<T: Serialize, P: AsRef<Path>>(data: &T, path: P) -> Result<(), AddressError> {
    info!("Serializing to binary.");
    let encode = match bincode::serialize(data) {
        Ok(enc) => enc,
        Err(source) => {
            let error = Bincode::new(source, line!(), file!().to_string());
            let error = AddressErrorKind::from(error);
            return Err(error.into());
        }
    };
    info!("Writing to file.");
    match std::fs::write(&path, encode) {
        Ok(_) => Ok(()),
        Err(source) => {
            let path = std::path::PathBuf::from(path.as_ref());
            Err(AddressErrorKind::from(Io::new(path, source, line!(), file!().to_string())).into())
        }
    }
}

/// The `load_bin` function loads the contents of a file at location `path` into a `Vec<u8>`.
/// May error reading the file, for example if the location is invalid, or when deserializing
/// the binary if the format is invalid.
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
        Err(source) => {
            let path = std::path::PathBuf::from(path.as_ref());
            Err(Io::new(path, source, line!(), file!().to_string()))
        }
    }
}

/// The `Csvify` trait indicates the type can be read from and to a csv file.
pub trait IntoCsv<T> {
    /// The `from_csv` method attempts to deserialize the data from a `csv` file located at `path`.
    fn from_csv<P: AsRef<Path>>(path: P) -> Result<T, Io>;
    /// The `to_csv` method attempts to serialize the data to a `csv` file at location `path`.
    fn to_csv<P: AsRef<Path>>(&mut self, path: P) -> Result<(), AddressErrorKind>;
}

/// The `Binnable` trait indicates the type can be read from and to a binary file.
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
                .unwrap_or_else(|_| "address=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .try_init()
        .is_ok()
    {};
    info!("Subscriber initialized.");
}
