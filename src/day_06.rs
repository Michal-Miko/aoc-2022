use std::path::PathBuf;

use crate::BoxedError;
use aoc_framework::{traits::*, AocSolution, AocStringIter, AocTask};
use color_eyre::eyre::eyre;
use itertools::Itertools;

pub struct Day06;

impl AocTask for Day06 {
    fn directory(&self) -> PathBuf {
        "tasks/day_06".into()
    }

    fn solution(&self, mut input: AocStringIter, phase: usize) -> Result<AocSolution, BoxedError> {
        let marker_window_size = match phase {
            1 => 4,
            2 => 14,
            _ => todo!(),
        };

        let datastream: Vec<char> = input
            .next()
            .ok_or(eyre!("Missing input"))?
            .chars()
            .collect();

        let (marker_pos, _) = datastream
            .windows(marker_window_size)
            .find_position(|window| window.iter().all_unique())
            .ok_or(eyre!("No unique window found"))?;

        (marker_pos + marker_window_size).solved()
    }
}
