use crate::types::Metric;

use parquet::basic::{Type as PhysicalType, LogicalType, Repetition};
use parquet::schema::types::Type;
use parquet::file::properties::WriterProperties;
use parquet::file::writer::{FileWriter, SerializedFileWriter};
use parquet::record::RecordWriter;

use std::error::Error;
use std::fs;
use std::path::PathBuf;
use std::rc::Rc;

pub fn write(output: PathBuf, metrics: Vec<Metric>) -> Result<(), Box<dyn Error>> {
    let mut fields = vec![];
    fields.push(
        Rc::new(Type::primitive_type_builder("name", PhysicalType::BYTE_ARRAY)
            .with_logical_type(LogicalType::UTF8)
            .with_repetition(Repetition::REQUIRED)
            .build()?
        )
    );

    fields.push(
        Rc::new(Type::primitive_type_builder("help", PhysicalType::BYTE_ARRAY)
            .with_logical_type(LogicalType::UTF8)
            .build()?
        )
    );

    fields.push(
        Rc::new(Type::primitive_type_builder("kind", PhysicalType::BYTE_ARRAY)
            .with_logical_type(LogicalType::UTF8)
            .with_repetition(Repetition::REQUIRED)
            .build()?
        )
    );

    fields.push(
        Rc::new(Type::primitive_type_builder("labels", PhysicalType::BYTE_ARRAY)
            .with_logical_type(LogicalType::UTF8)
            .with_repetition(Repetition::REQUIRED)
            .build()?
        )
    );

    fields.push(
        Rc::new(Type::primitive_type_builder("value", PhysicalType::DOUBLE)
            .with_repetition(Repetition::REQUIRED)
            .build()?
        )
    );

    let schema = Type::group_type_builder("hive_schema")
        .with_fields(&mut fields)
        .build()?;
    
    let file = fs::File::create(&output)?;

    let props = Rc::new(
        WriterProperties::builder()
            .set_compression(parquet::basic::Compression::GZIP)
            .build()
    );
    let mut writer = SerializedFileWriter::new(file, Rc::new(schema), props)?;
    let mut row_group = writer.next_row_group()?;

    metrics.as_slice().write_to_row_group(&mut row_group);

    writer.close_row_group(row_group)?;
    writer.close()?;

    Ok(())
}