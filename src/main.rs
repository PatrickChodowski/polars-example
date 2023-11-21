
use polars::prelude::*;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    if let Ok(df_csv0) = CsvReader::from_path("./data/mid.csv") {
        if let Ok(df_csv1) = df_csv0.infer_schema(Some(1000)).has_header(false).finish() {
            println!("{}", df_csv1);
            println!("dataset size: {}", df_csv1.estimated_size());
        } else {
            panic!("cant read file");
        }
    } else {
        panic!("no file");
    }
    println!("Time Elapsed: {}", now.elapsed().as_secs());
}
