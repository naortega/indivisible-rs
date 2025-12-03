/*
 * Copyright (C) 2021  Ortega Froysa, Nicolás <nicolas@ortegas.org>
 * Author: Ortega Froysa, Nicolás <nicolas@ortegas.org>
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
use std::rc::Rc;
use structopt::StructOpt;

mod worker;

#[derive(StructOpt)]
#[structopt(about = "A prime number generator and tester.")]
struct Opt {
	#[structopt(short, long, help = "Print all found primes")]
	verbose:bool,
	#[structopt(short, long, name = "FILE", help = "Import prime numbers from FILE")]
	import:Option<PathBuf>,
	#[structopt(short, long, help = "Test if n is prime instead of generation")]
	test:bool,
	#[structopt(help = "Ordinal of the prime to generate or number to test for primality")]
	n:u64,
	#[structopt(short, long, name = "n", help = "Number of threads to spawn")]
	jobs:Option<u64>,
}

fn main() {
	let opts = Opt::from_args();

	let mut primes = Rc::new(RefCell::new(VecDeque::<u64>::new()));

	if opts.import.is_some() {
		let in_file = File::open(opts.import.unwrap()).unwrap();
		let reader = BufReader::new(in_file);
		for p in reader.lines().into_iter() {
			primes.borrow_mut().push_back(p.unwrap().parse().unwrap());
		}
	}

	let jobs = match opts.jobs {
		Some(n) => n,
		None => 1, // TODO: use number of CPUs
	};
}
