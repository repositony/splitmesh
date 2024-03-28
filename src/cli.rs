// command line modules
use clap::builder::styling::{AnsiColor, Effects};
use clap::builder::Styles;
use clap::{arg, Parser};

/// Split the tallies of a meshtal into individual files
///
/// Files are simply split on lines starting with a "Mesh Tally Number" tag.
///
/// By default all meshes found in the meshtal file are copied into individual
/// files. Use the --mesh option to list specific tallies of interest. Use
/// --output to change the prefix of the output file names.
///
/// Examples
/// --------
///
///  Typical use
///     $ splitmesh file.msht
///
///  Only split off a subset of tallies
///     $ splitmesh file.msht --mesh 104 204 504
///
///  Change output file names to "mymesh_<id>.msht"
///     $ splitmesh file.msht --output mymesh
///
#[derive(Parser)]
#[command(
    verbatim_doc_comment,
    arg_required_else_help(true),
    after_help("Note: --help shows more information and examples"),
    term_width(76),
    hide_possible_values(true),
    override_usage("splitmesh <path> [options]"),
    styles=custom_style()
)]
pub struct Cli {
    // * Positional
    /// Path to input meshtal file
    #[arg(name = "path")]
    pub path: String,

    /// List of tallies to extract
    ///
    /// By default all meshes are extracted. Use this option to specify one or
    /// more tallies to be separated out. Invalid entries are ignored.
    #[arg(help_heading("Split options"))]
    #[arg(short, long)]
    #[arg(value_parser, num_args = 1.., value_delimiter = ' ')]
    #[clap(required = false)]
    #[arg(value_name = "id")]
    pub tallies: Vec<u32>,

    /// Prefix for output files
    ///
    /// Defaults to `fmesh`. File names provided are appended with the tally
    /// number as `<output>_<id>.msht`.
    #[arg(help_heading("Split options"))]
    #[arg(short, long)]
    #[arg(value_name = "path")]
    #[arg(default_value = "fmesh")]
    pub output: String,

    // * Flags
    /// Verbose logging (-v, -vv)
    ///
    /// If specified, the default log level of INFO is increased to DEBUG (-v)
    /// or TRACE (-vv). Errors and Warnings are always logged unless in quiet
    /// (-q) mode.
    #[arg(short, long)]
    #[arg(action = clap::ArgAction::Count)]
    pub verbose: u8,

    /// Supress all log output (overrules --verbose)
    #[arg(short, long)]
    pub quiet: bool,
}

/// Customise the colour styles for clap v4
fn custom_style() -> Styles {
    Styles::styled()
        .header(AnsiColor::Green.on_default() | Effects::BOLD)
        .usage(AnsiColor::Cyan.on_default() | Effects::BOLD | Effects::UNDERLINE)
        .literal(AnsiColor::Blue.on_default() | Effects::BOLD)
        .placeholder(AnsiColor::Magenta.on_default())
}
