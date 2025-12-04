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

use std::cell::RefCell;
use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::process;
use std::rc::Rc;
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
	#[structopt(short, long, name = "n", help = "Number of threads to spawn")]
	jobs:Option<u64>,
}

fn main() {
	let opts = Opt::from_args();

	let primes_list = Rc::new(RefCell::new(VecDeque::<u64>::new()));

	if opts.import.is_some() {
		let in_file = File::open(opts.import.unwrap()).unwrap();
		let reader = BufReader::new(in_file);
		for p in reader.lines().into_iter() {
			primes_list.borrow_mut().push_back(p.unwrap().parse().unwrap());
		}
	}

	let jobs = match opts.jobs {
		Some(n) => n,
		None => 1, // TODO: use number of CPUs
	};

	if opts.num == 0 {
		eprintln!("Invalid value for num: {}", opts.num);
		process::exit(1);
	}

	if opts.test && *primes_list.borrow().back().unwrap_or(&0) >= opts.num {
		for i in primes_list.borrow().iter() {
			if *i == opts.num {
				process::exit(0)
			}
		}
		process::exit(1)
	} else if !opts.test && primes_list.borrow().len() >= opts.num as usize {
		let res = *primes_list.borrow().get(opts.num as usize).unwrap();
		println!("{}", res);
	} else {
		let mut cand_gen = CandidateGenerator::new();

		while primes_list.borrow().len() < opts.num as usize {
			let cand = cand_gen.next();
			let mut is_prime = true;

			for i in primes_list.borrow().iter() {
				if cand % *i == 0 {
					is_prime = false;
					break;
				}
			}

			if is_prime {
				primes_list.borrow_mut().push_back(cand);
				if opts.verbose {
					println!("{}", cand);
				}
			}
		}

		if !opts.verbose {
			let last_prime = *primes_list.borrow().back().unwrap();
			println!("{}", last_prime);
		}
	}
}
