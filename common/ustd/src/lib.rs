// Copyright 2017-2018 Parity Technologies (UK) Ltd.
// This file is part of Substrate.

// Substrate is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Substrate is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Substrate.  If not, see <http://www.gnu.org/licenses/>.

// tag::description[]
//! Lowest-abstraction level for the Substrate runtime: just exports useful primitives from std
//! or core/alloc to be used with any code that depends on the runtime.
// end::description[]

#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(not(feature = "std"), feature(core_intrinsics))]
#![cfg_attr(not(feature = "std"), feature(alloc_prelude))]

use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(not(feature = "std"))] {
        pub extern crate alloc;

        pub use alloc::boxed;
        pub use alloc::collections;
        pub use alloc::fmt as alloc_fmt;
        pub use alloc::prelude::v1 as alloc_prelude;
        pub use alloc::rc;
        pub use alloc::string;
        pub use alloc::vec;

        pub use core::borrow;
        pub use core::cell;
        pub use core::clone;
        pub use core::cmp;
        pub use core::convert;
        pub use core::default;
        pub use core::f32;
        pub use core::f64;
        pub use core::fmt as core_fmt;
        pub use core::hash;
        pub use core::intrinsics;
        pub use core::i8;
        pub use core::i16;
        pub use core::i32;
        pub use core::i64;
        pub use core::i128;
        pub use core::isize;
        pub use core::iter;
        pub use core::marker;
        pub use core::mem;
        pub use core::num;
        pub use core::ops;
        pub use core::option;
        pub use core::prelude::v1 as core_prelude;
        pub use core::ptr;
        pub use core::result;
        pub use core::slice;
        pub use core::str;
        pub use core::u8;
        pub use core::u16;
        pub use core::u32;
        pub use core::u64;
        pub use core::u128;
        pub use core::usize;
    } else {
        pub use std::*;
    }
}

pub mod fmt {
    #[cfg(not(feature = "std"))]
    pub use crate::{alloc_fmt::*, core_fmt::*};
    #[cfg(feature = "std")]
    pub use std::fmt::*;
}

#[cfg(not(feature = "std"))]
#[macro_export]
macro_rules! format {
    ($($arg:tt)*) => ($crate::fmt::format(format_args!($($arg)*)))
}

/// Prelude of common useful imports.
///
/// This should include only things which are in the normal std prelude.
pub mod prelude {
    #[cfg(not(feature = "std"))]
    pub use crate::{alloc_prelude::*, core_prelude::*};
    #[cfg(feature = "std")]
    pub use std::prelude::v1::*;

    // Re-export `vec!` and `format!` macro here, but not in `std` mode, since
    // std's prelude already brings `vec!` and `format!` into the scope.
    #[cfg(not(feature = "std"))]
    pub use crate::format;
    #[cfg(not(feature = "std"))]
    pub use crate::vec;
}

#[cfg(test)]
#[cfg(not(feature = "std"))]
mod tests {
    use super::prelude::*;

    #[test]
    fn test_no_std_macro_vec() {
        let mut vec = Vec::new();
        vec.push(1);
        vec.push(2);
        vec.push(3);
        assert_eq!(vec![1, 2, 3], vec);
    }

    #[test]
    fn test_no_std_macro_format() {
        assert_eq!(
            format!("test_macro_format"),
            String::from("test_macro_format")
        );
    }
}
