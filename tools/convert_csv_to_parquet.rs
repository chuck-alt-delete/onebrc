use csv::Trim;
use arrow::array::{StringArray, Float64Array};
use arrow::datatypes::{DataType, Field, Schema};
use arrow::record_batch::RecordBatch;
use parquet::arrow::ArrowWriter;
use parquet::file::properties::WriterProperties;
use std::fs::File;
use std::sync::Arc;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let schema = Arc::new(Schema::new(vec![
        Field::new("station", DataType::Utf8, false),
        Field::new("temperature", DataType::Float64, false),
    ]));

    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b';')
        .trim(Trim::All)
        .from_path("/Users/chuck/scratch/onebrc/resources/measurements.csv")?;

    let file = File::create("/Users/chuck/scratch/onebrc/resources/measurements.parquet")?;
    let props = WriterProperties::builder().build();
    let mut writer = ArrowWriter::try_new(file, schema.clone(), Some(props))?;

    let mut records = Vec::new();

    for result in rdr.deserialize() {
        let record: (String, f64) = result?;
        records.push(record);

        if records.len() == 1_000_000 { // Process in batches of 1M records
            let batch = RecordBatch::try_new(
                schema.clone(),
                vec![
                    Arc::new(StringArray::from(
                        records.iter().map(|r| r.0.as_str()).collect::<Vec<&str>>(),
                    )),
                    Arc::new(Float64Array::from(
                        records.iter().map(|r| r.1).collect::<Vec<f64>>(),
                    )),
                ],
            )?;
            writer.write(&batch)?;
            records.clear();
        }
    }

    // Process the last batch if it's not empty
    if !records.is_empty() {
        let batch = RecordBatch::try_new(
            schema.clone(),
            vec![
                Arc::new(StringArray::from(
                    records.iter().map(|r| r.0.as_str()).collect::<Vec<&str>>(),
                )),
                Arc::new(Float64Array::from(
                    records.iter().map(|r| r.1).collect::<Vec<f64>>(),
                )),
            ],
        )?;
        writer.write(&batch)?;
    }

    writer.close()?;

    Ok(())
}
