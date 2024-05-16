use address::prelude::*;
use aid::prelude::*;
use clap::Parser;
use tracing::{error, info, trace, warn};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short = 'c', long, help = "Command to execute.")]
    command: String,
    #[arg(short = 's', long, help = "Path to source addresses.")]
    source: std::path::PathBuf,
    #[arg(short = 'k', long, help = "Address format for source.")]
    source_type: Option<String>,
    #[arg(short = 't', long, help = "Path to target addresses.")]
    target: Option<std::path::PathBuf>,
    #[arg(short = 'z', long, help = "Address format for target.")]
    target_type: Option<String>,
    #[arg(short = 'f', long, help = "Filter records by value.")]
    filter: Option<String>,
    #[arg(
        short = 'd',
        long,
        help = "Search addresses for duplicates.",
        default_value = "false",
        default_missing_value = "true"
    )]
    duplicates: bool,
    #[arg(
        short = 'o',
        default_value = "output.csv",
        default_missing_value = "output.csv",
        long,
        help = "Path for output records."
    )]
    output: std::path::PathBuf,
    #[arg(
        short = 'b',
        default_value = "false",
        default_missing_value = "true",
        long,
        help = "Flag for business licenses."
    )]
    business: bool,
    #[arg(short = 'a', long, help = "Alternate target for search addresses.")]
    alternate: Option<std::path::PathBuf>,
    #[arg(short = 'y', long, help = "Address format for alternate target.")]
    alternate_type: Option<String>,
}

// fn main() -> Result<(), std::io::Error> {
fn main() -> Clean<()> {
    let cli = Cli::parse();

    if tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "address=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .try_init()
        .is_ok()
    {};
    info!("Subscriber initialized.");

    match cli.command.as_str() {
        "filter" => {
            if let Some(filter) = cli.filter {
                info!("Filtering records.");
                if cli.business {
                    let match_records = BusinessMatchRecords::from_csv(cli.source.clone())?;
                    info!(
                        "Source records read: {} entries.",
                        match_records.records.len()
                    );
                    let mut filtered = match_records.filter(&filter);
                    info!("Records remaining: {} entries.", filtered.records.len());
                    filtered.to_csv(cli.output)?;
                } else {
                    let match_records = MatchRecords::from_csv(cli.source.clone())?;
                    info!(
                        "Source records read: {} entries.",
                        match_records.records.len()
                    );
                    let mut filtered = match_records.filter(&filter);
                    info!("Records remaining: {} entries.", filtered.records.len());
                    filtered.to_csv(cli.output)?;
                }
            } else {
                warn!("Filter parameter (-f or --filter) must be set.");
            }
        }
        // "drift" => {
        //     info!("Calculating spatial drift between datasets.");
        //     trace!("Reading source addresses.");
        //     let mut source_addresses = Addresses::default();
        //     if let Some(source_type) = &cli.source_type {
        //         match source_type.as_str() {
        //             "grants_pass" => {
        //                 source_addresses = Addresses::from(CityAddresses::from_csv(&cli.source)?)
        //             }
        //             "grants_pass_2022" => {
        //                 source_addresses =
        //                     Addresses::from(GrantsPass2022Addresses::from_csv(&cli.source)?)
        //             }
        //             _ => error!("Invalid source data type."),
        //         }
        //     } else {
        //         error!("No source data type provided.");
        //     }
        //
        //     trace!("Reading target addresses.");
        //     let mut target_addresses = Addresses::default();
        //     if let Some(target) = &cli.target {
        //         if let Some(target_type) = &cli.target_type {
        //             match target_type.as_str() {
        //                 "grants_pass" => {
        //                     target_addresses = Addresses::from(CityAddresses::from_csv(target)?)
        //                 }
        //                 "grants_pass_2022" => {
        //                     target_addresses =
        //                         Addresses::from(GrantsPass2022Addresses::from_csv(target)?)
        //                 }
        //                 _ => error!("Invalid target data type."),
        //             }
        //         } else {
        //             error!("No target data type provided.");
        //         }
        //     } else {
        //         error!("No target data specified.");
        //     }
        //
        //     let mut deltas = source_addresses.deltas(&target_addresses, 99.0);
        //     deltas.to_csv(cli.output.clone())?;
        // }
        "lexisnexis" => {
            info!("Reading source records.");
            let mut source_addresses = CommonAddresses::default();
            if let Some(source_type) = &cli.source_type {
                match source_type.as_str() {
                    "grants_pass" => {
                        source_addresses = CommonAddresses::from(
                            &GrantsPassAddresses::from_csv(cli.source.clone())?.records[..],
                        )
                    }
                    "josephine_county" => {
                        source_addresses = CommonAddresses::from(
                            &JosephineCountyAddresses2024::from_csv(cli.source.clone())?.records[..],
                        )
                    }
                    _ => error!("Unrecognized file format."),
                }
            }

            info!(
                "Source records read: {} entries.",
                source_addresses.values().len()
            );

            trace!("Reading exclusion addresses.");
            let mut target_addresses = CommonAddresses::default();
            if let Some(target) = &cli.target {
                if let Some(target_type) = &cli.target_type {
                    match target_type.as_str() {
                        "josephine_county" => {
                            target_addresses = CommonAddresses::from(
                                &JosephineCountyAddresses::from_csv(target)?.records[..],
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
            // target_addresses = target_addresses.citify();
            info!(
                "Exclusion records read: {} entries.",
                target_addresses.values().len()
            );
            let mut lx = LexisNexis::from_addresses(&source_addresses, &target_addresses)?;
            lx.to_csv(cli.output)?;
        }
        // "orphan_streets" => {
        //     info!("Reading source records.");
        //     let mut source_addresses = Addresses::default();
        //     if let Some(source_type) = &cli.source_type {
        //         match source_type.as_str() {
        //             "grants_pass" => {
        //                 source_addresses =
        //                     Addresses::from(CityAddresses::from_csv(cli.source.clone())?)
        //             }
        //             "grants_pass_2022" => {
        //                 source_addresses =
        //                     Addresses::from(GrantsPass2022Addresses::from_csv(cli.source.clone())?)
        //             }
        //             "josephine_county" => {
        //                 source_addresses =
        //                     Addresses::from(CountyAddresses::from_csv(cli.source.clone())?)
        //             }
        //             _ => error!("Unrecognized file format."),
        //         }
        //     }
        //
        //     info!(
        //         "Source records read: {} entries.",
        //         source_addresses.records_ref().len()
        //     );
        //
        //     trace!("Reading exclusion addresses.");
        //     let mut target_addresses = Addresses::default();
        //     if let Some(target) = &cli.target {
        //         if let Some(target_type) = &cli.target_type {
        //             match target_type.as_str() {
        //                 "josephine_county" => {
        //                     target_addresses = Addresses::from(CountyAddresses::from_csv(target)?)
        //                 }
        //                 _ => error!("Invalid target data type."),
        //             }
        //         } else {
        //             error!("No target data type provided.");
        //         }
        //     } else {
        //         error!("No target data specified.");
        //     }
        //     info!(
        //         "Exclusion records read: {} entries.",
        //         target_addresses.records_ref().len()
        //     );
        //     let orphans = &source_addresses.orphan_streets(&target_addresses);
        //     info!("{:?}", orphans);
        // }
        // "compare" => {
        //     if cli.business {
        //         info!("Matching business addresses.");
        //         info!("Reading source records.");
        //         let source_addresses = BusinessLicenses::from_csv(cli.source.clone())?;
        //         info!(
        //             "Source records read: {} entries.",
        //             source_addresses.records.len()
        //         );
        //         let source_addresses = source_addresses.deduplicate();
        //         info!(
        //             "Records deduplicated: {} remaining.",
        //             source_addresses.records.len()
        //         );
        //         info!("Reading comparison records.");
        //         let mut target_addresses = Addresses::default();
        //         if let Some(target) = &cli.target {
        //             if let Some(target_type) = &cli.target_type {
        //                 match target_type.as_str() {
        //                     "grants_pass" => {
        //                         target_addresses = Addresses::from(CityAddresses::from_csv(target)?)
        //                     }
        //                     "grants_pass_2022" => {
        //                         target_addresses =
        //                             Addresses::from(GrantsPass2022Addresses::from_csv(target)?)
        //                     }
        //                     _ => info!("Unrecognized file format."),
        //                 }
        //             }
        //             info!(
        //                 "Target records read: {} entries.",
        //                 target_addresses.records_ref().len()
        //             );
        //         }
        //         if let Some(alternate) = cli.alternate {
        //             info!("Comparing multiple targets.");
        //             let mut alt_target = Addresses::default();
        //             if let Some(target_type) = &cli.alternate_type {
        //                 match target_type.as_str() {
        //                     "grants_pass" => {
        //                         alt_target = Addresses::from(CityAddresses::from_csv(alternate)?)
        //                     }
        //                     "grants_pass_2022" => {
        //                         alt_target =
        //                             Addresses::from(GrantsPass2022Addresses::from_csv(alternate)?)
        //                     }
        //                     _ => error!("Unrecognized file format."),
        //                 }
        //             }
        //             info!(
        //                 "Alternate target records read: {} entries.",
        //                 alt_target.records_ref().len()
        //             );
        //             info!("Comparing records.");
        //             let mut match_records = BusinessMatchRecords::compare_chain(
        //                 &source_addresses,
        //                 &[&target_addresses, &alt_target],
        //             );
        //             info!("{:?} records categorized.", match_records.records.len());
        //             info!("Output file: {:?}", cli.output);
        //             match_records.to_csv(cli.output)?;
        //         } else {
        //             info!("Comparing records.");
        //             let mut match_records =
        //                 BusinessMatchRecords::compare(&source_addresses, &target_addresses);
        //             info!("{:?} records categorized.", match_records.records.len());
        //             info!("Output file: {:?}", cli.output);
        //             match_records.to_csv(cli.output)?;
        //         }
        //     } else {
        //         info!("Matching addresses.");
        //         info!("Reading source records.");
        //         let mut source_addresses = Addresses::default();
        //         if let Some(source_type) = &cli.source_type {
        //             match source_type.as_str() {
        //                 "grants_pass" => {
        //                     source_addresses = Addresses::from(CityAddresses::from_csv(cli.source)?)
        //                 }
        //                 "grants_pass_2022" => {
        //                     source_addresses =
        //                         Addresses::from(GrantsPass2022Addresses::from_csv(cli.source)?)
        //                 }
        //                 "josephine_county" => {
        //                     source_addresses =
        //                         Addresses::from(CountyAddresses::from_csv(cli.source)?)
        //                 }
        //                 _ => error!("Unrecognized file format."),
        //             }
        //         }
        //
        //         info!(
        //             "Source records read: {} entries.",
        //             source_addresses.records_ref().len()
        //         );
        //         if cli.duplicates {
        //             info!("Screening for duplicate records.");
        //             let mut same = source_addresses.filter("duplicate");
        //             info!("Duplicate records: {:?}", same.records_ref().len());
        //             info!("Output file: {:?}", cli.output);
        //             same.to_csv(cli.output)?;
        //         } else if let Some(target) = cli.target {
        //             info!("Reading comparison records.");
        //             let mut target_addresses = Addresses::default();
        //             if let Some(target_type) = cli.target_type {
        //                 match target_type.as_str() {
        //                     "grants_pass" => {
        //                         target_addresses = Addresses::from(CityAddresses::from_csv(target)?)
        //                     }
        //                     "grants_pass_2022" => {
        //                         target_addresses =
        //                             Addresses::from(GrantsPass2022Addresses::from_csv(target)?)
        //                     }
        //                     "josephine_county" => {
        //                         target_addresses =
        //                             Addresses::from(CountyAddresses::from_csv(target)?)
        //                     }
        //                     _ => error!("Unrecognized file format."),
        //                 }
        //             }
        //             info!(
        //                 "Comparison records read: {} entries.",
        //                 target_addresses.records_ref().len()
        //             );
        //             info!("Comparing records.");
        //             let mut match_records = MatchRecords::compare(
        //                 source_addresses.records_ref(),
        //                 target_addresses.records_ref(),
        //             );
        //             info!(
        //                 "{:?} records categorized.",
        //                 match_records.records_ref().len()
        //             );
        //             info!("Output file: {:?}", cli.output);
        //             match_records.to_csv(cli.output)?;
        //         }
        //     }
        // }
        "compare" => {
            info!("Reading source records.");
            let mut source = GeoAddresses::default();
            if let Some(source_type) = &cli.source_type {
                match source_type.as_str() {
                    "grants_pass" => {
                        source = GeoAddresses::from(
                            &GrantsPassSpatialAddresses::from_csv(cli.source.clone())?.records[..],
                        )
                    }
                    "josephine_county" => {
                        source = GeoAddresses::from(
                            &JosephineCountySpatialAddresses2024::from_csv(cli.source.clone())?
                                .records[..],
                        )
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
                                &GrantsPassSpatialAddresses::from_csv(target_path)?.records[..],
                            )
                        }
                        "josephine_county" => {
                            target = GeoAddresses::from(
                                &JosephineCountySpatialAddresses2024::from_csv(target_path)?
                                    .records[..],
                            )
                        }
                        _ => error!("Unrecognized file format."),
                    }
                }
            }
            info!("Comparing records.");
            let mut match_records = MatchRecords::compare(&source.records, &target.records);
            info!("{:?} records categorized.", match_records.values().len());
            info!("Output file: {:?}", cli.output);
            match_records.to_csv(cli.output)?;
        }
        _ => {}
    }

    Ok(())
}
