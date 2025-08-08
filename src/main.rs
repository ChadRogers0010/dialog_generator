use clap::Parser;

use generate::{build_query, build_query_c, csv};

use dialog_test::dialog_test;

mod dialog_test;

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
    Test {
        /// Number of times to run the test
        #[arg(short, default_value = "1")]
        count: u32,

        /// Multithread with Rayon
        #[arg(short)]
        multithread: bool,

        /// All responses succeed
        #[arg(short)]
        responses_true: bool,

        #[arg(short)]
        flatten: bool,
    },
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

        Commands::Test {
            count,
            multithread,
            responses_true,
            flatten,
        } => dialog_test(count, responses_true, multithread, flatten),
    }
}
