# Aether OS

> **Non-POSIX AI-Native Microkernel & Ephemeral OS Substrate**

Aether OS is a zero-dependency Proof of Concept (POC) for a next-generation, AI-native Operating System. Completely departing from legacy POSIX paradigms (no `/bin`, no monolithic filesystems, no persistent user apps), Native AI OS implements an **Ephemeral WebAssembly Isolate Architecture** featuring a **Declarative Generative Canvas** rendered directly to a 2D 32bpp linear framebuffer.

---

## 🌟 Key Architecture & Paradigms

```
 ┌────────────────────────────────────────────────────────────────────────┐
 │                    Simulated User Intent / Sensor Stream               │
 └───────────────────────────────────┬────────────────────────────────────┘
                                     │ (Multimodal intent feed)
                                     ▼
 ┌────────────────────────────────────────────────────────────────────────┐
 │  Layer 4: Simulated Multimodal Agent & Scenarios (/scenarios)          │
 │  - Scenario A: Invoice Camera Triage -> Dynamic Bill-Splitter Card     │
 │  - Scenario B: Zero-Silo Context Diff -> Interactive Visual Diff Card  │
 └───────────────────────────────────┬────────────────────────────────────┘
                                     │ (Generates WASM Bytecode & Schema)
                                     ▼
 ┌────────────────────────────────────────────────────────────────────────┐
 │  Layer 3: Declarative Generative Schema (/schema)                      │
 │  - Primitives: Card, Stack, Grid, Slider, Toggle, DiffViewer, Bill     │
 │  - Compact Parser & embedded-graphics Direct Rasterizer                │
 └───────────────────────────────────┬────────────────────────────────────┘
                                     │ (Renders UI / Validates Capability)
                                     ▼
 ┌────────────────────────────────────────────────────────────────────────┐
 │  Layer 2: Embedded WASM Ephemeral Runtime (/runtime)                   │
 │  - no_std Lightweight WASM Interpreter Arena                           │
 │  - Capabilities: sys_draw_card, sys_draw_slider, sys_read_stream      │
 │  - Zero-Persistence: Instant memory wipe & zeroing on isolate exit     │
 └───────────────────────────────────┬────────────────────────────────────┘
                                     │ (Hardware Capability Calls)
                                     ▼
 ┌────────────────────────────────────────────────────────────────────────┐
 │  Layer 1: Bare-Metal Microkernel Harness (/kernel)                     │
 │  - Memory Layout: Frame Allocator, Identity Paging Virtual Memory      │
 │  - Framebuffer Driver (VirtIO-GPU / VBE linear 1024x768 32bpp)         │
 │  - Serial UART 16550 streaming & Capability Space Security Broker      │
 └────────────────────────────────────────────────────────────────────────┘
```

### Core Features

- **No POSIX Lineage**: Eliminates standard filesystem hierarchies, root user accounts, and static executable binaries.
- **Ephemeral WASM Isolates**: Applications are dynamically compiled by background multimodal AI agents into lightweight WASM bytecode, executed inside transient memory arenas, and immediately dissolved.
- **Zero-Persistence Guarantee**: Memory allocations exist exclusively within sandboxed memory pools (`0x0100_0000`). After isolate execution completes, the memory pool undergoes a full zeroing scrub (`memset(0)`).
- **Capability-Based Security Broker**: All hardware interactions (framebuffer rendering, serial stream access, isolate instantiation) are strictly authorized via an untyped Object Capability Space.
- **Generative Declarative Canvas**: UI elements are synthesized on demand using declarative AST schemas (`Card`, `Slider`, `Toggle`, `DiffViewer`, `ItemizedBill`) rendered directly to a 32bpp 1024x768 framebuffer using `embedded-graphics`.
- **Modern Dark-Mode Aesthetics**: Rendered UI features ambient deep slate backgrounds (`#070A10`), grid patterns, cyan accent highlights (`#38BDF8`), and glassmorphic card borders.

---

## 📁 Repository Structure

```
Native_AI_OS/
├── Cargo.toml                  # Workspace definition
├── Makefile                    # Make targets (build, run, clean)
├── run_qemu.ps1                # PowerShell execution harness & snapshot generator
├── implementation_plan.md      # Detailed architectural blueprint & specification
├── kernel/                     # Layer 1: Microkernel Harness
│   └── src/
│       ├── main.rs             # Kernel entry point (_start), bootloader, event loop
│       ├── mm/                 # Memory management (Frame Allocator, Identity Paging)
│       ├── drivers/            # 1024x768x32bpp linear framebuffer & UART 16550 drivers
│       └── cap/                # Capability Broker & Security Space Guard
├── runtime/                    # Layer 2: Ephemeral WASM Runtime
│   └── src/
│       ├── lib.rs              # no_std WASM execution engine
│       ├── host_env.rs         # System call capability host functions
│       └── isolate.rs          # Isolate manager: allocation, execution, memory scrub
├── schema/                     # Layer 3: Declarative Micro-UI Schema
│   └── src/
│       ├── ui_ast.rs           # UI primitive AST definitions
│       ├── cbor_parser.rs      # CBOR binary schema decoder
│       └── rasterizer.rs       # embedded-graphics framebuffer rasterizer engine
└── scenarios/                  # Layer 4: Agent Engine & Test Scenarios
    └── src/
        ├── agent.rs            # Simulated multimodal generative agent
        ├── scenario_a.rs       # Invoice camera triage -> WASM bill-splitter card
        └── scenario_b.rs       # Context diff (Audio vs PDF) -> Interactive diff viewer card
```

---

## 🚀 Building & Running

### Prerequisites

- [Rust](https://www.rust-lang.org/) (2021 edition or newer)
- Target toolchain: `x86_64-unknown-none` (for bare-metal compilation target)
- Python 3.x (optional, for image conversion in `run_qemu.ps1`)

### 1. Build Workspace

```bash
cargo build
```

### 2. Execute Kernel Simulation

Run the kernel simulation directly using Cargo:

```bash
cargo run --package kernel
```

Alternatively, use the PowerShell execution script:

```powershell
./run_qemu.ps1
```

Or via Makefile:

```bash
make run
```

---

## 📊 Verification & Outputs

Upon running the kernel harness, the system generates the following outputs in the workspace root:

1. **`kernel_serial.log`**: Serial output trace capturing kernel boot stages, Capability Broker authorization logs, WASM isolate lifecycle steps, and zero-persistence audit verification.
2. **`canvas_output.bmp`**: 24-bit uncompressed bitmap screenshot of the 1024x768 32bpp linear framebuffer display.
3. **`framebuffer_canvas.png`**: Converted PNG screenshot of the visual canvas.

---

## 🛡️ License

This project is open-source under the MIT License.
