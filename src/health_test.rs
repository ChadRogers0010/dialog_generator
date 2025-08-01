#[test]
fn test_rust() {
    use crate::health::query;

    let predicates = (0..1000)
        .map(|_| rand::random_range(0..100))
        .collect::<Vec<i32>>();
    let mut response = vec![];
    std::hint::black_box(&predicates);

    let time = std::time::Instant::now();

    std::hint::black_box(query(&predicates, &mut response));
    let elapsed = time.elapsed();

    println!("Response {}: {response:?}", response.len());
    println!("Time elapsed: {elapsed:?}");
    assert!(false);
}
