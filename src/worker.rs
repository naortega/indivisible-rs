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

use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

pub struct Worker {
	primes_list:Rc<RefCell<VecDeque<u64>>>,
}

impl Worker {
	pub fn new(primes_list:Rc<RefCell<VecDeque<u64>>>) -> Worker {
		Worker {
			primes_list,
		}
	}

	pub fn run(&mut self) {
	}
}
