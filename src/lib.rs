use clap;
use std::error::Error;

type MyResult<E> = Result<E, Box<dyn Error>>;

pub struct Config {
    //For now, we will only support files
    files: Vec<String>,
}

pub fn get_args() -> MyResult<Config> {
    unimplemented!()
}

pub fn run(config: Config) -> MyResult<()> {
    unimplemented!()
}
