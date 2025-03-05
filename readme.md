# Meshtal file splitter (`splitmesh`)

[![GitHub release](https://img.shields.io/github/v/release/repositony/splitmesh?include_prereleases)](https://github.com/repositony/splitmesh/releases/latest)

Command line tool to separate tallies into individual files for large MCNP
meshtal outputs that contain multiple tallies.

```bash
Usage: splitmesh <path> [options]

Arguments:
  <path>  Path to input meshtal file

Options:
  -v, --verbose...       Verbose logging (-v, -vv)
  -q, --quiet            Supress all log output (overrules --verbose)
  -h, --help             Print help (see more with '--help')

Split options:
  -t, --tallies <id>...  List of tallies to extract
  -o, --output <name>    Prefix for output files

Note: --help shows more information and examples
```

Help is printed with the `-h` flag, and `--help` will show examples, default
values, examples, and any important behaviour.

## Overview

Splits up all meshes found in a meshtal file into their individual
files.

This is very useful for processing large meshtal files with multiple
tallies, or for just reducing file sizes to a minimum for post-processing.

By default, every tally found in the file is splt into individual files.

## Install

Download and unpack the latest binary executable release [here](https://github.com/repositony/splitmesh/releases/latest) for running in a terminal/powershell.

### Linux/MacOS

Unpack the relevant executable from the [latest release](https://github.com/repositony/splitmesh/releases/latest).

```bash
# Linux
tar -xjf splitmesh-x86_64-unknown-linux-gnu.tar.xz  # Generic linux
tar -xjf splitmesh-aarch64-unknown-linux-gnu.tar.xz # ARM64 Linux

# MacOS
tar -xjf splitmesh-x86_64-apple-darwin.tar.xz       # Intel macOS
tar -xjf splitmesh-aarch64-apple-darwin.tar.xz      # Apple Silicon macOS
```

And either run from there or add the executable to your `$PATH`.

```bash
./splitmesh -h
```

### Windows

Extract `splitmesh-x86_64-pc-windows-msvc.zip` from the [latest release](https://github.com/repositony/splitmesh/releases/latest).

Navigate to this folder and run from powershell.

```bash
.\splitmesh.exe -h
```

This may be set as an alias for convenience.

```powershell
Set-Alias -Name splitmesh -Value C:\Path\To\Folder\splitmesh.exe
```

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
