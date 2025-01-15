use address::{
    from_csv, Address, Addresses, BusinessLicenses, BusinessMatchRecords, Businesses,
    FireInspectionMatchRecords, FireInspections, GeoAddresses, GrantsPassAddresses,
    GrantsPassSpatialAddresses, IntoBin, IntoCsv, Io, JosephineCountyAddresses, MatchRecords, Nom,
    Parser, PartialAddress, PostalCommunity, SpatialAddresses, StreetNamePostType,
    StreetNamePreDirectional, SubaddressType,
};
use test_log::test;
use tracing::{info, trace};

#[derive(Clone, serde::Deserialize)]
struct AddressSample {
    address: String,
    // zip: String,
}

#[test]
// Loads city addresses and prints the length
fn load_city_addresses() -> anyhow::Result<()> {
    trace!("Deserializing city addresses from a csv file.");
    let file = "data/city_addresses_20240513.csv";
    let addresses = GrantsPassAddresses::from_csv(file)?;
    assert_eq!(addresses.len(), 27502);
    trace!("City addresses loaded: {} entries.", addresses.len());
    Ok(())
}

// Load JosephineCountyAddresses type
#[test]
fn load_county_addresses() -> anyhow::Result<()> {
    trace!("Deserializing county addresses from a csv file.");
    let file = "data/county_addresses_20240226.csv";
    let addresses = JosephineCountyAddresses::from_csv(file)?;
    assert_eq!(addresses.len(), 45134);
    trace!("County addresses loaded: {} entries.", addresses.len());
    Ok(())
}

// Load city addresses as GeoAddresses
#[test]
fn load_geo_addresses() -> anyhow::Result<()> {
    let file = "data/city_addresses_20241007.csv";
    let addresses = GrantsPassSpatialAddresses::from_csv(file)?;
    let geo_addresses = GeoAddresses::from(&addresses[..]);
    // let mut trial = CommonAddresses::from(&addresses[..]);
    // trial.to_csv("data/comp.csv")?;
    assert_eq!(addresses.len(), geo_addresses.len());
    info!("Geo addresses loaded: {} entries.", geo_addresses.len());
    Ok(())
}

#[test]
fn read_bus_licenses() -> Result<(), Io> {
    let file = "data/active_business_licenses.csv";
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
fn match_city_address() -> anyhow::Result<()> {
    let city_path = "data/addresses.data";
    let county_path = "data/county_addresses.data";
    let city_addresses = SpatialAddresses::load(city_path)?;
    assert_eq!(city_addresses.len(), 27818);
    let county_addresses = SpatialAddresses::load(county_path)?;
    assert_eq!(county_addresses.len(), 45564);
    info!("Matching single address.");
    let match_records = MatchRecords::new(&city_addresses[0].clone(), &county_addresses);
    info!("Record 0 is: {:?}", match_records[0]);
    Ok(())
}

#[test]
fn match_business_addresses() -> anyhow::Result<()> {
    let business_path = "data/business_licenses_20240520.csv";
    let city_path = "data/city_addresses_20240513.csv";
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
fn match_city_addresses() -> anyhow::Result<()> {
    let city_path = "data/addresses.data";
    let county_path = "data/county_addresses.data";
    let city_addresses = SpatialAddresses::load(city_path)?;
    let county_addresses = SpatialAddresses::load(county_path)?;
    let match_records = MatchRecords::compare(&city_addresses[0..10], &county_addresses);
    assert_eq!(match_records.len(), 10);
    Ok(())
}

#[test]
fn filter_status() -> anyhow::Result<()> {
    let city_path = "data/addresses.data";
    let county_path = "data/county_addresses.data";
    let city_addresses = SpatialAddresses::load(city_path)?;
    let county_addresses = SpatialAddresses::load(county_path)?;
    let match_records = MatchRecords::compare(&city_addresses[0..1000], &county_addresses);
    assert_eq!(match_records.len(), 1000);
    let filtered = match_records.clone().filter("status");
    assert_eq!(filtered.len(), 965);
    info!("Matches filtered by status.");
    Ok(())
}

#[test]
fn filter_missing() -> anyhow::Result<()> {
    let city_path = "data/addresses.data";
    let county_path = "data/county_addresses.data";
    let city_addresses = SpatialAddresses::load(city_path)?;
    let county_addresses = SpatialAddresses::load(county_path)?;
    let match_records = MatchRecords::compare(&city_addresses[0..1000], &county_addresses);
    assert_eq!(match_records.len(), 1000);
    let filtered = match_records.clone().filter("missing");
    assert_eq!(filtered.len(), 0);
    info!("Matches filtered by missing.");
    Ok(())
}

#[test]
fn address_number_parser() {
    let a1 = "1 FIRE MOUNTAIN WAY, Grants Pass";
    let a2 = "100 CENTURYLINK DR";
    let a3 = "100 LEWIS AVE, Grants Pass";
    assert_eq!(
        Parser::address_number(a1),
        Ok((" FIRE MOUNTAIN WAY, Grants Pass", Some(1)))
    );
    assert_eq!(
        Parser::address_number(a2),
        Ok((" CENTURYLINK DR", Some(100)))
    );
    assert_eq!(
        Parser::address_number(a3),
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
        Parser::post_type(a1),
        Ok((", Grants Pass", Some(StreetNamePostType::WAY)))
    );
    assert_eq!(
        Parser::post_type(a2),
        Ok(("", Some(StreetNamePostType::DRIVE)))
    );
    assert_eq!(
        Parser::post_type(a3),
        Ok((", Grants Pass", Some(StreetNamePostType::AVENUE)))
    );
}

#[test]
fn multi_word_parser() {
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
fn address_parser() -> anyhow::Result<()> {
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
fn load_fire_inspections() -> anyhow::Result<()> {
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
fn sort_fire_inspections() -> anyhow::Result<()> {
    let file_path = "p:/fire_inspections_matched.csv";
    let compared = FireInspectionMatchRecords::from_csv(file_path)?;
    let mut matching = compared.clone();
    matching.filter("matching");
    let mut divergent = compared.clone();
    divergent.filter("divergent");
    let mut missing = compared.clone();
    missing.filter("missing");
    matching.to_csv("p:/fire_inspections_matching.csv")?;
    divergent.to_csv("p:/fire_inspections_divergent.csv")?;
    missing.to_csv("p:/fire_inspections_missing.csv")?;
    Ok(())
}

#[test]
fn load_businesses() -> anyhow::Result<()> {
    let path = std::env::current_dir()?;
    let file_path = path.join("data/business_points.csv");
    let data = Businesses::from_raw_csv(file_path)?;
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
fn parse_address_sample() -> anyhow::Result<()> {
    let path = std::env::current_dir()?;
    let file_path = path.join("data/address_sample.csv");
    let samples: Vec<AddressSample> = from_csv(file_path)?;
    for sample in samples {
        match Parser::address(&sample.address) {
            Ok((_, address)) => {
                tracing::info!("{}", address.label());
                tracing::info!("{}", address.mailing());
            }
            Err(source) => {
                return Err(
                    Nom::new(sample.address.clone(), source, line!(), file!().to_string()).into(),
                )
            }
        }
    }
    Ok(())
}

// Checks that city address labels parse back to their parent address
#[test]
#[cfg_attr(feature = "ci", ignore)]
fn parse_city_address() -> anyhow::Result<()> {
    let city_path = "data/addresses.data";
    let city_addresses = SpatialAddresses::load(city_path)?;
    for sample in city_addresses.iter() {
        // if sample.street_name().as_str() == "GARDEN VALLEY" {
        let label = Address::label(sample);
        match Parser::address(&label) {
            Ok((_, address)) => {
                let address_label = address.label();
                if label != address_label {
                    tracing::info!("{}", label);
                    tracing::info!("{}", address_label);
                }
            }
            Err(source) => {
                return Err(Nom::new(label.clone(), source, line!(), file!().to_string()).into())
            }
        }
        // }
        // assert_eq!(label, address.label());
    }
    Ok(())
}

// Checks that county address labels parse back to their parent address
#[test]
#[cfg_attr(feature = "ci", ignore)]
fn parse_county_address() -> anyhow::Result<()> {
    let county_path = "data/county_addresses.data";
    let mut county_addresses = SpatialAddresses::load(county_path)?;
    tracing::info!("Standardizing county addresses.");
    county_addresses.standardize();

    for sample in county_addresses.iter() {
        // if sample.street_name().as_str() == "REDWOOD" && sample.number() == 3345 {
        let label = Address::label(sample);
        match Parser::address(&label) {
            Ok((_, mut address)) => {
                address.standardize();
                let address_label = address.label();
                if label != address_label {
                    // tracing::info!("Street name: {:?}", sample.street_name());
                    // tracing::info!("Street directional: {:?}", sample.directional());
                    tracing::info!("OG Address: {:?}", sample);
                    tracing::info!("Parsed Address: {:?}", address);
                    // Native label
                    tracing::info!("{}", label);
                    // Parsed label
                    tracing::info!("{}", address_label);
                }
            }
            Err(source) => {
                return Err(Nom::new(label.clone(), source, line!(), file!().to_string()).into())
            }
        }
    }
    Ok(())
}

#[test]
#[cfg_attr(feature = "ci", ignore)]
fn business_mailing() -> anyhow::Result<()> {
    let situs = "data/business_licenses_20240520.csv";
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
