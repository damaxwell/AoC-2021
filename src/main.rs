mod day01;
use std::fmt;

fn usage_exit(e: Option<&'static str> ) -> ! {
    if let Some(s) = e {
        println!("{}",s);
    }
    println!("USAGE:");
    println!("    AoC-2021 [-d] [day]");
    std::process::exit(1)
}

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

impl fmt::Display for AppArgs {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "AppArgs:\n")?;
        write!(f, "  Day: {}\n", self.day)?;
        write!(f, "  Data file path: {}\n", self.data_file_path)?;
        write!(f, "  Debug mode: {}\n", self.debug_mode)
    }    
}

impl AppArgs{
    fn parse<T>( args: T) -> Result<AppArgs, &'static str> 
      where T: Iterator<Item=String> {
        let mut day = None;
        let mut debug_mode = false;
        let mut data_file_path = None;

        let mut args = args.into_iter();
        // Skip the executable name.
        args.next();

        while let Some(arg) = args.next() {
            if arg == "-d" || arg == "--debug" {
                debug_mode = true;
                continue;
            }
            if arg == "-f" || arg == "--file" {
                data_file_path = match args.next() {
                    Some(f) => Some(f),
                    _ => return Err("Missing file path argument for -f/--file")
                }
            }

            // This had better be the last argument
            if let Some(_) = args.next() {
                return Err("Unknown extra arguments");
            }

            day = match arg.parse::<usize>() {
                Ok(d) => Some(d),
                _ => return Err("Unable to parse day argument")
            }
        }

        let day = match day {
            Some(d) => d,
            _ => return Err("Missing day argument")
        };

        let debug_suffix = if debug_mode { "-d"} else { "" };
        let data_file_path = data_file_path.unwrap_or_else( || format!("data/day{}{}.txt",day,debug_suffix) );

        Ok( AppArgs { debug_mode: debug_mode, data_file_path: data_file_path, day: day })
    }
}


fn main() {

    let args = AppArgs::parse( std::env::args() ).unwrap_or_else( |e| usage_exit(Some(e)) );

    println!("Arguments:\n {}",args);

    let solution = match args.day {
        1 => day01::solve(&args),
        _ => {
            println!("No solver available for day {}", args.day);
            std::process::exit(1);
        }
    };


    println!("{}", solution);
}
