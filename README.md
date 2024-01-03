# One Billion Row Challenge

See the [one billion row challenge](https://github.com/gunnarmorling/1brc/tree/main). This is a little Rust adaptation.

The goal is to take simple `{city: temp}` measurements and aggregate the min, mean, and max grouped by city.

## Generate data

Generate roughly 1 billion row csv file `resources/measurements.csv`. This is roughly 14GB.

```
cargo run --bin create_csv
```

## Compute the aggregation

```
cargo run
```

## Current score

- 405 seconds on an m1 macbook wjth 16GB kf RAM
