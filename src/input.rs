use clap::Parser;

pub type N = u64;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Input {
    #[arg(index=1)]
    pub number: N,
    #[arg(short, long, default_value_t=false)]
    pub metrics: bool,
}

impl Input {
    pub fn get() -> Self {
        Self::parse()
    }
}