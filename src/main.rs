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

//use std::env;
use structopt::StructOpt;
use std::path::PathBuf;
use std::collections::VecDeque;

#[derive(StructOpt)]
struct Opt {
	#[structopt(short, long)]
	verbose:bool,
	//#[structopt(short, long)]
	//import:Option<PathBuf>,
	n:usize,
}

fn main() {
	let opts = Opt::from_args();

	// get the first `n` primes
	let n = opts.n;
	let mut primes:VecDeque<u64> = VecDeque::with_capacity(n);
	// first prime
	if opts.verbose
	{
		println!("{}", 2);
	}
	primes.push_back(2);

	let mut candidate:u64 = 3;

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
		println!("{}", primes.back().unwrap());
	}
}
