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



#[repr(C)]
pub struct InterruptStack {
    spsr: u64,
    pc: u64,
    x31: u64,
    x30: u64,
    x29: u64,
    x28: u64,
    x27: u64,
    x26: u64,
    x25: u64,
    x24: u64,
    x23: u64,
    x22: u64,
    x21: u64,
    x20: u64,
    x19: u64,
    x18: u64,
    x17: u64,
    x16: u64,
    x15: u64,
    x14: u64,
    x13: u64,
    x12: u64,
    x11: u64,
    x10: u64,
    x9: u64,
    x8: u64,
    x7: u64,
    x6: u64,
    x5: u64,
    x4: u64,
    x3: u64,
    x2: u64,
    sp: u64,
    x1: u64,
    x0: u64,
}
pub unsafe fn disable_interrupts() {
    asm!("cpsid if", options(nomem, nostack));
}
pub unsafe fn enable_interrupts() {
    asm!("cpsie if", options(nomem, nostack));
}
pub unsafe fn is_enabled() -> bool {
    let v: usize;
    asm!("mrs {}, PRIMASK", out(reg)v);
    v != 0
}
pub unsafe fn halt() {
    asm!("wfi");
}
pub fn pause() {
    unsafe { asm!("yield"); }
}

