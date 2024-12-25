#[cfg(feature = "clap")]
mod cli;

#[cfg(feature = "fast")]
mod fast;

#[cfg(feature = "clap")]
pub fn main() -> Result<(), anyhow::Error> {
    cli::Cli::run()
}

#[cfg(feature = "fast")]
pub fn main() -> Result<(), anyhow::Error> {
    if std::env::args().any(|a| &a == "--version") {
        println!("0.12.2");
        Ok(())
    } else {
        fast::run()
    }
}
