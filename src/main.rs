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
	num:usize,
	#[structopt(short, long, name = "n", default_value = "1", help = "Number of threads to spawn")]
	jobs:u64,
}

fn main() {
	let opts = Opt::from_args();

	let mut prime_list = VecDeque::<u64>::new();
	let mut arr = Vec::new();
	for _ in 0..=opts.num {
		arr.push(false);
	}

	if opts.import.is_some() {
		let in_file = File::open(opts.import.unwrap()).unwrap();
		let reader = BufReader::new(in_file);
		for p in reader.lines().into_iter() {
			prime_list.push_back(p.unwrap().parse().unwrap());
		}
	}

	if opts.num < 2 {
		eprintln!("Invalid value for num: {}", opts.num);
		process::exit(1);
	}
	if opts.num > 2 {
		arr[2] = true;
	}
	if opts.num > 3 {
		arr[3] = true;
	}

	for x in 1..=(f64::sqrt(opts.num as f64) as u64 + 1) {
		for y in 1..=(f64::sqrt(opts.num as f64) as u64 + 1) {
			let n1 = ((4 * x * x) + (y * y)) as usize;
			if n1 <= opts.num && (n1 % 12 == 1 || n1 % 12 == 5) {
				arr[n1] = !arr[n1];
			}

			let n2 = ((3 * x * x) + (y * y)) as usize;
			if n2 <= opts.num && n2 % 12 == 7 {
				arr[n2] = !arr[n2];
			}

			let n3 = ((3 * x * x) - (y * y)) as usize;
			if x > y && n3 <= opts.num && n3 % 12 == 11 {
				arr[n3] = !arr[n3];
			}
		}
	}

	for i in 5..=(f64::sqrt(opts.num as f64) as u64 + 1) as usize {
		if !arr[i as usize] {
			continue;
		}

		let mut j = i * i;
		while j <= opts.num {
			arr[j] = false;
			j += i * i;
		}
	}

	for i in 2..=opts.num {
		if arr[i] {
			println!("{}", i);
			prime_list.push_back(i as u64);
		}
	}
}
