#[derive(clap::Parser)]
pub struct Cmd {
    #[clap(subcommand)]
    subcommand: Subcommand,
}

#[derive(clap::Subcommand)]
pub enum Subcommand {
    /// Set GPIO pin states.
    #[clap(name = "write")]
    Write,
    /// Reag GPIO pin states.
    #[clap(name = "read")]
    Read,
    /// Chip-select (cs) control.
    #[clap(name = "cs")]
    Cs,
}

impl Cmd {
    pub fn run(self) -> anyhow::Result<()> {
        use Subcommand::*;
        match self.subcommand {
            Read => println!("Read"),
            Write => println!("Write"),
            Cs => println!("Chip Select")
        }

        Ok(())
    }
}
