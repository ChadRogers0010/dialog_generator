use clap::Parser;

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
    #[arg(short, default_value = "./test.csv")]
    csv_path: String,

    #[command(subcommand)]
    command: Commands,
}
#[derive(clap::Subcommand)]
enum Commands {
    Csv {
        statements: u32,
        predicates: u32,
    },
    Build {
        #[arg(default_value = "100")]
        lines_per_module: usize,
    },
    BuildC {
        #[arg(default_value = "100")]
        lines_per_module: usize,
    },
    Test,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Csv {
            statements,
            predicates,
        } => csv::create_test_csv(predicates, statements),

        Commands::Build { lines_per_module } => {
            struct_builder::build_query(cli.csv_path, lines_per_module)
        }

        #[allow(unused)]
        Commands::BuildC { lines_per_module } => build_query_c(cli.csv_path, lines_per_module),

        Commands::Test => test_dialog_test(),
    }
}
