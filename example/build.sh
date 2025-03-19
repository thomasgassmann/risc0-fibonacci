#!/bin/bash

CC=gcc CC_riscv32im_risc0_zkvm_elf=~/.risc0/cpp/bin/riscv32-unknown-elf-gcc \
    RUSTFLAGS="-C opt-level=0 -C no-prepopulate-passes -C passes=loweratomic -C link-arg=-Ttext=0x00200800 -C panic=abort" \
    RISC0_FEATURE_bigint2=1 \
    cargo +risc0 build --release \
        --target riscv32im-risc0-zkvm-elf --manifest-path Cargo.toml --features risc0
