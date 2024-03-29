#![cfg_attr(not(feature = "std"), no_std)]

pub mod builder;
mod error;
mod flags;
mod interpreter;
mod num;
mod opcode;
pub mod script;
pub mod sign;
mod stack;
mod verify;

pub use primitives::*;

pub use self::builder::Builder;
pub use self::error::Error;
pub use self::flags::VerificationFlags;
pub use self::interpreter::{eval_script, verify_script};
pub use self::num::Num;
pub use self::opcode::Opcode;
pub use self::script::{
    is_witness_commitment_script, Script, ScriptAddress, ScriptType, ScriptWitness,
};
pub use self::sign::{SignatureVersion, TransactionInputSigner, UnsignedTransactionInput};
pub use self::stack::Stack;
pub use self::verify::{NoopSignatureChecker, SignatureChecker, TransactionSignatureChecker};
