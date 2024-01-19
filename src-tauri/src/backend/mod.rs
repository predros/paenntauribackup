mod common;
mod linear;

use std::collections::HashMap;

pub use linear::linear_analysis;

pub use common::AnalysisError;
use common::*;

use crate::models::{MemberResult, NodeReaction};

pub struct AnalysisResults {
    pub loadcase_results: HashMap<usize, Vec<MemberResult>>,
    pub loadcase_reactions: HashMap<usize, Vec<NodeReaction>>,

    pub combination_results: HashMap<usize, Vec<MemberResult>>,
    pub combination_reactions: HashMap<usize, Vec<NodeReaction>>,
}
