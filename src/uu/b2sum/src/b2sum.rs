// This file is part of the uutils coreutils package.
//
// For the full copyright and license information, please view the LICENSE
// file that was distributed with this source code.

// spell-checker:ignore (ToDO) algo

use clap::Command;

use uu_checksum_common::{standalone_checksum_app_with_length, standalone_with_length_main};

use uucore::checksum::{AlgoKind, BlakeLength, parse_blake_length};
use uucore::error::UResult;
use uucore::translate;

#[uucore::main]
pub fn uumain(args: impl uucore::Args) -> UResult<()> {
    let calculate_blake2b_length = |n: Option<usize>| match n {
        None | Some(0) => Ok(None),
        Some(n) => parse_blake_length(AlgoKind::Blake2b, BlakeLength::String(&n.to_string()))
            .map(Some),
    };
    standalone_with_length_main(AlgoKind::Blake2b, uu_app(), args, calculate_blake2b_length)
}

#[inline]
pub fn uu_app() -> Command {
    standalone_checksum_app_with_length(translate!("b2sum-about"), translate!("b2sum-usage"))
        .name("b2sum")
}
