use parquet::file::writer::RowGroupWriter;
use parquet_derive::ParquetRecordWriter;
use prometheus_parse::*;

use std::collections::HashMap;
use std::convert::TryFrom;
use std::io;

pub trait RecordWriter<T> {
    fn write_to_row_group(&self, row_group_writer: &mut Box<dyn RowGroupWriter>);
}

#[derive(Clone, Debug, ParquetRecordWriter)]
pub struct Metric {
    pub name: String,
    pub help: Option<String>,
    pub kind: String,
    pub labels: String,
    pub value: f64
}

impl Metric {
    pub fn set_help(&mut self, help: &str) {
        self.help = Some(help.to_owned());
    }
}

impl TryFrom<Sample> for Metric {
    type Error = io::Error;

    fn try_from(sample: Sample) -> Result<Self, Self::Error> {
        // Force a conversion from Sample to OurSample so we can access the inner fields
        // This is a huge hack but the original crate doesn't exist
        let (kind, value) = match sample.value {
            Value::Counter(ref v) => (String::from("counter"), *v),
            Value::Gauge(ref v) => (String::from("gauge"), *v),
            Value::Untyped(ref v) => (String::from("untyped"), *v),
            _ => {
                return Err(io::Error::new(io::ErrorKind::Other, "unimplemented"));
            }
        };

        let map: HashMap<_, _> = sample.labels.iter().collect();
        let labels = serde_json::to_string(&map)?;

        Ok(Metric {
            name: sample.metric,
            help: None,

            labels,
            kind,
            value
        })
    }
}