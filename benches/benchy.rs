#[macro_use]
extern crate criterion;
extern crate quicksort;
extern crate serde;
extern crate serde_json;

use criterion::Criterion;
use std::fs::File;
use std::io::prelude::*;

fn quicksort_test(criterion: &mut Criterion) {
    let mut array_file = File::open("/Users/tristangreeno/workspace/array-gen/array.txt")
        .expect("File not found!");
    let mut contents = String::new();
    array_file.read_to_string(&mut contents).expect("Something went wrong reading file!");
    let vec: Vec<i16> = serde_json::from_str(&contents).expect("Could not serialize!");
    criterion.bench_function("quicksort of 10,000,000 i16",
                             move |b| {
                                 let mut vect = vec.to_vec();
                                 b.iter(|| { vect.sort_unstable() });
                             });
}

criterion_group!(benches, quicksort_test);
criterion_main!(benches);
