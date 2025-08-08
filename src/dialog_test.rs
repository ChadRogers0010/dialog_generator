#![allow(unused_assignments, unused)]
use std::sync::{Arc, Mutex};

use dialog_lib::query;
use rayon::iter::ParallelIterator;
use rayon::iter::{IntoParallelIterator, IntoParallelRefIterator};

const DEFAULT_CAPICITY: usize = 5000;
const TEMP_CAPICITY: usize = 100;

pub fn dialog_test(count: u32, responses_true: bool, multithread: bool, flatten: bool) {
    let predicate_default = match responses_true {
        true => 50,
        false => 0,
    };

    let predicates = (0..1000)
        // .map(|_| rand::random_range(0..100))
        .map(|_| predicate_default)
        .collect::<Vec<i32>>();
    std::hint::black_box(&predicates);

    let queries = dialog_lib::query();

    for _ in 0..count {
        let mut response_count = 0;
        let time = std::time::Instant::now();

        let mut responses = Vec::with_capacity(DEFAULT_CAPICITY);

        match (multithread, flatten) {
            (false, _) => {
                for q in &queries {
                    q(&predicates, &mut responses);
                }
            }

            (true, true) => {
                responses.extend(
                    (&queries)
                        .into_par_iter()
                        .map(|q| {
                            let mut temp_vec = Vec::with_capacity(TEMP_CAPICITY);
                            q(&predicates, &mut temp_vec);
                            temp_vec
                        })
                        .flatten()
                        .collect::<Vec<&'static str>>(),
                );
            }

            (true, false) => {
                let mut thing = Arc::new(Mutex::new(&mut responses));
                (&queries).into_par_iter().map(|q| {
                    let mut temp = Vec::with_capacity(TEMP_CAPICITY);
                    q(&predicates, &mut temp);
                    let mut handle = thing.lock().unwrap();
                    handle.extend(temp)
                });
            }
            _ => panic!(),
        }

        let elapsed = time.elapsed();

        response_count = responses.len();
        println!("Response {}", response_count);
        println!("Time elapsed: {elapsed:?}");
        drop(responses);
    }
}
