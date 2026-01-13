pub mod dbc;
use autonexus_common::{AutoNexusResult, CanFrame, LinFrame};
use autonexus_hw::HardwareAdapter;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

pub struct UdsSession {
    adapter: Arc<dyn HardwareAdapter>,
    request_id: u32,
    response_id: u32,
}

impl UdsSession {
    pub fn new(adapter: Arc<dyn HardwareAdapter>, request_id: u32, response_id: u32) -> Self {
        Self {
            adapter,
            request_id,
            response_id,
        }
    }

    pub async fn diagnostic_session_control(&self, session_type: u8) -> AutoNexusResult<Vec<u8>> {
        let frame = CanFrame {
            id: self.request_id,
            data: vec![0x02, 0x10, session_type, 0x00, 0x00, 0x00, 0x00, 0x00],
            is_extended: false,
            is_fd: false,
            timestamp: 0,
        };
        self.adapter.send_can(frame).await?;

        loop {
            let response = self.adapter.read_can().await?;
            if response.id == self.response_id
                && response.data.len() > 1
                && response.data[1] == 0x50
            {
                return Ok(response.data);
            }
            // In real app, we would have a timeout here.
        }
    }

    pub async fn read_data_by_identifier(&self, identifier: u16) -> AutoNexusResult<Vec<u8>> {
        let id_high = (identifier >> 8) as u8;
        let id_low = (identifier & 0xFF) as u8;

        let frame = CanFrame {
            id: self.request_id,
            data: vec![0x03, 0x22, id_high, id_low, 0x00, 0x00, 0x00, 0x00],
            is_extended: false,
            is_fd: false,
            timestamp: 0,
        };

        self.adapter.send_can(frame).await?;

        loop {
            let response = self.adapter.read_can().await?;
            if response.id == self.response_id
                && response.data.len() > 1
                && response.data[1] == 0x62
            {
                return Ok(response.data);
            }
        }
    }
}

pub struct LinSession {
    adapter: Arc<dyn HardwareAdapter>,
}

impl LinSession {
    pub fn new(adapter: Arc<dyn HardwareAdapter>) -> Self {
        Self { adapter }
    }

    /// Sends a master request frame (Master provides data)
    pub async fn send_master_request(&self, id: u8, data: Vec<u8>) -> AutoNexusResult<()> {
        let frame = LinFrame {
            id,
            data,
            timestamp: 0,
        };
        self.adapter.send_lin(frame).await
    }

    /// Sends a slave response request (Master requests data from slave)
    pub async fn request_slave_response(&self, _id: u8) -> AutoNexusResult<LinFrame> {
        // In a real adapter, this would trigger a header transmission and then a reception.
        // For the mock, we just call read_lin.
        self.adapter.read_lin().await
    }
}

pub struct DoIpSession {
    pub target_address: u16,
    pub source_address: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DoIpMessage {
    pub protocol_version: u8,
    pub inverse_protocol_version: u8,
    pub payload_type: u16,
    pub payload_length: u32,
    pub payload: Vec<u8>,
}

impl DoIpSession {
    pub fn new(target_address: u16, source_address: u16) -> Self {
        Self {
            target_address,
            source_address,
        }
    }

    pub fn pack_message(&self, payload_type: u16, payload: Vec<u8>) -> DoIpMessage {
        DoIpMessage {
            protocol_version: 0x02,
            inverse_protocol_version: 0xFD,
            payload_type,
            payload_length: payload.len() as u32,
            payload,
        }
    }
}
