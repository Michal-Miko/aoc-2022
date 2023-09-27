use std::path::PathBuf;

use crate::BoxedError;
use aoc_framework::{traits::*, AocSolution, AocStringIter, AocTask};
use color_eyre::{eyre::eyre, Report};
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

pub struct Day04;

lazy_static! {
    static ref LINE_REGEX: Regex = Regex::new(r#"(\d+)-(\d+),(\d+)-(\d+)"#).expect("Invalid regex");
}

#[derive(Debug)]
pub struct Pair {
    left: (i32, i32),
    right: (i32, i32),
}

impl Pair {
    fn full_overlap(&self) -> bool {
        let left_in_right = self.left.0 >= self.right.0 && self.left.1 <= self.right.1;
        let right_in_left = self.right.0 >= self.left.0 && self.right.1 <= self.left.1;
        left_in_right || right_in_left
    }

    fn partial_overlap(&self) -> bool {
        self.left.0 <= self.right.1 && self.left.1 >= self.right.0
    }
}

impl TryFrom<String> for Pair {
    type Error = Report;

    fn try_from(value: String) -> Result<Self, Report> {
        let matches: [&str; 4] = LINE_REGEX
            .captures(&value)
            .ok_or(eyre!("Invalid pair: {value}"))?
            .extract()
            .1;

        Ok(Self {
            left: (matches[0].parse()?, matches[1].parse()?),
            right: (matches[2].parse()?, matches[3].parse()?),
        })
    }
}

impl AocTask for Day04 {
    fn directory(&self) -> PathBuf {
        "tasks/day_04".into()
    }

    fn solution(&self, input: AocStringIter, phase: usize) -> Result<AocSolution, BoxedError> {
        let filter = match phase {
            1 => Pair::full_overlap,
            2 => Pair::partial_overlap,
            _ => todo!(),
        };

        input
            .map(Pair::try_from)
            .filter_ok(filter)
            .process_results(|results| results.count())?
            .solved()
    }
}
