//! Sequences for the ESP32C6/ESP32H2.

use std::sync::Arc;

use super::RiscvDebugSequence;
use crate::MemoryInterface;

/// The debug sequence implementation for the ESP32C6/ESP32H2.
pub struct ESP32C6H2(());

impl ESP32C6H2 {
    /// Creates a new debug sequence handle for the ESP32C6/ESP32H2.
    pub fn create() -> Arc<dyn RiscvDebugSequence> {
        Arc::new(Self(()))
    }
}

impl RiscvDebugSequence for ESP32C6H2 {
    fn on_connect(
        &self,
        interface: &mut crate::architecture::riscv::communication_interface::RiscvCommunicationInterface,
    ) -> Result<(), crate::Error> {
        tracing::info!("Disabling esp32c6/esp32h2 watchdogs...");
        // disable super wdt
        interface.write_word_32(0x600B1C20, 0x50D83AA1u32)?; // write protection off
        let current = interface.read_word_32(0x600B_1C1C)?;
        interface.write_word_32(0x600B_1C1C, current | 1 << 18)?; // set RTC_CNTL_SWD_AUTO_FEED_EN
        interface.write_word_32(0x600B1C20, 0x0)?; // write protection on

        // tg0 wdg
        interface.write_word_32(0x6000_8064, 0x50D83AA1u32)?; // write protection off
        interface.write_word_32(0x6000_8048, 0x0)?;
        interface.write_word_32(0x6000_8064, 0x0)?; // write protection on

        // tg1 wdg
        interface.write_word_32(0x6000_9064, 0x50D83AA1u32)?; // write protection off
        interface.write_word_32(0x6000_9048, 0x0)?;
        interface.write_word_32(0x6000_9064, 0x0)?; // write protection on

        // rtc wdg
        interface.write_word_32(0x600B_1C18, 0x50D83AA1u32)?; // write protection off
        interface.write_word_32(0x600B_1C00, 0x0)?;
        interface.write_word_32(0x600B_1C18, 0x0)?; // write protection on

        Ok(())
    }
}
