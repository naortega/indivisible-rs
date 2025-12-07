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
 * @param start:usize Beginning of the segment (inclusive).
 * @param end:usize End of the segment (inclusive).
 *
 * @return List of primes found in segment.
 */
pub fn work_segment(start:usize, end:usize) -> Vec<u64> {
	let mut found_primes = Vec::<u64>::new();
	let mut arr = vec![false; end - start + 1];

	if start < 2 && end > 2 {
		arr[2] = true;
	}
	if start < 3 && end > 3 {
		arr[3] = true;
	}

	let sqrt_of_num = f64::sqrt(end as f64) as usize;
	for x in 1..=sqrt_of_num {
		let xx4 = 4 * x * x;
		let xx3 = 3 * x * x;
		for y in 1..=sqrt_of_num {
			let yy = y * y;

			let n1 = xx4 + yy;
			if n1 <= end && (n1 % 12 == 1 || n1 % 12 == 5) {
				arr[n1] = !arr[n1];
			}

			let n2 = xx3 + yy;
			if n2 <= end && n2 % 12 == 7 {
				arr[n2] = !arr[n2];
			}

			if x > y {
				let n3 = xx3 - yy;
				if n3 <= end && n3 % 12 == 11 {
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
		while j <= end {
			arr[j] = false;
			j += i * i;
		}
	}

	for i in 2..=end {
		if arr[i] {
			found_primes.push(i as u64);
		}
	}

	found_primes
}
