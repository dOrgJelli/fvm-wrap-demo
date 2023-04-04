pub mod wrap;
pub use wrap::*;

use fvm_ipld_encoding;

pub fn encode_message(args: ArgsEncodeMessage) -> Vec<u8> {
    fvm_ipld_encoding::to_vec(&args.message).unwrap()
}
