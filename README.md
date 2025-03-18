# the `destination` crate

_A library providing types and methods for managing physical addresses in a municipality._

![Destination Logo](./data/destination_logo.jpg)
_Logo art created using Gemini AI._

## Project Goals

Critical public and private services depend upon reliable and accurate address information, from emergency response to sewer lines and internet. Historic address information is often poorly standardized, and as a result, modern address databases can present a challenge for parsing, comparison and search. The motivation for this project stemmed from the difficulty our staff experienced reconciling our address database with our emergency dispatch provider. The tools developed in response have helped us to assign and reduce discrepancies and improve accuracy. While developed for use in Grants Pass, Oregon, the core logic is designed to work with any municipality. Users can import their address data by implementing the `Address` trait on their own types, or by adhering to one of the current supported formats (e.g. the `common` format).

The purpose of this library is to facilitate the classification and organization of addresses for physical locations. We categorize addresses using elements from the FGDC and NENA specifications. The crate facilitates reconciliation of address databases through the `compare` module. Some functionality, such as the generation of LexisNexis tables, is tailored for local use by our staff, and not intended for wider use. This library is under active development, and may experience breaking changes in API.

Use this library to:

- Parse unstructured text to validated address data.
- Identify matching, divergent and missing addresses between two datasets.
- Geolocate records by address (business licenses, fire inspections, etc.).
- Compare the distance between matching addresses in two datasets.
- Identify duplicate addresses within a dataset.
- Generate the LexisNexis table for an address dataset.
