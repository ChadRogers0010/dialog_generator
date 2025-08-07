use clap::Parser;

use build_query::build_query;
use csv::create_test_csv;
use gen_c::build_query_c;
use test_dialog_test::test_dialog_test;

mod build_query;
mod csv;
pub mod dialog_test;
mod gen_c;
mod radix_array;
mod struct_builder;
mod utils;

#[cfg(test)]
mod health;
#[cfg(test)]
mod health_test;
mod test_dialog_test;

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

    test_dialog_test();
}
