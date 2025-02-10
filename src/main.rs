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

// internal modules
mod cli;

// re-exports for convenience
use cli::Cli;

fn main() -> Result<()> {
    // set up the command line interface and logging
    let cli = Cli::parse();
    cli::init_logging(&cli)?;

    // Open the large HDF5 file in read-only mode
    let input_file = File::open("large_input.h5")?;

    // Access "group_b"
    let group_b = input_file.group("group_b")?;

    // Create a new HDF5 file to store only "group_b"
    let output_file = File::create("output.h5")?;
    let new_group_b = output_file.create_group("group_b")?;

    // Get list of datasets inside "group_b"
    let datasets = group_b.member_names()?;

    // Initialize progress bar
    let pb = ProgressBar::new(datasets.len() as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template(
                "{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos}/{len} copied",
            )
            .unwrap()
            .progress_chars("#>-"),
    );

    // Copy datasets with progress tracking
    for dataset_name in datasets {
        let src_dataset = group_b.dataset(&dataset_name)?;
        let dst_dataset = src_dataset.copy(&new_group_b, &dataset_name)?;

        pb.inc(1); // Update progress bar
    }

    pb.finish_with_message("Copy complete!");
    println!("Successfully copied 'group_b' to output.h5");

    // info!("Splitting \"{}\"", cli.path);

    // // Take a second to check the file for valid tallies
    // debug!("Checking at least one tally exists in file");
    // let id = check_for_tally(&cli)?;
    // debug!("  - first relevant mesh: fmesh {id}");

    // // Only then move on to split up the file
    // debug!("Writing new files");
    // let writer = writer(&f!("{}_{id}.msht", cli.output))?;
    // split_meshtal_files(&cli, writer)

    Ok(())
}

// /// Helper function for cleaning up file IO boilerplate
// fn reader(path: &str) -> Result<BufReader<File>> {
//     let file: File = File::open(path)?;
//     trace!("New bufreader for {path}");
//     Ok(BufReader::new(file))
// }

// /// Helper function for cleaning up file IO boilerplate
// fn writer(path: &str) -> Result<BufWriter<File>> {
//     let file: File = File::create(path)?;
//     trace!("New bufwriter for {path}");
//     Ok(BufWriter::new(file))
// }

// /// Make sure the file contains at least one mesh before doing anything
// fn check_for_tally(cli: &Cli) -> Result<u32> {
//     let reader = reader(&cli.path)?;

//     for line in reader.lines().map_while(Result::ok) {
//         let l = line.trim_start();

//         if !is_new_mesh(l) {
//             continue;
//         }

//         let (_, id) = mesh_id(l).map_err(|_| anyhow!("Failed to parse id from:\n \"{l}\""))?;

//         // If at least one is relevent break early and carry on with the file splitting
//         if cli.tallies.is_empty() || cli.tallies.contains(&id) {
//             trace!("First relevant mesh tally found: fmesh {id}");
//             return Ok(id);
//         }
//     }

//     Err(anyhow!("No relevant meshes found in file"))
// }

// /// Copies the relevant content to appropriate files
// fn split_meshtal_files(cli: &Cli, mut writer: BufWriter<File>) -> Result<()> {
//     let mut is_relevant_mesh = false;

//     let reader = reader(&cli.path)?;
//     for line in reader.lines() {
//         let line = line.unwrap();

//         // decide what to do whenever a new mesh is found
//         if is_new_mesh(line.trim_start()) {
//             let (_, id) = mesh_id(line.trim_start())
//                 .map_err(|_| anyhow!("Failed to parse id from:\n \"{line}\""))?;

//             if cli.tallies.is_empty() || cli.tallies.contains(&id) {
//                 let output = f!("{}_{id}.msht", cli.output);
//                 info!("  - {output}");
//                 writer = BufWriter::new(File::create(&output)?);
//                 is_relevant_mesh = true;
//             } else {
//                 is_relevant_mesh = false;
//             }
//         }

//         if is_relevant_mesh {
//             writer.write_all(line.as_bytes())?;
//             writer.write_all(b"\n")?;
//         }
//     }

//     Ok(())
// }
