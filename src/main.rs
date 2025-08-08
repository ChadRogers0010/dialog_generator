use clap::{CommandFactory, Parser};

use generate::{build_query, build_query_c, csv};

mod dialog_test;

#[derive(Parser, Debug)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Debug print Cli
    #[arg(short, hide = true)]
    debug: bool,

    // #[arg(long)]
    #[arg(long, hide = true)]
    markdown_help: bool,
}

const LINES_PER_MODULE: &'static str = "100";

#[derive(clap::Subcommand, Debug)]
enum Commands {
    /// Generate a test csv
    Csv {
        /// Number of if statements
        #[arg(short)]
        statements: u32,

        /// Number of predicates to check
        #[arg(short)]
        predicates: u32,
    },

    /// build the Dialog_lib::query()
    Build {
        /// Number of lines per module
        #[arg(short,default_value = LINES_PER_MODULE)]
        lines_per_module: usize,

        /// Path to csv
        #[arg(short, default_value = "./test.csv")]
        csv_path: String,
    },

    /// build the c query
    BuildC {
        /// Number of lines per module
        #[arg(short,default_value = LINES_PER_MODULE)]
        lines_per_module: usize,

        /// Path to csv
        #[arg(short, default_value = "./test.csv")]
        csv_path: String,
    },

    /// Test a build
    Test {
        /// Number of times to run the test
        #[arg(short, default_value = "1")]
        count: u32,

        /// All responses succeed
        #[arg(short)]
        responses_true: bool,

        /// Multithread with Rayon
        #[arg(short)]
        multithread: bool,

        /// Rayon's into_par_iter().map().collect()
        #[arg(short)]
        flatten: bool,

        /// Test every test case
        #[arg(short)]
        all: bool,
    },
}

fn main() {
    let cli = Cli::parse();

    if cli.debug {
        println!("{cli:?}");
    }

    // Invoked as: `$ my-app --markdown-help`
    if cli.markdown_help {
        clap_markdown::print_help_markdown::<Cli>();
    }

    match cli.command {
        Some(Commands::Csv {
            statements,
            predicates,
        }) => csv::create_test_csv(predicates, statements),

        Some(Commands::Build { lines_per_module }) => build_query(cli.csv_path, lines_per_module),

        #[allow(unused)]
        Some(Commands::BuildC { lines_per_module }) => {
            build_query_c(cli.csv_path, lines_per_module)
        }

        Some(Commands::Test {
            count,
            multithread,
            responses_true,
            flatten,
            all,
        }) => dialog_test::dialog_test(count, responses_true, multithread, flatten, all),

        None => {
            Cli::command().print_help().unwrap();
        }
    }
}
