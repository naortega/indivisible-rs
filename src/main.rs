/*
 * Copyright (C) 2025  Nicolás Ortega Froysa <nicolas@ortegas.org>
 * Author: Nicolás Ortega Froysa <nicolas@ortegas.org>
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::process;
use structopt::StructOpt;

mod candidate;
use candidate::CandidateGenerator;
mod worker;

#[derive(StructOpt)]
#[structopt(about = "A prime number generator and tester.")]
struct Opt {
	#[structopt(short, long, help = "Print all found primes")]
	verbose:bool,
	#[structopt(short, long, name = "FILE", help = "Import prime numbers from FILE")]
	import:Option<PathBuf>,
	#[structopt(short, long, help = "Test if num is prime instead of generation")]
	test:bool,
	#[structopt(help = "Ordinal of the prime to generate or number to test for primality")]
	num:u64,
	#[structopt(short, long, name = "n", default_value = "1", help = "Number of threads to spawn")]
	jobs:u64,
}

fn main() {
	let opts = Opt::from_args();

	let mut prime_list = VecDeque::<u64>::new();

	if opts.import.is_some() {
		let in_file = File::open(opts.import.unwrap()).unwrap();
		let reader = BufReader::new(in_file);
		for p in reader.lines().into_iter() {
			prime_list.push_back(p.unwrap().parse().unwrap());
		}
	}

	if opts.num == 0 {
		eprintln!("Invalid value for num: {}", opts.num);
		process::exit(1);
	}

	if opts.test && *prime_list.back().unwrap_or(&0) >= opts.num {
		for i in prime_list.iter() {
			if *i == opts.num {
				process::exit(0)
			}
		}
		process::exit(1)
	} else if !opts.test && prime_list.len() >= opts.num as usize {
		let res = *prime_list.get(opts.num as usize).unwrap();
		println!("{}", res);
	} else {
		let mut cand_gen = CandidateGenerator::new();
		if !prime_list.is_empty() {
			cand_gen.calc_base(*prime_list.back().unwrap());
		}

		loop {
			let cand = cand_gen.next();
			if opts.test && cand > opts.num {
				break;
			}

			let mut is_prime = true;
			for p in prime_list.iter() {
				if cand % *p == 0 {
					is_prime = false;
					break;
				}
			}

			if is_prime {
				prime_list.push_back(cand);
				if opts.verbose {
					println!("{}", cand);
				}

				if !opts.test && prime_list.len() == opts.num as usize {
					break;
				}
			}
		}

		if opts.test {
			if *prime_list.back().unwrap() == opts.num {
				process::exit(0)
			} else {
				process::exit(1)
			}
		} else if !opts.verbose {
			let last_prime = *prime_list.back().unwrap();
			println!("{}", last_prime);
		}
	}
}
