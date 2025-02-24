use criterion::{Criterion, criterion_group, criterion_main};
use destination::{
    GrantsPassSpatialAddresses, IntoCsv, JosephineCountySpatialAddresses2024, MatchRecords,
    SpatialAddresses,
};

pub fn compare_records(c: &mut Criterion) {
    let city_path = "data/city_addresses_20241007.csv";
    let county_path = "data/county_addresses_20241007.csv";
    let city_addresses = GrantsPassSpatialAddresses::from_csv(city_path).unwrap();
    let source_addresses = SpatialAddresses::from(&city_addresses[..]);
    let county_addresses = JosephineCountySpatialAddresses2024::from_csv(county_path).unwrap();
    let target_addresses = SpatialAddresses::from(&county_addresses[..]);
    let mut group = c.benchmark_group("throughput group");
    group.throughput(criterion::Throughput::Bytes(
        std::mem::size_of_val(&*source_addresses) as u64
            * std::mem::size_of_val(&*target_addresses) as u64,
    ));
    group.bench_function("compare records", |b| {
        b.iter(|| {
            MatchRecords::compare(&source_addresses[0..10], &target_addresses[0..1000]);
        })
    });
    group.finish();
}

criterion_group!(benches, compare_records);
criterion_main!(benches);
