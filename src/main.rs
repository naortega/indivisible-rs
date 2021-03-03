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

use structopt::StructOpt;
use std::path::PathBuf;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::VecDeque;

#[derive(StructOpt)]
struct Opt {
	#[structopt(short, long)]
	verbose:bool,
	#[structopt(short, long)]
	import:Option<PathBuf>,
	n:usize,
}

fn main() {
	let opts = Opt::from_args();

	// get the first `n` primes
	let n = opts.n;
	let mut primes:VecDeque<u64> = VecDeque::with_capacity(n);
	let mut candidate:u64;

	if opts.import.is_some()
	{
		let in_file = File::open(opts.import.unwrap()).unwrap();
		let reader = BufReader::new(in_file);
		for (index, line) in reader.lines().enumerate()
		{
			let line = line.unwrap();
			let aux:u64 = line.parse().unwrap();
			primes.push_back(aux);
		}

		candidate = *primes.back().unwrap() + 2;
	}
	else
	{
		// first prime
		if opts.verbose
		{
			println!("{}", 2);
		}
		primes.push_back(2);
		candidate = 3;
	}

	while primes.len() < n
	{
		let mut is_prime = true;
		let limit = (candidate as f64).sqrt() as u64;
		for i in primes.iter().take_while(|x| **x <= limit)
		{
			if candidate % *i == 0
			{
				is_prime = false;
				break;
			}
		}

		if is_prime
		{
			if opts.verbose
			{
				println!("{}", candidate);
			}
			primes.push_back(candidate);
		}
		candidate += 2;
	}

	if !opts.verbose
	{
		println!("{}", primes.get(n-1).unwrap());
	}
}
