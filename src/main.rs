use address::data::*;
use clap::Parser;
use tracing::info;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short = 's', long)]
    source: std::path::PathBuf,
    #[arg(short = 't', long)]
    target: Option<std::path::PathBuf>,
    #[arg(short = 'f', long)]
    filter: Option<String>,
    #[arg(short = 'o', long)]
    output: std::path::PathBuf,
}

fn main() -> Result<(), std::io::Error> {
    let cli = Cli::parse();

    if let Ok(()) = tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .try_init()
    {};
    info!("Subscriber initialized.");

    match cli.filter {
        Some(filter) => {
            info!("Filtering records.");
            let match_records = MatchRecords::from_csv(cli.source)?;
            info!(
                "Source records read: {} entries.",
                match_records.records.len()
            );
            let mut filtered = match_records.filter(&filter);
            info!("Records remaining: {} entries.", filtered.records.len());
            filtered.to_csv(cli.output)?;
        }
        None => {
            info!("Reading source records.");
            let city_addresses = CityAddresses::from_csv(cli.source)?;
            let source_addresses = Addresses::from(city_addresses);
            info!(
                "Source records read: {} entries.",
                source_addresses.records.len()
            );
            info!("Reading comparison records.");
            if let Some(target) = cli.target {
                let county_addresses = CountyAddresses::from_csv(target)?;
                let target_addresses = Addresses::from(county_addresses);
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
    }
    Ok(())
}
