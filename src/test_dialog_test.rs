pub fn test_dialog_test() {
    let predicates = (0..1000)
        // .map(|_| rand::random_range(0..100))
        .map(|_| 50)
        .collect::<Vec<i32>>();
    let mut response = vec![];
    std::hint::black_box(&predicates);

    let time = std::time::Instant::now();

    std::hint::black_box(dialog_lib::query(&predicates, &mut response));
    let elapsed = time.elapsed();

    println!("Response {}", response.len());
    println!("Time elapsed: {elapsed:?}");
}
