use clap::Parser;

pub type N = u64;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Input {
    #[arg(index=1)]
    pub number: N,
}

impl Input {
    pub fn get() -> Self {
        Self::parse()
    }
}