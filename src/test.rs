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

use std::collections::VecDeque;

pub fn is_prime_f(n:u64, b:u64) -> bool
{
	assert_ne!(b, 0);
	assert_ne!(b, 1);

	let mut start = b;
	if start == 2
	{
		if n % 2 == 0
		{
			return false;
		}
		else
		{
			start += 1;
		}
	}
	// skip even numbers
	else if start % 2 == 0
	{
		start += 1;
	}

	let limit = (n as f64).sqrt() as u64 + 1;
	for i in (start..limit).step_by(2)
	{
		if n % i == 0
		{
			return false;
		}
	}
	return true;
}

pub fn is_prime(n:u64) -> bool
{
	return is_prime_f(n, 2);
}

pub fn is_prime_mem(n:u64, primes:&VecDeque<u64>) -> bool
{
	let limit = (n as f64).sqrt() as u64;
	for i in primes.iter().take_while(|x| **x <= limit)
	{
		if n % *i == 0
		{
			return false;
		}
	}
	return true;
}
