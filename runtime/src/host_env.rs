//! Capability Host Functions exposed to Ephemeral WASM Isolates

use schema::ui_ast::MicroUIElement;
use schema::cbor_parser::SchemaDecoder;

pub struct CapabilityContext {
    pub active_card: Option<MicroUIElement>,
    pub serial_logs: Vec<String>,
}

impl CapabilityContext {
    pub fn new() -> Self {
        Self {
            active_card: None,
            serial_logs: Vec::new(),
        }
    }

    pub fn sys_log(&mut self, msg: &str) {
        let entry = format!("[WASM CAPABILITY LOG]: {}", msg);
        self.serial_logs.push(entry);
    }

    pub fn sys_emit_card(&mut self, payload: &[u8]) -> bool {
        if let Some(card) = SchemaDecoder::decode_card_payload(payload) {
            self.active_card = Some(card);
            self.sys_log("Successfully emitted declarative canvas card.");
            true
        } else {
            self.sys_log("Failed to decode card payload capability.");
            false
        }
    }
}
