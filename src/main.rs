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
use std::sync::{Arc, RwLock};
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use structopt::StructOpt;
use rayon::ThreadPoolBuilder;

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
	#[structopt(short, long, name = "n", default_value = "1", help = "Number of threads to spawn")]
	jobs:usize,
}

struct BatchResult {
	batch_id: u64,
	primes: Vec<u64>,
}

fn main() {
	let opts = Opt::from_args();
	let mut prime_list_raw = Vec::new();

	if opts.import.is_some() {
		let in_file = File::open(opts.import.unwrap()).unwrap();
		let reader = BufReader::new(in_file);
		for p in reader.lines().into_iter() {
			let prime:u64 = p.unwrap().parse().unwrap();
			if (prime as usize) > opts.num {
				break;
			}

			prime_list_raw.push(prime);
		}
	}

	if opts.num < 2 {
		eprintln!("Invalid value for num: {}", opts.num);
		process::exit(1);
	}

	let pool = ThreadPoolBuilder::new().num_threads(opts.jobs).build().unwrap();
	let mut pending_tasks = 0;
	/* force u64 to avoid overflow if sieve size / opts.num is larger than i32::MAX.
	 * This is unlikely, but possible.
	 */
	let mut batch_num: u64 = 0;
	let mut last_batch_id: u64 = 0;
	let mut remaining_batches = VecDeque::<BatchResult>::new();
	let (tx, rx): (Sender<BatchResult>, Receiver<BatchResult>) = mpsc::channel();

	let mut start:usize = if prime_list_raw.is_empty() {
		2
	} else {
		(*prime_list_raw.last().unwrap() + 1) as usize
	};
	let mut end = if start + opts.sieve < opts.num {
		start + opts.sieve
	} else {
		opts.num + 1
	};

	let prime_list = Arc::new(RwLock::new(prime_list_raw));
	loop {
		if (start < opts.num) && ((prime_list.read().unwrap().is_empty() && pending_tasks == 0) ||
			(*prime_list.read().unwrap().last().unwrap_or(&0)).pow(2) >= (end as u64))
		{
			let prime_list_clone = Arc::clone(&prime_list);
			let tx_clone = tx.clone();
			batch_num += 1;
			let batch_id = batch_num;
			pool.spawn(move || {
				let res = worker::work_segment(&prime_list_clone, start, end);
				tx_clone.send(BatchResult{batch_id, primes: res}).unwrap();
			});
			pending_tasks += 1;
			start += opts.sieve;
			end = if end + opts.sieve < opts.num {
				end + opts.sieve
			} else {
				opts.num + 1
			};
		} else if pending_tasks > 0 {
			let res = rx.recv().unwrap();
			pending_tasks -= 1;
			if res.batch_id != last_batch_id + 1 {
				if remaining_batches.is_empty() || (res.batch_id > remaining_batches.back().unwrap().batch_id) {
					remaining_batches.push_back(res);
				} else if res.batch_id < remaining_batches.front().unwrap().batch_id {
					remaining_batches.push_front(res);
				} else {
					let mut i = 0;
					while i < remaining_batches.len() {
						if res.batch_id < remaining_batches[i].batch_id {
							remaining_batches.insert(i, res);
							break;
						}
						i += 1;
					}
				}
			} else {
				last_batch_id = res.batch_id;

				if opts.verbose {
					for p in &res.primes {
						println!("{}", *p);
					}
				}
				prime_list.write().unwrap().append(&mut res.primes.clone());

				loop {
					match remaining_batches.pop_front_if(|br| br.batch_id == last_batch_id + 1) {
						Some(br) => {
							last_batch_id = br.batch_id;
							if opts.verbose {
								for p in &br.primes {
									println!("{}", *p);
								}
							}
							prime_list.write().unwrap().append(&mut br.primes.clone());
						},
						None => break,
					};
				}
			}
		} else {
			// no more tasks to be added, and all tasks have been processed
			break;
		}
	}

	if opts.test {
		if *prime_list.read().unwrap().last().unwrap() == (opts.num as u64) {
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
		println!("{}", prime_list.read().unwrap().last().unwrap());
	}
}
