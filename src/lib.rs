use clap::Parser;
use std::error::Error;

type MyResult<E> = Result<E, Box<dyn Error>>;

#[derive(Parser)] //Using clap v3 derive feature, similar to structops
pub struct Config {
    //For now, we will only support files
    #[clap(default_value = "-")]
    // "-" prompts the application to read from the stdin
    /*Use triple bars to add text to the help menu */
    /// input File(s)
    files: Vec<String>,

    #[clap(long, short = 'k')]
    /// Secret key to encrypt.
    // To make an argument optional, simply wrap them in an Option
    key: Option<String>,
}

pub fn get_args() -> MyResult<Config> {
    let config = Config::parse();

    Ok(config)
}

pub fn run(config: Config) -> MyResult<()> {
    /*
    How the app should work:
    1. If key is not provided, generate a new one.
    2. Proceed to encrypt

    */

    match config.key {
        Some(key) => {}
        None => {}
    }

    let files = config.files;

    // iterate through all the files and encrypting them

    for file in files {}

    Ok(())
}
