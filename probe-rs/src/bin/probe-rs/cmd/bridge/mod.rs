pub mod can;
pub mod gpio;
pub mod i2c;
pub mod spi;

#[derive(clap::Parser)]
pub struct Cmd {
    #[clap(subcommand)]
    subcommand: Subcommand,
}

#[derive(clap::Subcommand)]
pub enum Subcommand {
    /// CAN Bus interface control.
    #[clap(name = "can")]
    Can(can::Cmd),
    /// GPIO interface control.
    #[clap(name = "gpio")]
    Gpio(gpio::Cmd),
    /// I2C interface control.
    #[clap(name = "i2c")]
    I2c(i2c::Cmd),
    /// SPI interface control.
    #[clap(name = "spi")]
    Spi(spi::Cmd),
}

impl Cmd {
    pub fn run(self) -> anyhow::Result<()> {
        use Subcommand::*;
        match self.subcommand {
            Can(cmd) => cmd.run(),
            Gpio(cmd) => cmd.run(),
            I2c(cmd) => cmd.run(),
            Spi(cmd) => cmd.run(),
        }
    }
}
