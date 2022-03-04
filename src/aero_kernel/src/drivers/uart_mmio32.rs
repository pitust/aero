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

use core::fmt;
use core::fmt::Write;

use spin::Once;

use crate::utils::sync::Mutex;

static SERIAL: Once<Mutex<Option<SerialPort>>> = Once::new();

bitflags::bitflags! {
    pub struct InterruptEnable: u8 {
        const RECEIVED = 1;
        const SENT = 1 << 1;
        const ERRORED = 1 << 2;
        const STATUS_CHANGE = 1 << 3;
    }
}

bitflags::bitflags! {
    pub struct LineStatus: u8 {
        const INPUT_FULL = 1;
        const OUTPUT_EMPTY = 1 << 5;
    }
}

/// An interface to a serial port that allows sending out individual bytes.
#[repr(transparent)]
pub struct SerialPort(*mut u8);

unsafe impl Send for SerialPort {}
unsafe impl Sync for SerialPort {}

impl SerialPort {
    #[inline(always)]
    pub const unsafe fn new(addr: *mut u8) -> Self {
        Self(addr)
    }

    pub fn send_byte(&mut self, byte: u8) {
        unsafe {
            match byte {
                8 | 0x7F => {
                    self.0.write(8);
                    self.0.write(b' ');
                    self.0.write(8);
                }
                _ => {
                    self.0.write(byte);
                }
            }
        }
    }
}

impl fmt::Write for SerialPort {
    fn write_str(&mut self, string: &str) -> fmt::Result {
        for byte in string.bytes() {
            self.send_byte(byte);
        }

        Ok(())
    }
}

/// Initialize the serial ports if avaliable.
pub unsafe fn init(addr: *mut u8) {
    unsafe {
        let serial = SerialPort::new(addr);

        SERIAL.call_once(move || Mutex::new(Some(serial)));
    }
}

pub macro serial_print($($arg:tt)*) {
    crate::drivers::uart_mmio32::_serial_print(format_args!($($arg)*))
}

pub macro serial_println {
    () => ($crate::drivers::uart_mmio32::serial_print!("\n")),
    ($($arg:tt)*) => ($crate::drivers::uart_mmio32::serial_print!("{}\n", format_args!($($arg)*)))
}

#[doc(hidden)]
pub fn _serial_print(args: fmt::Arguments) {
    SERIAL.get().map(|c| {
        //
        let serial = c.lock_irq();
        if let Some(serial) = *serial {
            serial.write_fmt(args)
                .expect("failed to write to COM1")
        }
    });
}
