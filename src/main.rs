use clap::Parser;

use build_query::build_query;
use csv::create_test_csv;
use gen_c::build_query_c;

mod build_query;
mod csv;
mod gen_c;
mod radix_array;
mod struct_builder;
mod utils;

#[cfg(test)]
pub mod dialog_test;
#[cfg(alt_test)]
pub mod dialog_test;
#[cfg(test)]
mod health;
#[cfg(test)]
mod health_test;
#[cfg(test)]
mod test_dialog_test;

#[cfg(alt_test)]
fn foob() {
    use crate::dialog_test::*;

    let predicates = (0..1000)
        .map(|_| rand::random_range(0..100))
        .collect::<Vec<i32>>();
    let mut response = vec![];
    std::hint::black_box(&predicates);

    let time = std::time::Instant::now();

    std::hint::black_box(query(&predicates, &mut response));
    let elapsed = time.elapsed();

    println!("Response {}", response.len());
    println!("Time elapsed: {elapsed:?}");
    assert!(false);
}

#[cfg(not(alt_test))]
fn foob() {
    println!("Alt_test_not_present");
}

#[derive(Parser)]
struct Cli {
    #[arg(short)]
    /// Build the rust verion
    build: bool,
    /// build the c version
    #[arg(short)]
    c_build: bool,
    #[arg(short)]
    /// Number of parameters
    test: Option<i32>,
    #[arg(short, default_value = "100")]
    /// number of if statements
    statements: i32,
    #[arg(short)]
    query: bool,
    #[arg(short)]
    new_test: bool,
}

fn main() {
    let cli = Cli::parse();
    if let Some(n) = cli.test {
        create_test_csv(n, cli.statements);
    }

    let csv = "./test.csv";
    if cli.build {
        build_query(csv);
    }

    if cli.c_build {
        build_query_c(csv)
    }

    if cli.new_test {
        struct_builder::build_query(csv, 100);
    }

    foob();
}
