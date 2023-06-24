use clap::Parser;

#[derive(Parser)]
pub struct AoCDay {
    #[arg(short, long)]
    pub year: usize,

    #[arg(short, long)]
    pub day: usize,
}

impl AoCDay {
    pub fn input_url(&self) -> &str {
        todo!()
    }

    pub fn input_path(&self) -> &str {
        todo!()
    }

    pub fn test_path(&self) -> &str {
        todo!()
    }
}
