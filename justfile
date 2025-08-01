set shell := ["powershell.exe", "-c"]
set windows-shell := ["powershell.exe", "-c"]

# Variables
city := "data/grants_pass_addresses_20250731.csv"
county := "data/josephine_county_addresses_20250731.csv"
business := "data/business_licenses_20250317.csv"

default:
  @just --list --unsorted

# Load city addresses and save to binary in the data directory.
load_city file=city:
  cargo run --release -- -c save -s {{file}} -k common -o data/addresses.data

# Load county addresses and save to binary in the data directory.
load_county file=county:
  cargo run --release -- -c save -s {{file}} -k common -o data/county_addresses.data

# Calculate spatial deltas between matching addresses.
drift city=city county=county out="c:/users/erose/documents/drift.csv":
  cargo run --release -- -c drift -s {{city}} -k common -t {{county}} -z common -o {{out}}

# Find street names present in the City that are missing in the County dataset and may be mislabeled.
orphans city=city county=county:
  cargo run --release -- -c orphan_streets -s {{city}} -k common -t {{county}} -z common

# Find duplicate addresses with dataset 'file' from source 'type'.
duplicates file=city type="grants_pass" out="duplicates.csv":
  cargo run --release -- -c duplicates -s {{file}} -k {{type}} -o {{out}}

# Compare business licenses to city addresses and sort by matching, divergent and missing.
business file=business compare=city out="c:/users/erose/documents/":
  cargo run --release -- -c business -s {{file}} -t {{compare}} -z grants_pass -o {{out}}business_match.csv
  cargo run --release -- -c filter -s {{out}}business_match.csv -k "business" -f matching -o {{out}}business_matching.csv
  cargo run --release -- -c filter -s {{out}}business_match.csv -k "business" -f divergent -o {{out}}business_divergent.csv
  cargo run --release -- -c filter -s {{out}}business_match.csv -k "business" -f missing -o {{out}}business_missing.csv

filter_parcels parcels="../../documents/compare_parcels.csv" out="c:/users/erose/documents/":
  cargo run --release -- -c filter -s {{parcels}} -k "partial" -f matching -o {{out}}compare_parcels_matching.csv
  cargo run --release -- -c filter -s {{parcels}} -k "partial" -f divergent -o {{out}}compare_parcels_divergent.csv
  cargo run --release -- -c filter -s {{parcels}} -k "partial" -f missing -o {{out}}compare_parcels_missing.csv

