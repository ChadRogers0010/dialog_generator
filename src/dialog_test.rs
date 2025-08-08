use std::sync::{Arc, Mutex};
use std::time::Duration;

use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;
use utils::flush_cache;

const DEFAULT_CAPICITY: usize = 5000;
const TEMP_CAPICITY: usize = 100;

pub fn dialog_test(count: u32, responses_true: bool, multithread: bool, flatten: bool, all: bool) {
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

    if all {
        test_all(count, &queries, &predicates);
        return;
    }

    for i in 0..count {
        let response_count;
        let time = std::time::Instant::now();

        let mut responses = Vec::with_capacity(DEFAULT_CAPICITY);

        match (multithread, flatten) {
            (false, _) => test_default(&mut responses, &queries, &predicates),
            (true, false) => test_multithread(&mut responses, &queries, &predicates),
            (true, true) => test_multithread_flatten(&mut responses, &queries, &predicates),
        }

        let elapsed = time.elapsed();

        response_count = responses.len();
        println!("Response {}", response_count);
        println!("Time elapsed: {elapsed:?}");

        if i != count - 1 {
            flush_cache();
        }
    }
}

#[allow(unused)]
fn test_all(
    count: u32,
    queries: &Vec<for<'a, 'b> fn(&'a [i32], &'b mut Vec<&'static str>)>,
    predicates: &[i32],
) {
    let permutations = [
        ((false, false), "default"),
        ((true, false), "multithread"),
        ((true, true), "multithread_flatten"),
    ];
    let mut time_scores = Vec::new();

    for ((multithread, flatten), name) in permutations {
        let mut response_count;

        let mut responses = Vec::with_capacity(DEFAULT_CAPICITY);

        let mut average_sum = 0;
        for i in 0..count {
            let time = std::time::Instant::now();

            match (multithread, flatten) {
                (false, _) => test_default(&mut responses, &queries, &predicates),
                (true, false) => test_multithread(&mut responses, &queries, &predicates),
                (true, true) => test_multithread_flatten(&mut responses, &queries, &predicates),
            }

            let elapsed = time.elapsed();

            average_sum += elapsed.as_nanos();

            response_count = responses.len();
            // print!("{name}");
            // print!(" | Response {}", response_count);
            // println!(" | Time elapsed: {elapsed:?}");

            if i != count - 1 {
                flush_cache();
            }
            responses.clear();
        }
        let average_time = Duration::from_nanos(average_sum as u64 / count as u64);

        time_scores.push((name, average_time));

        // if count > 1 {
        //     println!("{name} average: {:?}", Duration::from_nanos(average_time));
        // }
    }

    for (name, average_time) in time_scores {
        println!("{name}: {average_time:?}");
    }
}

fn test_default(
    responses: &mut Vec<&'static str>,
    queries: &Vec<for<'a, 'b> fn(&'a [i32], &'b mut Vec<&'static str>)>,
    predicates: &[i32],
) {
    for q in queries {
        q(&predicates, responses);
    }
}

fn test_multithread(
    responses: &mut Vec<&'static str>,
    queries: &Vec<for<'a, 'b> fn(&'a [i32], &'b mut Vec<&'static str>)>,
    predicates: &[i32],
) {
    let arc_mut_vec = Arc::new(Mutex::new(responses));
    (&queries).into_par_iter().for_each(|q| {
        let mut temp = Vec::with_capacity(TEMP_CAPICITY);
        q(&predicates, &mut temp);
        arc_mut_vec.lock().unwrap().extend(temp)
    });
}

fn test_multithread_flatten(
    responses: &mut Vec<&'static str>,
    queries: &Vec<for<'a, 'b> fn(&'a [i32], &'b mut Vec<&'static str>)>,
    predicates: &[i32],
) {
    responses.extend(
        (&queries)
            .into_par_iter()
            .map(|q| {
                let mut temp = Vec::with_capacity(TEMP_CAPICITY);
                q(&predicates, &mut temp);
                temp
            })
            .flatten()
            .collect::<Vec<&'static str>>(),
    );
}
