use crate::Solution;
use crate::AppArgs;
use anyhow::{anyhow,Result};
use std::io::BufRead;

pub fn solve(args: &AppArgs) -> Result<Solution> {

	let r = args.open_problem_file()?;

	let mut pos = 0;
	let mut depth_or_aim = 0;
	let mut true_depth = 0;

	for line in r.lines() {

		let line = line?;

		let mut tokens = line.split(" ");

		let directive = tokens.next()
		                      .ok_or_else(|| anyhow!("Missing directive"))?
		                      .as_ref();
		let step = tokens.next()
				         .ok_or_else(|| anyhow!("Missing directive argument for `{}`", directive))?
				         .parse::<i64>()?;

		match directive {
			"forward" => {
				pos += step;
				true_depth += depth_or_aim*step;	
			}
			"up" => {
				depth_or_aim -= step;
			}
			"down" => {
				depth_or_aim += step;				
			}
			_ => {
				return Err(anyhow!("Unknown directive: {}", directive));
			}
		}
	}

	let star1 = pos*depth_or_aim;
	let star2 = pos*true_depth;

	return Ok( Solution {
		part_a: star1,
		part_b: Some(star2)
	} ) 
}