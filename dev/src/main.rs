use anchor_cli::config::{BootstrapMode, ConfigOverride, ProgramArch};
use anchor_dev::config::initialize;
use anchor_dev::testing;
use clap::{Parser, Subcommand};
use log::info;
use regex::Regex;
use std::env::current_dir;

#[derive(Parser)]
#[clap(
    name = env!("CARGO_PKG_NAME"),
    about = env!("CARGO_PKG_DESCRIPTION"),
    rename_all = "kebab-case"
)]
struct Args {
    /// Test case filter
    #[clap(long)]
    filter: Option<String>,

    /// Action
    #[clap(subcommand)]
    action: Action,
}

#[derive(Subcommand)]
enum Action {
    /// List all test cases
    List,

    /// Build all test cases
    Build,
}

fn main() {
    initialize();

    let Args { filter, action } = Args::parse();
    let all_test_cases = testing::list();
    let filtered_cases = match filter {
        None => all_test_cases,
        Some(filter) => {
            let regex =
                Regex::new(&filter).unwrap_or_else(|_| panic!("invalid regex filter: {filter}"));
            all_test_cases
                .into_iter()
                .filter(|t| regex.is_match(&t.name))
                .collect()
        }
    };

    match action {
        Action::List => {
            for case in filtered_cases {
                println!("[{}]", case.name);
                println!("  - path: {}", case.path.display());
            }
        }
        Action::Build => {
            for case in filtered_cases {
                info!("building {}", case.name);

                let cwd = current_dir().unwrap();
                std::env::set_current_dir(case.path).unwrap();
                anchor_cli::build(
                    &ConfigOverride::default(),
                    false,
                    None,
                    None,
                    false,
                    false,
                    None,
                    None,
                    None,
                    BootstrapMode::None,
                    None,
                    None,
                    vec![],
                    vec![],
                    false,
                    ProgramArch::Sbf,
                )
                .unwrap();
                std::env::set_current_dir(cwd).unwrap();
            }
        }
    }
}
