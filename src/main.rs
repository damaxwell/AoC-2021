mod day01;
mod day02;
use std::fmt;
use std::fs::{File};
use std::io::{BufReader};
use anyhow::{anyhow,Result};



type Solver = fn(&AppArgs) -> Result<Solution>;
static SOLVERS: &[Solver] = &[ day01::solve, 
                               day02::solve ];


pub struct Solution {
    part_a: i64,
    part_b: Option<i64>
}

impl fmt::Display for Solution {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Solution:\n")?;
        write!(f, "  Part a: {}\n", self.part_a)?;
        match self.part_b {
            Some(b) => write!(f, "  Part b: {}", b),
            _ => write!(f,"  Part b: not completed")
        }
    }
}

pub struct AppArgs {
    debug_mode: bool,
    data_file_path: String,
    day: usize
}

impl AppArgs{
    fn parse<T>( args: T) -> Result<AppArgs> 
      where T: Iterator<Item=String> {
        let mut day = None;
        let mut debug_mode = false;
        let mut data_file_path = None;

        let mut args = args.into_iter();
        // Skip the executable name.
        args.next();

        while let Some(arg) = args.next() {
            if arg.starts_with('-') {
                if arg == "-d" || arg == "--debug" {
                    debug_mode = true;
                    continue;
                }
                if arg == "-f" || arg == "--file" {
                    data_file_path = match args.next() {
                        Some(f) => Some(f),
                        _ => return Err(anyhow!("Missing file path argument for -f/--file"))
                    };
                    continue;
                }
                return Err(anyhow!("Unknown argument {}",arg))
            }

            // This had better be the last argument
            if let Some(arg2) = args.next() {
                return Err(anyhow!("Unknown extra argument {}", arg2));
            }

            day = match arg.parse::<usize>() {
                Ok(d) => Some(d),
                _ => return Err(anyhow!("Unable to parse day argument: {}", arg))
            }

        }

       let day = match day {
            Some(d) => d,
            _ => return Err(anyhow!("Missing day argument"))
        };


        let debug_suffix = if debug_mode { "-d"} else { "" };
        let data_file_path = data_file_path.unwrap_or_else( || format!("data/day{:02}{}.txt",day,debug_suffix) );

        Ok( AppArgs { debug_mode: debug_mode, data_file_path: data_file_path, day: day })
    }


    pub fn open_problem_file(&self) -> Result<BufReader<File>>{
        let f = File::open(&self.data_file_path)?;
        Ok(BufReader::new(f))
    }
    

}

impl fmt::Display for AppArgs {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "AppArgs:\n")?;
        write!(f, "  Day: {}\n", self.day)?;
        write!(f, "  Data file path: {}\n", self.data_file_path)?;
        write!(f, "  Debug mode: {}\n", self.debug_mode)
    }    
}


fn usage() {
    let app_name:String = std::env::args().next().unwrap_or_else( || String::from(""));

    println!("usage:");
    println!("    {} [-d,--debug] [-f/--file PATH] DAY", app_name);
}

fn main() -> Result<()> {

    let args = AppArgs::parse( std::env::args() ).map_err( |e| {usage(); e} )?;

    let solver = SOLVERS.get(args.day-1)
                        .ok_or_else(|| anyhow!("No solver available for day {}", args.day))?;

    let solution = solver(&args)?;

    println!("{}",solution);

    Ok(())
}
