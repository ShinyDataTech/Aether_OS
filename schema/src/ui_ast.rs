//! Declarative Micro-UI Schema AST for Ephemeral OS

#[derive(Debug, Clone)]
pub enum ContainerKind {
    Card {
        id: u32,
        title: String,
        width: u32,
        height: u32,
        x: i32,
        y: i32,
    },
    Stack {
        direction: StackDirection,
        spacing: u32,
    },
    Grid {
        columns: u32,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StackDirection {
    Vertical,
    Horizontal,
}

#[derive(Debug, Clone)]
pub enum InputControl {
    Slider {
        id: u32,
        card_id: u32,
        label: String,
        min: f32,
        max: f32,
        val: f32,
    },
    Toggle {
        id: u32,
        card_id: u32,
        label: String,
        enabled: bool,
    },
    Button {
        id: u32,
        card_id: u32,
        label: String,
        action: String,
    },
}

#[derive(Debug, Clone)]
pub enum OutputDisplay {
    DataGrid {
        card_id: u32,
        headers: Vec<String>,
        rows: Vec<Vec<String>>,
    },
    DiffViewer {
        card_id: u32,
        title: String,
        left_header: String,
        right_header: String,
        diff_lines: Vec<DiffLine>,
    },
    ItemizedBill {
        card_id: u32,
        items: Vec<BillItem>,
        tax_rate: f32,
        tip_percentage: f32,
        people_count: u32,
    },
}

#[derive(Debug, Clone)]
pub struct BillItem {
    pub name: String,
    pub price: f32,
}

#[derive(Debug, Clone)]
pub struct DiffLine {
    pub kind: DiffLineKind,
    pub text: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiffLineKind {
    Unchanged,
    Added,
    Removed,
}

#[derive(Debug, Clone)]
pub struct MicroUIElement {
    pub container: ContainerKind,
    pub inputs: Vec<InputControl>,
    pub outputs: Vec<OutputDisplay>,
}

impl MicroUIElement {
    pub fn new_card(id: u32, title: &str, x: i32, y: i32, width: u32, height: u32) -> Self {
        Self {
            container: ContainerKind::Card {
                id,
                title: title.to_string(),
                width,
                height,
                x,
                y,
            },
            inputs: Vec::new(),
            outputs: Vec::new(),
        }
    }
}
