use crate::Solution;
use crate::AppArgs;
use anyhow::{anyhow,Result};
use std::io::BufRead;

pub fn sum( v: &[i64] ) -> i64 {
	let mut s: i64 = 0;
	for val in v {
		s +=  val;
	}
	s
}

pub fn solve(args: &AppArgs) -> Result<Solution> {

	let r = args.open_problem_file()?;

	let mut measurements = Vec::new();
	for line in r.lines() {
		let current = line?.parse::<i64>()?;
		measurements.push(current);
	}

	let mut prev = 0;
	let mut i_count = 0;

	for (k, &current) in measurements.iter().enumerate() {
		if k > 0 {
			if prev < current {
				i_count += 1;
			}			
		}
		prev = current;
	}

	let mut window = [ measurements[0], measurements[1] , measurements[2] ];
	let mut prev = sum(&window);

	let mut i_count2 = 0;

	for (k, &current) in measurements.iter().skip(3).enumerate() {
		window[k % 3] = current;
		let w_sum = sum(&window);
		if w_sum > prev {
			i_count2 += 1;
		}
		prev = w_sum;				
	}

	return Ok( Solution {
		part_a: i_count,
		part_b: Some( i_count2 )
	} ) 
}