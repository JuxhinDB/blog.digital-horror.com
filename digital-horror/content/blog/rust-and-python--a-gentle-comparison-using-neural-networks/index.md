+++
title = "Rust & Python—A Gentle Comparison using Simple Neural Networks"
description = "Contrasting Python and Rust programming languages through the trivial application of the Kohonen-Grossberg Neural Network model."
date = 2018-12-23

[taxonomies]
tags = ["rust", "ml"]

[extra]
archive = "This is an old post that has been migrated over one or more times. It may contain issues with certain images and formatting."
+++

#### Disclaimer

> I am by no means proficient in Rust as you very well will see. As a  result, these results should be taken with a pinch of salt. Any  improvements are most welcome! There are some great discussions over at the [/r/rust](https://www.reddit.com/r/rust/comments/a8wfpf/rust_pythona_gentle_comparison_using_simple/) subreddit for more information about code optimisation that I highly recommend reading.

## Introduction

I recently had a task to implement a very simple [Kohonen-Grossberg Neural Network](https://en.wikipedia.org/wiki/Self-organizing_map) which was particularly fun due to being relatively simple to implement.

My initial implementation was in Python with less than 60 lines of  code. I wrapped a CLI around it and sat at around 90 lines of code.

After some thought, I figured that this would be a great  learning experience for Rust (and it was) and would give me the  opportunity to compare the two languages from multiple perspectives.

### Implementation

The following are the two implementations of KNN in Python and Rust. Keep in mind that this was originally written in Python and then ported to Rust.

#### Python Implementation

The following is the Python implementation which you can find [here](https://gist.github.com/JuxhinDB/47f90374af3a6328f5090401da09128b).

```python
__author__ = 'Juxhin Dyrmishi Brigjaj'

import sys
import math
import random
import argparse


def parse_args():
    parser = argparse.ArgumentParser(description='A naive Kohonen-Grossberg Counterpropogation Network in Python')
    parser.add_argument('-l', '--learning-rate', metavar='R', type=float, required=True,
                        help='Float indicating the learning rate (step) the network should use')
    parser.add_argument('-f', '--csv-file', type=str, required=True,
                        help='Path to CSV file containing dataset')
    parser.add_argument('-e', '--epoch', type=int, help="Number of epochs to complete", required=True, default=1000)
    parser.add_argument('-n', '--neurons', type=int, help="Number of neurons (units) to generate", required=True, default=3)
    return parser.parse_args()


def normalise(rows: list=()) -> list:

    _result = []

    for row in rows:
        _vector_length = math.sqrt(sum([x**2 for x in row]))
        _result.append([round(x / _vector_length, 4) for x in row])

    return _result


def generate_random_units(col_len: int, row_len: int) -> list:
    _result = []
    for _ in range(0, row_len):

        _result.append([round(random.uniform(0.0, 1.0), 4) for _ in range(0, col_len)])
    return _result


def calculate_nets(row, units):
    _nets = []
    for unit in units:
        _net = 0.0
        for i, _ in enumerate(unit):
            _net += round(row[i] * unit[i], 4)
        _nets.append(round(_net, 4))
    return _nets


def update_units(learning_rate: float, nets: list, row: list, units: list) -> bool:
    _i = nets.index(max(nets))

    for _j, column in enumerate(row):
        units[_i][_j] = round(units[_i][_j] + learning_rate * (column - units[_i][_j]), 4)


def main():
    args = parse_args()

    learning_rate = args.learning_rate
    unnormalised_dataset = []

    try:
        with open(args.csv_file, 'r') as csv_file:
            for line in csv_file:
                unnormalised_dataset.append([float(x) for x in line.split(',')])
    except TypeError as e:
        print("[!] FATAL: Dataset is malformed. Unable to parse values as floats.\n{}".format(str(e)))

    print("[+] Normalising dataset")

    rows = normalise(unnormalised_dataset)

    for row in rows:
        print('\t'.join([str(x) for x in row]))

    # Used to determine the number of columns in generate_random_units call
    # assuming that the dataset is consistent in width

    __unit_length = len(unnormalised_dataset[0])
    random_units = generate_random_units(__unit_length, args.neurons)

    print("\n[+] Starting Weights:")
    for unit in random_units:
        print(','.join([str(x) for x in unit]))
    print()


    for i in range(1, args.epoch + 1):

        if i % 100 == 0:
            print("[+] Running Epoch #{}".format(str(i)))
        for row in rows:
            nets = calculate_nets(row, random_units)
            update_units(learning_rate, nets, row, random_units)

    print("\n[+] Final Weights:")
    for unit in random_units:
        print(','.join([str(x) for x in unit]))

if __name__ == '__main__':
    main()
```

#### Rust Implementation

You can find the Rust implementation [here](https://github.com/JuxhinDB/rust-snippets/blob/master/KohonenGrossberg-NN/src/main.rs).

```rust
#[macro_use]
extern crate clap;

use std::fs::File;

use clap::App;
use rand::distributions::{Distribution, Standard};


fn normalise(rows: &mut Vec<Vec<f32>>) {
    for row in rows.iter_mut() {
        let vector_length = row.into_iter().map(|x| x.powf(2.0)).fold(0.0, |a, b| a + b).sqrt();
        *row = row.into_iter().map(|x| *x / vector_length).collect();
    }
}

fn generate_random_units(col_len: &usize, row_len: &usize) -> Vec<Vec<f32>> {
    let mut rng = rand::thread_rng();

    std::iter::repeat_with(||
        Standard.sample_iter(&mut rng).take(*col_len).collect())
        .take(*row_len)
        .collect()
}

fn calculate_nets(row: &Vec<f32>, units: &Vec<Vec<f32>>) -> Vec<f32> {

    let mut nets: Vec<f32> = Vec::with_capacity(units.len());

    for unit in units.iter() {
        let mut _net = 0.0;

        for (i, _) in unit.iter().enumerate() {
            unsafe {
                _net += row.get_unchecked(i as usize) * unit.get_unchecked(i as usize);
            }
        }

        nets.push(_net);
    }

    nets
}

fn update_units(learning_rate: f32, nets: &Vec<f32>, row: &Vec<f32>, units: &mut Vec<Vec<f32>>) {

    // Sub-optimal...
    let mut iter = nets.iter().enumerate();
    let init = iter.next().unwrap();

    // https://stackoverflow.com/questions/53903318/rust-idiomatic-way-to-get-index-of-max-float-value-in-a-vec?noredirect=1#comment94651877_53903318
    let _i = iter.try_fold(init, |acc, x| {

        if let Some(_i) = x.1.partial_cmp(acc.1) {
            Some(if let std::cmp::Ordering::Greater = _i {
                x
            } else {
                acc
            })
        } else {
            None
        }
    }).unwrap().0;

    row.iter().enumerate().for_each(|(_j, column)| {
        units[_i][_j] += learning_rate * (column - units[_i][_j]);
    });
}

fn main() {
    let yaml = load_yaml!("clap-cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let learning_rate = value_t!(matches, "LearningRate", f32).unwrap_or_else(|e| e.exit());
    let epoch = value_t!(matches, "Epoch", usize).unwrap_or_else(|e| e.exit());
    let neurons = value_t!(matches, "Neurons", usize).unwrap_or_else(|e| e.exit());

    let mut dataset: Vec<Vec<f32>> = Vec::new();

    let file = File::open(matches.value_of("CSVFile").unwrap()).unwrap();
    let mut reader = csv::Reader::from_reader(file);

    for result in reader.records() {

        dataset.push(result.unwrap().iter().map(|x| {
            x.parse::<f32>().unwrap()
        }).collect());
    }

    println!("\n[+] Normalising dataset");
    normalise(&mut dataset);

    for row in &dataset {
        println!("{:?}", &row);
    }

    let __unit_length = &dataset[0].len();
    let mut units = generate_random_units(__unit_length, &neurons);
    println!("\nStarting Weights:");
    units.iter().for_each(|unit| {
        println!("{:?}", unit)
    });

    println!();

    for i in 1..epoch+1 {
        if i % 100 == 0 {
            println!("[+] Running Epoch #{:?}", &epoch);
        }


        for row in &dataset {
            let nets = calculate_nets(&row, &units);
            update_units(learning_rate, &nets, &row, &mut units);
        }
    }

    println!("\n[+] Final Weights:");
    units.iter().for_each(|unit| {
        println!("{:?}", unit)
    });
}
```

If you want to test this with the sample dataset I used, you can find it [here](https://web.archive.org/web/20190509223238/https://gist.github.com/JuxhinDB/2003e14bd6521afcc285077036f938e7). Ofcourse, feel free to test this with larger datasets, both in dimensions and count.

## Usage

The CLI for both implementations are essentially the same (minus defaults not being handled in Rust via Clap).

```none
KohonenGrossberg-NN.exe --help
KohonenGrossberg-NN 1.0
Juxhin D. Brigjaj <juxhinbox at gmail.com>
A naive Kohonen-Grossberg Counterpropogation Network in Rust

USAGE:
    KohonenGrossberg-NN.exe --csv-file <CSVFile> --epoch <Epoch> --learning-rate <R> --neurons <Neurons>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -f, --csv-file <CSVFile>    Path to CSV file containing dataset
    -e, --epoch <Epoch>         Number of epochs to complete
    -l, --learning-rate <R>     Float indicating the learning rate (step) the network should use (i.e. 0.1)
    -n, --neurons <Neurons>     Number of neurons (units) to generate
```

Running it against the previous dataset with the following parameters.

* Learning Rate: 0.1
* Epoch(s): 100
* Neurons: 3

```none
KohonenGrossberg-NN.exe -f "\path\to\2d-unnormalised-dataset.csv" -e 100 -l 0.1 -n 3

[+] Normalising dataset
[-0.85355574, 0.5210016]
... TRUNCATED ...
[0.99772507, -0.06741386]

Starting Weights:
[0.8944668, 0.8694155]
[0.0746305, 0.84058756]
[0.34859443, 0.71816105]

[+] Running Epoch #100

[+] Final Weights:
[0.95343673, -0.24061918]
[-0.75190365, 0.6541567]
[0.43543196, 0.8938945]
```

If we take each distinct section and plot it using [desmos](https://www.desmos.com/calculator) we can observe the result.

##### Legend

* Green circle: normalised dataset
* Purple circle: Initial random weights
* Black cross: Weights localised to each cluster

> _Unfortunately these images have been lost in time. If demanded, I can attempt to recreate them_.

## Performance

Starting off with the Python implementation on a small dataset with 5000 data entries.

```none
Measure-Command { python .\KohonenGrossberg-NN.py -f ".\2d-unnormalised-dataset.csv" -e 100 -l 0.1 -n 3 }

TotalSeconds      : 4.2432031
TotalMilliseconds : 4243.2031
```

Compared with the Rust implementation on the same dataset.

```none
Measure-Command { .\KohonenGrossberg-NN.exe -f ".\2d-unnormalised-dataset.csv" -e 100 -n 3 -l 0.1 }

TotalSeconds      : 0.0667547
TotalMilliseconds : 66.7547
```

## Results

I kept increasing the dataset in checkpoints increasing over time up to 150k lines.

```none
Python      Rust        Lines
72.114      13.1077     24
117.7726    18.2308     48
141.9611    18.8265     100
476.7803    21.0633     500
884.6529    23.1228     1000
4243.2031   66.7547     4999
124274.4748 1751.4639   150000
```

Whilst I did expect Rust to be faster, the margin seemed excessive. Profiling the Python implementation, I noticed the following.

```none
Name                                # Calls Time(ms)
<built-in method builtins.round>    5508904	2701
calculate_nets	                    499900	3588
update_units	                      499900	1273
main	                              1	    5098
```

Looks like the built-in `round()` function took 53% of the execution time! Removing all floating-point `round()` calls alters the result by a noticeable degree.

```none
Python      Rust        Lines
59.4613     13.1077     24
64.5703     18.2308     48
79.1932     18.8265     100
174.9629    21.0633     500
304.2106    23.1228     1000
1321.0426   66.7547     4999
39051.7759  1751.4639   150000
```

Overall we are looking at a ~22x advantage that Rust has over Python. This seemed more reasonble compared to the previous result.

## Thoughts

The idea behind this post was not to point out how fast Rust is compared to Python. Rather, how fast it is against the ease-of-use that Python provides.

There are many simple scenarios that Python handles beautifully if accurate assumptions are made. For example, getting the index of the largest `f32` in a list.

In Python we can simply write the following.

```python
_i = nets.index(max(nets))
```

The same can't be said for Rust (see [this](https://web.archive.org/web/20190509223238/https://stackoverflow.com/questions/53903318/rust-idiomatic-way-to-get-index-of-max-float-value-in-a-vec?noredirect=1#comment94651877_53903318) StackOverflow question I posted).

```rust
let mut iter = nets.iter().enumerate();
let init = iter.next().unwrap();

let _i = iter.try_fold(init, |acc, x| {
    if let Some(_i) = x.1.partial_cmp(acc.1) {
        Some(if let std::cmp::Ordering::Greater = _i {
            x
        } else {
            acc
        })
    } else {
        None
    }
}).unwrap().0;
```

Another example is generating the initial random weights. Using uniform distribution in python is beautiful.

```python
for _ in range(0, row_len):
      _result.append([random.uniform(0.0, 1.0) for _ in range(0, col_len)])
```

Whereas with Rust it's far more obscure and hard to understand, at the very least to an untrained eye (in functional programming).

> **Note** &mdash; looking back at this blog post I would definitely say that the second example is a lot more intuitive to read!

```rust
let mut rng = rand::thread_rng();

std::iter::repeat_with(||
    Standard.sample_iter(&mut rng).take(*col_len).collect())
    .take(*row_len)
    .collect()
```

## Conclusion

Despite the odd quirks and syntax, I'm in love with Rust, its performance, compiler and community. That said, I still use Python every day for most operations and feel that I can use Rust and Python hand-in-hand to complement eachother when needed.I can also see myself creating effective Proof-of-Concepts in Python then porting them over to Rust
