mod day01;
mod day02;
use std::fmt;
use std::fs::{File};
use std::io::{BufReader};
use anyhow::{anyhow,Result,Context};



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
    help_mode: bool,
    data_file_path: Option<String>,
    day: usize
}

impl AppArgs{
    fn parse<T>( args: T) -> Result<AppArgs> 
      where T: Iterator<Item=String> {
        let mut day = None;
        let mut help_mode = false;
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
                if arg == "-h" || arg == "--help" {
                    help_mode = true;
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



        Ok( AppArgs { help_mode: help_mode, 
                      debug_mode: debug_mode, 
                      data_file_path: data_file_path, 
                      day: day })
    }


    pub fn open_problem_file(&self) -> Result<BufReader<File>>{

        let default_path: String;
        let data_file_path = match self.data_file_path {
            Some(ref path) => { path }
            None => {
                let mut debug_suffix = "";
                if self.debug_mode {
                    debug_suffix = "-d";
                };
                default_path = format!("data/day{:02}{}.txt",self.day,debug_suffix);
                &default_path
            }
        };

        // let data_file_path = "data/day02.txt";
        let f = File::open(data_file_path)
                     .with_context(|| format!("Opening file: {}", data_file_path) )?;
        Ok(BufReader::new(f))
    }
    

}

impl fmt::Display for AppArgs {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "AppArgs:\n")?;
        write!(f, "  Day: {}\n", self.day)?;
        match self.data_file_path {
            Some(ref p) => {
                write!(f, "  Data file path: {}\n", p)?;
            }
            None => {
                write!(f, "  Data file path: DEFAULT")?;                
            }
        }
        write!(f, "  Debug mode: {}\n", self.debug_mode)
    }    
}


fn usage() {
    let app_name:String = std::env::args().next().unwrap_or_else( || String::from(""));

    println!("usage:");
    println!("    {} [-h,--help] [-d,--debug] [-f/--file PATH] DAY", app_name);
    println!("");
    println!("Input file is inferred if not given by --file:");
    println!("  data/dayXX.txt or");
    println!("  data/dayXX-d.txt in debug mode");
}

fn main() -> Result<()> {

    let args = AppArgs::parse( std::env::args() ).map_err( |e| {usage(); e} )?;

    if args.help_mode {
        usage();
        return Ok(());
    }

    let solver = SOLVERS.get(args.day-1)
                        .ok_or_else(|| anyhow!("No solver available for day {}", args.day))?;

    let solution = solver(&args)?;

    println!("{}",solution);

    Ok(())
}
