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

pub struct CandidateGenerator {
	base:u64,
	// first use of the base
	first_use:bool,
}

impl CandidateGenerator {
	pub fn new() -> Self {
		CandidateGenerator {
			base: 0,
			first_use: true,
		}
	}

	pub fn calc_base(&mut self, last_prime:u64) {
		if last_prime == 2 {
			self.base = 0;
			self.first_use = false;
		} else if last_prime == 3 {
			self.base = 6;
			self.first_use = true;
		} else {
			let modulo = last_prime % 6;
			if modulo == 1 {
				self.base = last_prime + 5;
				self.first_use = true;
			} else if modulo == 5 {
				self.base = last_prime + 1;
				self.first_use = false;
			} else {
				panic!("Invalid last prime {}" , last_prime);
			}
		}
	}

	pub fn next(&mut self) -> u64 {
		/*
		 * All primes, except 2 and 3, will be equal to (n * 6 ± 1). This avoids
		 * multiples of three, optimizing our counting.
		 */
		let val;

		if self.base != 0 {
			if self.first_use {
				val = self.base - 1;
			} else {
				val = self.base + 1;
			}
		} else {
			if self.first_use {
				val = 2;
			} else {
				val = 3;
			}
		}

		if !self.first_use {
			self.base += 6;
		}
		self.first_use = !self.first_use;

		val
	}
}
