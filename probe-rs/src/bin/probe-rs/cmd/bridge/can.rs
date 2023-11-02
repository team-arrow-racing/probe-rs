#[derive(clap::Parser)]
pub struct Cmd {
    #[clap(subcommand)]
    subcommand: Subcommand,
}

#[derive(clap::Subcommand)]
pub enum Subcommand {
    /// Send a CAN Bus frame.
    #[clap(name = "write")]
    Write,
    /// Read CAN Bus frames.
    #[clap(name = "read")]
    Read,
}

impl Cmd {
    pub fn run(self) -> anyhow::Result<()> {
        use Subcommand::*;
        match self.subcommand {
            Read => println!("CAN Read"),
            Write => println!("CAN Write"),
        }

        Ok(())
    }
}
