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

/**
 * @brief Work on a segment.
 *
 * @param known_primes:&Vec<u64> List of known primes at least until sqrt(end).
 * @param start:usize Beginning of the segment (inclusive).
 * @param end:usize End of the segment (exclusive).
 *
 * @return List of primes found in segment.
 */
pub fn work_segment(known_primes:&Vec<u64>, start:usize, end:usize) -> Vec<u64> {
	let mut sieve = vec![true; end - start];
	let mut found_primes = Vec::new();

	let sqrt_end = (end as f64).sqrt() as usize;

	for p in known_primes {
		let prime = *p as usize;
		if prime > sqrt_end {
			break;
		}

		let modu = start % prime;
		let mut mult = if modu == 0 {
			start
		} else {
			start + prime - modu
		};
		while mult < end {
			sieve[mult - start] = false;
			mult += prime;
		}
	}

	for i in 0..(end - start) {
		if sieve[i] {
			let prime = i + start;
			found_primes.push(prime as u64);

			let mut mult = prime * prime;
			while mult < end {
				sieve[mult - start] = false;
				mult += prime;
			}
		}
	}

	found_primes
}
