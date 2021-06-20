/* 
 * Copyright (C) 2021 The Aero Project Developers.
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

use bit_field::BitField;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct Color(u32);

impl Color {
    pub const WHITE: Self = Self::from_hex(0xFFFFFF);
    pub const BLACK: Self = Self::from_hex(0x000000);

    pub fn from_rgb(r: u8, g: u8, b: u8, a: u8) -> Self {
        let mut hex: u32 = 0;

        hex.set_bits(0..8, r as u32);
        hex.set_bits(8..16, g as u32);
        hex.set_bits(16..24, b as u32);
        hex.set_bits(24..32, a as u32);

        Self::from_hex(hex)
    }

    #[inline(always)]
    pub const fn from_hex(hex: u32) -> Self {
        Self(hex)
    }

    #[inline(always)]
    pub const fn inner(&self) -> u32 {
        self.0
    }

    #[inline(always)]
    pub fn get_red_bit(&self) -> u8 {
        (self.0.get_bits(0..8) & 255) as u8
    }

    #[inline(always)]
    pub fn get_green_bit(&self) -> u8 {
        (self.0.get_bits(8..16) & 255) as u8
    }

    #[inline(always)]
    pub fn get_blue_bit(&self) -> u8 {
        (self.0.get_bits(16..24) & 255) as u8
    }

    #[inline(always)]
    pub fn get_alpha_bit(&self) -> u8 {
        (self.0.get_bits(24..32) & 255) as u8
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ColorCode(Color, Color);

impl ColorCode {
    #[inline(always)]
    pub fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode(foreground, background)
    }

    #[inline(always)]
    pub fn get_foreground(&self) -> Color {
        self.0
    }

    #[inline(always)]
    pub fn get_background(&self) -> Color {
        self.1
    }
}
