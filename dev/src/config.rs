use std::env;
use std::fmt::{Display, Formatter};
use std::path::PathBuf;
use std::process::Command;
use std::sync::atomic::{AtomicBool, Ordering};

use lazy_static::lazy_static;
use log::trace;
use simplelog::{ColorChoice, ConfigBuilder, LevelFilter, TermLogger, TerminalMode};

use crate::utils;

lazy_static! {
    /// Path to project base
    pub static ref PROJECT_PATH: PathBuf = {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        assert!(path.pop());
        path
    };

    /// Path to the `cargo` binary
    pub static ref CARGO_BIN: PathBuf = {
        env::var_os("CARGO_BIN").map_or_else(|| "cargo".into(), PathBuf::from)
    };

    /// Version string of the `cargo` binary
    static ref CARGO_VERSION: String = {
        utils::execute_command_expect_stdout_only(
            Command::new(CARGO_BIN.as_path()).arg("--version")
        ).unwrap_or_else(|e| panic!("{e}"))
    };

    /// Path to the `solana` binary
    pub static ref SOLANA_BIN: PathBuf = {
        env::var_os("SOLANA_BIN").map_or_else(|| "solana".into(), PathBuf::from)
    };

    /// Version string of the `solana` binary
    static ref SOLANA_VERSION: String = {
        utils::execute_command_expect_stdout_only(
            Command::new(SOLANA_BIN.as_path()).arg("--version")
        ).unwrap_or_else(|e| panic!("{e}"))
    };
}

/// Marks whether initialization is completed
static INITIALIZED: AtomicBool = AtomicBool::new(false);

/// Mode of operation
pub enum Mode {
    /// production mode
    Prod,
    /// development mode
    Dev,
    /// debug mode
    Debug,
    /// verbose mode
    Verbose,
}

impl Display for Mode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Prod => write!(f, "production"),
            Self::Dev => write!(f, "development"),
            Self::Debug => write!(f, "debug"),
            Self::Verbose => write!(f, "verbose"),
        }
    }
}

lazy_static! {
    /// Which mode to run on (default to development mode)
    pub static ref MODE: Mode = {
        let setting = env::var(format!("ANCHOR_VERBOSE"))
            .or(env::var("VERBOSE"))
            .or(env::var("V"));
        let verbosity = match setting {
            Ok(val) => val.parse::<usize>().ok(),
            Err(_) => None,
        }.unwrap_or(1);

        match verbosity {
            0 => Mode::Prod,
            1 => Mode::Dev,
            2 => Mode::Debug,
            _ => Mode::Verbose,
        }
    };
}

/// initialize all configs
pub fn initialize() {
    // check whether we need to run the initialization process
    match INITIALIZED.compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst) {
        Ok(false) => (),
        Err(true) => return,
        _ => panic!("invalid result from atomic reading"),
    }

    // logging
    let level = match *MODE {
        Mode::Prod => LevelFilter::Warn,
        Mode::Dev => LevelFilter::Info,
        Mode::Debug => LevelFilter::Debug,
        Mode::Verbose => LevelFilter::Trace,
    };
    let mut config = ConfigBuilder::new();
    config
        .set_location_level(LevelFilter::Off)
        .set_target_level(LevelFilter::Off)
        .set_thread_level(LevelFilter::Off)
        .set_time_level(LevelFilter::Off);
    TermLogger::init(
        level,
        config.build(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )
    .expect("logging facility should be initialized");

    // record versions
    trace!("Cargo version: {}", CARGO_VERSION.as_str());
    trace!("Solana version: {}", SOLANA_VERSION.as_str());
}
