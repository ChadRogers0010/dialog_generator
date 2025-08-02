// #[cfg(alt_test)]

pub fn test_dialog_test() {
    use super::dialog_test::query;

    let predicates = (0..1000)
        // .map(|_| rand::random_range(0..100))
        .map(|_| 50)
        .collect::<Vec<i32>>();
    let mut response = vec![];
    std::hint::black_box(&predicates);

    let time = std::time::Instant::now();

    std::hint::black_box(query(&predicates, &mut response));
    let elapsed = time.elapsed();

    println!("Response {}", response.len());
    println!("Time elapsed: {elapsed:?}");
}

// #[cfg(not(alt_test))]
// pub fn test_dialog_test() {
//     println!("Alt_test_not_present");
// }
