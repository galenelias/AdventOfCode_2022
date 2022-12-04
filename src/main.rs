// #[macro_use] extern crate lazy_static;
extern crate clap;
extern crate itertools;
extern crate regex;
extern crate num;
extern crate emergence;

use clap::Parser;
use std::io::{self, BufRead};
use std::io::{BufReader};
use std::fs::File;

mod day1;
mod day2;
mod day3;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
   /// Name of the person to greet
   #[arg(short, long)]
   name: String,

   /// Number of times to greet
   day: u32,
}

fn main() {
	let matches = App::new("Advent of Code")
		.author("Galen Elias, gelias@gmail.com")
		.version("0.1.0")
		.about("Advent of code solutions in Rust")
		.arg(
			Arg::with_name("day")
				.short("d")
				.required(true)
				.index(1)
				.help("specifies which day's challenge to run")
				.validator(|str|
					str.parse::<u32>()
						.or(Err("day must be an integer".to_owned()))
						.and_then(|v| match v {
							0..=25 => Ok(()),
							_ => Err("day must be between 1 and 25".to_owned())
						})))
		.arg(
			Arg::with_name("file")
				.short("f")
				.takes_value(true)
				.help("Uses a file instead of reading from standard in"))
		.arg(
			Arg::with_name("stdin")
				.short("i")
				.help("Specifies we should read input from stdin")
		)
		.after_help("Longer explaination to appear after the options when \
					displaying the help information from --help or -h")
		.get_matches();

	let day = matches.value_of("day").unwrap().parse::<u32>().unwrap();

	let input;
	if matches.is_present("file") {
		let f = File::open(matches.value_of("file").unwrap()).unwrap();
		let file = BufReader::new(&f);
		input = file.lines()
			.filter_map(|l| l.ok())
			.collect::<Vec<String>>();
	} else if matches.is_present("stdin") {
		let stdin = io::stdin();
		input = stdin.lock().lines()
			.filter_map(|l| l.ok())
			.collect::<Vec<String>>();
	} else {
		let aoc_fetcher = emergence::AoC::new(2022).expect("Couldn't instantiate AoC object");
		let prob_input = aoc_fetcher.read_or_fetch(day as usize).expect("Couldn't fetch problem input");
		input = prob_input.trim_end_matches('\n').split('\n').map(String::from).collect::<Vec<String>>();
	}

	match day {
		1 => day1::solve(input),
		2 => day2::solve(input),
		3 => day3::solve(input),
		_ => println!("Oops! Day {} isn't implemented yet!", day)
	}
}
