//! Compact Schema Binary & Serialized UI Decoder

use crate::ui_ast::*;

pub struct SchemaDecoder;

impl SchemaDecoder {
    /// Decodes compact binary protocol stream into MicroUIElement cards
    pub fn decode_card_payload(payload: &[u8]) -> Option<MicroUIElement> {
        if payload.is_empty() {
            return None;
        }

        // Tag format: [0x55, 0x49] (UI Tag)
        if payload.len() < 4 || payload[0] != 0x55 || payload[1] != 0x49 {
            return None;
        }

        let card_id = payload[2] as u32;
        let kind = payload[3];

        match kind {
            0x01 => {
                // Bill splitter card
                let mut card = MicroUIElement::new_card(
                    card_id,
                    "INVOICE TRIAGE & BILL SPLITTER",
                    40,
                    60,
                    460,
                    420,
                );
                card.outputs.push(OutputDisplay::ItemizedBill {
                    card_id,
                    items: vec![
                        BillItem { name: "Architectural Audit".into(), price: 120.00 },
                        BillItem { name: "WASM Ephemeral Isolation".into(), price: 85.50 },
                        BillItem { name: "VirtIO Display Pipeline".into(), price: 44.50 },
                    ],
                    tax_rate: 0.10,
                    tip_percentage: 18.0,
                    people_count: 3,
                });
                card.inputs.push(InputControl::Slider {
                    id: 1,
                    card_id,
                    label: "Split Between (People)".into(),
                    min: 1.0,
                    max: 10.0,
                    val: 3.0,
                });
                card.inputs.push(InputControl::Slider {
                    id: 2,
                    card_id,
                    label: "Tip Percentage (%)".into(),
                    min: 0.0,
                    max: 30.0,
                    val: 18.0,
                });
                card.inputs.push(InputControl::Button {
                    id: 3,
                    card_id,
                    label: "Disburse & Terminate".into(),
                    action: "sys_flush".into(),
                });
                Some(card)
            }
            0x02 => {
                // Context diff card
                let mut card = MicroUIElement::new_card(
                    card_id,
                    "ZERO-SILO CONTEXT DIFF VIEWER",
                    520,
                    60,
                    460,
                    420,
                );
                card.outputs.push(OutputDisplay::DiffViewer {
                    card_id,
                    title: "Audio Transcript vs Contractor PDF Quote".into(),
                    left_header: "Phone Transcript".into(),
                    right_header: "Contractor Quote".into(),
                    diff_lines: vec![
                        DiffLine { kind: DiffLineKind::Unchanged, text: "Scope: Bare-metal WASM Sandbox".into() },
                        DiffLine { kind: DiffLineKind::Removed, text: "- Agreed Cost: $1,200 total".into() },
                        DiffLine { kind: DiffLineKind::Added, text: "+ Quoted Cost: $1,850 total (Mismatch!)".into() },
                        DiffLine { kind: DiffLineKind::Unchanged, text: "Delivery: Non-POSIX Baremetal Engine".into() },
                        DiffLine { kind: DiffLineKind::Added, text: "+ Added: $200 expedite surcharge".into() },
                    ],
                });
                card.inputs.push(InputControl::Button {
                    id: 10,
                    card_id,
                    label: "Approve Override".into(),
                    action: "sys_approve".into(),
                });
                card.inputs.push(InputControl::Button {
                    id: 11,
                    card_id,
                    label: "Reject & Dispute".into(),
                    action: "sys_reject".into(),
                });
                Some(card)
            }
            _ => None,
        }
    }
}
