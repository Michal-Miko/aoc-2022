use std::path::PathBuf;

use crate::BoxedError;
use aoc_framework::{traits::*, AocSolution, AocStringIter, AocTask};
use itertools::Itertools;

pub struct Day01;

impl AocTask for Day01 {
    fn directory(&self) -> PathBuf {
        "tasks/day_01".into()
    }

    fn solution(&self, input: AocStringIter, phase: usize) -> Result<AocSolution, BoxedError> {
        let elves_to_count = match phase {
            1 => 1,
            2 => 3,
            _ => todo!(),
        };

        input
            .fold(vec![0], |mut acc, cal| {
                match cal.as_str() {
                    "" => {
                        acc.push(0);
                    }
                    x => {
                        if let Some(last) = acc.last_mut() {
                            *last += x.parse().unwrap_or(0)
                        }
                    }
                };
                acc
            })
            .iter()
            .sorted()
            .rev()
            .take(elves_to_count)
            .sum::<i32>()
            .solved()
    }
}
