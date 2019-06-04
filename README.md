# prometheus-parquet

A simple tool to scrape a Prometheus endpoint and dump the set of metrics into a Parquet file.

Useful for running in a cron, then storing the resulting Parquet file in your storage+querying service of choice.

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
