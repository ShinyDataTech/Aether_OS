//! Untyped Memory Capability Table

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CapRights {
    Read,
    Write,
    ExecuteWasm,
    DrawFramebuffer,
}

pub struct CapabilityEntry {
    pub cap_id: u32,
    pub region_base: usize,
    pub region_size: usize,
    pub rights: CapRights,
}

pub struct CapabilityTable {
    pub entries: Vec<CapabilityEntry>,
}

impl CapabilityTable {
    pub fn new() -> Self {
        Self { entries: Vec::new() }
    }

    pub fn grant(&mut self, cap_id: u32, region_base: usize, region_size: usize, rights: CapRights) {
        self.entries.push(CapabilityEntry {
            cap_id,
            region_base,
            region_size,
            rights,
        });
    }

    pub fn verify(&self, cap_id: u32, rights: CapRights) -> bool {
        self.entries.iter().any(|entry| entry.cap_id == cap_id && entry.rights == rights)
    }
}
