#[macro_use]
extern crate clap;

use clap::{App, Arg};
use std::iter::FromIterator;
use std::collections::HashSet;

#[derive(Debug)]
struct BerlekampMassey {
    N: usize,
    s: Vec<i32>,
}

impl BerlekampMassey {
    fn new(sequence: Vec<i32>) -> BerlekampMassey {
        BerlekampMassey {
            N: sequence.len(),
            s: sequence
        }
    }

    fn to_string(&self, polynomial: HashSet<i32>) -> String {
        let mut result = "".to_string();

        let mut list: Vec<i32> = polynomial.iter().map(|x| *x).collect();
        list.sort_by(|a, b| b.cmp(a));

        for i in list.iter() {
            if *i == 0 {
                result.push_str("1");
            } else {
                result.push_str(&format!("x^{:?}", i)[..]);
            }

            if *i != *list.last().unwrap() {
                result.push_str(" + ");
            }
        }

        result
    }

    fn compute(&mut self) -> (String, usize) { 
        let mut length = 0;
        let mut f: HashSet<i32> = HashSet::new();

        for k in 0..self.N {
            if self.s[k] == 1 {
                break;
            }

            // Used to denote the polynomial
            let _f = [k + 1, 0];  // Compiler cries if done inline due to temp value dropping too early
            f = HashSet::from_iter(_f.iter().map(|i| *i as i32));
            length = k + 1;

            let mut g: HashSet<i32> = HashSet::new();
            g.insert(0);

            let mut a: i32 = k as i32;
            let mut b: i32 = 0;

            for n in k+1..self.N { 
                let mut d = 0;

                &f.iter().for_each(|element| {
                    d ^= self.s[*element as usize + n - length];                    
                });

                if d == 0 {
                    b += 1;
                } else {
                    if 2 * length > n { 
                        let _tmp: Vec<i32> = g.iter().map(|element| a - b + *element).collect();
                        let _tmp_set: HashSet<&i32> = HashSet::from_iter(_tmp.iter());
                        let mut _new_set: HashSet<i32> = HashSet::new();

                        _tmp_set.iter().for_each(|element| {
                            if !f.contains(element) {
                                _new_set.insert(**element);
                            }
                        });     

                        f.iter().for_each(|element| {
                            if !_tmp_set.contains(element) {
                                _new_set.insert(*element);
                            }
                        });

                        f = _new_set;

                        b += 1;                   
                    } else {
                        let _f = f.clone();
                        f.clear();
                        let mut _tmp = HashSet::new();
                     
                        for element in _f.iter() {
                            _tmp.insert(b - a + element);
                        }
                        
                        _tmp.iter().for_each(|element| {
                            if !g.contains(element) {
                                f.insert(*element);
                            }
                        });

                        g.iter().for_each(|element| {
                            if !_tmp.contains(element) {
                                f.insert(*element);
                            }
                        });                        

                        g = _f;
                        length = n + 1 - length;
                        a = b;
                        b = n as i32 - length as i32 + 1;
                    }
                }                  
            }
        }

        (self.to_string(f), length)
    }
}

fn main() {
    let matches = App::new("Berlekamp-Massey")
                    .arg(Arg::with_name("sequence")
                        .multiple(true))
                    .get_matches();

    let string_sequence = matches.values_of_lossy("sequence").unwrap();
    let sequence: Vec<i32> = string_sequence.iter().map(|x| x.parse::<i32>().unwrap()).collect();

    let mut bm = BerlekampMassey::new(sequence);
    let (result, l) = bm.compute();

    println!("Input Sequence:\t{:?}", bm.s);
    println!("\tResult:\t{:?}\n\tLength: {:?}", result, l);
}
