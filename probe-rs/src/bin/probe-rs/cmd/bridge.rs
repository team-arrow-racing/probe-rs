

#[derive(clap::Parser)]
pub struct Cmd {
    #[clap(subcommand)]
    subcommand: Subcommand,
}

#[derive(clap::Subcommand)]
pub enum Subcommand {
    /// CAN Bus interface control.
    #[clap(name = "can")]
    Can,
    /// GPIO interface control.
    #[clap(name = "gpio")]
    Gpio,
    /// I2C interface control.
    #[clap(name = "i2c")]
    I2c,
    /// SPI interface control.
    #[clap(name = "spi")]
    Spi,
}

impl Cmd {
    pub fn run(self) -> anyhow::Result<()> {
        use Subcommand::*;
        match self.subcommand {
            Can => println!("CAN Hello World"),
            Gpio => println!("GPIO Hello World"),
            I2c => println!("I2C Hello World"),
            Spi => println!("SPI Hello World"),
        }

        Ok(())
    }
}
