use std::{cell::RefCell, fmt::Display, path::PathBuf, rc::Rc};

use crate::BoxedError;
use aoc_framework::{traits::*, AocSolution, AocStringIter, AocTask};
use color_eyre::{eyre::eyre, Report};
use itertools::Itertools;
use thiserror::Error;

pub struct Day05;

#[derive(Error, Debug)]
enum DockyardError {
    #[error("Invalid level: {level}")]
    LevelParseError { level: String },
}

enum Crane {
    CrateMover9000,
    CrateMover9001,
}

struct Dockyard {
    stacks: Vec<Vec<char>>,
    crane: Crane,
}

impl Dockyard {
    fn new(size: usize, crane: Crane) -> Self {
        Self {
            stacks: vec![vec![]; size],
            crane,
        }
    }

    fn parse_state(&mut self, level: String) -> Result<(), DockyardError> {
        if !level.contains('[') {
            return Err(DockyardError::LevelParseError { level });
        }

        level
            .chars()
            .chunks(4)
            .into_iter()
            .map(|mut chunk| chunk.nth(1).unwrap_or(' '))
            .enumerate()
            .for_each(|(index, container)| {
                if container != ' ' {
                    self.stacks[index].insert(0, container)
                }
            });

        Ok(())
    }

    fn parse_instruction(&mut self, instruction: String) -> Result<(), Report> {
        let tokens: Result<Vec<usize>, _> = instruction
            .split_whitespace()
            .skip(1)
            .step_by(2)
            .map(|str| str.parse())
            .collect();

        let (count, from, to) = tokens?
            .into_iter()
            .next_tuple()
            .ok_or(eyre!("Invalid operation definiton: {instruction}"))?;

        self.move_container(count, from - 1, to - 1)?;

        Ok(())
    }

    fn move_container(&mut self, count: usize, from: usize, to: usize) -> Result<(), Report> {
        let mut containers = vec![];
        for _ in 0..count {
            let container = self.stacks[from]
                .pop()
                .ok_or(eyre!("Invalid move operation: ({count}, {from}, {to})"))?;

            // CrateMover9000 moves containers 1 at a time, 9001 moves them all at once
            if matches!(self.crane, Crane::CrateMover9000) {
                self.stacks[to].push(container);
            } else {
                containers.insert(0, container);
            }
        }
        self.stacks[to].extend(containers);
        Ok(())
    }
}

impl Display for Dockyard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.stacks
                .iter()
                .flat_map(|stack| stack.last())
                .collect::<String>()
        )
    }
}

impl AocTask for Day05 {
    fn directory(&self) -> PathBuf {
        "tasks/day_05".into()
    }

    fn solution(&self, input: AocStringIter, phase: usize) -> Result<AocSolution, BoxedError> {
        let crane = match phase {
            1 => Crane::CrateMover9000,
            2 => Crane::CrateMover9001,
            _ => todo!(),
        };

        let mut peekable_input = input.peekable();
        let dockyard = Rc::new(RefCell::new(Dockyard::new(
            peekable_input
                .peek()
                .map(|first| first.len() / 4 + 1)
                .ok_or("No input")?,
            crane,
        )));

        peekable_input
            .skip_while(|line| dockyard.borrow_mut().parse_state(line.to_owned()).is_ok())
            .skip(2)
            .try_for_each(|line| dockyard.borrow_mut().parse_instruction(line))?;

        dockyard.borrow().solved()
    }
}
