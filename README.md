# streaming_calc_haskell

[![Build Status](https://travis-ci.com/sycured/streaming_calc_rust.svg?branch=main)](https://travis-ci.com/sycured/streaming_calc_rust)

Bandwidth calculation for streaming server | Rewrite from my original in Python

## Compilation

    cargo build --release

## Usage

### Determine necessary server bandwidth

    ./target/release/streaming_calc_rust bwserver nblisteners bitrate

    ./target/release/streaming_calc_rust bwserver 250 64

**Output**

    Number of listeners : 250
    Bitrate (kb/s) : 64
    Server bandwidth (Mib/s) : 15625


### Determine the amount of data used for the streaming

    /target/release/streaming_calc_rust usagebw nblisteners bitrate nbdays nbhours

    /target/release/streaming_calc_rust usagebw 250 64 1 24

**Output**

    Number of listeners : 250
    Bitrate (kb/s) : 64
    Number of days : 1
    Number of hours by days : 24
    Bandwidth used (GiB) : 164794.92
