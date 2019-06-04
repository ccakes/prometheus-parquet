# prometheus-parquet

A simple tool to scrape a Prometheus endpoint and dump the set of metrics into a Parquet file.

Useful for running in a cron, then storing the resulting Parquet file in your storage+querying service of choice.

## Building

This uses some dependencies which are only in Git, so it can't be hosted on crates.io. To run this you'll need the nightly Rust compiler and to build it from source.

See [rustup.rs](https://rustup.rs) for installing the Rust compiler.

```bash
# Install nightly
$ rustup toolchain install nightly

# Clone and build
$ git clone https://github.com/ccakes/prometheus-parquet
$ cd prometheus-parquet
$ cargo build --release

# Run
target/release/prometheus-parquet --help
```

## Usage

```bash
$ prometheus-parquet --prometheus http://127.0.0.1:9090/metrics --output $(hostname).parquet

# optional
$ aws s3 cp $(hostname).parquet s3://my-bucket/prom_dumps/year=$(date +%Y)/month=$(date +%m)/day=$(date +%d)/hour=$(date +%H)/
```

## Contributing

PRs, bug reports, general complaining is all welcome

## License

See [LICENSE](LICENSE).
