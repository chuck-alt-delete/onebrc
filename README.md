# One Billion Row Challenge

See the [one billion row challenge](https://github.com/gunnarmorling/1brc/tree/main). This is a little Rust adaptation.

The goal is to take simple `{station: temperature}` measurements and aggregate the min, mean, and max grouped by weather station.

## Generate data

Generate roughly 1 billion row csv file `resources/measurements.csv`. This is roughly 14GB and takes a while to generate.

```
cargo run --bin create_csv
```

## Compute the aggregation

```
cargo run
```

## Benchmark

### Rust

Current score:
- 405 seconds on a 2022 m2 MacBook Pro with 16GB of RAM

### DuckDB

For comparison, I ran the following using `duckdb`:

```sql
set temp_directory = '/tmp' ;
set memory_limit = '10GB';

explain analyze
    select
        station,
        min(temperature),
        avg(temperature),
        max(temperature)
    from 
        read_csv(
            'resources/measurements.csv',
            delim = ';',
            columns = {
                'station': 'VARCHAR',
                'temperature': 'DOUBLE'
            }
        )
    group by station 
    order by station;
```

The results are:
- 336 seconds to read the CSV
- 45 seconds to perform the aggregation

Not significantly faster than the Rust code. No difference when using `csv` vs `parquet`.

[Other folks using duckdb](https://rmoff.net/2024/01/03/1%EF%B8%8F⃣%EF%B8%8F-1brc-in-sql-with-duckdb/#extra-bonus-bit---using-parquet) saw ~25 seconds using `csv` and ~7 seconds using `parquet`, which tells me my system really takes a hit when needing to set `temp_directory` for disk swap space.

