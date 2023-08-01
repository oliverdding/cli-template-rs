use clap::CommandFactory;
use clap_complete::{generate, Generator};
use std::ffi::OsStr;
use std::{env, path::Path};

pub async fn run<G: Generator>(shell: G) {
    let mut cli = crate::cli::Cli::command();

    // get the name of the executable at runtime. see also: https://stackoverflow.com/a/36848555
    let bin_name = match env::current_exe()
        .ok()
        .as_ref()
        .map(Path::new)
        .and_then(Path::file_name)
        .and_then(OsStr::to_str)
        .map(String::from)
    {
        Some(v) => v,
        None => cli.get_name().to_string(),
    };

    generate(shell, &mut cli, bin_name, &mut std::io::stdout());
}
