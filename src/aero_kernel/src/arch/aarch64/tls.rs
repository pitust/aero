/*
 * Copyright (C) 2021-2022 The Aero Project Developers.
 *
 * This file is part of The Aero Project.
 *
 * Aero is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * Aero is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with Aero. If not, see <https://www.gnu.org/licenses/>.
 */



pub struct Kpcr {
    pub cpuid: usize,
}

/// SAFETY: TIPDR_EL1 should point to the kernel PCR.
pub fn get_cpuid() -> usize {
    get_kpcr().cpuid
}

/// SAFETY: TIPDR_EL1 should point to the kernel PCR.
pub fn get_kpcr() -> &'static mut Kpcr {
    let kpcr: usize;
    asm!(
        "mrs {}, TPIDR_EL1", out(reg) kpcr
    );
    unsafe { (&mut *(kpcr as *mut Kpcr)) as &'static mut Kpcr }
}
