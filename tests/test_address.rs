use address::data::*;
use tracing::info;

#[test]
fn load_city_addresses() -> Result<(), std::io::Error> {
    match tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .try_init()
    {
        Ok(()) => {}
        Err(_) => {}
    };
    info!("Subscriber initialized.");

    let file = "p:/city_addresses.csv";
    let addresses = CityAddresses::from_csv(file)?;
    info!(
        "City addresses loaded: {} entries.",
        addresses.records.len()
    );
    Ok(())
}

#[test]
fn load_county_addresses() -> Result<(), std::io::Error> {
    match tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .try_init()
    {
        Ok(()) => {}
        Err(_) => {}
    };
    info!("Subscriber initialized.");

    let file = "p:/county_addresses.csv";
    let addresses = CountyAddresses::from_csv(file)?;
    info!(
        "City addresses loaded: {} entries.",
        addresses.records.len()
    );
    info!("Record 1059:");
    info!("{:?}", addresses.records[1058]);
    info!("Record 28091:");
    info!("{:?}", addresses.records[28090]);
    info!("Record 19424:");
    info!("{:?}", addresses.records[19423]);
    Ok(())
}

#[test]
fn match_city_address() -> Result<(), std::io::Error> {
    match tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .try_init()
    {
        Ok(()) => {}
        Err(_) => {}
    };
    info!("Subscriber initialized.");
    let city_path = "p:/city_addresses.csv";
    let county_path = "p:/county_addresses.csv";
    let city_addresses = CityAddresses::from_csv(city_path)?;
    let county_addresses = CountyAddresses::from_csv(county_path)?;
    let match_records =
        MatchRecords::new(city_addresses.records[0].clone(), county_addresses.records);
    info!("Record 0 is: {:?}", match_records.records[0]);

    Ok(())
}

#[test]
fn match_city_addresses() -> Result<(), std::io::Error> {
    match tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .try_init()
    {
        Ok(()) => {}
        Err(_) => {}
    };
    info!("Subscriber initialized.");
    let city_path = "p:/city_addresses.csv";
    let county_path = "p:/county_addresses.csv";
    let city_addresses = CityAddresses::from_csv(city_path)?;
    let county_addresses = CountyAddresses::from_csv(county_path)?;
    let match_records = MatchRecords::compare(
        city_addresses.records[(0..10)].to_vec(),
        county_addresses.records,
    );
    info!("Records: {:?}", match_records.records);

    Ok(())
}

#[test]
fn filter_status() -> Result<(), std::io::Error> {
    match tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .try_init()
    {
        Ok(()) => {}
        Err(_) => {}
    };
    info!("Subscriber initialized.");
    let city_path = "p:/city_addresses.csv";
    let county_path = "p:/county_addresses.csv";
    let city_addresses = CityAddresses::from_csv(city_path)?;
    let county_addresses = CountyAddresses::from_csv(county_path)?;
    let match_records = MatchRecords::compare(
        city_addresses.records[(0..10)].to_vec(),
        county_addresses.records,
    );
    let filtered = match_records.filter("status");
    info!("Records: {:?}", filtered.records);

    Ok(())
}

#[test]
fn filter_missing() -> Result<(), std::io::Error> {
    match tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .try_init()
    {
        Ok(()) => {}
        Err(_) => {}
    };
    info!("Subscriber initialized.");
    let city_path = "p:/city_addresses.csv";
    let county_path = "p:/county_addresses.csv";
    let city_addresses = CityAddresses::from_csv(city_path)?;
    let county_addresses = CountyAddresses::from_csv(county_path)?;
    let match_records = MatchRecords::compare(
        city_addresses.records[(0..100)].to_vec(),
        county_addresses.records,
    );
    let filtered = match_records.filter("missing");
    info!("Records: {:?}", filtered.records);

    Ok(())
}
