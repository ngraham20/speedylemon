# speedylemon
Designed off of https://github.com/killer415tv/gw2_speedometer as a base

The original speedometer is purely designed for Windows (with good reason, as Windows is the target platform for Guild Wars 2). However, as Proton works well and even seems to outperform Windows, making the speedometer cross-platform may be desired.

The codebase is also fairly complex, and I hope to simplify or at least re-organize it, as well as re-write parts of it in Rust.

Rust crate as an example: https://crates.io/crates/mumblelink_reader

Memory Mapping:
https://www.youtube.com/watch?v=8hVLcyBkSXY&ab_channel=ChrisKanich

## First, Though
First, instead of re-writing the program or even beginning to add a Rust backend, I'm just going to
1. Read the data from the GW2 Mumble API
2. Read the checkpoint file into memory
3. Have Livesplit auto-split upon crossing a checkpoint

## Keeping Track of Checkpoints
- Checkpoints contained in a vec
- When crossing the next checkpoint, pop it and split

## Mumble Interface
- Mumble Interface for now will only track the racer position