# splitmesh

Command line tool to separate tallies into individual files for large MCNP
meshtal outputs that contain multiple tallies.

```text
Usage: splitmesh <path> [options]
```

## Description

Splits up all meshes found in a meshtal file into their individual
files.

This is very useful for processing large meshtal files with multiple
tallies, or for just reducing file sizes to a minimum for post-processing.

Help is printed with the `-h` flag, and `--help` will show examples, default
values, examples, and any important behaviour.

By default, every tally found in the file is splt into individual files.

### How to choose specific tallies

Use the `--tallies`  option to specify one or more tallies to be separated
out. Invalid entries are simply ignored.

```bash
# Extract only tallies with ID 104, 204, and 504 from the primary file
splitmesh /path/to/file.msht --tallies 104 204 504
```

### How to change the file names

The name of the output files is appended with the tally number as
`<output>_<id>.msht`. Output defaults to `fmesh`, but this may be changed.

```bash
# Change output file names to "mymesh_<id>.msht"
splitmesh /path/to/file.msht --output mymesh
```

## Install

Direct from github:

```shell
cargo install --git https://github.com/repositony/splitmesh.git
```

All executables are under `~/.cargo/bin/`, which should already be in your path
after installing Rust.

<details>
  <summary>Click here if you have never used Rust</summary>

If you have never used the Rust programming language, the toolchain is easily
installed from the [official website](https://www.rust-lang.org/tools/install)

```shell
curl https://sh.rustup.rs -sSf | sh
```

This should have added `source $HOME/.cargo/env` to the bash profile, so update
your environment with `source ~/.bashrc`.

</details>
