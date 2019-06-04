use structopt::StructOpt;

mod parquet;
mod types;

use types::Metric;

use std::convert::TryFrom;
use std::error::Error;
use std::io;
use std::path::PathBuf;

/// prometheus-parquet
#[derive(StructOpt)]
#[structopt(name = "prometheus-parquet")]
struct Args {
    /// Prometheus addr
    #[structopt(short = "p", long = "prometheus")]
    prometheus: String,

    /// Output file
    #[structopt(short = "o", long = "output")]
    output: PathBuf,

    /// Logging verbosity
    #[structopt(short = "v", parse(from_occurrences))]
    verbose: u8
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let args = Args::from_args();

    // Derive log verbosity from args
    let log_level = match args.verbose {
        0 => log::LevelFilter::Info,
        1 => log::LevelFilter::Debug,
        _ => log::LevelFilter::Trace
    };

    // Init logger
    fern::Dispatch::new()
        .level(log::LevelFilter::Warn)
        .level_for(module_path!(), log_level)
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}: {}",
                record.level(),
                message
            ))
        })
        .chain(io::stdout())
        .apply()?;

    // Check our output doesn't exist
    if args.output.exists() {
        log::error!("Output file already exists!");
        std::process::exit(1);
    }

    let response = minreq::get(args.prometheus).send()?;
    let lines: Vec<_> = response.body
        .lines()
        .map(|s| Ok(s.to_owned()))
        .collect();

    let prom = prometheus_parse::Scrape::parse(lines.into_iter())?;
    let docs = prom.docs.clone();

    let metrics: Vec<Metric> = prom.samples.into_iter()
        .filter_map(|sample| {
            Metric::try_from(sample).ok()
        })
        .map(|mut metric| {
            if let Some(help) = docs.get(&metric.name) {
                metric.set_help(help);
            }

            metric
        })
        .collect();
    
    log::debug!("Parsed {} metrics from Prometheus", metrics.len());
    
    parquet::write(args.output, metrics)
}