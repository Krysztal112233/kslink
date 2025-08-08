use clap::{arg, Args, Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "kslink-rules",
    author = env!("CARGO_PKG_AUTHORS"),
    version = env!("CARGO_PKG_VERSION"),
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Create a new rule
    Create,

    /// Format rule files
    Fmt(FmtCmd),

    /// Validate rule files
    Check(CheckCmd),
}

#[derive(Args)]
pub struct FmtCmd {
    /// Rule file(s) to format
    #[arg(short, long)]
    pub file: Option<Vec<String>>,

    /// Path pattern
    #[arg(short, long)]
    pub pattern: Option<Vec<String>>,

    /// Preview changes without writing files
    #[arg(short, long)]
    pub dry: bool,
}

#[derive(Args)]
pub struct CheckCmd {
    /// Rule file(s) to validate
    #[arg(short, long)]
    pub file: Option<Vec<String>>,

    /// Path pattern
    #[arg(short, long)]
    pub directory: Option<String>,

    /// Number of files to process concurrently
    #[arg(long)]
    pub parallel: Option<usize>,
}
