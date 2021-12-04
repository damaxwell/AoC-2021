use crate::Solution;
use crate::AppArgs;
use anyhow::Result;
use std::io::BufRead;


fn filter_search(diag_codes: &[usize], current_bit: usize, find_most_common: bool) -> usize {

	if diag_codes.len() == 1 {
		return diag_codes[0];
	}

	let mask = 1<<(current_bit-1);
	let num_codes = diag_codes.len();

	// The list is sorted. Find the transition index where the current bit 
	// goes from 0 to 1. (This could be faster with a binary search, I suppose...)
	let mut split_pos = num_codes;
	for (k,&c) in diag_codes.iter().enumerate() {
		if (c & mask) > 0 {
			split_pos = k;
			break;
		}
	}

	// Now determine which "half" of the list to look in next.  The first part
	// of the list has the 0's in this bit, and the second part has the 1's.
	// If we are looking for the most common bit value, then we go with
	// whichever slice is longer, with a tie going to the ones. 
	// If we are looking for the least common bit value, then we do exactly the
	// opposite.

	let mut seek_in_ones = num_codes >= 2*split_pos;

	if !find_most_common {
		seek_in_ones = !seek_in_ones;
	}

	if seek_in_ones {
		return filter_search(&diag_codes[split_pos..num_codes], current_bit-1, find_most_common);		
	} else {
		return filter_search(&diag_codes[0..split_pos], current_bit-1, find_most_common);				
	}
}


pub fn solve(args: &AppArgs) -> Result<Solution> {

	let r = args.open_problem_file()?;
	let mut lines = r.lines();

	// Figure out how many bits per binary integer in the codes
	let line1 = lines.next().unwrap()?;
	let bit_count = line1.as_bytes().len();

	// Convert the line we just read and all subsequent to integers.
	let mut diag_codes: Vec<usize> = Vec::new();
	diag_codes.push( usize::from_str_radix(&line1,2)? );
	for line in lines {
		let line = line?;
		diag_codes.push( usize::from_str_radix(&line,2)? );
	}

	// Sort the list!
	diag_codes.sort();
	let num_codes = diag_codes.len();

	// ==============================================================
	// Part 1

	let mut gamma = 0;
	for bit in 0..bit_count {
		let mask = 1<<bit;
		let ones_count = diag_codes.iter().fold(0, |v,x| if x&mask > 0 { v + 1 } else { v } );
		if 2*ones_count > num_codes {
			gamma += mask;
		}
	}
	let epsilon = (!gamma) & ( (1<<bit_count) - 1);

	let star1 = (gamma*epsilon) as i64;

	// ==============================================================
	// Part 2

	let find_most_common = true;
	let ox_code = filter_search(&diag_codes, bit_count, find_most_common);

	let find_most_common = false;
	let co2_code = filter_search(&diag_codes, bit_count, find_most_common);

	let star2 = (ox_code*co2_code) as i64;

	return Ok( Solution {
		part_a: star1,
		part_b: Some(star2)
	} ) 

}
