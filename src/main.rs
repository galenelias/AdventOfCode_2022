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
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
   /// Reads puzzle input from the specified file
   #[arg(short, long)]
   file: Option<String>,

   /// Reads puzzle input from standard in
   #[arg(short, long)]
   stdin: bool,

   /// Specifies which day's challenge to run
   day: u32,
}

fn main() {
	let cli = Cli::parse();

	let input;
	if let Some(file_name) = cli.file {
		let f = File::open(file_name).unwrap();
		let file = BufReader::new(&f);
		input = file.lines()
			.filter_map(|l| l.ok())
			.collect::<Vec<String>>();
	} else if cli.stdin {
		let stdin = io::stdin();
		input = stdin.lock().lines()
			.filter_map(|l| l.ok())
			.collect::<Vec<String>>();
	} else {
		let aoc_fetcher = emergence::AoC::new(2022).expect("Couldn't instantiate AoC object");
		let prob_input = aoc_fetcher.read_or_fetch(cli.day as usize).expect("Couldn't fetch problem input");
		input = prob_input.trim_end_matches('\n').split('\n').map(String::from).collect::<Vec<String>>();
	}

	match cli.day {
		1 => day1::solve(input),
		2 => day2::solve(input),
		3 => day3::solve(input),
		4 => day4::solve(input),
		5 => day5::solve(input),
		6 => day6::solve(input),
		7 => day7::solve(input),
		8 => day8::solve(input),
		9 => day9::solve(input),
		10 => day10::solve(input),
		11 => day11::solve(input),
		12 => day12::solve(input),
		13 => day13::solve(input),
		14 => day14::solve(input),
		15 => day15::solve(input),
		16 => day16::solve(input),
		17 => day17::solve(input),
		18 => day18::solve(input),
		19 => day19::solve(input),
		_ => println!("Oops! Day {} isn't implemented yet!", cli.day)
	}
}
