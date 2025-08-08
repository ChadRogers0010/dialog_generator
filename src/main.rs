use clap::Parser;

use generate::{build_query, build_query_c, csv};

use test_dialog_test::test_dialog_test;

mod dialog_test;
mod test_dialog_test;

#[derive(Parser)]
struct Cli {
    #[arg(short, default_value = "./test.csv")]
    csv_path: String,

    #[command(subcommand)]
    command: Commands,
}

const LINES_PER_MODULE: &'static str = "100";
#[derive(clap::Subcommand)]
enum Commands {
    Csv {
        #[arg(short)]
        statements: u32,
        #[arg(short)]
        predicates: u32,
    },
    Build {
        #[arg(default_value = LINES_PER_MODULE)]
        lines_per_module: usize,
    },
    BuildC {
        #[arg(default_value = LINES_PER_MODULE)]
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

        Commands::Build { lines_per_module } => build_query(cli.csv_path, lines_per_module),

        #[allow(unused)]
        Commands::BuildC { lines_per_module } => build_query_c(cli.csv_path, lines_per_module),

        Commands::Test => test_dialog_test(),
    }
}
