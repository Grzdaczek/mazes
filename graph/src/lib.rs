mod algorithm;
mod graph;
mod matrix;

pub use crate::algorithm::*;
pub use crate::graph::*;

use std::fmt::Debug;

#[derive(Default)]
pub struct Void;

impl Debug for Void {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("_").finish()
    }
}
