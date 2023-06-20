use address::address::*;
use address::business::*;
use address::compare::*;
use address::import::*;
use clap::Parser;
use tracing::{error, info};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
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
    #[arg(short = 'o', long, help = "Path for output records.")]
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

fn main() -> Result<(), std::io::Error> {
    let cli = Cli::parse();

    if let Ok(()) = tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .try_init()
    {};
    info!("Subscriber initialized.");

    if let Some(filter) = cli.filter {
        info!("Filtering records.");
        if cli.business {
            let match_records = BusinessMatchRecords::from_csv(cli.source)?;
            info!(
                "Source records read: {} entries.",
                match_records.records.len()
            );
            let mut filtered = match_records.filter(&filter);
            info!("Records remaining: {} entries.", filtered.records.len());
            filtered.to_csv(cli.output)?;
        } else {
            let match_records = MatchRecords::from_csv(cli.source)?;
            info!(
                "Source records read: {} entries.",
                match_records.records.len()
            );
            let mut filtered = match_records.filter(&filter);
            info!("Records remaining: {} entries.", filtered.records.len());
            filtered.to_csv(cli.output)?;
        }
    } else if cli.business {
        info!("Matching business addresses.");
        info!("Reading source records.");
        let source_addresses = BusinessLicenses::from_csv(cli.source)?;
        info!(
            "Source records read: {} entries.",
            source_addresses.records.len()
        );
        let source_addresses = source_addresses.deduplicate();
        info!(
            "Records deduplicated: {} remaining.",
            source_addresses.records.len()
        );
        info!("Reading comparison records.");
        let mut target_addresses = Addresses::default();
        if let Some(target) = cli.target {
            if let Some(target_type) = cli.target_type {
                match target_type.as_str() {
                    "grants_pass" => {
                        target_addresses = Addresses::from(CityAddresses::from_csv(target)?)
                    }
                    "grants_pass_2022" => {
                        target_addresses =
                            Addresses::from(GrantsPass2022Addresses::from_csv(target)?)
                    }
                    _ => info!("Unrecognized file format."),
                }
            }
            info!(
                "Target records read: {} entries.",
                target_addresses.records.len()
            );
        }
        if let Some(alternate) = cli.alternate {
            info!("Comparing multiple targets.");
            let mut alt_target = Addresses::default();
            if let Some(target_type) = cli.alternate_type {
                match target_type.as_str() {
                    "grants_pass" => {
                        alt_target = Addresses::from(CityAddresses::from_csv(alternate)?)
                    }
                    "grants_pass_2022" => {
                        alt_target = Addresses::from(GrantsPass2022Addresses::from_csv(alternate)?)
                    }
                    _ => error!("Unrecognized file format."),
                }
            }
            info!(
                "Alternate target records read: {} entries.",
                alt_target.records.len()
            );
            info!("Comparing records.");
            let mut match_records = BusinessMatchRecords::compare_chain(
                &source_addresses,
                &[&target_addresses, &alt_target],
            );
            info!("{:?} records categorized.", match_records.records.len());
            info!("Output file: {:?}", cli.output);
            match_records.to_csv(cli.output)?;
        } else {
            info!("Comparing records.");
            let mut match_records =
                BusinessMatchRecords::compare(&source_addresses, &target_addresses);
            info!("{:?} records categorized.", match_records.records.len());
            info!("Output file: {:?}", cli.output);
            match_records.to_csv(cli.output)?;
        }
    } else {
        info!("Matching addresses.");
        info!("Reading source records.");
        let mut source_addresses = Addresses::default();
        if let Some(source_type) = cli.source_type {
            match source_type.as_str() {
                "grants_pass" => {
                    source_addresses = Addresses::from(CityAddresses::from_csv(cli.source)?)
                }
                "grants_pass_2022" => {
                    source_addresses =
                        Addresses::from(GrantsPass2022Addresses::from_csv(cli.source)?)
                }
                "josephine_county" => {
                    source_addresses = Addresses::from(CountyAddresses::from_csv(cli.source)?)
                }
                _ => error!("Unrecognized file format."),
            }
        }

        info!(
            "Source records read: {} entries.",
            source_addresses.records.len()
        );
        if cli.duplicates {
            info!("Screening for duplicate records.");
            let mut same = source_addresses.filter("duplicate");
            info!("Duplicate records: {:?}", same.records.len());
            info!("Output file: {:?}", cli.output);
            same.to_csv(cli.output)?;
        } else if let Some(target) = cli.target {
            info!("Reading comparison records.");
            let mut target_addresses = Addresses::default();
            if let Some(target_type) = cli.target_type {
                match target_type.as_str() {
                    "grants_pass" => {
                        target_addresses = Addresses::from(CityAddresses::from_csv(target)?)
                    }
                    "grants_pass_2022" => {
                        target_addresses =
                            Addresses::from(GrantsPass2022Addresses::from_csv(target)?)
                    }
                    "josephine_county" => {
                        target_addresses = Addresses::from(CountyAddresses::from_csv(target)?)
                    }
                    _ => error!("Unrecognized file format."),
                }
            }
            info!(
                "Comparison records read: {} entries.",
                target_addresses.records.len()
            );
            info!("Comparing records.");
            let mut match_records =
                MatchRecords::compare(&source_addresses.records, &target_addresses.records);
            info!("{:?} records categorized.", match_records.records.len());
            info!("Output file: {:?}", cli.output);
            match_records.to_csv(cli.output)?;
        }
    }
    Ok(())
}
