//! Object Capability Space & Guard

use crate::mm::cap_table::{CapabilityTable, CapRights};

pub struct CapabilityBroker {
    pub table: CapabilityTable,
}

impl CapabilityBroker {
    pub fn new() -> Self {
        let mut broker = Self { table: CapabilityTable::new() };
        // Bootstrap root WASM sandbox capabilities
        broker.table.grant(1, 0x0100_0000, 0x07FF_FFFF, CapRights::ExecuteWasm);
        broker.table.grant(2, 0xFD00_0000, 0x0100_0000, CapRights::DrawFramebuffer);
        broker
    }

    pub fn authorize_execution(&self, cap_id: u32) -> bool {
        self.table.verify(cap_id, CapRights::ExecuteWasm)
    }

    pub fn authorize_draw(&self, cap_id: u32) -> bool {
        self.table.verify(cap_id, CapRights::DrawFramebuffer)
    }
}
