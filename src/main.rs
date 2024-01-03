use std::time::Instant;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::sync::{Arc, Mutex};
use std::thread;

struct WeatherData {
    min_temp: f64,
    max_temp: f64,
    total_temp: f64,
    count: u64,
}

impl WeatherData {
    fn new() -> WeatherData {
        WeatherData {
            min_temp: f64::INFINITY,
            max_temp: f64::NEG_INFINITY,
            total_temp: 0.0,
            count: 0,
        }
    }

    fn update(&mut self, temp: f64) {
        self.min_temp = self.min_temp.min(temp);
        self.max_temp = self.max_temp.max(temp);
        self.total_temp += temp;
        self.count += 1;
    }

    fn combine(&mut self, other: &WeatherData) {
        self.min_temp = self.min_temp.min(other.min_temp);
        self.max_temp = self.max_temp.max(other.max_temp);
        self.total_temp += other.total_temp;
        self.count += other.count;
    }
}

fn process_chunk(chunk: Vec<String>) -> HashMap<String, WeatherData> {
    let mut results = HashMap::new();
    for line in chunk {
        let parts: Vec<&str> = line.split(';').collect();
        if parts.len() != 2 {
            continue;
        }
        let station = parts[0].to_string();
        let temp: f64 = parts[1].parse().unwrap_or_default();
        
        results.entry(station)
               .or_insert_with(WeatherData::new)
               .update(temp);
    }
    results
}

fn main() -> io::Result<()> {
    let timer = Instant::now();
    let path = "/Users/chuck/scratch/onebrc/resources/measurements.csv";
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let chunk_size = 10_000_000; // Number of lines per chunk
    let mut chunk = Vec::with_capacity(chunk_size);
    let mut handles = vec![];
    let final_results = Arc::new(Mutex::new(HashMap::new()));

    for line in reader.lines() {
        let line = line?;
        chunk.push(line);

        if chunk.len() == chunk_size {
            let final_results_clone = Arc::clone(&final_results);
            let chunk_to_process = std::mem::replace(&mut chunk, Vec::with_capacity(chunk_size));

            let handle = thread::spawn(move || {
                let chunk_results = process_chunk(chunk_to_process);
                let mut final_res = final_results_clone.lock().unwrap();
                for (station, data) in chunk_results {
                    final_res.entry(station)
                             .or_insert_with(WeatherData::new)
                             .combine(&data);
                }
            });
            handles.push(handle);
        }
    }

    // Process the last chunk if it's not empty
    if !chunk.is_empty() {
        let final_results_clone = Arc::clone(&final_results);
        let chunk_to_process = chunk;

        let handle = thread::spawn(move || {
            let chunk_results = process_chunk(chunk_to_process);
            let mut final_res = final_results_clone.lock().unwrap();
            for (station, data) in chunk_results {
                final_res.entry(station)
                         .or_insert_with(WeatherData::new)
                         .combine(&data);
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let final_res = final_results.lock().unwrap();
    for (station, data) in final_res.iter() {
        println!("{}: min = {:.1}, max = {:.1}, mean = {:.1}", 
                 station, 
                 data.min_temp, 
                 data.max_temp, 
                 data.total_temp / data.count as f64);
    }
    let duration = timer.elapsed();
    print!("Time elapsed: {:?}", duration);

    Ok(())
}


