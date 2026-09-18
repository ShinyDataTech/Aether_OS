# Implementation Plan: Non-POSIX AI-Native Microkernel & Ephemeral OS Simulation (POC)

We will build a zero-dependency, revolutionary Proof of Concept (POC) for a next-generation AI-native Operating System running entirely in simulation. This OS departs completely from legacy POSIX paradigms (no `/bin`, no monolithic filesystems, no persistent user apps), adopting an **Ephemeral WebAssembly Isolate Architecture** with a **Declarative Generative Canvas** rendered directly to a 2D framebuffer.

---

## Architectural Breakdown & Core Constraints

```
 ┌────────────────────────────────────────────────────────────────────────┐
 │                    Simulated User Intent / Sensor Stream               │
 └───────────────────────────────────┬────────────────────────────────────┘
                                     │ (Multimodal intent feed)
                                     ▼
 ┌────────────────────────────────────────────────────────────────────────┐
 │  Layer 4: Simulated Multimodal Agent & Scenarios                       │
 │  - Scenario A: Invoice Camera Triage -> Dynamic Bill-Splitter Card     │
 │  - Scenario B: Zero-Silo Context Diff -> Interactive Visual Diff Card  │
 └───────────────────────────────────┬────────────────────────────────────┘
                                     │ (Generates WASM Bytecode & Schema)
                                     ▼
 ┌────────────────────────────────────────────────────────────────────────┐
 │  Layer 3: Declarative Generative Schema (/schema)                      │
 │  - Primitives: Card, Stack, Grid, Slider, Toggle, DiffViewer           │
 │  - Compact Parser & embedded-graphics Direct Rasterizer                │
 └───────────────────────────────────┬────────────────────────────────────┘
                                     │ (Renders UI / Validates Capability)
                                     ▼
 ┌────────────────────────────────────────────────────────────────────────┐
 │  Layer 2: Embedded WASM Ephemeral Runtime (/runtime)                   │
 │  - `no_std` Lightweight WASM Interpreter Arena                         │
 │  - Capabilities: `sys_draw_card`, `sys_draw_slider`, `sys_read_stream` │
 │  - Zero-Persistence: Instant memory wipe & zeroing on exit             │
 └───────────────────────────────────┬────────────────────────────────────┘
                                     │ (Hardware Capability Calls)
                                     ▼
 ┌────────────────────────────────────────────────────────────────────────┐
 │  Layer 1: Bare-Metal Microkernel Harness (/kernel)                     │
 │  - Bootloader handoff & memory layout: 0x01000000 Arena                │
 │  - Framebuffer Driver (VirtIO-GPU / VBE linear 1024x768 32bpp)         │
 │  - Serial UART 16550 streaming & untyped Capability Space              │
 └────────────────────────────────────────────────────────────────────────┘
```

---

## User Review Required

> [!IMPORTANT]
> - **Execution Target**: The primary target is `x86_64-unknown-none` using `qemu-system-x86_64` (with VirtIO display/linear framebuffer & serial input).
> - **Zero Persistence Guarantees**: All WASM heap/stack allocations exist strictly inside a transient memory arena (`0x0100_0000`), which is explicitly scrubbed (`memset(0)`) immediately after each isolate completes.
> - **Pure Framebuffer Canvas**: UI rendering is powered by `embedded-graphics` on top of a 32bpp RGBA linear framebuffer, styled with a modern dark-mode aesthetic (curated HSL colors, card glassmorphism, accent sliders, crisp font rendering).

---

## Proposed Repository Structure & Component Blueprint

```
Native_AI_OS/
├── Cargo.toml                  # Workspace definition
├── kernel/                     # Layer 1: Bare-metal Kernel Harness
│   ├── Cargo.toml
│   └── src/
│       ├── main.rs             # Kernel entry point (_start) & panic handler
│       ├── mm/
│       │   ├── allocator.rs    # Physical frame bump/bitmap allocator
│       │   ├── paging.rs       # Memory table identity mapping
│       │   └── cap_table.rs    # Untyped memory & capability table
│       ├── drivers/
│       │   ├── serial.rs       # UART 16550 driver for logs & intent streams
│       │   └── framebuffer.rs  # 1024x768 linear framebuffer draw target
│       └── cap/
│           └── space.rs        # Object capability space and permission guard
├── runtime/                    # Layer 2: Ephemeral WASM Runtime
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs              # Embedded no_std WASM engine
│       ├── host_env.rs         # Capability host functions (draw, slider, stream)
│       └── isolate.rs          # Isolate manager: alloc, execute, wipe
├── schema/                     # Layer 3: Declarative Micro-UI Schema
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs
│       ├── ui_ast.rs           # Schema structs (Card, Slider, DiffViewer, etc.)
│       ├── cbor_parser.rs      # Binary/compact UI schema decoder
│       └── rasterizer.rs       # embedded-graphics draw target mapper
└── scenarios/                  # Layer 4: Multimodal Agent Harness & Test Scenarios
    ├── Cargo.toml
    └── src/
        ├── lib.rs
        ├── agent.rs            # Generative agent mock engine
        ├── scenario_a.rs       # Scenario A: Invoice triage & WASM bill splitter
        └── scenario_b.rs       # Scenario B: Context diff & visual diff card
```

---

## Proposed Changes per Layer

### Layer 1: Bare-Metal Microkernel Harness (`/kernel`)
- **[NEW] [`kernel/Cargo.toml`](file:///c:/Users/wei.liu/Documents/Native_AI_OS/kernel/Cargo.toml)**: Define `no_std` dependencies (`embedded-graphics-core`, `embedded-graphics`, `spin`, `volatile`).
- **[NEW] [`kernel/src/main.rs`](file:///c:/Users/wei.liu/Documents/Native_AI_OS/kernel/src/main.rs)**: Kernel entry point `_start`, hardware initialization, capability space bootstrap, and event loop.
- **[NEW] [`kernel/src/mm/allocator.rs`](file:///c:/Users/wei.liu/Documents/Native_AI_OS/kernel/src/mm/allocator.rs)**: Frame allocator and physical page manager.
- **[NEW] [`kernel/src/drivers/framebuffer.rs`](file:///c:/Users/wei.liu/Documents/Native_AI_OS/kernel/src/drivers/framebuffer.rs)**: Linear Framebuffer driver wrapping a 1024x768x32bpp video buffer and implementing `embedded-graphics::draw_target::DrawTarget`.
- **[NEW] [`kernel/src/drivers/serial.rs`](file:///c:/Users/wei.liu/Documents/Native_AI_OS/kernel/src/drivers/serial.rs)**: Standard 16550 UART driver for input intent stream reading and debug output.
- **[NEW] [`kernel/src/cap/space.rs`](file:///c:/Users/wei.liu/Documents/Native_AI_OS/kernel/src/cap/space.rs)**: Object Capability Table verifying handle permissions before executing runtime host functions.

### Layer 2: Embedded WASM Ephemeral Runtime (`/runtime`)
- **[NEW] [`runtime/Cargo.toml`](file:///c:/Users/wei.liu/Documents/Native_AI_OS/runtime/Cargo.toml)**: Ephemeral runtime crate configured for `no_std`.
- **[NEW] [`runtime/src/host_env.rs`](file:///c:/Users/wei.liu/Documents/Native_AI_OS/runtime/src/host_env.rs)**: Implement host functions:
  - `sys_draw_card(card_id, x, y, width, height, title_ptr, title_len)`
  - `sys_draw_slider(card_id, slider_id, min, max, value, label_ptr, label_len)`
  - `sys_draw_diff(card_id, old_ptr, old_len, new_ptr, new_len)`
  - `sys_read_stream(stream_id, buf_ptr, buf_len) -> u32`
- **[NEW] [`runtime/src/isolate.rs`](file:///c:/Users/wei.liu/Documents/Native_AI_OS/runtime/src/isolate.rs)**: Isolate lifecycle supervisor that instantiates bytecode, executes functions within bounds, and executes a memory scrub (`memset(0)`) upon completion.

### Layer 3: Declarative Generative Schema (`/schema`)
- **[NEW] [`schema/Cargo.toml`](file:///c:/Users/wei.liu/Documents/Native_AI_OS/schema/Cargo.toml)**: UI Schema crate definition.
- **[NEW] [`schema/src/ui_ast.rs`](file:///c:/Users/wei.liu/Documents/Native_AI_OS/schema/src/ui_ast.rs)**: Declarative micro-UI primitives (`Card`, `Stack`, `Slider`, `Button`, `DiffViewer`).
- **[NEW] [`schema/src/rasterizer.rs`](file:///c:/Users/wei.liu/Documents/Native_AI_OS/schema/src/rasterizer.rs)**: High-level UI engine translating schema specifications into styled `embedded-graphics` visual elements with modern dark mode aesthetic (slate background, cyan accents, glassmorphic card borders, itemized breakdown text).

### Layer 4: Simulated Multimodal Agent & Scenarios (`/scenarios`)
- **[NEW] [`scenarios/src/scenario_a.rs`](file:///c:/Users/wei.liu/Documents/Native_AI_OS/scenarios/src/scenario_a.rs)**: Multimodal camera invoice scenario: camera frame -> WASM bill-splitting isolate -> renders dynamic card with item breakdown & interactive sliders -> recalculates totals -> dissolves.
- **[NEW] [`scenarios/src/scenario_b.rs`](file:///c:/Users/wei.liu/Documents/Native_AI_OS/scenarios/src/scenario_b.rs)**: Zero-silo context diff scenario: phone transcript vs. contractor PDF -> WASM diff card isolate -> renders visual side-by-side diff with approve/reject actions -> dissolves.

### Automation & Scripts
- **[NEW] [`Makefile`](file:///c:/Users/wei.liu/Documents/Native_AI_OS/Makefile)** & **[NEW] [`run_qemu.ps1`](file:///c:/Users/wei.liu/Documents/Native_AI_OS/run_qemu.ps1)**: Automation scripts to build the workspace, compile for `x86_64-unknown-none`, launch `qemu-system-x86_64`, record serial stream output logs, and generate visual framebuffer capture snapshots.

---

## Verification Plan

### Automated Verification
1. **Compilation**: Run `cargo check` and `cargo build --target x86_64-unknown-none` across all workspace crates.
2. **Kernel Simulation Run**: Execute `run_qemu.ps1` to launch QEMU with serial console logging and display framebuffer output.
3. **Isolate Memory Wipe Verification**: Check serial logs to verify that the WASM memory pool is zeroed out before and after isolate execution cycles.

### Manual & Visual Verification
1. **Scenario Execution**: Confirm in serial log output that Scenario A and Scenario B execute sequentially, spawning WASM isolates and rendering micro-UI cards.
2. **Framebuffer Capture Artifact**: Save a visual screen frame of the QEMU display showing the generated card, sliders, and diff viewer, and present it in the `walkthrough.md` artifact.
