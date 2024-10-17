set shell := ["powershell.exe", "-c"]
set windows-shell := ["powershell.exe", "-c"]

load_city file="data/city_addresses_20241007.csv":
  cargo run --release -- -c save -s {{file}} -k grants_pass -o ../ams/data/addresses.data

load_county file="data/county_addresses_20241007.csv":
  cargo run --release -- -c save -s {{file}} -k josephine_county -o ../ams/data/county_addresses.data

drift:
  cargo run --release -- -c drift -s data/city_addresses_20241007.csv -k grants_pass -t data/county_addresses_20241007.csv -z josephine_county -o c:/users/erose/documents/drift.csv

orphans:
  cargo run --release -- -c orphan_streets -s data/city_addresses_20241007.csv -k grants_pass -t data/county_addresses_20241007.csv -z josephine_county

duplicates file="data/city_addresses_20241007.csv" type="grants_pass" out="duplicates.csv":
  cargo run --release -- -c duplicates -s {{file}} -k {{type}} -o {{out}}
