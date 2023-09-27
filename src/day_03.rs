use std::path::PathBuf;

use crate::BoxedError;
use aoc_framework::{traits::*, AocSolution, AocStringIter, AocTask};
use itertools::Itertools;

pub struct Day03;

trait Priority {
    fn priority(&self) -> i32;
}

impl Priority for char {
    fn priority(&self) -> i32 {
        match self {
            'a'..='z' => 1 + *self as i32 - 'a' as i32,
            'A'..='Z' => 27 + *self as i32 - 'A' as i32,
            _ => 0,
        }
    }
}

struct Bag {
    compartments: Vec<Vec<char>>,
}

impl Bag {
    fn common_item_priority(&self) -> i32 {
        let compartments = self.compartments.len();
        self.compartments
            .iter()
            .flat_map(|comp| comp.iter().unique())
            .counts()
            .iter()
            .find_map(|(item, in_compartments)| {
                if in_compartments == &compartments {
                    Some(**item)
                } else {
                    None
                }
            })
            .map(|item| item.priority())
            .unwrap_or(0)
    }
}

impl From<String> for Bag {
    fn from(value: String) -> Self {
        let items_per_compartment = value.len() / 2;
        Self {
            compartments: value
                .chars()
                .chunks(items_per_compartment)
                .into_iter()
                .map(|comp| comp.collect())
                .collect(),
        }
    }
}

impl From<[String; 3]> for Bag {
    fn from(value: [String; 3]) -> Self {
        Self {
            compartments: value.iter().map(|str| str.chars().collect()).collect(),
        }
    }
}

impl AocTask for Day03 {
    fn directory(&self) -> PathBuf {
        "tasks/day_03".into()
    }

    fn solution(&self, input: AocStringIter, phase: usize) -> Result<AocSolution, BoxedError> {
        match phase {
            1 => input
                .map(|str| Bag::from(str).common_item_priority())
                .sum::<i32>(),
            2 => input
                .array_chunks()
                .map(|arr| Bag::from(arr).common_item_priority())
                .sum(),
            _ => todo!(),
        }
        .solved()
    }
}
