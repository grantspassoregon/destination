//! The `utils` module contains utility functions accessed by multiple data types, where declaring
//! a stand-alone function eliminates code duplication in different methods.
use aid::prelude::*;
use indicatif::{ProgressBar, ProgressStyle};
use serde::de::{Deserialize, DeserializeOwned, Deserializer};
use serde::Serialize;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::time::Duration;
use tracing::info;

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
pub fn to_csv<T: Serialize + Clone>(
    item: &mut Vec<T>,
    path: PathBuf,
) -> Result<(), std::io::Error> {
    let mut wtr = csv::Writer::from_path(path)?;
    for i in item.clone() {
        wtr.serialize(i)?;
    }
    wtr.flush()?;
    Ok(())
}

/// Generic function to deserialize data types from a CSV file.  Called by methods to avoid code
/// duplication.
pub fn from_csv<T: DeserializeOwned + Clone, P: AsRef<std::path::Path>>(
    path: P,
) -> Result<Vec<T>, std::io::Error> {
    let mut records = Vec::new();
    let file = std::fs::File::open(path)?;
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

/// The `save` method serializes the contents of self into binary and writes to a file at
/// location `path`.  Errors bubble up from serialization in [`bincode`] or file system access during write.
pub fn save<T: Serialize, P: AsRef<Path>>(data: &T, path: P) -> Clean<()> {
    info!("Serializing to binary.");
    let encode = bincode::serialize(data)?;
    info!("Writing to file.");
    std::fs::write(path, encode)?;
    Ok(())
}

/// The `load_bin` method loads the contents of a file at location `path` into a `Vec<u8>`.
/// May error reading the file, for example if the location is invalid, or when deserializing
/// the binary if the format is invalid.
pub fn load_bin<'a, P: AsRef<Path>>(path: P) -> Clean<Vec<u8>> {
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
    let vec = fs::read(path)?;
    bar.finish_with_message("Loaded!");
    Ok(vec)
}

pub trait Portable<T> {
    fn load<P: AsRef<Path>>(path: P) -> Clean<T>;
    fn save<P: AsRef<Path>>(&self, path: P) -> Clean<()>;
    fn from_csv<P: AsRef<Path>>(path: P) -> Clean<T>;
    fn to_csv<P: AsRef<Path>>(&mut self, path: P) -> Clean<()>;
}
