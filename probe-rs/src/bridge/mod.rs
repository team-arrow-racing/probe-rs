pub mod can;

use crate::DebugProbeError;

#[derive(thiserror::Error, Debug)]
pub enum BridgeError {
    #[error("Debug Probe Error")]
    DebugProbe(#[from] DebugProbeError),
}

pub struct BridgeInterface;
