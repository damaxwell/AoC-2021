use crate::Solution;
use crate::AppArgs;
use anyhow::Result;
use std::io::BufRead;


pub fn solve(args: &AppArgs) -> Result<Solution> {

  let r = args.open_problem_file()?;

  let mut measurements = Vec::new();
  for line in r.lines() {
    measurements.push( line?.parse::<i64>()? );
  }

  let mut prev = measurements[0];
  let mut star1 = 0;

  for &current in measurements.iter().skip(1) {
    if prev < current {
      star1 += 1;
    }     
    prev = current;
  }

  let mut window = [ measurements[0], measurements[1] , measurements[2] ];

  #[inline]
  fn sum( v: &[i64;3] ) -> i64 {
    v[0] + v[1] + v[2]
  }

  let mut prev_sum = sum(&window);

  let mut star2 = 0;

  for (k, &current_value) in measurements.iter().skip(3).enumerate() {
    window[k % 3] = current_value;
    let current_sum = sum(&window);
    if current_sum > prev_sum {
      star2 += 1;
    }
    prev_sum = current_sum;       
  }

  return Ok( Solution {
    part_a: star1,
    part_b: Some( star2 )
  } ) 
}