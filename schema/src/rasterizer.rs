//! Direct Framebuffer Rasterizer for Declarative Micro-UIs

use crate::ui_ast::*;

pub struct DisplayBuffer<'a> {
    pub width: u32,
    pub height: u32,
    pub buffer: &'a mut [u32],
}

impl<'a> DisplayBuffer<'a> {
    pub fn new(width: u32, height: u32, buffer: &'a mut [u32]) -> Self {
        Self { width, height, buffer }
    }

    pub fn clear(&mut self, color: u32) {
        self.buffer.fill(color);
    }

    pub fn draw_pixel(&mut self, x: i32, y: i32, color: u32) {
        if x >= 0 && (x as u32) < self.width && y >= 0 && (y as u32) < self.height {
            let idx = (y as u32 * self.width + x as u32) as usize;
            if idx < self.buffer.len() {
                self.buffer[idx] = color;
            }
        }
    }

    pub fn fill_rect(&mut self, x: i32, y: i32, w: u32, h: u32, color: u32) {
        for dy in 0..h {
            let py = y + dy as i32;
            if py < 0 || py as u32 >= self.height { continue; }
            for dx in 0..w {
                let px = x + dx as i32;
                if px >= 0 && (px as u32) < self.width {
                    let idx = (py as u32 * self.width + px as u32) as usize;
                    if idx < self.buffer.len() {
                        self.buffer[idx] = color;
                    }
                }
            }
        }
    }

    pub fn draw_rect_outline(&mut self, x: i32, y: i32, w: u32, h: u32, thickness: u32, color: u32) {
        self.fill_rect(x, y, w, thickness, color);
        self.fill_rect(x, y + h as i32 - thickness as i32, w, thickness, color);
        self.fill_rect(x, y, thickness, h, color);
        self.fill_rect(x + w as i32 - thickness as i32, y, thickness, h, color);
    }

    pub fn draw_text_simple(&mut self, text: &str, start_x: i32, start_y: i32, color: u32) {
        // Draw character blocks/glyphs for text rendering on the raw pixel buffer
        let char_width = 8;
        let char_height = 12;
        let mut x = start_x;
        let mut y = start_y;

        for ch in text.chars() {
            if ch == '\n' {
                x = start_x;
                y += char_height + 4;
                continue;
            }

            // Simple 8x12 character matrix rendering for zero-std fallback
            let glyph_bits = get_ascii_glyph(ch);
            for row in 0..12 {
                let line_bits = glyph_bits[row];
                for col in 0..8 {
                    if (line_bits & (1 << (7 - col))) != 0 {
                        self.draw_pixel(x + col, y + row as i32, color);
                    }
                }
            }
            x += char_width;
        }
    }
}

pub struct SchemaRasterizer;

impl SchemaRasterizer {
    pub fn render(card: &MicroUIElement, canvas: &mut DisplayBuffer) {
        match &card.container {
            ContainerKind::Card { id: _, title, width, height, x, y } => {
                let card_x = *x;
                let card_y = *y;
                let card_w = *width;
                let card_h = *height;

                // Card background - Dark Glassmorphism (#111827)
                canvas.fill_rect(card_x, card_y, card_w, card_h, 0x111827);
                // Glowing border (#3B82F6)
                canvas.draw_rect_outline(card_x, card_y, card_w, card_h, 2, 0x3B82F6);

                // Title bar (#1E293B)
                canvas.fill_rect(card_x + 2, card_y + 2, card_w - 4, 32, 0x1E293B);
                // Title text (#38BDF8)
                canvas.draw_text_simple(title, card_x + 12, card_y + 12, 0x38BDF8);

                let mut current_y = card_y + 44;

                // Render Output Displays
                for output in &card.outputs {
                    match output {
                        OutputDisplay::ItemizedBill { items, tax_rate, tip_percentage, people_count, .. } => {
                            canvas.draw_text_simple("ITEMIZED TRIAGE BREAKDOWN:", card_x + 16, current_y, 0x94A3B8);
                            current_y += 20;

                            let mut subtotal = 0.0f32;
                            for item in items {
                                subtotal += item.price;
                                let line = format_item_line(&item.name, item.price);
                                canvas.draw_text_simple(&line, card_x + 24, current_y, 0xF8FAFC);
                                current_y += 18;
                            }

                            let tax = subtotal * tax_rate;
                            let tip = subtotal * (tip_percentage / 100.0);
                            let total = subtotal + tax + tip;
                            let per_person = total / (*people_count as f32);

                            canvas.fill_rect(card_x + 16, current_y, card_w - 32, 1, 0x334155);
                            current_y += 8;

                            canvas.draw_text_simple(&format_calc_line("Subtotal", subtotal), card_x + 24, current_y, 0xCBD5E1);
                            current_y += 16;
                            canvas.draw_text_simple(&format_calc_line("Tax (10%)", tax), card_x + 24, current_y, 0xCBD5E1);
                            current_y += 16;
                            canvas.draw_text_simple(&format_calc_line("Tip", tip), card_x + 24, current_y, 0xCBD5E1);
                            current_y += 18;

                            // Total Box (#0F172A)
                            canvas.fill_rect(card_x + 16, current_y, card_w - 32, 40, 0x0F172A);
                            canvas.draw_rect_outline(card_x + 16, current_y, card_w - 32, 40, 1, 0x10B981);

                            let total_str = format_calc_line("TOTAL", total);
                            canvas.draw_text_simple(&total_str, card_x + 28, current_y + 8, 0x10B981);
                            let split_str = format_calc_line("PER PERSON", per_person);
                            canvas.draw_text_simple(&split_str, card_x + 28, current_y + 22, 0x34D399);

                            current_y += 50;
                        }
                        OutputDisplay::DiffViewer { title, diff_lines, .. } => {
                            canvas.draw_text_simple(title, card_x + 16, current_y, 0xF59E0B);
                            current_y += 24;

                            for line in diff_lines {
                                let (bg_color, text_color) = match line.kind {
                                    DiffLineKind::Unchanged => (0x1E293B, 0x94A3B8),
                                    DiffLineKind::Added => (0x064E3B, 0x34D399),
                                    DiffLineKind::Removed => (0x7F1D1D, 0xFCA5A5),
                                };

                                canvas.fill_rect(card_x + 16, current_y, card_w - 32, 22, bg_color);
                                canvas.draw_text_simple(&line.text, card_x + 22, current_y + 5, text_color);
                                current_y += 26;
                            }
                            current_y += 10;
                        }
                        _ => {}
                    }
                }

                // Render Input Controls
                for input in &card.inputs {
                    match input {
                        InputControl::Slider { label, min: _, max: _, val, .. } => {
                            canvas.draw_text_simple(label, card_x + 16, current_y, 0xE2E8F0);
                            let val_str = format_int_val(*val as u32);
                            canvas.draw_text_simple(&val_str, card_x + card_w as i32 - 60, current_y, 0x38BDF8);
                            current_y += 16;

                            // Slider track (#334155)
                            canvas.fill_rect(card_x + 16, current_y, card_w - 32, 8, 0x334155);
                            // Fill bar (#0284C7)
                            let fill_w = ((card_w - 32) as f32 * ((*val - 1.0) / 9.0).clamp(0.0, 1.0)) as u32;
                            canvas.fill_rect(card_x + 16, current_y, fill_w, 8, 0x0284C7);
                            // Knob (#38BDF8)
                            canvas.fill_rect(card_x + 16 + fill_w as i32 - 4, current_y - 3, 10, 14, 0x38BDF8);

                            current_y += 24;
                        }
                        InputControl::Button { label, action: _, .. } => {
                            let btn_w = (card_w - 32) / 2 - 4;
                            let btn_x = if label.contains("Reject") || label.contains("Disburse") {
                                card_x + card_w as i32 - btn_w as i32 - 16
                            } else {
                                card_x + 16
                            };

                            let (btn_bg, btn_fg) = if label.contains("Reject") {
                                (0x991B1B, 0xFEE2E2)
                            } else if label.contains("Approve") {
                                (0x065F46, 0xD1FAE5)
                            } else {
                                (0x2563EB, 0xEFF6FF)
                            };

                            canvas.fill_rect(btn_x, current_y, btn_w, 28, btn_bg);
                            canvas.draw_rect_outline(btn_x, current_y, btn_w, 28, 1, 0x60A5FA);
                            canvas.draw_text_simple(label, btn_x + 12, current_y + 8, btn_fg);
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }
}

fn format_item_line(name: &str, price: f32) -> String {
    let price_int = price as u32;
    let price_dec = ((price - price_int as f32) * 100.0) as u32;
    let mut s = String::new();
    s.push_str(name);
    let dots = 30_usize.saturating_sub(name.len());
    for _ in 0..dots { s.push('.'); }
    s.push('$');
    s.push_str(&price_int.to_string());
    s.push('.');
    if price_dec < 10 { s.push('0'); }
    s.push_str(&price_dec.to_string());
    s
}

fn format_calc_line(label: &str, val: f32) -> String {
    let val_int = val as u32;
    let val_dec = ((val - val_int as f32) * 100.0) as u32;
    let mut s = String::new();
    s.push_str(label);
    let dots = 24_usize.saturating_sub(label.len());
    for _ in 0..dots { s.push(' '); }
    s.push('$');
    s.push_str(&val_int.to_string());
    s.push('.');
    if val_dec < 10 { s.push('0'); }
    s.push_str(&val_dec.to_string());
    s
}

fn format_int_val(val: u32) -> String {
    let mut s = String::new();
    s.push_str("[ ");
    s.push_str(&val.to_string());
    s.push_str(" ]");
    s
}

fn get_ascii_glyph(c: char) -> &'static [u8; 12] {
    match c {
        'A' => &[0x1C, 0x36, 0x63, 0x63, 0x7F, 0x63, 0x63, 0x63, 0x00, 0x00, 0x00, 0x00],
        'B' => &[0x7E, 0x63, 0x63, 0x7E, 0x63, 0x63, 0x63, 0x7E, 0x00, 0x00, 0x00, 0x00],
        'C' => &[0x1E, 0x33, 0x60, 0x60, 0x60, 0x33, 0x1E, 0x00, 0x00, 0x00, 0x00, 0x00],
        'D' => &[0x7C, 0x66, 0x63, 0x63, 0x63, 0x66, 0x7C, 0x00, 0x00, 0x00, 0x00, 0x00],
        'E' => &[0x7F, 0x60, 0x60, 0x7C, 0x60, 0x60, 0x7F, 0x00, 0x00, 0x00, 0x00, 0x00],
        'F' => &[0x7F, 0x60, 0x60, 0x7C, 0x60, 0x60, 0x60, 0x00, 0x00, 0x00, 0x00, 0x00],
        'G' => &[0x1E, 0x33, 0x60, 0x67, 0x63, 0x33, 0x1F, 0x00, 0x00, 0x00, 0x00, 0x00],
        'H' => &[0x63, 0x63, 0x63, 0x7F, 0x63, 0x63, 0x63, 0x00, 0x00, 0x00, 0x00, 0x00],
        'I' => &[0x3C, 0x18, 0x18, 0x18, 0x18, 0x18, 0x3C, 0x00, 0x00, 0x00, 0x00, 0x00],
        'J' => &[0x1E, 0x0C, 0x0C, 0x0C, 0x6C, 0x6C, 0x38, 0x00, 0x00, 0x00, 0x00, 0x00],
        'K' => &[0x63, 0x66, 0x6C, 0x78, 0x6C, 0x66, 0x63, 0x00, 0x00, 0x00, 0x00, 0x00],
        'L' => &[0x60, 0x60, 0x60, 0x60, 0x60, 0x60, 0x7F, 0x00, 0x00, 0x00, 0x00, 0x00],
        'M' => &[0x63, 0x77, 0x7F, 0x6B, 0x63, 0x63, 0x63, 0x00, 0x00, 0x00, 0x00, 0x00],
        'N' => &[0x63, 0x73, 0x7B, 0x6F, 0x67, 0x63, 0x63, 0x00, 0x00, 0x00, 0x00, 0x00],
        'O' => &[0x1C, 0x36, 0x63, 0x63, 0x63, 0x36, 0x1C, 0x00, 0x00, 0x00, 0x00, 0x00],
        'P' => &[0x7E, 0x63, 0x63, 0x7E, 0x60, 0x60, 0x60, 0x00, 0x00, 0x00, 0x00, 0x00],
        'Q' => &[0x1C, 0x36, 0x63, 0x63, 0x6B, 0x36, 0x1D, 0x0E, 0x00, 0x00, 0x00, 0x00],
        'R' => &[0x7E, 0x63, 0x63, 0x7E, 0x6C, 0x66, 0x63, 0x00, 0x00, 0x00, 0x00, 0x00],
        'S' => &[0x1E, 0x33, 0x30, 0x1E, 0x03, 0x33, 0x1E, 0x00, 0x00, 0x00, 0x00, 0x00],
        'T' => &[0x7F, 0x18, 0x18, 0x18, 0x18, 0x18, 0x18, 0x00, 0x00, 0x00, 0x00, 0x00],
        'U' => &[0x63, 0x63, 0x63, 0x63, 0x63, 0x36, 0x1C, 0x00, 0x00, 0x00, 0x00, 0x00],
        'V' => &[0x63, 0x63, 0x63, 0x36, 0x36, 0x1C, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00],
        'W' => &[0x63, 0x63, 0x63, 0x6B, 0x7F, 0x77, 0x63, 0x00, 0x00, 0x00, 0x00, 0x00],
        'X' => &[0x63, 0x36, 0x1C, 0x08, 0x1C, 0x36, 0x63, 0x00, 0x00, 0x00, 0x00, 0x00],
        'Y' => &[0x63, 0x63, 0x36, 0x1C, 0x08, 0x08, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00],
        'Z' => &[0x7F, 0x06, 0x0C, 0x18, 0x30, 0x60, 0x7F, 0x00, 0x00, 0x00, 0x00, 0x00],
        'a'..='z' => get_ascii_glyph(c.to_ascii_uppercase()),
        '0' => &[0x1C, 0x36, 0x63, 0x6B, 0x63, 0x36, 0x1C, 0x00, 0x00, 0x00, 0x00, 0x00],
        '1' => &[0x18, 0x38, 0x18, 0x18, 0x18, 0x18, 0x7E, 0x00, 0x00, 0x00, 0x00, 0x00],
        '2' => &[0x1E, 0x33, 0x06, 0x0C, 0x18, 0x30, 0x7F, 0x00, 0x00, 0x00, 0x00, 0x00],
        '3' => &[0x7F, 0x06, 0x0C, 0x1E, 0x03, 0x33, 0x1E, 0x00, 0x00, 0x00, 0x00, 0x00],
        '4' => &[0x0C, 0x1C, 0x3C, 0x6C, 0x7F, 0x0C, 0x0C, 0x00, 0x00, 0x00, 0x00, 0x00],
        '5' => &[0x7F, 0x60, 0x7E, 0x03, 0x03, 0x33, 0x1E, 0x00, 0x00, 0x00, 0x00, 0x00],
        '6' => &[0x1C, 0x30, 0x60, 0x7E, 0x63, 0x33, 0x1E, 0x00, 0x00, 0x00, 0x00, 0x00],
        '7' => &[0x7F, 0x03, 0x06, 0x0C, 0x18, 0x18, 0x18, 0x00, 0x00, 0x00, 0x00, 0x00],
        '8' => &[0x1E, 0x33, 0x33, 0x1E, 0x33, 0x33, 0x1E, 0x00, 0x00, 0x00, 0x00, 0x00],
        '9' => &[0x1E, 0x33, 0x33, 0x1F, 0x03, 0x06, 0x1C, 0x00, 0x00, 0x00, 0x00, 0x00],
        ':' => &[0x00, 0x18, 0x18, 0x00, 0x18, 0x18, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
        '$' => &[0x18, 0x3E, 0x60, 0x3C, 0x06, 0x7C, 0x18, 0x00, 0x00, 0x00, 0x00, 0x00],
        '%' => &[0x63, 0x66, 0x0C, 0x18, 0x30, 0x66, 0x63, 0x00, 0x00, 0x00, 0x00, 0x00],
        '(' => &[0x0C, 0x18, 0x30, 0x30, 0x30, 0x18, 0x0C, 0x00, 0x00, 0x00, 0x00, 0x00],
        ')' => &[0x30, 0x18, 0x0C, 0x0C, 0x0C, 0x18, 0x30, 0x00, 0x00, 0x00, 0x00, 0x00],
        '+' => &[0x00, 0x18, 0x18, 0x7E, 0x18, 0x18, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
        '-' => &[0x00, 0x00, 0x00, 0x7E, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
        '.' => &[0x00, 0x00, 0x00, 0x00, 0x00, 0x18, 0x18, 0x00, 0x00, 0x00, 0x00, 0x00],
        ',' => &[0x00, 0x00, 0x00, 0x00, 0x18, 0x18, 0x30, 0x00, 0x00, 0x00, 0x00, 0x00],
        '[' => &[0x3C, 0x30, 0x30, 0x30, 0x30, 0x30, 0x3C, 0x00, 0x00, 0x00, 0x00, 0x00],
        ']' => &[0x3C, 0x0C, 0x0C, 0x0C, 0x0C, 0x0C, 0x3C, 0x00, 0x00, 0x00, 0x00, 0x00],
        _   => &[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
    }
}
