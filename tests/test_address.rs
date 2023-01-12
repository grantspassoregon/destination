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
    info!("City addresses loaded: {} entries.", addresses.records.len());
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
    info!("City addresses loaded: {} entries.", addresses.records.len());
    info!("Record 1059:");
    info!("{:?}", addresses.records[1058]);
    info!("Record 28091:");
    info!("{:?}", addresses.records[28090]);
    info!("Record 19424:");
    info!("{:?}", addresses.records[19423]);
    Ok(())
}
