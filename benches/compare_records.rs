use address::data::*;
use criterion::{criterion_group, criterion_main, Criterion};

pub fn compare_records(c: &mut Criterion) {
    let city_path = "c:/users/erose/documents/city_addresses.csv";
    let county_path = "c:/users/erose/documents/county_addresses.csv";
    let city_addresses = CityAddresses::from_csv(city_path).unwrap();
    let county_addresses = CountyAddresses::from_csv(county_path).unwrap();
    c.bench_function("compare records", |b| {
        b.iter(|| {
            MatchRecords::compare(
                city_addresses.clone().records[(0..10)].to_vec(),
                county_addresses.records.clone(),
            );
        })
    });
}

criterion_group!(benches, compare_records);
criterion_main!(benches);
