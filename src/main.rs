//! Command line tool to split up meshtal files
#![doc(hidden)]

// standard libraries
use std::format as f;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};

// external crates
use anyhow::{anyhow, Result};
use clap::Parser;
use log::*;

// nom parser combinators, overkill but easier to expand for other things later
use nom::bytes::complete::tag;
use nom::character::complete::{digit1, space1};
use nom::sequence::{preceded, tuple};
use nom::IResult;

// internal modules
mod cli;
use cli::Cli;

fn main() -> Result<()> {
    // set up the command line interface and logging
    let cli = Cli::parse();
    init_logging(&cli);

    info!("Splitting \"{}\"", cli.path);

    // Take a second to check the file for valid tallies
    debug!("Checking at least one tally exists in file");
    let id = check_for_tally(&cli)?;
    debug!("  - first relevant mesh: fmesh {id}");

    // Only then move on to split up the file
    debug!("Writing new files");
    let writer = writer(&f!("{}_{id}.msht", cli.output))?;
    split_meshtal_files(&cli, writer)
}

/// Helper function for cleaning up file IO boilerplate
fn reader(path: &str) -> Result<BufReader<File>> {
    let file: File = File::open(path)?;
    trace!("New bufreader for {path}");
    Ok(BufReader::new(file))
}

/// Helper function for cleaning up file IO boilerplate
fn writer(path: &str) -> Result<BufWriter<File>> {
    let file: File = File::create(path)?;
    trace!("New bufwriter for {path}");
    Ok(BufWriter::new(file))
}

/// Make sure the file contains at least one mesh before doing anything
fn check_for_tally(cli: &Cli) -> Result<u32> {
    let reader = reader(&cli.path)?;

    for line in reader.lines().map_while(Result::ok) {
        let l = line.trim_start();

        if !is_new_mesh(l) {
            continue;
        }

        let (_, id) = mesh_id(l).map_err(|_| anyhow!("Failed to parse id from:\n \"{l}\""))?;

        // If at least one is relevent break early and carry on with the file splitting
        if cli.tallies.is_empty() || cli.tallies.contains(&id) {
            trace!("First relevant mesh tally found: fmesh {id}");
            return Ok(id);
        }
    }

    Err(anyhow!("No relevant meshes found in file"))
}

/// Copies the relevant content to appropriate files
fn split_meshtal_files(cli: &Cli, mut writer: BufWriter<File>) -> Result<()> {
    let mut is_relevant_mesh = false;

    let reader = reader(&cli.path)?;
    for line in reader.lines() {
        let line = line.unwrap();

        // decide what to do whenever a new mesh is found
        if is_new_mesh(line.trim_start()) {
            let (_, id) = mesh_id(line.trim_start())
                .map_err(|_| anyhow!("Failed to parse id from:\n \"{line}\""))?;

            if cli.tallies.is_empty() || cli.tallies.contains(&id) {
                let output = f!("{}_{id}.msht", cli.output);
                info!("  - {output}");
                writer = BufWriter::new(File::create(&output)?);
                is_relevant_mesh = true;
            } else {
                is_relevant_mesh = false;
            }
        }

        if is_relevant_mesh {
            writer.write_all(line.as_bytes())?;
            writer.write_all(b"\n")?;
        }
    }

    Ok(())
}

/// Quick check for the new tally tag
fn is_new_mesh(i: &str) -> bool {
    i.starts_with("Mesh Tally Number")
}

/// Parse the number following a `Mesh Tally Number` tag to a u32
fn mesh_id(i: &str) -> IResult<&str, u32> {
    let (_, tally_id) = preceded(tuple((tag("Mesh Tally Number"), space1)), digit1)(i)?;
    nom::character::complete::u32(tally_id)
}

/// Sets up logging at runtime to allow for multiple verbosity levels
fn init_logging(cli: &Cli) {
    stderrlog::new()
        .module(module_path!())
        .quiet(cli.quiet)
        .verbosity(cli.verbose as usize + 2)
        .show_level(false)
        .color(stderrlog::ColorChoice::Never)
        .timestamp(stderrlog::Timestamp::Off)
        .init()
        .unwrap();
}
