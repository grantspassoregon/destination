use clap::Parser;
use destination::{
    trace_init, Addresses, BusinessLicenses, BusinessMatchRecords, Cartesian, Cli, CommonAddresses,
    GeoAddresses, GrantsPassAddresses, GrantsPassSpatialAddresses, IntoBin, IntoCsv,
    JosephineCountyAddresses, JosephineCountyAddresses2024, JosephineCountySpatialAddresses2024,
    LexisNexis, MatchPartialRecords, MatchRecords, SpatialAddress, SpatialAddresses,
    SpatialAddressesRaw,
};
use tracing::{error, info, trace, warn};

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    trace_init();

    match cli.command.as_str() {
        "filter" => {
            if let Some(filter) = cli.filter {
                info!("Filtering records.");
                if let Some(source) = cli.source_type {
                    match source.as_str() {
                        "business" => {
                            let match_records = BusinessMatchRecords::from_csv(cli.source.clone())?;
                            info!("Source records read: {} entries.", match_records.len());
                            let mut filtered = match_records.filter(&filter);
                            info!("Records remaining: {} entries.", filtered.len());
                            filtered.to_csv(cli.output)?;
                        }
                        "partial" => {
                            let match_records = MatchPartialRecords::from_csv(cli.source.clone())?;
                            info!("Source records read: {} entries.", match_records.len());
                            let mut filtered = match_records.clone().filter(&filter);
                            info!("Records remaining: {} entries.", filtered.len());
                            filtered.to_csv(cli.output)?;
                        }
                        "full" => {
                            let match_records = MatchRecords::from_csv(cli.source.clone())?;
                            info!("Source records read: {} entries.", match_records.len());
                            let mut filtered = match_records.clone().filter(&filter);
                            info!("Records remaining: {} entries.", filtered.len());
                            filtered.to_csv(cli.output)?;
                        }
                        _ => warn!("Unrecognized source type: {source}"),
                    }
                }
            } else {
                warn!("Filter parameter (-f or --filter) must be set.");
            }
        }
        "drift" => {
            info!("Calculating spatial drift between datasets.");
            trace!("Reading source addresses.");
            let mut source_addresses = SpatialAddresses::default();
            if let Some(source_type) = &cli.source_type {
                match source_type.as_str() {
                    "grants_pass" => {
                        source_addresses = SpatialAddresses::from(
                            &GrantsPassSpatialAddresses::from_csv(&cli.source)?[..],
                        )
                    }
                    "josephine_county" => {
                        source_addresses = SpatialAddresses::from(
                            &JosephineCountySpatialAddresses2024::from_csv(&cli.source)?[..],
                        )
                    }
                    _ => error!("Invalid source data type."),
                }
            } else {
                error!("No source data type provided.");
            }

            trace!("Reading target addresses.");
            let mut target_addresses = SpatialAddresses::default();
            if let Some(target) = &cli.target {
                if let Some(target_type) = &cli.target_type {
                    match target_type.as_str() {
                        "grants_pass" => {
                            target_addresses = SpatialAddresses::from(
                                &GrantsPassSpatialAddresses::from_csv(target)?[..],
                            )
                        }
                        "josephine_county" => {
                            target_addresses = SpatialAddresses::from(
                                &JosephineCountySpatialAddresses2024::from_csv(target)?[..],
                            )
                        }
                        _ => error!("Invalid target data type."),
                    }
                } else {
                    error!("No target data type provided.");
                }
            } else {
                error!("No target data specified.");
            }

            let mut deltas =
                <SpatialAddress as Cartesian>::deltas(&source_addresses, &target_addresses, 99.0);
            deltas.to_csv(cli.output.clone())?;
        }
        "lexisnexis" => {
            info!("Reading source records.");
            let mut source_addresses = CommonAddresses::default();
            if let Some(source_type) = &cli.source_type {
                match source_type.as_str() {
                    "grants_pass" => {
                        source_addresses = CommonAddresses::from(
                            &GrantsPassAddresses::from_csv(cli.source.clone())?[..],
                        )
                    }
                    "josephine_county" => {
                        source_addresses = CommonAddresses::from(
                            &JosephineCountyAddresses2024::from_csv(cli.source.clone())?[..],
                        )
                    }
                    "common" => {
                        source_addresses = CommonAddresses::from(SpatialAddressesRaw::from_csv(
                            cli.source.clone(),
                        )?)
                    }
                    _ => error!("Unrecognized file format."),
                }
            }

            info!("Source records read: {} entries.", source_addresses.len());

            trace!("Reading exclusion addresses.");
            let mut target_addresses = CommonAddresses::default();
            if let Some(target) = &cli.target {
                if let Some(target_type) = &cli.target_type {
                    match target_type.as_str() {
                        "grants_pass" => {
                            target_addresses =
                                CommonAddresses::from(&GrantsPassAddresses::from_csv(target)?[..])
                        }
                        "josephine_county" => {
                            target_addresses = CommonAddresses::from(
                                &JosephineCountyAddresses::from_csv(target)?[..],
                            )
                        }
                        "common" => {
                            target_addresses =
                                CommonAddresses::from(SpatialAddressesRaw::from_csv(target)?)
                        }
                        _ => error!("Invalid target data type."),
                    }
                } else {
                    error!("No target data type provided.");
                }
            } else {
                error!("No target data specified.");
            }
            info!(
                "Exclusion records read: {} entries.",
                target_addresses.len()
            );
            let mut lx = LexisNexis::from_addresses(&source_addresses, &target_addresses)?;
            lx.to_csv(cli.output)?;
        }
        "save" => {
            info!("Loading and saving addresses...");
            trace!("Reading source addresses.");
            let mut source_addresses = SpatialAddresses::default();
            if let Some(source_type) = &cli.source_type {
                match source_type.as_str() {
                    "grants_pass" => {
                        source_addresses = SpatialAddresses::from(
                            &GrantsPassSpatialAddresses::from_csv(&cli.source)?[..],
                        )
                    }
                    "josephine_county" => {
                        source_addresses = SpatialAddresses::from(
                            &JosephineCountySpatialAddresses2024::from_csv(&cli.source)?[..],
                        );
                        source_addresses.standardize();
                    }
                    _ => error!("Invalid source data type."),
                }
            } else {
                error!("No source data type provided.");
            }
            if !source_addresses.is_empty() {
                source_addresses.save(&cli.output)?;
                info!("Addresses saved to {:?}", &cli.output);
            } else {
                warn!("All records dropped.  Aborting save.");
            }
        }
        "orphan_streets" => {
            info!("Reading source records.");
            let mut source_addresses = CommonAddresses::default();
            if let Some(source_type) = &cli.source_type {
                match source_type.as_str() {
                    "grants_pass" => {
                        source_addresses = CommonAddresses::from(
                            &GrantsPassSpatialAddresses::from_csv(cli.source.clone())?[..],
                        )
                    }
                    "josephine_county" => {
                        source_addresses = CommonAddresses::from(
                            &JosephineCountySpatialAddresses2024::from_csv(cli.source.clone())?[..],
                        )
                    }
                    _ => error!("Unrecognized file format."),
                }
            }

            info!("Source records read: {} entries.", source_addresses.len());

            trace!("Reading exclusion addresses.");
            let mut target_addresses = CommonAddresses::default();
            if let Some(target) = &cli.target {
                if let Some(target_type) = &cli.target_type {
                    match target_type.as_str() {
                        "josephine_county" => {
                            target_addresses = CommonAddresses::from(
                                &JosephineCountySpatialAddresses2024::from_csv(target)?[..],
                            )
                        }
                        _ => error!("Invalid target data type."),
                    }
                } else {
                    error!("No target data type provided.");
                }
            } else {
                error!("No target data specified.");
            }
            info!(
                "Exclusion records read: {} entries.",
                target_addresses.len()
            );
            let orphans = &source_addresses.orphan_streets(&target_addresses);
            info!("{:?}", orphans);
        }
        "duplicates" => {
            info!("Reading source records.");
            let mut source_addresses = CommonAddresses::default();
            if let Some(source_type) = &cli.source_type {
                match source_type.as_str() {
                    "grants_pass" => {
                        source_addresses = CommonAddresses::from(
                            &GrantsPassSpatialAddresses::from_csv(cli.source.clone())?[..],
                        )
                    }
                    "josephine_county" => {
                        source_addresses = CommonAddresses::from(
                            &JosephineCountySpatialAddresses2024::from_csv(cli.source.clone())?[..],
                        )
                    }
                    _ => error!("Unrecognized file format."),
                }
            }

            info!("Source records read: {} entries.", source_addresses.len());
            info!("Screening addresses for duplicate records.");
            let mut duplicates = CommonAddresses::from(&source_addresses.filter("duplicate")[..]);
            info!("Duplicate records: {:?}", duplicates.len());
            info!("Output file: {:?}", cli.output);
            duplicates.to_csv(cli.output)?;
        }
        "business" => {
            info!("Matching business addresses.");
            info!("Reading source records.");
            let source_addresses = BusinessLicenses::from_csv(cli.source.clone())?;
            info!("Source records read: {} entries.", source_addresses.len());
            let mut source_addresses = source_addresses.deduplicate();
            source_addresses.detype_subaddresses()?;
            info!(
                "Records deduplicated: {} remaining.",
                source_addresses.len()
            );
            info!("Reading comparison records.");
            let mut target_addresses = GeoAddresses::default();
            if let Some(target) = &cli.target {
                if let Some(target_type) = &cli.target_type {
                    match target_type.as_str() {
                        "grants_pass" => {
                            target_addresses = GeoAddresses::from(
                                &GrantsPassSpatialAddresses::from_csv(target)?[..],
                            )
                        }
                        _ => info!("Unrecognized file format."),
                    }
                }
                info!("Target records read: {} entries.", target_addresses.len());
            }
            if let Some(alternate) = cli.alternate {
                info!("Comparing multiple targets.");
                let mut alt_target = GeoAddresses::default();
                if let Some(target_type) = &cli.alternate_type {
                    match target_type.as_str() {
                        "grants_pass" => {
                            alt_target = GeoAddresses::from(
                                &GrantsPassSpatialAddresses::from_csv(alternate)?[..],
                            )
                        }
                        _ => error!("Unrecognized file format."),
                    }
                }
                info!(
                    "Alternate target records read: {} entries.",
                    alt_target.len()
                );
                info!("Comparing records.");
                let mut match_records = BusinessMatchRecords::compare_chain(
                    &source_addresses,
                    &[&target_addresses, &alt_target],
                );
                info!("{:?} records categorized.", match_records.len());
                info!("Output file: {:?}", cli.output);
                match_records.to_csv(cli.output)?;
            } else {
                info!("Comparing records.");
                let mut match_records =
                    BusinessMatchRecords::compare(&source_addresses, &target_addresses);
                info!("{:?} records categorized.", match_records.len());
                info!("Output file: {:?}", cli.output);
                match_records.to_csv(cli.output)?;
            }
        }
        "compare" => {
            info!("Reading source records.");
            let mut source = GeoAddresses::default();
            if let Some(source_type) = &cli.source_type {
                match source_type.as_str() {
                    "grants_pass" => {
                        source = GeoAddresses::from(
                            &GrantsPassSpatialAddresses::from_csv(cli.source.clone())?[..],
                        )
                    }
                    "josephine_county" => {
                        source = GeoAddresses::from(
                            &JosephineCountySpatialAddresses2024::from_csv(cli.source.clone())?[..],
                        )
                    }
                    "common" => {
                        source =
                            GeoAddresses::from(SpatialAddressesRaw::from_csv(cli.source.clone())?)
                    }
                    _ => error!("Unrecognized file format."),
                }
            }
            info!("Reading target records.");
            let mut target = GeoAddresses::default();
            if let Some(target_type) = &cli.target_type {
                if let Some(target_path) = &cli.target {
                    match target_type.as_str() {
                        "grants_pass" => {
                            target = GeoAddresses::from(
                                &GrantsPassSpatialAddresses::from_csv(target_path)?[..],
                            )
                        }
                        "josephine_county" => {
                            target = GeoAddresses::from(
                                &JosephineCountySpatialAddresses2024::from_csv(target_path)?[..],
                            );
                            target.standardize();
                        }
                        "common" => {
                            target = GeoAddresses::from(SpatialAddressesRaw::from_csv(target_path)?)
                        }
                        _ => error!("Unrecognized file format."),
                    }
                }
            }
            info!("Comparing records.");

            info!("Remove retired addresses from source.");
            info!("Source records prior: {}", source.len());
            source.filter_field("active", "");
            // source = GeoAddresses::from(&source.filter("active")[..]);
            info!("Source records post: {}", source.len());

            let mut match_records = MatchRecords::compare(&source, &target);
            info!("{:?} records categorized.", match_records.len());
            info!("Output file: {:?}", cli.output);
            match_records.to_csv(cli.output)?;
        }
        _ => {}
    }

    Ok(())
}
