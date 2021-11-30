use crate::Solution;
use crate::AppArgs;
use crate::open_problem_file;
use anyhow::{anyhow,Result};

pub fn solve(_args: &AppArgs) -> Result<Solution> {

	return Err(anyhow!("No solution found"))
	// return Ok(None);

	// open_problem_file(&_args.data_file_path)?;

	// return Some( Solution {
	// 	part_a: 0,
	// 	part_b: None
	// } )
}