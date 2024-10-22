set shell := ["powershell.exe", "-c"]
set windows-shell := ["powershell.exe", "-c"]

# load city addresses and save to binary in the AMS data directory
load_city file="data/city_addresses_20241007.csv":
  cargo run --release -- -c save -s {{file}} -k grants_pass -o ../ams/data/addresses.data

# load county addresses and save to binary in the AMS data directory
load_county file="data/county_addresses_20241007.csv":
  cargo run --release -- -c save -s {{file}} -k josephine_county -o ../ams/data/county_addresses.data

drift:
  cargo run --release -- -c drift -s data/city_addresses_20241007.csv -k grants_pass -t data/county_addresses_20241007.csv -z josephine_county -o c:/users/erose/documents/drift.csv

orphans:
  cargo run --release -- -c orphan_streets -s data/city_addresses_20241007.csv -k grants_pass -t data/county_addresses_20241007.csv -z josephine_county

duplicates file="data/city_addresses_20241007.csv" type="grants_pass" out="duplicates.csv":
  cargo run --release -- -c duplicates -s {{file}} -k {{type}} -o {{out}}

business file="data/business_licenses_20240520.csv" compare="data/city_addresses_20241007.csv":
  cargo run --release -- -c business -s {{file}} -t {{compare}} -z grants_pass -o c:/users/erose/documents/business_match.csv
  cargo run --release -- -c filter -b -s c:/users/erose/documents/business_match.csv -f matching -o c:/users/erose/documents/business_matching.csv
  cargo run --release -- -c filter -b -s c:/users/erose/documents/business_match.csv -f divergent -o c:/users/erose/documents/business_divergent.csv
  cargo run --release -- -c filter -b -s c:/users/erose/documents/business_match.csv -f missing -o c:/users/erose/documents/business_missing.csv
