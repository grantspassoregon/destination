use clap::Parser;

/// The `Cli` struct provides the command-line interface for the `address` library.
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// The `command` field specifies the command for the program to run. Currently accepts
    /// 'compare', 'drift', 'filter', 'save', 'orphan_streets' and 'lexisnexis' as values.
    ///
    /// * filter
    ///   * takes [`crate::MatchRecords`] as input
    ///   * takes [`crate::BusinessMatchRecords`] with the `-b` flag
    #[arg(
        short = 'c',
        long,
        help = "Command to execute.  Valid commands include 'compare', 'drift', 'filter', 'orphan_streets', 'lexisnexis' and 'save'"
    )]
    pub command: String,
    /// The `source` field specifies the path the source address file.
    #[arg(short = 's', long, help = "Path to source addresses.")]
    pub source: std::path::PathBuf,
    /// The `source_type` field contains a designator for the address source.  Currently accepts
    /// 'grants_pass' and 'josephine_county' as values.
    #[arg(short = 'k', long, help = "Address format for source.")]
    pub source_type: Option<String>,
    /// The `target` field specifies the path the target address file.
    #[arg(short = 't', long, help = "Path to target addresses.")]
    pub target: Option<std::path::PathBuf>,
    /// The `target_type` field contains a designator for the address target.  Currently accepts
    /// 'grants_pass' and 'josephine_county' as values.
    #[arg(short = 'z', long, help = "Address format for target.")]
    pub target_type: Option<String>,
    /// The `filter` field contains a value to filter the target data.  Currently accepts
    /// `missing`, `divergent`, `matching`, `subaddress`, `floor`, `building` and `status` as
    /// values.
    #[arg(short = 'f', long, help = "Filter records by value.")]
    pub filter: Option<String>,
    /// The `duplicates` flag instructs the program to search for duplicate addresses.
    #[arg(
        short = 'd',
        long,
        help = "Search addresses for duplicates.",
        default_value = "false",
        default_missing_value = "true"
    )]
    pub duplicates: bool,
    /// The `output` field specifies the path for the output file.
    #[arg(
        short = 'o',
        default_value = "output.csv",
        default_missing_value = "output.csv",
        long,
        help = "Path for output records."
    )]
    pub output: std::path::PathBuf,
    /// The `business` flag indicates the source addresses are from business licenses.
    #[arg(
        short = 'b',
        default_value = "false",
        default_missing_value = "true",
        long,
        help = "Flag for business licenses."
    )]
    pub business: bool,
    /// The `alternate` field specifies an alternate target path for addresses.
    #[arg(short = 'a', long, help = "Alternate target for search addresses.")]
    pub alternate: Option<std::path::PathBuf>,
    /// The `alternate_type` field contains a designator for the target addresses.  Currently
    /// accepts 'grants_pass' and 'josephine_county'.
    #[arg(short = 'y', long, help = "Address format for alternate target.")]
    pub alternate_type: Option<String>,
}
