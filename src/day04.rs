use crate::Solution;
use crate::AppArgs;
use anyhow::{anyhow,Result};
use std::io::BufRead;

const BOARD_SIZE:usize = 5;

#[derive(Debug)]
struct BingoBoard {
  values: [ [i64; BOARD_SIZE]; BOARD_SIZE],
  marks: [ [bool; BOARD_SIZE]; BOARD_SIZE]
}

impl BingoBoard {
  fn new<T>( lines: &mut T ) -> Result<BingoBoard>
  where T: Iterator<Item=std::io::Result<String>>
  {
    let mut values = [ [ 0; BOARD_SIZE] ; BOARD_SIZE];
    for i in 0..BOARD_SIZE {
      let line = lines.next().ok_or_else(|| anyhow!("unexpected end of file"))??;
      let mut row = line.split_whitespace();
      for j in 0..BOARD_SIZE {
        values[i][j] = row.next()
                          .ok_or_else(|| anyhow!("unexpected end of line"))?
                          .parse::<i64>()?;
      }
    }
    Ok( BingoBoard {
      values: values,
      marks: [ [false; 5]; 5 ]
    } )
  }

  fn update(&mut self, call: i64) {
    for i in 0..BOARD_SIZE {
      for j in 0..BOARD_SIZE {
        if self.values[i][j] == call { 
                    self.marks[i][j] = true; }
      }
    }
  }

  fn is_a_winner(&self) -> bool {
    'outer_rows: for i in 0..BOARD_SIZE {
      for j in 0..BOARD_SIZE {
        if self.marks[i][j] == false {
          continue 'outer_rows;
        }
      }
      return true;
    }

    'outer_cols: for j in 0..BOARD_SIZE {
      for i in 0..BOARD_SIZE {
        if self.marks[i][j] == false {
          continue 'outer_cols;
        }
      }
      return true;
    }

    return false;
  }

  fn score(&self) -> i64 {
    let mut score = 0;
    for i in 0..BOARD_SIZE {
      for j in 0..BOARD_SIZE {
        if !self.marks[i][j] {
          score += self.values[i][j];
        }
      }
    }
    score
  }
}


pub fn solve(args: &AppArgs) -> Result<Solution> {

  let r = args.open_problem_file()?;

  let mut lines = r.lines();
  let calls: Vec<i64>= lines.next()
                            .ok_or_else(|| anyhow!("unexpected empty file"))??
                .split(",")
                .map( |x| x.parse::<i64>())
                .collect::<Result<Vec<_>,_>>()?;

  let mut boards = Vec::new();
  let mut in_play = Vec::new();
  while lines.next().is_some() {
    boards.push(BingoBoard::new(& mut lines)?);
    in_play.push(true);   
  }

  let mut star1 = 0;
  let mut star2 = 0;
  let mut boards_in_play = boards.len();
  let board_count = boards.len();

  'outer: for &c in &calls {
    for (k,board) in (&mut boards).iter_mut().enumerate() {
      if !in_play[k] {
        continue;
      }
      board.update(c);
      if board.is_a_winner() {
        in_play[k] = false;

        if boards_in_play == board_count {
          star1 = board.score()*c;
        }

        boards_in_play -= 1;
        if boards_in_play == 0 {
          star2 = board.score()*c;
          break 'outer;
        }
      }
    }
  }

  return Ok( 
        Solution {
         part_a: star1,
         part_b: Some(star2)
        }
  )
}
