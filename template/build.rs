use clap::CommandFactory;
use clap_complete::generate_to;
use miette::{IntoDiagnostic, Result};
use std::fs;

include!("src/cli.rs");

fn main() -> Result<()> {
    fs::create_dir_all("completions").into_diagnostic()?;

    let mut cli = Cli::command();

    let bin_name = cli.get_name().to_string();

    for &shell in Shell::value_variants() {
        generate_to(shell, &mut cli, bin_name.clone(), "completions").into_diagnostic()?;
    }

    Ok(())
}
