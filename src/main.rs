//! Command line tool to split up meshtal files
#![doc(hidden)]

// standard libraries
use std::format as f;
// use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};

// use hdf5::filters::blosc_set_nthreads;
use hdf5::{dataset, Dataset, DatasetBuilderData, File, Group, H5Type};

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
    let file = File::open("../../ntools/crates/mesh/data/meshes/fmesh_8xx.r.h5")?;

    // group      /results
    // group      /results/mesh_tally
    // group      /results/mesh_tally/mesh_tally_804
    // group      /results/mesh_tally/mesh_tally_814
    // group      /results/mesh_tally/mesh_tally_824
    // group      /results/mesh_tally/mesh_tally_834
    // group      /results/mesh_tally/mesh_tally_844
    // group      /results/mesh_tally/mesh_tally_854

    // get the tallys from the cli, either all or a subset of numbers.
    // if all, easy, do them
    // if a set

    dbg!(&cli.tallies);

    // let src_group = file.group("/results/mesh_tally/mesh_tally_814")?; // Change "target_group" to the group you want to copy

    // // let mesh_tally_group = file.group("/results/mesh_tally")?;

    // // let _ = fs::remove_file("destination.h5"); // Remove if exists
    // // let dest_file = File::create("destination.h5")?;
    // // let dest_group = dest_file.create_group("target_group")?;

    // // copy_group(&src_group, &dest_group)?;

    // // Access the "results/mesh_tally" group
    let mesh_tally_group = file.group("/results/mesh_tally")?;
    println!("group = {:?}", mesh_tally_group);

    // for name in mesh_tally_group.member_names()? {
    for name in mesh_tally_group.member_names()? {
        // let name = group.name();

        println!("name = {name}");
        if let Some(number) = name.strip_prefix("mesh_tally_") {
            println!("number = {number}");
            if let Ok(tally_id) = number.parse::<u32>() {
                println!("Found fmesh {}, extracting", tally_id);

                // * then if value in cli values
                if cli.tallies.is_empty() || cli.tallies.contains(&tally_id) {
                    println!(" - Extracting fmesh {tally_id}");

                    let output_file = File::create(format!("fmesh_{tally_id}.h5"))?;

                    let group_name = format!("/results/mesh_tally/{name}");
                    let group = output_file.create_group(&group_name)?;

                    let src_group = mesh_tally_group.group(&name)?;

                    for dataset in group.datasets()? {
                        println!("  - dataset: {}", dataset.name())
                        // builder.with_data(dataset.clone()).create("test")?;
                    }

                    for attribute in group.attr_names()? {
                        println!("  - attr   : {}", attribute)
                        // builder.with_data(dataset.clone()).create("test")?;
                    }

                    for member in group.member_names()? {
                        println!("  - member : {}", member)
                    }

                    // // Copy all objects from the original group to the new group
                    // for obj in src_group.member_names()? {
                    //     src_group.copy(&obj, &new_group)?;
                    // }

                    // for obj in src_group.member_names()? {
                    //     let dataset = src_group.dataset(&obj)?;
                    //     let data: Vec<u8> = dataset.read_raw()?;
                    //     new_group
                    //         .new_dataset_builder()
                    //         .with_data(&data)
                    //         .create(&*obj)?;
                    // }

                    // Copy datasets from the original group to the new group
                    // for obj in group.member_names()? {
                    //     let dataset = group.dataset(&obj)?;
                    //     let dtype = dataset.dtype()?;
                    //     match dtype.id_type()? {
                    //         hdf5::types::TypeDescriptor::Integer { .. } => {
                    //             let data: Vec<i32> = dataset.read()?;
                    //             new_group
                    //                 .new_dataset::<i32>()
                    //                 .create(&obj, dataset.shape())?
                    //                 .write(&data)?;
                    //         }
                    //         hdf5::types::TypeDescriptor::Float { .. } => {
                    //             let data: Vec<f64> = dataset.read()?;
                    //             new_group
                    //                 .new_dataset::<f64>()
                    //                 .create(&obj, dataset.shape())?
                    //                 .write(&data)?;
                    //         }
                    //         _ => return Err("Unsupported data type".into()),
                    //     }
                    // }

                    // builder
                    //     .with_data(src_group.attr("has_collision_binning")?)
                    //     .create("test")?;

                    // println!("{:?}", a)

                    // let data = src_group.attr("comment_line_count")?;
                    // let a = new_group
                    //     .new_attr_builder()
                    //     .with_data(data.as_writer())
                    //     .create("test_data")?;

                    // let dataset = group.datasets()?;
                    // builder.with_data(dataset[0]).create("test")?;

                    // output_file

                    // for dataset_name in group.member_names()? {
                    //     let src_dataset = group.dataset(&dataset_name)?;
                    //     src_dataset.copy(&new_group, &dataset_name)?;
                    // }
                } else {
                    println!(" - Skipping fmesh {tally_id}");
                }
            }
        }
    }

    // // Access "group_b"
    // let group_b = input_file.group("group_b")?;

    // // Create a new HDF5 file to store only "group_b"
    // let output_file = File::create("output.h5")?;
    // let new_group_b = output_file.create_group("group_b")?;

    // // Get list of datasets inside "group_b"
    // let datasets = group_b.member_names()?;

    // // Initialize progress bar
    // let pb = ProgressBar::new(datasets.len() as u64);
    // pb.set_style(
    //     ProgressStyle::default_bar()
    //         .template(
    //             "{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos}/{len} copied",
    //         )
    //         .unwrap()
    //         .progress_chars("#>-"),
    // );

    // // Copy datasets with progress tracking
    // for dataset_name in datasets {
    //     let src_dataset = group_b.dataset(&dataset_name)?;
    //     let dst_dataset = src_dataset.copy(&new_group_b, &dataset_name)?;

    //     pb.inc(1); // Update progress bar
    // }

    // pb.finish_with_message("Copy complete!");
    // println!("Successfully copied 'group_b' to output.h5");

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
