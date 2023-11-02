#[derive(clap::Parser)]
pub struct Cmd {
    #[clap(subcommand)]
    subcommand: Subcommand,
}

#[derive(clap::Subcommand)]
pub enum Subcommand {
    /// Write I2C messages.
    #[clap(name = "write")]
    Write,
    /// Read I2C messages.
    #[clap(name = "read")]
    Read {
        no_wait: bool,
    },
}

impl Cmd {
    pub fn run(self) -> anyhow::Result<()> {
        use Subcommand::*;
        match self.subcommand {
            Read => println!("Read"),
            Write => println!("Write"),
        }

        Ok(())
    }
}
