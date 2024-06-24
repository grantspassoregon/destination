use address::prelude::*;
use aid::prelude::*;
use tracing::{info, trace};

#[derive(Clone, serde::Deserialize)]
struct AddressSample {
    address: String,
    // zip: String,
}

#[test]
#[cfg_attr(feature = "ci", ignore)]
fn load_ecso_addresses() -> Clean<()> {
    if tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .try_init()
        .is_ok()
    {};
    info!("Subscriber initialized.");

    trace!("Deserializing county addresses from a csv file.");
    let file = "tests/test_data/county_addresses_20240508.csv";
    let addresses = JosephineCountySpatialAddresses2024::from_csv(file)?;
    assert_eq!(addresses.len(), 45205);
    trace!("City addresses loaded: {} entries.", addresses.len());
    let mut spatial = SpatialAddresses::from(&addresses[..]);
    info!("Addresses loaded: {}", spatial.len());
    spatial.citify();
    spatial.save("tests/test_data/county_addresses.data")?;
    // let addresses = GrantsPassAddresses::from_csv(file)?;
    // assert_eq!(addresses.records.len(), 27437);
    Ok(())
}

#[test]
#[cfg_attr(feature = "ci", ignore)]
fn load_city_addresses() -> Clean<()> {
    if tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .try_init()
        .is_ok()
    {};
    info!("Subscriber initialized.");

    trace!("Deserializing city addresses from a csv file.");
    let file = "tests/test_data/city_addresses_20240513.csv";
    let addresses = GrantsPassAddresses::from_csv(file)?;
    assert_eq!(addresses.len(), 27509);
    trace!("City addresses loaded: {} entries.", addresses.len());
    Ok(())
}

#[test]
#[cfg_attr(feature = "ci", ignore)]
fn save_city_addresses() -> Clean<()> {
    if tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .try_init()
        .is_ok()
    {};
    info!("Subscriber initialized.");

    trace!("Opening city addresses from a csv file.");
    let file = "tests/test_data/city_addresses_20240513.csv";
    let addresses = GrantsPassSpatialAddresses::from_csv(file)?;
    let addresses = SpatialAddresses::from(&addresses[..]);
    trace!("Saving city addresses to binary.");
    addresses.save("tests/test_data/addresses.data")?;
    Ok(())
}

// #[test]
// #[cfg_attr(feature = "ci", ignore)]
// fn save_county_addresses() -> Clean<()> {
//     if tracing_subscriber::fmt()
//         .with_max_level(tracing::Level::TRACE)
//         .try_init()
//         .is_ok()
//     {};
//     info!("Subscriber initialized.");
//
//     trace!("Opening county addresses from a csv file.");
//     let file = "c:/users/erose/documents/county_addresses_20240418.csv";
//     let addresses = JosephineCountySpatialAddresses::from_csv(file)?;
//     let addresses = SpatialAddresses::from(&addresses.records[..]);
//     trace!("Saving county addresses to binary.");
//     addresses.save("c:/users/erose/documents/county_addresses.data")?;
//     Ok(())
// }

#[test]
fn load_county_addresses() -> Clean<()> {
    if tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .try_init()
        .is_ok()
    {};
    info!("Subscriber initialized.");

    trace!("Deserializing county addresses from a csv file.");
    let file = "tests/test_data/county_addresses_20240226.csv";
    let addresses = JosephineCountyAddresses::from_csv(file)?;
    assert_eq!(addresses.len(), 45134);
    trace!("County addresses loaded: {} entries.", addresses.len());
    Ok(())
}

#[test]
fn load_geo_addresses() -> Clean<()> {
    if tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .try_init()
        .is_ok()
    {};
    info!("Subscriber initialized.");

    let file = "tests/test_data/city_addresses_20240513.csv";
    let addresses = GrantsPassSpatialAddresses::from_csv(file)?;
    let geo_addresses = GeoAddresses::from(&addresses[..]);
    assert_eq!(addresses.len(), geo_addresses.len());
    info!("Geo addresses loaded: {} entries.", geo_addresses.len());
    Ok(())
}

// #[test]
// fn read_gp2022_addresses() -> Result<(), std::io::Error> {
//     if tracing_subscriber::fmt()
//         .with_max_level(tracing::Level::TRACE)
//         .try_init()
//         .is_ok()
//     {};
//     info!("Subscriber initialized.");
//
//     let file = "c:/users/erose/Documents/addresses_2022.csv";
//     let addresses = GrantsPass2022Addresses::from_csv(file)?;
//     info!(
//         "City addresses loaded: {} entries.",
//         addresses.records.len()
//     );
//     let addresses = CommonAddresses::from(addresses);
//     info!(
//         "Addresses converted: {} entries.",
//         addresses.records_ref().len()
//     );
//     Ok(())
// }

#[test]
#[cfg_attr(feature = "ci", ignore)]
fn business_licenses() -> Clean<()> {
    if tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .try_init()
        .is_ok()
    {};
    info!("Subscriber initialized.");

    let file = "tests/test_data/business_licenses_20240520.csv";
    let licenses = BusinessLicenses::from_csv(file)?;
    info!("Business licenses loaded: {} entries.", licenses.len());
    let mut licenses = licenses.deduplicate();
    licenses.detype_subaddresses()?;
    info!(
        "Business licenses deduplicated: {} entries.",
        licenses.len()
    );
    let city_path = "tests/test_data/city_addresses_20240513.csv";
    let city_addresses = GrantsPassSpatialAddresses::from_csv(city_path)?;
    let mut match_records = BusinessMatchRecords::compare(&licenses, &city_addresses);
    info!("Match records: {}", match_records.len());
    // info!("{:?}", match_records.records[0]);
    match_records.to_csv("c:/users/erose/geojson/business_match.csv".into())?;
    let points = Businesses::from_csv("tests/test_data/business_points.csv")?;
    info!("Business points: {}", points.len());

    info!("{:?}", licenses[0]);
    let mut matching = match_records.clone().filter("matching");
    matching.to_csv("c:/users/erose/documents/business_matching.csv".into())?;
    let mut divergent = match_records.clone().filter("divergent");
    divergent.to_csv("c:/users/erose/documents/business_divergent.csv".into())?;
    let mut missing = match_records.clone().filter("missing");
    missing = missing.filter("local");
    missing.to_csv("c:/users/erose/documents/business_missing.csv".into())?;

    Ok(())
}

#[test]
#[cfg_attr(feature = "ci", ignore)]
fn read_bus_licenses() -> Result<(), std::io::Error> {
    if tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .try_init()
        .is_ok()
    {};
    info!("Subscriber initialized.");

    let file = "tests/test_data/active_business_licenses.csv";
    let licenses = BusinessLicenses::from_csv(file)?;
    info!("Business licenses loaded: {} entries.", licenses.len());
    // info!("Record 171:");
    // info!("{:?}", licenses.records[171]);
    assert!(licenses[0].pre_directional() == Some(StreetNamePreDirectional::NORTHEAST));
    info!("NE reads.");
    assert!(licenses[3].pre_directional() == Some(StreetNamePreDirectional::NORTHWEST));
    info!("NW reads.");
    assert!(licenses[1].pre_directional() == Some(StreetNamePreDirectional::SOUTHEAST));
    info!("SE reads.");
    assert!(licenses[109].pre_directional() == Some(StreetNamePreDirectional::SOUTHEAST));
    info!("SOUTHEAST reads.");
    assert!(licenses[0].post_type() == Some(StreetNamePostType::STREET));
    info!("ST reads.");
    assert!(licenses[1].post_type() == Some(StreetNamePostType::STREET));
    info!("St reads.");
    assert!(licenses[109].post_type() == Some(StreetNamePostType::STREET));
    info!("STREET reads.");
    assert!(licenses[171].post_type() == Some(StreetNamePostType::AVENUE));
    info!("Ave reads.");
    assert!(licenses[88].post_type() == Some(StreetNamePostType::BOULEVARD));
    info!("BOULEVARD reads.");
    assert!(licenses[134].post_type() == Some(StreetNamePostType::DRIVE));
    info!("Dr reads.");
    assert!(licenses[5].post_type() == Some(StreetNamePostType::HIGHWAY));
    info!("HWY reads.");
    assert!(licenses[214].post_type() == Some(StreetNamePostType::HIGHWAY));
    info!("Hwy reads.");
    assert!(licenses[405].post_type() == Some(StreetNamePostType::HIGHWAY));
    info!("HIGHWAY reads.");
    Ok(())
}

#[test]
fn match_city_address() -> Clean<()> {
    if tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .try_init()
        .is_ok()
    {};
    info!("Subscriber initialized.");
    // let city_path = "tests/test_data/city_addresses_20240226.csv";
    // let county_path = "tests/test_data/county_addresses_20240226.csv";
    let city_path = "tests/test_data/addresses.data";
    let county_path = "tests/test_data/county_addresses.data";
    let city_addresses = SpatialAddresses::load(city_path)?;
    assert_eq!(city_addresses.len(), 27509);
    let county_addresses = SpatialAddresses::load(county_path)?;
    assert_eq!(county_addresses.len(), 45205);
    info!("Matching single address.");
    let match_records = MatchRecords::new(&city_addresses[0].clone(), &county_addresses);
    info!("Record 0 is: {:?}", match_records[0]);
    Ok(())
}

#[test]
#[cfg_attr(feature = "ci", ignore)]
fn match_business_addresses() -> Clean<()> {
    if tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .try_init()
        .is_ok()
    {};
    info!("Subscriber initialized.");
    // let business_path = "tests/test_data/active_business_licenses.csv";
    let business_path = "tests/test_data/business_licenses_20240520.csv";
    let city_path = "tests/test_data/city_addresses_20240513.csv";
    let business_addresses = BusinessLicenses::from_csv(business_path)?;
    let city_addresses = GrantsPassSpatialAddresses::from_csv(city_path)?;
    let match_records = BusinessMatchRecords::compare(&business_addresses, &city_addresses);
    assert_eq!(match_records.len(), 6100);
    info!("Business addresses match against commmon addresses.");

    Ok(())
}

// #[test]
// fn match_business_address_chain() -> Result<(), std::io::Error> {
//     if tracing_subscriber::fmt()
//         .with_max_level(tracing::Level::TRACE)
//         .try_init()
//         .is_ok()
//     {};
//     info!("Subscriber initialized.");
//     let business_path = "c:/users/erose/documents/active_business_licenses.csv";
//     let city_path = "c:/users/erose/documents/addresses_20230411.csv";
//     let city2022_path = "c:/users/erose/documents/addresses_2022.csv";
//     let business_addresses = BusinessLicenses::from_csv(business_path)?;
//     let city_addresses = CityAddresses::from_csv(city_path)?;
//     let city2022_addresses = GrantsPass2022Addresses::from_csv(city2022_path)?;
//     let target_addresses = Addresses::from(city_addresses);
//     let other_addresses = Addresses::from(city2022_addresses);
//     let match_records = BusinessMatchRecords::compare_chain(
//         &business_addresses,
//         &[&target_addresses, &other_addresses],
//     );
//     info!("Records: {:?}", match_records.records.len());
//
//     Ok(())
// }

#[test]
fn match_city_addresses() -> Clean<()> {
    if tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .try_init()
        .is_ok()
    {};
    info!("Subscriber initialized.");
    let city_path = "./tests/test_data/addresses.data";
    let county_path = "./tests/test_data/county_addresses.data";
    let city_addresses = SpatialAddresses::load(city_path)?;
    let county_addresses = SpatialAddresses::load(county_path)?;
    let match_records = MatchRecords::compare(&city_addresses[0..10].to_vec(), &county_addresses);
    assert_eq!(match_records.len(), 10);
    Ok(())
}

#[test]
fn filter_status() -> Clean<()> {
    if tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .try_init()
        .is_ok()
    {};
    info!("Subscriber initialized.");
    let city_path = "tests/test_data/addresses.data";
    let county_path = "tests/test_data/county_addresses.data";
    let city_addresses = SpatialAddresses::load(city_path)?;
    let county_addresses = SpatialAddresses::load(county_path)?;
    let match_records = MatchRecords::compare(&city_addresses[0..1000].to_vec(), &county_addresses);
    assert_eq!(match_records.len(), 1000);
    let filtered = match_records.clone().filter("status");
    assert_eq!(filtered.len(), 967);
    info!("Matches filtered by status.");
    Ok(())
}

#[test]
fn filter_missing() -> Clean<()> {
    if tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .try_init()
        .is_ok()
    {};
    info!("Subscriber initialized.");
    let city_path = "tests/test_data/addresses.data";
    let county_path = "tests/test_data/county_addresses.data";
    let city_addresses = SpatialAddresses::load(city_path)?;
    let county_addresses = SpatialAddresses::load(county_path)?;
    let match_records = MatchRecords::compare(&city_addresses[0..1000].to_vec(), &county_addresses);
    assert_eq!(match_records.len(), 1000);
    let filtered = match_records.clone().filter("missing");
    assert_eq!(filtered.len(), 3);
    info!("Matches filtered by missing.");
    Ok(())
}

#[test]
fn address_number_parser() {
    let a1 = "1 FIRE MOUNTAIN WAY, Grants Pass";
    let a2 = "100 CENTURYLINK DR";
    let a3 = "100 LEWIS AVE, Grants Pass";
    assert_eq!(
        Parser::address_number(&a1),
        Ok((" FIRE MOUNTAIN WAY, Grants Pass", Some(1)))
    );
    assert_eq!(
        Parser::address_number(&a2),
        Ok((" CENTURYLINK DR", Some(100)))
    );
    assert_eq!(
        Parser::address_number(&a3),
        Ok((" LEWIS AVE, Grants Pass", Some(100)))
    );
}

#[test]
fn address_number_suffix_parser() {
    let a1 = "1/2 LEWIS AVE";
    let a2 = " 1/2 LEWIS AVE";
    let a3 = " 3/4 LEWIS AVE";
    let a4 = " LEWIS AVE";
    assert_eq!(
        Parser::address_number_suffix(a1),
        Ok((" LEWIS AVE", Some("1/2")))
    );
    assert_eq!(
        Parser::address_number_suffix(a2),
        Ok((" LEWIS AVE", Some("1/2")))
    );
    assert_eq!(
        Parser::address_number_suffix(a3),
        Ok((" LEWIS AVE", Some("3/4")))
    );
    assert_eq!(Parser::address_number_suffix(a4), Ok(("LEWIS AVE", None)));
}

#[test]
fn pre_directional_parser() {
    let a1 = "NW 6TH ST";
    let a2 = "LEWIS AVE";
    let a3 = " NW 6TH ST";
    assert_eq!(
        Parser::pre_directional(a1),
        Ok(("6TH ST", Some(StreetNamePreDirectional::NORTHWEST)))
    );
    assert_eq!(Parser::pre_directional(a2), Ok(("LEWIS AVE", None)));
    assert_eq!(
        Parser::pre_directional(a3),
        Ok(("6TH ST", Some(StreetNamePreDirectional::NORTHWEST)))
    );
}

#[test]
fn street_type_parser() {
    let a1 = " WAY, Grants Pass";
    let a2 = "DR";
    let a3 = " AVE, Grants Pass";
    assert_eq!(
        Parser::post_type(&a1),
        Ok((", Grants Pass", Some(StreetNamePostType::WAY)))
    );
    assert_eq!(
        Parser::post_type(&a2),
        Ok(("", Some(StreetNamePostType::DRIVE)))
    );
    assert_eq!(
        Parser::post_type(&a3),
        Ok((", Grants Pass", Some(StreetNamePostType::AVENUE)))
    );
}

#[test]
fn multi_word_parser() {
    if tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .try_init()
        .is_ok()
    {};
    let a1 = " FIRE MOUNTAIN WAY";
    let a2 = " CENTURYLINK DR";
    let a3 = " ROGUE RIVER AVE, Grants Pass";
    let a4 = " TOO LONG NAME LN";
    assert_eq!(
        Parser::street_name(a1),
        Ok(("WAY", Some("FIRE MOUNTAIN".to_string())))
    );
    assert_eq!(
        Parser::street_name(a2),
        Ok(("DR", Some("CENTURYLINK".to_string())))
    );
    assert_eq!(
        Parser::street_name(a3),
        Ok(("AVE, Grants Pass", Some("ROGUE RIVER".to_string())))
    );
    assert_eq!(
        Parser::street_name(a4),
        Ok(("LN", Some("TOO LONG NAME".to_string())))
    );
}

// #[test]
// fn recursive_post_type_parser() {
//     let a1 = " VIEW LN,";
//     let a2 = " GARDEN RD";
//     let a3 = " VIEW AVE Food Trailer";
//     assert_eq!(
//         Parser::post_type(a1),
//         Ok((
//             ",",
//             vec![StreetNamePostType::VIEW, StreetNamePostType::LANE]
//         ))
//     );
//     assert_eq!(
//         recursive_post_type(a2),
//         Ok((
//             "",
//             vec![StreetNamePostType::GARDEN, StreetNamePostType::ROAD]
//         ))
//     );
//     assert_eq!(
//         recursive_post_type(a3),
//         Ok((
//             " Food Trailer",
//             vec![StreetNamePostType::VIEW, StreetNamePostType::AVENUE]
//         ))
//     );
// }

// #[test]
// fn complete_street_name_parser() {
//     let a1 = " FIRE MOUNTAIN WAY";
//     let a2 = " NW CENTURYLINK DR";
//     let a3 = " MOUNTAIN VIEW AVE, Grants Pass";
//     let a4 = " MOUNTAIN VIEW AVE Food Trailer, Grants Pass";
//     assert_eq!(
//         parse_complete_street_name(a1),
//         Ok((
//             "",
//             (None, vec!["FIRE", "MOUNTAIN"], StreetNamePostType::WAY)
//         ))
//     );
//     assert_eq!(
//         parse_complete_street_name(a2),
//         Ok((
//             "",
//             (
//                 Some(StreetNamePreDirectional::NORTHWEST),
//                 vec!["CENTURYLINK"],
//                 StreetNamePostType::DRIVE
//             )
//         ))
//     );
//     assert_eq!(
//         parse_complete_street_name(a3),
//         Ok((
//             ", Grants Pass",
//             (None, vec!["MOUNTAIN", "VIEW"], StreetNamePostType::AVENUE)
//         ))
//     );
//     assert_eq!(
//         parse_complete_street_name(a4),
//         Ok((
//             " Food Trailer, Grants Pass",
//             (None, vec!["MOUNTAIN", "VIEW"], StreetNamePostType::AVENUE)
//         ))
//     );
// }

#[test]
fn subaddress_type_parser() {
    let a1 = " STE A";
    let a2 = " SUITE B";
    let a3 = "UNIT 1";
    let a4 = " #A";

    assert_eq!(
        Parser::subaddress_type(a1),
        Ok((" A", Some(SubaddressType::Suite)))
    );
    assert_eq!(
        Parser::subaddress_type(a2),
        Ok((" B", Some(SubaddressType::Suite)))
    );
    assert_eq!(
        Parser::subaddress_type(a3),
        Ok((" 1", Some(SubaddressType::Unit)))
    );
    assert_eq!(Parser::subaddress_type(a4), Ok((" #A", None)));
}

#[test]
fn subaddress_element_parser() {
    let a1 = " A";
    let a2 = " #B";
    let a3 = " #A & B";
    let a4 = " &";
    assert_eq!(Parser::subaddress_id(a1), Ok(("", Some("A".to_string()))));
    assert_eq!(Parser::subaddress_id(a2), Ok(("", Some("B".to_string()))));
    assert_eq!(Parser::subaddress_id(a3), Ok(("", Some("A B".to_string()))));
    assert_eq!(Parser::subaddress_id(a4), Ok((" &", None)));
}

#[test]
fn subaddress_elements_parser() {
    let a1 = " #A & B";
    let a2 = " Food Trailer";
    let a3 = "";
    assert_eq!(Parser::subaddress_id(a1), Ok(("", Some("A B".to_string()))));
    assert_eq!(
        Parser::subaddress_id(a2),
        Ok(("", Some("Food Trailer".to_string())))
    );
    assert_eq!(Parser::subaddress_id(a3), Ok(("", None)));
}

#[test]
fn subaddress_identifiers_parser() {
    let a1 = " A";
    let a2 = " #B, Grants Pass";
    let a3 = "";
    let a4 = " #A & B";
    // TODO: Subaddress ID does not parse apostrophes
    // let a5 = " Mac's";
    let a6 = " Food Trailer, Grants Pass";
    assert_eq!(Parser::subaddress_id(a1), Ok(("", Some("A".to_string()))));
    assert_eq!(
        Parser::subaddress_id(a2),
        Ok(("Grants Pass", Some("B".to_string())))
    );
    assert_eq!(Parser::subaddress_id(a3), Ok(("", None)));
    assert_eq!(Parser::subaddress_id(a4), Ok(("", Some("A B".to_string()))));
    // assert_eq!(
    //     Parser::subaddress_id(a5),
    //     Ok(("", Some("Mac's".to_string())))
    // );
    assert_eq!(
        Parser::subaddress_id(a6),
        Ok(("Grants Pass", Some("Food Trailer".to_string())))
    );
}

#[test]
fn address_parser() -> Clean<()> {
    let a1 = "1002 RAMSEY AVE, GRANTS PASS";
    let a2 = "1012 NW 6TH ST";
    let a3 = "1035 NE 6TH ST #B, GRANTS PASS";
    let a4 = "1072 ROGUE RIVER HWY #A & B, Grants Pass";
    let a5 = "932 SW MOUNTAIN VIEW AVE Food Trailer, Grants Pass";
    let a6 = "1650 1/2 NE TERRACE DR";
    let a7 = "212 NE SAVAGE ST STE A";

    let mut a1_comp = PartialAddress::default();
    a1_comp.set_address_number(1002);
    a1_comp.set_street_name("RAMSEY");
    a1_comp.set_post_type(&StreetNamePostType::AVENUE);
    a1_comp.postal_community = Some(PostalCommunity::GrantsPass);
    let (_, a1_parsed) = Parser::address(a1)?;

    let mut a2_comp = PartialAddress::default();
    a2_comp.set_address_number(1012);
    a2_comp.set_pre_directional(&StreetNamePreDirectional::NORTHWEST);
    a2_comp.set_street_name("6TH");
    a2_comp.set_post_type(&StreetNamePostType::STREET);
    let (_, a2_parsed) = Parser::address(a2)?;

    let mut a3_comp = PartialAddress::default();
    a3_comp.set_address_number(1035);
    a3_comp.set_pre_directional(&StreetNamePreDirectional::NORTHEAST);
    a3_comp.set_street_name("6TH");
    a3_comp.set_post_type(&StreetNamePostType::STREET);
    a3_comp.set_subaddress_identifier("B");
    a3_comp.postal_community = Some(PostalCommunity::GrantsPass);
    let (_, a3_parsed) = Parser::address(a3)?;

    let mut a4_comp = PartialAddress::default();
    a4_comp.set_address_number(1072);
    a4_comp.set_street_name("ROGUE RIVER");
    a4_comp.set_post_type(&StreetNamePostType::HIGHWAY);
    a4_comp.set_subaddress_identifier("A B");
    a4_comp.postal_community = Some(PostalCommunity::GrantsPass);
    let (_, a4_parsed) = Parser::address(a4)?;

    let mut a5_comp = PartialAddress::default();
    a5_comp.set_address_number(932);
    a5_comp.set_pre_directional(&StreetNamePreDirectional::SOUTHWEST);
    a5_comp.set_street_name("MOUNTAIN VIEW");
    a5_comp.set_post_type(&StreetNamePostType::AVENUE);
    a5_comp.set_subaddress_identifier("Food Trailer");
    a5_comp.postal_community = Some(PostalCommunity::GrantsPass);
    let (_, a5_parsed) = Parser::address(a5)?;

    let mut a6_comp = PartialAddress::default();
    a6_comp.set_address_number(1650);
    a6_comp.set_address_number_suffix(Some("1/2"));
    a6_comp.set_pre_directional(&StreetNamePreDirectional::NORTHEAST);
    a6_comp.set_street_name("TERRACE");
    a6_comp.set_post_type(&StreetNamePostType::DRIVE);
    let (_, a6_parsed) = Parser::address(a6)?;

    let mut a7_comp = PartialAddress::default();
    a7_comp.set_address_number(212);
    a7_comp.set_pre_directional(&StreetNamePreDirectional::NORTHEAST);
    a7_comp.set_street_name("SAVAGE");
    a7_comp.set_post_type(&StreetNamePostType::STREET);
    a7_comp.set_subaddress_type(&SubaddressType::Suite);
    a7_comp.set_subaddress_identifier("A");
    let (_, a7_parsed) = Parser::address(a7)?;

    assert_eq!(a1_parsed, a1_comp);
    assert_eq!(a2_parsed, a2_comp);
    assert_eq!(a3_parsed, a3_comp);
    assert_eq!(a4_parsed, a4_comp);
    assert_eq!(a5_parsed, a5_comp);
    assert_eq!(a6_parsed, a6_comp);
    assert_eq!(a7_parsed, a7_comp);
    Ok(())
}

#[test]
#[cfg_attr(feature = "ci", ignore)]
fn load_fire_inspections() -> Clean<()> {
    if let Ok(()) = tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .try_init()
    {};
    let file_path = "p:/fire_inspection.csv";
    let fire = FireInspections::from_csv(file_path)?;
    info!("First address: {:?}", fire[0]);
    Ok(())
}

// #[test]
// fn compare_fire_inspections() -> Clean<()> {
//     if let Ok(()) = tracing_subscriber::fmt()
//         .with_max_level(tracing::Level::TRACE)
//         .try_init()
//     {};
//     let file_path = "p:/fire_inspection.csv";
//     let fire = FireInspections::from_csv(file_path)?;
//     let path = std::env::current_dir()?;
//     let file_path = path.join("tests/test_data/city_addresses_20230626.csv");
//     let addresses = CityAddresses::from_csv(file_path)?;
//     let addresses = Addresses::from(addresses);
//     let mut compared =
//         FireInspectionMatchRecords::from(&FireInspectionMatches::compare(&fire, &addresses));
//     compared.to_csv("p:/fire_inspections_matched.csv".into())?;
//     info!("Total records: {}.", compared.records().len());
//
//     Ok(())
// }

#[test]
#[cfg_attr(feature = "ci", ignore)]
fn sort_fire_inspections() -> Clean<()> {
    let file_path = "p:/fire_inspections_matched.csv";
    let compared = FireInspectionMatchRecords::from_csv(file_path)?;
    let mut matching = compared.clone();
    matching.filter("matching");
    let mut divergent = compared.clone();
    divergent.filter("divergent");
    let mut missing = compared.clone();
    missing.filter("missing");
    matching.to_csv("p:/fire_inspections_matching.csv".into())?;
    divergent.to_csv("p:/fire_inspections_divergent.csv".into())?;
    missing.to_csv("p:/fire_inspections_missing.csv".into())?;
    Ok(())
}

#[test]
fn load_businesses() -> Clean<()> {
    if let Ok(()) = tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .try_init()
    {};
    let path = std::env::current_dir()?;
    let file_path = path.join("tests/test_data/business_points.csv");
    let data = Businesses::from_csv(file_path)?;
    assert_eq!(
        Some("C".to_owned()),
        data[18].address().subaddress_identifier()
    );
    info!("Parses subaddress identifier with #.");
    assert_eq!(
        Some("1/2".to_owned()),
        data[167].address().address_number_suffix()
    );
    info!("Parses address number suffix 1/2.");
    assert_eq!(
        Some(SubaddressType::Suite),
        data[216].address().subaddress_type()
    );
    info!("Parses subaddress type STE.");
    Ok(())
}

#[test]
#[cfg_attr(feature = "ci", ignore)]
fn parse_address_sample() -> Clean<()> {
    if let Ok(()) = tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .try_init()
    {};
    let path = std::env::current_dir()?;
    let file_path = path.join("tests/test_data/address_sample.csv");
    let samples: Vec<AddressSample> = address::prelude::from_csv(file_path)?;
    // let city_path = "./tests/test_data/addresses.data";
    // let city_addresses = SpatialAddresses::load(city_path)?;
    for sample in samples {
        let (_, address) = Parser::address(&sample.address)?;
        tracing::info!("{}", address.label());
    }
    // let match_records = MatchRecords::compare(
    //     &city_addresses.records[0..10].to_vec(),
    //     &county_addresses.records,
    // );
    Ok(())
}

#[test]
#[cfg_attr(feature = "ci", ignore)]
fn address_samples() -> Clean<()> {
    if let Ok(()) = tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .try_init()
    {};
    let path = std::env::current_dir()?;
    let file_path = path.join("tests/test_data/address_sample.csv");
    let samples: Vec<AddressSample> = address::prelude::from_csv(file_path)?;
    for sample in samples {
        let (_rem, address) = Parser::address(&sample.address)?;
        tracing::info!("{}", address.mailing());
        // tracing::info!("{}", address.complete_address());
    }
    // let city_path = "./tests/test_data/addresses.data";
    // let city_addresses = SpatialAddresses::load(city_path)?;
    // for sample in city_addresses.iter() {
    //     if sample.street_name_pre_modifier().is_some() {
    //         tracing::info!("{:?}", sample.label());
    //         let (_rem, address) = Parser::address(&sample.label())?;
    //         tracing::info!("{}", address.label());
    //     }
    // }
    // let match_records = MatchRecords::compare(
    //     &city_addresses.records[0..10].to_vec(),
    //     &county_addresses.records,
    // );
    Ok(())
}

#[test]
#[cfg_attr(feature = "ci", ignore)]
fn parse_city_address() -> Clean<()> {
    if let Ok(()) = tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .try_init()
    {};
    let city_path = "./tests/test_data/addresses.data";
    let city_addresses = SpatialAddresses::load(city_path)?;
    for sample in city_addresses.iter() {
        let label = Address::label(sample);
        let (_, address) = Parser::address(&label)?;
        let address_label = address.label();
        if label != address_label {
            tracing::info!("{}", label);
            tracing::info!("{}", address_label);
        }
        // assert_eq!(label, address.label());
    }
    Ok(())
}

#[test]
#[cfg_attr(feature = "ci", ignore)]
fn parse_county_address() -> Clean<()> {
    if let Ok(()) = tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .try_init()
    {};
    let county_path = "./tests/test_data/county_addresses.data";
    let county_addresses = SpatialAddresses::load(county_path)?;
    for sample in county_addresses.iter() {
        // if sample.street_name() == "PARK PLAZA" {
        let label = Address::label(sample);
        let (_, address) = Parser::address(&label)?;
        let address_label = address.label();
        if label != address_label {
            tracing::info!("{}", label);
            tracing::info!("{}", address_label);
        }
        //     tracing::info!("{:#?}", sample);
        // }
        // assert_eq!(label, address.label());
    }
    Ok(())
}

#[test]
#[cfg_attr(feature = "ci", ignore)]
fn business_mailing() -> Clean<()> {
    if tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .try_init()
        .is_ok()
    {};
    info!("Subscriber initialized.");

    let situs = "tests/test_data/business_licenses_20240520.csv";
    let situs = BusinessLicenses::from_csv(situs)?;
    info!("Business licenses loaded: {} entries.", situs.len());
    let mut situs = situs.deduplicate();
    situs.detype_subaddresses()?;
    info!("Business licenses deduplicated: {} entries.", situs.len());
    let mailing = "c:/users/erose/documents/business_licenses_mailing_20240530.csv";
    let mailing = BusinessLicenses::from_csv(mailing)?;
    info!("Business licenses loaded: {} entries.", mailing.len());
    let mut mailing = mailing.deduplicate();
    mailing.detype_subaddresses()?;
    info!("Business licenses deduplicated: {} entries.", mailing.len());

    let mut mail = Vec::new();
    for site in situs.iter() {
        let matching = mailing.clone().filter("license", &site.license());
        if !matching.is_empty() {
            mail.push(matching[0].clone());
        }
    }
    tracing::info!("Mailing list: {} records", mail.len());
    Ok(())
}
