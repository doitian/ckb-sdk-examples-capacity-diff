// Import from `core` instead of from `std` since we are in no-std mode
use core::result::Result;

// Import CKB syscalls and structures
// https://docs.rs/ckb-std/
use ckb_std::{
    ckb_constants::Source,
    ckb_types::prelude::*,
    debug,
    high_level::{load_cell, load_cell_capacity, load_script, load_witness_args, QueryIter},
};

use crate::error::Error;

pub fn main() -> Result<(), Error> {
    let script = load_script()?;

    let inputs_capacity =
        QueryIter::new(load_cell_capacity, Source::GroupInput).sum::<u64>() as i64;
    let outputs_capacity = QueryIter::new(load_cell, Source::Output)
        .filter(|cell| cell.lock().as_slice() == script.as_slice())
        .map(|cell| cell.capacity().unpack())
        .sum::<u64>() as i64;
    debug!("inputs: {}, outputs: {}", inputs_capacity, outputs_capacity);
    let diff_capacity = outputs_capacity - inputs_capacity;

    let expected_witness = diff_capacity.to_le_bytes();
    let witness = load_witness_args(0, Source::GroupInput)
        .unwrap()
        .lock()
        .to_opt()
        .unwrap()
        .raw_data();
    debug!(
        "witness: {:?}, expected: {:?}",
        witness.as_ref(),
        expected_witness
    );
    if witness.as_ref() == expected_witness {
        Ok(())
    } else {
        Err(Error::InvalidWitnessError)
    }
}
