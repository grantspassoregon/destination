use address::address::*;
use address::address_components::*;
use address::business::*;
use address::compare::*;
use address::error::AddressError;
use address::import::*;
use address::parser::*;
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

    let file = "c:/users/erose/documents/addresses_20230411.csv";
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
fn read_gp2022_addresses() -> Result<(), std::io::Error> {
    if let Ok(()) = tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .try_init()
    {};
    info!("Subscriber initialized.");

    let file = "c:/users/erose/Documents/addresses_2022.csv";
    let addresses = GrantsPass2022Addresses::from_csv(file)?;
    info!(
        "City addresses loaded: {} entries.",
        addresses.records.len()
    );
    let addresses = Addresses::from(addresses);
    info!("Addresses converted: {} entries.", addresses.records.len());
    Ok(())
}

#[test]
fn read_business_licenses() -> Result<(), std::io::Error> {
    if let Ok(()) = tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .try_init()
    {};
    info!("Subscriber initialized.");

    let file = "c:/users/erose/Documents/active_business_licenses.csv";
    let licenses = BusinessLicenses::from_csv(file)?;
    info!(
        "Business licenses loaded: {} entries.",
        licenses.records.len()
    );
    // info!("Record 171:");
    // info!("{:?}", licenses.records[171]);
    assert!(licenses.records[0].pre_directional() == Some(StreetNamePreDirectional::NORTHEAST));
    info!("NE reads.");
    assert!(licenses.records[3].pre_directional() == Some(StreetNamePreDirectional::NORTHWEST));
    info!("NW reads.");
    assert!(licenses.records[1].pre_directional() == Some(StreetNamePreDirectional::SOUTHEAST));
    info!("SE reads.");
    assert!(licenses.records[109].pre_directional() == Some(StreetNamePreDirectional::SOUTHEAST));
    info!("SOUTHEAST reads.");
    assert!(licenses.records[0].post_type() == Some(StreetNamePostType::STREET));
    info!("ST reads.");
    assert!(licenses.records[1].post_type() == Some(StreetNamePostType::STREET));
    info!("St reads.");
    assert!(licenses.records[109].post_type() == Some(StreetNamePostType::STREET));
    info!("STREET reads.");
    assert!(licenses.records[171].post_type() == Some(StreetNamePostType::AVENUE));
    info!("Ave reads.");
    assert!(licenses.records[88].post_type() == Some(StreetNamePostType::BOULEVARD));
    info!("BOULEVARD reads.");
    assert!(licenses.records[134].post_type() == Some(StreetNamePostType::DRIVE));
    info!("Dr reads.");
    assert!(licenses.records[5].post_type() == Some(StreetNamePostType::HIGHWAY));
    info!("HWY reads.");
    assert!(licenses.records[214].post_type() == Some(StreetNamePostType::HIGHWAY));
    info!("Hwy reads.");
    assert!(licenses.records[405].post_type() == Some(StreetNamePostType::HIGHWAY));
    info!("HIGHWAY reads.");
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
    let source_addresses = Addresses::from(city_addresses);
    let county_addresses = CountyAddresses::from_csv(county_path)?;
    let target_addresses = Addresses::from(county_addresses);
    let match_records = MatchRecords::new(
        &source_addresses.records[0].clone(),
        &target_addresses.records,
    );
    info!("Record 0 is: {:?}", match_records.records[0]);

    Ok(())
}

#[test]
fn match_business_addresses() -> Result<(), std::io::Error> {
    if let Ok(()) = tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .try_init()
    {};
    info!("Subscriber initialized.");
    let business_path = "c:/users/erose/documents/active_business_licenses.csv";
    let city_path = "c:/users/erose/documents/addresses_20230411.csv";
    let business_addresses = BusinessLicenses::from_csv(business_path)?;
    let city_addresses = CityAddresses::from_csv(city_path)?;
    let target_addresses = Addresses::from(city_addresses);
    let match_records = BusinessMatchRecords::compare(&business_addresses, &target_addresses);
    info!("Records: {:?}", match_records.records.len());

    Ok(())
}

#[test]
fn match_business_address_chain() -> Result<(), std::io::Error> {
    if let Ok(()) = tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .try_init()
    {};
    info!("Subscriber initialized.");
    let business_path = "c:/users/erose/documents/active_business_licenses.csv";
    let city_path = "c:/users/erose/documents/addresses_20230411.csv";
    let city2022_path = "c:/users/erose/documents/addresses_2022.csv";
    let business_addresses = BusinessLicenses::from_csv(business_path)?;
    let city_addresses = CityAddresses::from_csv(city_path)?;
    let city2022_addresses = GrantsPass2022Addresses::from_csv(city2022_path)?;
    let target_addresses = Addresses::from(city_addresses);
    let other_addresses = Addresses::from(city2022_addresses);
    let match_records = BusinessMatchRecords::compare_chain(
        &business_addresses,
        &[&target_addresses, &other_addresses],
    );
    info!("Records: {:?}", match_records.records.len());

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
    let source_addresses = Addresses::from(city_addresses);
    let county_addresses = CountyAddresses::from_csv(county_path)?;
    let target_addresses = Addresses::from(county_addresses);
    let match_records = MatchRecords::compare(
        &source_addresses.records[(0..10)].to_vec(),
        &target_addresses.records,
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
    let source_addresses = Addresses::from(city_addresses);
    let county_addresses = CountyAddresses::from_csv(county_path)?;
    let target_addresses = Addresses::from(county_addresses);
    let match_records = MatchRecords::compare(
        &source_addresses.records[(0..10)].to_vec(),
        &target_addresses.records,
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
    let source_addresses = Addresses::from(city_addresses);
    let county_addresses = CountyAddresses::from_csv(county_path)?;
    let target_addresses = Addresses::from(county_addresses);
    let match_records = MatchRecords::compare(
        &source_addresses.records[(0..100)].to_vec(),
        &target_addresses.records,
    );
    let filtered = match_records.filter("missing");
    info!("Records: {:?}", filtered.records);

    Ok(())
}

#[test]
fn address_number_parser() {
    let a1 = "1 FIRE MOUNTAIN WAY, Grants Pass";
    let a2 = "100 CENTURYLINK DR";
    let a3 = "100 LEWIS AVE, Grants Pass";
    assert_eq!(
        parse_address_number(&a1),
        Ok((" FIRE MOUNTAIN WAY, Grants Pass", 1))
    );
    assert_eq!(parse_address_number(&a2), Ok((" CENTURYLINK DR", 100)));
    assert_eq!(
        parse_address_number(&a3),
        Ok((" LEWIS AVE, Grants Pass", 100))
    );
}

#[test]
fn pre_directional_parser() {
    let a1 = "NW 6TH ST";
    let a2 = "LEWIS AVE";
    let a3 = " NW 6TH ST";
    assert_eq!(
        parse_pre_directional(a1),
        Ok((" 6TH ST", Some(StreetNamePreDirectional::NORTHWEST)))
    );
    assert_eq!(parse_pre_directional(a2), Ok(("LEWIS AVE", None)));
    assert_eq!(
        parse_pre_directional(a3),
        Ok((" 6TH ST", Some(StreetNamePreDirectional::NORTHWEST)))
    );
}

#[test]
fn street_name_parser() {
    let a1 = "LEWIS AVE, Grants Pass";
    let a2 = "NW 6TH ST, Grants Pass";
    let a3 = " CENTURYLINK DR";
    assert_eq!(
        parse_street_name(a1),
        Ok((" AVE, Grants Pass", (None, "LEWIS")))
    );
    assert_eq!(
        parse_street_name(a2),
        Ok((
            " ST, Grants Pass",
            (Some(StreetNamePreDirectional::NORTHWEST), "6TH")
        ))
    );
    assert_eq!(parse_street_name(a3), Ok((" DR", (None, "CENTURYLINK"))));
}

#[test]
fn street_type_parser() {
    let a1 = " WAY, Grants Pass";
    let a2 = "DR";
    let a3 = " AVE, Grants Pass";
    assert_eq!(
        parse_post_type(&a1),
        Ok((", Grants Pass", Some(StreetNamePostType::WAY)))
    );
    assert_eq!(
        parse_post_type(&a2),
        Ok(("", Some(StreetNamePostType::DRIVE)))
    );
    assert_eq!(
        parse_post_type(&a3),
        Ok((", Grants Pass", Some(StreetNamePostType::AVENUE)))
    );
}

#[test]
fn multi_word_parser() {
    let a1 = " FIRE MOUNTAIN WAY";
    let a2 = " CENTURYLINK DR";
    let a3 = " ROGUE RIVER AVE, Grants Pass";
    let a4 = " TOO LONG NAME LN";
    assert_eq!(multi_word(a1), Ok((" WAY", vec!["FIRE", "MOUNTAIN"])));
    assert_eq!(multi_word(a2), Ok((" DR", vec!["CENTURYLINK"])));
    assert_eq!(
        multi_word(a3),
        Ok((" AVE, Grants Pass", vec!["ROGUE", "RIVER"]))
    );
    assert_eq!(multi_word(a4), Ok((" LN", vec!["TOO", "LONG", "NAME"])));
}

#[test]
fn recursive_post_type_parser() {
    let a1 = " VIEW LN,";
    let a2 = " GARDEN RD";
    let a3 = " VIEW AVE Food Trailer";
    assert_eq!(
        recursive_post_type(a1),
        Ok((
            ",",
            vec![StreetNamePostType::VIEW, StreetNamePostType::LANE]
        ))
    );
    assert_eq!(
        recursive_post_type(a2),
        Ok((
            "",
            vec![StreetNamePostType::GARDEN, StreetNamePostType::ROAD]
        ))
    );
    assert_eq!(
        recursive_post_type(a3),
        Ok((
            " Food Trailer",
            vec![StreetNamePostType::VIEW, StreetNamePostType::AVENUE]
        ))
    );
}

#[test]
fn complete_street_name_parser() {
    let a1 = " FIRE MOUNTAIN WAY";
    let a2 = " NW CENTURYLINK DR";
    let a3 = " MOUNTAIN VIEW AVE, Grants Pass";
    let a4 = " MOUNTAIN VIEW AVE Food Trailer, Grants Pass";
    assert_eq!(
        parse_complete_street_name(a1),
        Ok((
            "",
            (None, vec!["FIRE", "MOUNTAIN"], StreetNamePostType::WAY)
        ))
    );
    assert_eq!(
        parse_complete_street_name(a2),
        Ok(("", (Some(StreetNamePreDirectional::NORTHWEST), vec!["CENTURYLINK"], StreetNamePostType::DRIVE)))
    );
    assert_eq!(
        parse_complete_street_name(a3),
        Ok((
            ", Grants Pass",
            (None, vec!["MOUNTAIN", "VIEW"], StreetNamePostType::AVENUE)
        ))
    );
    assert_eq!(
        parse_complete_street_name(a4),
        Ok((
            " Food Trailer, Grants Pass",
            (None, vec!["MOUNTAIN", "VIEW"], StreetNamePostType::AVENUE)
        ))
    );
}

#[test]
fn subaddress_element_parser() {
    let a1 = " A";
    let a2 = " #B";
    let a3 = " #A & B";
    let a4 = " &";
    assert_eq!(parse_subaddress_element(a1), Ok(("", Some("A"))));
    assert_eq!(parse_subaddress_element(a2), Ok(("", Some("B"))));
    assert_eq!(parse_subaddress_element(a3), Ok((" & B", Some("A"))));
    assert_eq!(parse_subaddress_element(a4), Ok(("", None)));
}

#[test]
fn subaddress_elements_parser() {
    let a1 = " #A & B";
    let a2 = " Food Trailer";
    let a3 = "";
    assert_eq!(parse_subaddress_elements(a1), Ok(("", vec!["A", "B"])));
    assert_eq!(parse_subaddress_elements(a2), Ok(("", vec!["Food", "Trailer"])));
    assert_eq!(parse_subaddress_elements(a3), Ok(("", Vec::new())));
}

#[test]
fn subaddress_parser() {
    let a1 = " A";
    let a2 = " #B, Grants Pass";
    let a3 = "";
    let a4 = " #A & B";
    let a5 = " Mac's";
    let a6 = " Food Trailer, Grants Pass";
    assert_eq!(parse_subaddress(a3), Ok(("", None)));
    assert_eq!(parse_subaddress(a1), Ok(("", Some(vec!["A"]))));
    assert_eq!(parse_subaddress(a2), Ok((", Grants Pass", Some(vec!["B"]))));
    assert_eq!(parse_subaddress(a4), Ok(("", Some(vec!["A", "B"]))));
    assert_eq!(parse_subaddress(a5), Ok(("", Some(vec!["Mac's"]))));
    assert_eq!(parse_subaddress(a6), Ok((", Grants Pass", Some(vec!["Food", "Trailer"]))));
}

#[test]
fn address_parser() {
    let a1 = "1002 RAMSEY AVE, GRANTS PASS";
    let a2 = "1012 NW 6TH ST";
    let a3 = "1035 NE 6TH ST #B, GRANTS PASS";
    let a4 = "1072 ROGUE RIVER HWY #A & B, Grants Pass";
    let a5 = "932 SW MOUNTAIN VIEW AVE Food Trailer, Grants Pass";

    let mut a1_comp = PartialAddress::new();
    a1_comp.set_address_number(1002);
    a1_comp.set_street_name("RAMSEY");
    a1_comp.set_post_type(&StreetNamePostType::AVENUE);
    let (_, a1_parsed) = parse_address(a1).unwrap();

    let mut a2_comp = PartialAddress::new();
    a2_comp.set_address_number(1012);
    a2_comp.set_pre_directional(&StreetNamePreDirectional::NORTHWEST);
    a2_comp.set_street_name("6TH");
    a2_comp.set_post_type(&StreetNamePostType::STREET);
    let (_, a2_parsed) = parse_address(a2).unwrap();

    let mut a3_comp = PartialAddress::new();
    a3_comp.set_address_number(1035);
    a3_comp.set_pre_directional(&StreetNamePreDirectional::NORTHEAST);
    a3_comp.set_street_name("6TH");
    a3_comp.set_post_type(&StreetNamePostType::STREET);
    a3_comp.set_subaddress_identifier("B");
    let (_, a3_parsed) = parse_address(a3).unwrap();

    let mut a4_comp = PartialAddress::new();
    a4_comp.set_address_number(1072);
    a4_comp.set_street_name("ROGUE RIVER");
    a4_comp.set_post_type(&StreetNamePostType::HIGHWAY);
    a4_comp.set_subaddress_identifier("A B");
    let (_, a4_parsed) = parse_address(a4).unwrap();

    let mut a5_comp = PartialAddress::new();
    a5_comp.set_address_number(932);
    a5_comp.set_pre_directional(&StreetNamePreDirectional::SOUTHWEST);
    a5_comp.set_street_name("MOUNTAIN VIEW");
    a5_comp.set_post_type(&StreetNamePostType::AVENUE);
    a5_comp.set_subaddress_identifier("Food Trailer");
    let (_, a5_parsed) = parse_address(a5).unwrap();

    assert_eq!(a1_parsed, a1_comp);
    assert_eq!(a2_parsed, a2_comp);
    assert_eq!(a3_parsed, a3_comp);
    assert_eq!(a4_parsed, a4_comp);
    assert_eq!(a5_parsed, a5_comp);
}

#[test]
fn load_fire_inspections() -> Result<(), AddressError> {
    if let Ok(()) = tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .try_init()
    {};
    let file_path = "p:/fire_inspection.csv";
    let fire = FireInspections::from_csv(file_path)?;
    info!("First address: {:?}", fire.records()[0]);
    Ok(())
}

#[test]
fn compare_fire_inspections() -> Result<(), AddressError> {
    if let Ok(()) = tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .try_init()
    {};
    let file_path = "p:/fire_inspection.csv";
    let fire = FireInspections::from_csv(file_path)?;
    let fire = PartialAddresses::from(&fire);

    let file_path = "c:/users/erose/documents/addresses_20230411.csv";
    let addresses = CityAddresses::from_csv(file_path)?;
    let addresses = Addresses::from(addresses);
    let match_records = MatchPartialRecords::compare_partial(&fire, &addresses);
    info!("First match is: {:?}", match_records.records()[0]);
    info!("Match 100 is: {:?}", match_records.records()[99]);

    Ok(())
}
