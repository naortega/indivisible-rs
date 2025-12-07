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
	num:u64,
	#[structopt(short, long, name = "n", default_value = "1", help = "Number of threads to spawn")]
	jobs:u64,
}

const SEGMENT_SIZE:usize = 0x40000000;

fn main() {
	let opts = Opt::from_args();

	let mut prime_list = Vec::<usize>::new();
	let mut arr = vec![false; SEGMENT_SIZE];

	if opts.import.is_some() {
		let in_file = File::open(opts.import.unwrap()).unwrap();
		let reader = BufReader::new(in_file);
		for p in reader.lines().into_iter() {
			prime_list.push(p.unwrap().parse().unwrap());
		}
	}

	let limit = (opts.num * 5) as usize;
	if opts.num < 2 || limit > SEGMENT_SIZE {
		eprintln!("Invalid value for num: {}", opts.num);
		process::exit(1);
	}
	if opts.num > 2 {
		arr[2] = true;
	}
	if opts.num > 3 {
		arr[3] = true;
	}

	let sqrt_of_num = f64::sqrt(opts.num as f64) as usize;
	for x in 1..=sqrt_of_num {
		let xx4 = 4 * x * x;
		let xx3 = 3 * x * x;
		for y in 1..=sqrt_of_num {
			let yy = y * y;

			let n1 = xx4 + yy;
			if n1 % 12 == 1 || n1 % 12 == 5 {
				arr[n1] = !arr[n1];
			}

			let n2 = xx3 + yy;
			if n2 % 12 == 7 {
				arr[n2] = !arr[n2];
			}

			if x > y {
				let n3 = xx3 - yy;
				if n3 % 12 == 11 {
					arr[n3] = !arr[n3];
				}
			}
		}
	}

	for i in 5..=sqrt_of_num {
		if !arr[i] {
			continue;
		}

		let mut j = i * i;
		while j <= (opts.num as usize) {
			arr[j] = false;
			j += i * i;
		}
	}

	for i in 2..=(opts.num as usize) {
		if arr[i] {
			if opts.verbose && !opts.test {
				println!("{}", i);
			}
			prime_list.push(i);
		}
	}

	if !opts.verbose && !opts.test {
		println!("{}", prime_list.last().unwrap());
	} else if opts.test {
		if *prime_list.last().unwrap() == (opts.num as usize) {
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
	}
}
