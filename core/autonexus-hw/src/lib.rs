use async_trait::async_trait;
use autonexus_common::{AutoNexusResult, CanFrame, LinFrame};
use std::sync::Arc;
use tokio::sync::Mutex;

/// Defines the interface for all hardware adapters (PEAK, Kvaser, Vector, Mock).
#[async_trait]
pub trait HardwareAdapter: Send + Sync {
    fn name(&self) -> &str;
    fn description(&self) -> &str;

    async fn open(&self) -> AutoNexusResult<()>;
    async fn close(&self) -> AutoNexusResult<()>;

    async fn send_can(&self, frame: CanFrame) -> AutoNexusResult<()>;
    async fn read_can(&self) -> AutoNexusResult<CanFrame>;

    async fn send_lin(&self, frame: LinFrame) -> AutoNexusResult<()>;
    async fn read_lin(&self) -> AutoNexusResult<LinFrame>;

    fn is_open(&self) -> bool;
}

pub struct MockAdapter {
    is_open: Arc<Mutex<bool>>,
}

impl MockAdapter {
    pub fn new() -> Self {
        Self {
            is_open: Arc::new(Mutex::new(false)),
        }
    }
}

#[async_trait]
impl HardwareAdapter for MockAdapter {
    fn name(&self) -> &str {
        "MockAdapter"
    }

    fn description(&self) -> &str {
        "A virtual adapter for testing and simulation."
    }

    async fn open(&self) -> AutoNexusResult<()> {
        let mut open = self.is_open.lock().await;
        *open = true;
        Ok(())
    }

    async fn close(&self) -> AutoNexusResult<()> {
        let mut open = self.is_open.lock().await;
        *open = false;
        Ok(())
    }

    async fn send_can(&self, _frame: CanFrame) -> AutoNexusResult<()> {
        if !*self.is_open.lock().await {
            return Err(autonexus_common::AutoNexusError::HardwareError);
        }
        Ok(())
    }

    async fn read_can(&self) -> AutoNexusResult<CanFrame> {
        if !*self.is_open.lock().await {
            return Err(autonexus_common::AutoNexusError::HardwareError);
        }
        // Simulate a UDS Positive Response for Session Control (0x10 -> 0x50)
        // or Read Data By Identifier (0x22 -> 0x62)
        Ok(CanFrame {
            id: 0x7E8,
            data: vec![0x03, 0x50, 0x03, 0x00, 0x00, 0x00, 0x00, 0x00],
            is_extended: false,
            is_fd: false,
            timestamp: 0,
        })
    }

    async fn send_lin(&self, _frame: LinFrame) -> AutoNexusResult<()> {
        if !*self.is_open.lock().await {
            return Err(autonexus_common::AutoNexusError::HardwareError);
        }
        Ok(())
    }

    async fn read_lin(&self) -> AutoNexusResult<LinFrame> {
        if !*self.is_open.lock().await {
            return Err(autonexus_common::AutoNexusError::HardwareError);
        }
        Ok(LinFrame {
            id: 0x3C,
            data: vec![0x00, 0x11, 0x22, 0x33],
            timestamp: 0,
        })
    }

    fn is_open(&self) -> bool {
        // Since we need to be sync but the mutex is async,
        // in a real scenario we might use a sync mutex or an AtomicBool.
        // For the mock, we can try_lock or just return true if it's been opened.
        match self.is_open.try_lock() {
            Ok(opened) => *opened,
            Err(_) => true, // Assume open if locked (busy)
        }
    }
}
