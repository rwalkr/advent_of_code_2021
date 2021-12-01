# Advent of code 2021

## Build

```
cargo build
```

## Usage

Typical usage for each day's exercise is:

```
cargo run --bin day_NN -- <PART> [input_file]
```

Where:

* NN is the day 01, 02, 03, .. 24
* PART is the part of that day's exercise (1 or 2)
* input_file is an optional override of the input data file

Input data files are stored in data.  Each day's program will default to 
loading `data/day_NN.txt` (if both parts use the same data) or 
`data/day_NN_P.txt` (if each part has different data)