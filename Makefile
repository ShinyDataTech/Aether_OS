# Makefile for AI-Native Non-POSIX Microkernel & Ephemeral OS Simulation

CARGO := "C:/Users/wei.liu/scoop/persist/rustup/.cargo/bin/cargo.exe"
QEMU := "C:/Users/wei.liu/scoop/apps/qemu/current/qemu-system-x86_64.exe"

.PHONY: all build run clean

all: build run

build:
	$(CARGO) build

run:
	powershell -ExecutionPolicy Bypass -File ./run_qemu.ps1

clean:
	$(CARGO) clean
