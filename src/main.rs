use std::io::Error;

fn main() {
    let _ = prometheus_parquet::run()
        .or_else::<Error, _>(|e| {
            eprintln!("{}", e);
            std::process::exit(1);
        });
}