

pub mod commands {
    const CLOSE: u8 = 0x01;
    const GET_RWCMD_STATUS: u8 = 0x02;
    const GET_CLOCK: u8 = 0x03;

    // SPI
    const SPI_INIT: u8 = 0x20;
    const SPI_WRITE: u8 = 0x21;
    const SPI_READ: u8 = 0x22;
    const SPI_CS: u8 = 0x23;

    // I2C
    const I2C_INIT: u8 = 0x30;
    const I2C_WRITE: u8 = 0x31;
    const I2C_READ: u8 = 0x32;
    const I2C_READ_NO_WAIT: u8 = 0x33; // new in V3B3
    const I2C_GET_READ_DATA: u8 = 0x34; // new in V3B3

    // CAN
    const CAN_INIT: u8 = 0x40;
    const CAN_WRITE_MSG: u8 = 0x41;
    const CAN_INIT_FILTER: u8 = 0x43;
    const CAN_START_MSG_RX: u8 = 0x44;
    const CAN_STOP_MSG_RX: u8 = 0x45;
    const CAN_GET_RXMSG_NB: u8 = 0x46;
    const CAN_GET_RXMSG: u8 = 0x47;

    // GPIO
    const GPIO_INIT: u8 = 0x60;
    const GPIO_SET_RESET: u8 = 0x61;
    const GPIO_READ: u8 = 0x62;
}

/// STLink bridge status codes and errors.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Status {
    Ok,
    SpiError,
    I2cError,
    CanError,
    InitNotDone,
    UnknownCmd,
    BadParam,
    TimeoutError,
    AbortTransmission,
    InternalError,
    CmdBusy,
    Other(u8),
}

impl From<u8> for Status {
    fn from(value: u8) -> Status {
        use Status::*;
        match value {
            0x80 => Ok,
            0x02 => SpiError,
            0x03 => I2cError,
            0x04 => CanError,
            0x07 => InitNotDone,
            0x08 => UnknownCmd,
            0x09 => BadParam,
            0x0A => TimeoutError,
            0x0B => AbortTransmission,
            0x0C => InternalError,
            0x0D => CmdBusy,
            v => Other(v)
        }
    }
}
