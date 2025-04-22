# nc-to-pq

[![crates.io](https://img.shields.io/crates/v/nc-to-pq.svg)](https://crates.io/crates/nc-to-pq)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT) A simple command-line tool written in Rust to convert NetCDF4 files (.nc) to Apache Parquet format (.parquet).

This tool leverages the `peroxide` crate for reading NetCDF files and writing Parquet files.

**Important Constraint:** Apache Parquet is a columnar storage format. This tool requires that the input NetCDF4 file is structured such that each variable can be read as a **1-dimensional vector**, corresponding directly to a column in the output Parquet file. It will likely fail or produce incorrect results if your NetCDF variables have more than one dimension (e.g., grids, matrices).

## Features

* Reads variables from a NetCDF4 file.
* Writes the data to an Apache Parquet file.
* Supports specifying output file path.
* Supports basic compression options (`snappy` or `uncompressed`).

## Installation

Ensure you have Rust and Cargo installed. You can then install `nc-to-pq` directly from crates.io (once published):

```bash
cargo install nc-to-pq
```

Or, clone the repository and build manually:

```bash
git clone https://github.com/Axect/nc-to-pq
cd nc-to-pq
cargo build --release
# The executable will be in ./target/release/nc-to-pq
```

## Usage

```bash
nc-to-pq <INPUT_NETCDF_FILE> [OPTIONS]
```

**Arguments:**

  * `<INPUT_NETCDF_FILE>`: Path to the input NetCDF4 file (.nc).

**Options:**

  * `-o, --output <FILE>`: Specifies the path for the output Parquet file. If omitted, the output file will have the same name as the input file but with a `.parquet` extension, placed in the same directory.
  * `-c, --compression <COMPRESSION>`: Specifies the compression algorithm to use.
      * `default`: No compression (Uncompressed). This is the default if the option is omitted.
      * `snappy`: Use Snappy compression.
  * `-h, --help`: Print help information.
  * `-V, --version`: Print version information.

**Examples:**

1.  **Basic Conversion:** Convert `data.nc` to `data.parquet` (uncompressed).

    ```bash
    nc-to-pq data.nc
    ```

2.  **Specify Output File:** Convert `input.nc` to `output_data.parquet`.

    ```bash
    nc-to-pq input.nc -o output_data.parquet
    ```

3.  **Use Snappy Compression:** Convert `measurements.nc` to `measurements.parquet` using Snappy compression.

    ```bash
    nc-to-pq measurements.nc -c snappy
    ```

4.  **Specify Output and Compression:** Convert `input.nc` to `compressed_output.parquet` using Snappy.

    ```bash
    nc-to-pq input.nc -o compressed_output.parquet -c snappy
    ```

## License

This project is licensed under the [Your Chosen License, e.g., MIT License] - see the LICENSE file for details.

## Contributing

Contributions are welcome! Please feel free to submit issues or pull requests to the [GitHub repository](https://github.com/Axect/nc-to-pq).
