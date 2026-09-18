//! Bare-Metal Microkernel Entrypoint & Ephemeral OS Simulator Harness

pub mod mm;
pub mod drivers;
pub mod cap;

use mm::allocator::FrameAllocator;
use mm::paging::PageTableManager;
use cap::space::CapabilityBroker;
use schema::rasterizer::{DisplayBuffer, SchemaRasterizer};
use scenarios::scenario_a::run_scenario_a;
use scenarios::scenario_b::run_scenario_b;
use std::fs::File;
use std::io::Write;

fn main() {
    let mut serial_out: Vec<String> = Vec::new();

    serial_out.push("=========================================================================".into());
    serial_out.push("  BOOTING AI-NATIVE NON-POSIX OS MICROKERNEL (EPHEMERAL WASM SUBSTRATE)  ".into());
    serial_out.push("=========================================================================".into());
    serial_out.push("[BOOT]: Architecture: x86_64-unknown-none / QEMU Simulation Substrate".into());
    serial_out.push("[BOOT]: Lineage Audit: POSIX Trees (/bin, /usr) = NONE. Monolithic Users = NONE.".into());
    serial_out.push("[BOOT]: Physical Frame Allocator initialized @ 0x0010_0000 (1024 frames)".into());

    let _allocator = FrameAllocator::new(0x0010_0000, 1024);
    let mut paging = PageTableManager::new(0x0009_0000);
    paging.map_identity(0x0010_0000, 0x0010_0000, 0x00EE_0000);
    serial_out.push("[BOOT]: Identity Paging Virtual Memory mapping enabled.".into());

    let broker = CapabilityBroker::new();
    serial_out.push("[CAPABILITY BROKER]: Bootstrap Root Capability Space active.".into());
    if broker.authorize_execution(1) {
        serial_out.push("[CAPABILITY BROKER]: Ephemeral WASM Arena @ 0x0100_0000 (CAP #1: GRANTED)".into());
    }
    if broker.authorize_draw(2) {
        serial_out.push("[CAPABILITY BROKER]: VirtIO Linear Framebuffer @ 0xFD00_0000 (CAP #2: GRANTED)".into());
    }

    // Allocate 1024x768 32bpp linear video buffer (3,145,728 bytes)
    let width = 1024u32;
    let height = 768u32;
    let mut raw_buffer = vec![0u32; (width * height) as usize];
    let mut canvas = DisplayBuffer::new(width, height, &mut raw_buffer);

    // Clear background to Ambient Deep Slate (#070A10)
    canvas.clear(0x070A10);

    // Draw ambient background grid lines (#111927)
    for x in (0..width as i32).step_by(40) {
        canvas.fill_rect(x, 0, 1, height, 0x111927);
    }
    for y in (0..height as i32).step_by(40) {
        canvas.fill_rect(0, y, width, 1, 0x111927);
    }

    // Render Canvas Header Banner
    canvas.fill_rect(0, 0, width, 40, 0x0F172A);
    canvas.fill_rect(0, 39, width, 2, 0x38BDF8);
    canvas.draw_text_simple("AI-NATIVE OS [SIMULATED CANVAS] -- ZERO-PERSISTENCE WASM ISOLATES ACTIVE", 20, 14, 0x38BDF8);

    // Execute Scenario A: Multimodal Invoice Triage -> Ephemeral WASM Bill Splitter Card
    let scenario_a_card = run_scenario_a(0x0100_0000, 0x0100_0000, &mut serial_out);
    if let Some(card_a) = scenario_a_card {
        SchemaRasterizer::render(&card_a, &mut canvas);
        serial_out.push("[RASTERIZER]: Rendered Scenario A Card (Invoice Triage) to Framebuffer.".into());
    }

    // Execute Scenario B: Zero-Silo Context Diff -> Ephemeral WASM Visual Diff Card
    let scenario_b_card = run_scenario_b(0x0200_0000, 0x0100_0000, &mut serial_out);
    if let Some(card_b) = scenario_b_card {
        SchemaRasterizer::render(&card_b, &mut canvas);
        serial_out.push("[RASTERIZER]: Rendered Scenario B Card (Zero-Silo Context Diff) to Framebuffer.".into());
    }

    serial_out.push("=========================================================================".into());
    serial_out.push("[KERNEL EVENT LOOP]: Ephemeral OS execution completed successfully.".into());
    serial_out.push("=========================================================================".into());

    // Write Serial Log output to kernel_serial.log
    if let Ok(mut f) = File::create("kernel_serial.log") {
        for line in &serial_out {
            println!("{}", line);
            let _ = writeln!(f, "{}", line);
        }
    }

    // Write Canvas Output to standard uncompressed 24-bit BMP image file (canvas_output.bmp)
    if let Ok(mut f) = File::create("canvas_output.bmp") {
        let file_header_size = 14u32;
        let info_header_size = 40u32;
        let data_offset = file_header_size + info_header_size;
        let data_size = width * height * 3;
        let file_size = data_offset + data_size;

        let mut header = Vec::with_capacity(54);
        // BITMAPFILEHEADER
        header.extend_from_slice(b"BM");
        header.extend_from_slice(&file_size.to_le_bytes());
        header.extend_from_slice(&0u32.to_le_bytes()); // Reserved
        header.extend_from_slice(&data_offset.to_le_bytes());

        // BITMAPINFOHEADER
        header.extend_from_slice(&info_header_size.to_le_bytes());
        header.extend_from_slice(&(width as i32).to_le_bytes());
        header.extend_from_slice(&(-(height as i32)).to_le_bytes()); // Top-down
        header.extend_from_slice(&1u16.to_le_bytes()); // Planes
        header.extend_from_slice(&24u16.to_le_bytes()); // 24bpp
        header.extend_from_slice(&0u32.to_le_bytes()); // BI_RGB
        header.extend_from_slice(&data_size.to_le_bytes());
        header.extend_from_slice(&2835u32.to_le_bytes()); // 72 DPI X
        header.extend_from_slice(&2835u32.to_le_bytes()); // 72 DPI Y
        header.extend_from_slice(&0u32.to_le_bytes());
        header.extend_from_slice(&0u32.to_le_bytes());

        let _ = f.write_all(&header);

        let mut pixel_bytes = Vec::with_capacity(data_size as usize);
        for pixel in canvas.buffer.iter() {
            let r = ((*pixel >> 16) & 0xFF) as u8;
            let g = ((*pixel >> 8) & 0xFF) as u8;
            let b = (*pixel & 0xFF) as u8;
            pixel_bytes.push(b);
            pixel_bytes.push(g);
            pixel_bytes.push(r);
        }
        let _ = f.write_all(&pixel_bytes);
    }
}
