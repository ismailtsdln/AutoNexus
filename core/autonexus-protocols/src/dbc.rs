use autonexus_common::CanFrame;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Signal {
    pub name: String,
    pub start_bit: u8,
    pub bit_length: u8,
    pub is_little_endian: bool,
    pub is_signed: bool,
    pub factor: f64,
    pub offset: f64,
}

#[derive(Debug, Clone)]
pub struct Message {
    pub id: u32,
    pub name: String,
    pub length: u8,
    pub signals: Vec<Signal>,
}

pub struct DbcDatabase {
    pub messages: HashMap<u32, Message>,
}

impl DbcDatabase {
    pub fn new() -> Self {
        Self {
            messages: HashMap::new(),
        }
    }

    pub fn parse(content: &str) -> Self {
        let mut db = Self::new();
        let mut current_msg_id: Option<u32> = None;

        for line in content.lines() {
            let line = line.trim();
            if line.starts_with("BO_") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 4 {
                    let id_str = parts[1];
                    if let Ok(id) = id_str.parse::<u32>() {
                        let name = parts[2].trim_end_matches(':').to_string();
                        let length = parts[3].parse::<u8>().unwrap_or(8);
                        db.messages.insert(
                            id,
                            Message {
                                id,
                                name,
                                length,
                                signals: Vec::new(),
                            },
                        );
                        current_msg_id = Some(id);
                    }
                }
            } else if line.starts_with("SG_") && current_msg_id.is_some() {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 5 {
                    // SG_ SignalName : StartBit|BitLen@LE/Signed (Factor,Offset) [Min|Max] "Unit" Receiver
                    let name = parts[1].to_string();
                    let bit_info = parts[3]; // e.g. "0|16@1+"

                    let at_pos = bit_info.find('@').unwrap_or(0);
                    let pipe_pos = bit_info.find('|').unwrap_or(0);

                    let start_bit = bit_info[..pipe_pos].parse::<u8>().unwrap_or(0);
                    let bit_length = bit_info[pipe_pos + 1..at_pos].parse::<u8>().unwrap_or(0);

                    let le_signed = &bit_info[at_pos + 1..];
                    let is_little_endian = le_signed.starts_with('1');
                    let is_signed = le_signed.contains('-');

                    let factor_offset = parts[4]; // e.g. "(0.1,0)"
                    let fo_clean = factor_offset.trim_matches(|c| c == '(' || c == ')');
                    let fo_parts: Vec<&str> = fo_clean.split(',').collect();
                    let factor = fo_parts
                        .get(0)
                        .and_then(|s| s.parse::<f64>().ok())
                        .unwrap_or(1.0);
                    let offset = fo_parts
                        .get(1)
                        .and_then(|s| s.parse::<f64>().ok())
                        .unwrap_or(0.0);

                    if let Some(msg) = db.messages.get_mut(&current_msg_id.unwrap()) {
                        msg.signals.push(Signal {
                            name,
                            start_bit,
                            bit_length,
                            is_little_endian,
                            is_signed,
                            factor,
                            offset,
                        });
                    }
                }
            }
        }
        db
    }

    pub fn decode(&self, frame: &CanFrame) -> Option<HashMap<String, f64>> {
        let msg = self.messages.get(&frame.id)?;
        let mut decoded = HashMap::new();

        for sig in &msg.signals {
            let mut raw_val: u64 = 0;

            // Basic bit extraction for Little Endian (simpler for prototype)
            if sig.is_little_endian {
                let _start_byte = (sig.start_bit / 8) as usize;
                let _end_byte = ((sig.start_bit + sig.bit_length - 1) / 8) as usize;

                let mut bit_idx = 0;
                for i in sig.start_bit..(sig.start_bit + sig.bit_length) {
                    let byte_idx = (i / 8) as usize;
                    let bit_in_byte = i % 8;
                    if byte_idx < frame.data.len() {
                        let bit = (frame.data[byte_idx] >> bit_in_byte) & 1;
                        raw_val |= (bit as u64) << bit_idx;
                    }
                    bit_idx += 1;
                }
            }

            // Apply factor and offset
            let final_val = (raw_val as f64) * sig.factor + sig.offset;
            decoded.insert(sig.name.clone(), final_val);
        }

        Some(decoded)
    }
}
