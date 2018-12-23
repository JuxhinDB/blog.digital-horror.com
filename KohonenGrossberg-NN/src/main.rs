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
