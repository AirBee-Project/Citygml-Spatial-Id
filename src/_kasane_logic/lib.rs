//! # Kasane Logic
//!
//! **Kasane Logic** is a Rust library that extends the 4-dimensional spatial information notation
//! defined by IPA (Information-technology Promotion Agency) and enables logical operations on
//! space-time IDs. The calculations are implemented using pure Rust functions, and operate
//! correctly and efficiently in any environment without external dependencies.
//!
//! ## Features
//!
//! - Representation of 4-dimensional (X, Y, F, T) space through [`SpaceTimeId`](id::SpaceTimeId)
//! - Flexible description of range specifications and infinite ranges with [`DimensionRange`](id::DimensionRange)
//! - Set management and duplicate elimination with [`SpaceTimeIdSet`](set::SpaceTimeIdSet)
//! - Support for union (OR), intersection (AND), complement (NOT), and symmetric difference (XOR) operators
//! - Lightweight configuration independent of execution environment
//!
//! ## Usage Examples
//!
//! ```rust
//! use kasane_logic::{id::{DimensionRange, SpaceTimeId}, set::SpaceTimeIdSet};
//!
//! // Create a space-time ID
//! let stid = SpaceTimeId::new(4, DimensionRange::Single(5), DimensionRange::Single(3),
//!                             DimensionRange::Single(10), 60, DimensionRange::Single(100)).unwrap();
//!
//! // Create a set and perform operations
//! let set_a = SpaceTimeIdSet::from(stid);
//! let complement = !&set_a;
//! ```

pub mod function;
pub mod id;
pub mod set;

#[cfg(test)]
pub mod tests;
