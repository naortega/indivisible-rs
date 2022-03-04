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

mod test;

#[derive(StructOpt)]
#[structopt(about = "A prime number generator and tester.")]
struct Opt
{
	#[structopt(short, long, help = "Print all found primes")]
	verbose:bool,
	#[structopt(short, long, name = "FILE", help = "Import prime numbers from FILE")]
	import:Option<PathBuf>,
	#[structopt(short, long, help = "Test if n is prime instead of generation")]
	test:bool,
	#[structopt(help = "Ordinal of the prime to generate")]
	n:u64,
}

fn main()
{
	let opts = Opt::from_args();

	// get the first `n` primes
	let n = opts.n;
	let mut primes:VecDeque<u64> = VecDeque::new();

	if opts.import.is_some()
	{
		let in_file = File::open(opts.import.unwrap()).unwrap();
		let reader = BufReader::new(in_file);
		for line in reader.lines()
		{
			let line = line.unwrap();
			let aux:u64 = line.parse().unwrap();
			primes.push_back(aux);
		}
	}

	if opts.test
	{
		let mut res:bool;
		// if no primes were imported, test from beginning (2)
		if primes.len() == 0
		{
			res = test::is_prime(n);
		}
		// `n` should be in `primes` if the last prime is larger than `n`
		else if primes.back().unwrap() >= &(n)
		{
			res = primes.contains(&(n));
		}
		// we can memory test `n` if the last prime is >= sqrt(n)
		else if primes.back().unwrap() >= &((n as f64).sqrt() as u64)
		{
			res = test::is_prime_mem(n, &primes)
		}
		/*
		 * if we have less primes than sqrt(n) then we can test all those
		 * prior to the last prime in the list, and then begin testing odd
		 * numbers.
		 */
		else
		{
			res = test::is_prime_mem(n, &primes);
			if res
			{
				res = test::is_prime_f(n, primes.back().unwrap() + 2);
			}
		}

		if res
		{
			if opts.verbose
			{
				println!("true");
			}
			std::process::exit(0);
		}
		else
		{
			if opts.verbose
			{
				println!("false");
			}
			std::process::exit(1);
		}
	}
	else
	{
		// if `primes` already contains the nth prime, print it
		if primes.len() >= n as usize
		{
			println!("{}", primes.get((n as usize) - 1).unwrap());
		}
		else
		{
			let mut candidate:u64;

			if primes.len() == 0
			{
				// assume 2 as a prime
				primes.push_back(2);
				if opts.verbose
				{
					println!("{}", 2);
				}
				candidate = 3;
			}
			else if primes.len() == 1
			{
				candidate = 3;
			}
			else
			{
				candidate = *primes.back().unwrap() + 2;
			}

			while primes.len() < n as usize
			{
				if test::is_prime_mem(candidate, &primes)
				{
					primes.push_back(candidate);
					if opts.verbose
					{
						println!("{}", candidate);
					}
				}

				candidate += 2;
			}

			if !opts.verbose
			{
				println!("{}", primes.get((n as usize) - 1).unwrap());
			}
		}
	}
}
