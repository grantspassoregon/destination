use address::data::*;
use criterion::{criterion_group, criterion_main, Criterion};

pub fn compare_records(c: &mut Criterion) {
    let city_path = "c:/users/erose/documents/city_addresses.csv";
    let county_path = "c:/users/erose/documents/county_addresses.csv";
    let city_addresses = CityAddresses::from_csv(city_path).unwrap();
    let source_addresses = Addresses::from(city_addresses);
    let county_addresses = CountyAddresses::from_csv(county_path).unwrap();
    let target_addresses = Addresses::from(county_addresses);
    c.bench_function("compare records", |b| {
        b.iter(|| {
            MatchRecords::compare(
                &source_addresses.records[(0..10)].to_vec(),
                &target_addresses.records[(0..1000)],
            );
        })
    });
}

criterion_group!(benches, compare_records);
criterion_main!(benches);
