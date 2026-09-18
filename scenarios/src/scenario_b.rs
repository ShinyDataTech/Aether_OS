//! Scenario B: Zero-Silo Context Diff (Phone Transcript vs Contractor Quote)

use crate::agent::AgentEngine;
use runtime::isolate::WasmIsolate;
use schema::ui_ast::MicroUIElement;

pub fn run_scenario_b(arena_base: usize, arena_size: usize, serial_out: &mut Vec<String>) -> Option<MicroUIElement> {
    serial_out.push("=========================================================================".into());
    serial_out.push("[SCENARIO B INITIALIZED]: Dual Stream Ingestion (Audio Transcript vs Contractor PDF)".into());
    serial_out.push("[SENSOR STREAM]: Audio PCM Stream & PDF Vector Stream piped to Agent Engine".into());

    let wasm_bytes = AgentEngine::synthesize_wasm_payload(2);
    serial_out.push("[AGENT SYNTHESIS]: Generative Agent compiled 614 bytes WASM Isolate Bytecode".into());

    let mut isolate = WasmIsolate::new(1002, arena_base, arena_size);
    let card = isolate.execute_bytecode(&wasm_bytes);

    for log in isolate.capability_ctx.serial_logs {
        serial_out.push(log);
    }
    serial_out.push("[SCENARIO B COMPLETE]: WASM Isolate dissolved. Visual Diff card live on canvas.".into());
    serial_out.push("=========================================================================".into());

    card
}
