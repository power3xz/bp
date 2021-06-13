use anyhow::{Context, Result};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Cli {
    pattern: String,
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

#[derive(Debug)]
struct CustomError(String);

fn main() -> Result<()> {
    let args = Cli::from_args();
    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("colud not read file `{}`", args.path.to_str().unwrap()))?;
    for line in content.lines() {
        println!("{}", line)
    }
    Ok(())
}
