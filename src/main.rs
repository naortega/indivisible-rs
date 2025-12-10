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

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::process;
use structopt::StructOpt;

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
	#[structopt(help = "Max of the prime to generate or number to test for primality")]
	num:usize,
	#[structopt(short, long, name = "SIZE", default_value = "10485760", help = "Set a custom sieve size")]
	sieve:usize,
	//#[structopt(short, long, name = "n", default_value = "1", help = "Number of threads to spawn")]
	//jobs:u64,
}

fn main() {
	let opts = Opt::from_args();
	let mut prime_list = Vec::new();

	if opts.import.is_some() {
		let in_file = File::open(opts.import.unwrap()).unwrap();
		let reader = BufReader::new(in_file);
		for p in reader.lines().into_iter() {
			let prime:u64 = p.unwrap().parse().unwrap();
			if (prime as usize) > opts.num {
				break;
			}

			prime_list.push(prime);
		}
	}

	if opts.num < 2 {
		eprintln!("Invalid value for num: {}", opts.num);
		process::exit(1);
	}

	let mut start:usize = if prime_list.is_empty() {
		2
	} else {
		(*prime_list.last().unwrap() + 1) as usize
	};
	while start < opts.num {
		let end = if start + opts.sieve < opts.num {
			start + opts.sieve
		} else {
			opts.num + 1
		};
		let mut new_primes = worker::work_segment(&prime_list, start, end);

		if opts.verbose {
			for p in &new_primes {
				println!("{}", *p);
			}
		}
		prime_list.append(&mut new_primes);

		start += opts.sieve;
	}

	if opts.test {
		if *prime_list.last().unwrap() == (opts.num as u64) {
			if opts.verbose {
				println!("{} is prime", opts.num);
			}
			process::exit(0);
		} else {
			if opts.verbose {
				println!("{} is composite", opts.num);
			}
			process::exit(1);
		}
	} else if !opts.verbose {
		println!("{}", prime_list.last().unwrap());
	}
}
