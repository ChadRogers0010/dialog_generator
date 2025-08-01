use clap::Parser;

use build_query::build_query;
use csv::create_test_csv;
use gen_c::build_query_c;

mod build_query;
mod csv;
mod gen_c;
mod radix_array;
mod utils;

#[cfg(test)]
mod health;
#[cfg(test)]
mod health_test;

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
}
