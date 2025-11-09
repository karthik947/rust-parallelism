#![allow(unused)]
mod basics;
mod shared_state;

use basics::*;
use shared_state::*;

fn main() {
    // basics::demo1::demo();
    shared_state::barrier_pipeline::demo();
}
