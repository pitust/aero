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

use core::alloc::Layout;

use ::alloc::{alloc::alloc_zeroed, boxed::Box, sync::Arc, vec, vec::Vec};

use crate::{mem::*, utils::sync::Mutex};

use super::controlregs::read_cr3;

pub fn level_5_paging_enabled() -> bool {
    false // TODO: level 5 paging
}

struct KernelAddressSpace {
    owned_pages: Vec<usize>,
    mem: Box<[usize; 512]>,
}
struct UserlandAddressSpace {
    owned_pages: Vec<usize>,
    mem: Box<[usize; 256]>,
}
// You may have seen this function in my (pitust's) other kernels, because I wrote this once and it *works*
unsafe fn get_pte_ptr(
    owned_pages: &mut Vec<usize>,
    page_table: &mut [usize],
    va: usize,
    is_kernel: bool
) -> Option<*mut usize> {
    let mut page_table = &page_table[0] as *const _ as *mut usize;
    let mut control_pte = 0 as *mut usize;
    let va_val = va & 0x000f_ffff_ffff_f000;
    let offsets = [
        (va_val >> 12 >> 9 >> 9 >> 9) & 0x1ff,
        (va_val >> 12 >> 9 >> 9) & 0x1ff,
        (va_val >> 12 >> 9) & 0x1ff,
        (va_val >> 12) & 0x1ff,
    ];
    let mut i = -1;
    for key in offsets {
        i += 1;
        if page_table.offset(key as isize).read() & 0x80 != 0 {
            return None;
        }
        if page_table.offset(key as isize).read() & 1 == 0 && i != 3 {
            let page = pmm_alloc_page();
            core::ptr::write_bytes(
                (page + crate::PHYSICAL_MEMORY_OFFSET.as_usize()) as *mut u8,
                0,
                4096,
            );
            page_table.offset(key as isize).write(page | 7);
            if !is_kernel { owned_pages.push(page); }
        }
        control_pte = page_table.offset(key as isize);
        page_table = ((page_table.offset(key as isize).read() & 0x000f_ffff_ffff_f000)
            + crate::PHYSICAL_MEMORY_OFFSET.as_usize()) as *mut usize;
    }

    return Some(control_pte);
}

fn encode_flags(flags: MapFlags) -> usize {
    let mut hw_flags = 1;
    if !flags.contains(MapFlags::READ) {
        log::warn!("unreadable mappings are not possible on x86")
    }
    if !flags.contains(MapFlags::USER) && !flags.contains(MapFlags::KERNEL) {
        panic!("Invalid map flags: {flags:?}")
    }
    if flags.contains(MapFlags::USER) && flags.contains(MapFlags::KERNEL) {
        panic!("Invalid map flags: {flags:?}")
    }
    if flags.contains(MapFlags::WRITE) {
        hw_flags |= 1 << 1;
    }
    if flags.contains(MapFlags::USER) {
        hw_flags |= 1 << 2;
    }
    if flags.contains(MapFlags::NO_CACHE) {
        hw_flags |= 16;
    }
    if !flags.contains(MapFlags::EXEC) {
        hw_flags |= 1 << 63;
    }

    hw_flags
}

impl AddressSpace for UserlandAddressSpace {
    fn map(
        &mut self,
        phys: usize,
        virt: VirtAddr,
        len: usize,
        flags: MapFlags,
    ) -> Result<(), &'static str> {
        todo!()
    }

    fn unmap(&mut self, virt: VirtAddr, len: usize) -> Result<(), &'static str> {
        todo!()
    }

    fn fork(&mut self) -> Result<AddressSpaceHandle, &'static str> {
        todo!()
    }

    fn set_flags(
        &mut self,
        virt: VirtAddr,
        len: usize,
        flags: MapFlags,
    ) -> Result<(), &'static str> {
        todo!()
    }

    fn lookup(&mut self, virt: VirtAddr) -> Option<(usize, MapFlags)> {
        todo!()
    }

    fn set_fault_handler(&mut self, handler: Box<dyn PageFaultHandler>) {
        todo!()
    }

    fn select(&mut self) {
        todo!()
    }

    fn arch_get_handle(&self) -> usize {
        todo!()
    }
}
impl AddressSpace for KernelAddressSpace {
    fn map(
        &mut self,
        phys: usize,
        virt: VirtAddr,
        len: usize,
        flags: MapFlags,
    ) -> Result<(), &'static str> {
        assert!(
            virt.as_usize() > 0xff00_0000_0000_0000,
            "kernel can only map kernel pages"
        );
        assert!(!level_5_paging_enabled(), "TODO: la57");

        for offset in 0..(len >> 12) {
            let p = unsafe {
                get_pte_ptr(
                    &mut self.owned_pages,
                    &mut *self.mem,
                    virt.as_usize() + (offset << 12),
                    true,
                )
            };
            if let Some(ptr) = p {
                if let Some(vmf) = crate::mem::vm_frame(phys) {
                    vmf.add_ref();
                }

                unsafe {
                    *ptr = encode_flags(flags) | phys;
                }
            } else {
                return Err("we got a huge page in there somehow");
            }
        }

        Ok(())
    }

    fn unmap(&mut self, virt: VirtAddr, len: usize) -> Result<(), &'static str> {
        todo!()
    }

    fn fork(&mut self) -> Result<AddressSpaceHandle, &'static str> {
        todo!()
    }

    fn set_flags(
        &mut self,
        virt: VirtAddr,
        len: usize,
        flags: MapFlags,
    ) -> Result<(), &'static str> {
        todo!()
    }

    fn lookup(&mut self, virt: VirtAddr) -> Option<(usize, MapFlags)> {
        todo!()
    }

    fn set_fault_handler(&mut self, handler: Box<dyn PageFaultHandler>) {
        todo!()
    }

    fn select(&mut self) {
        todo!()
    }

    fn arch_get_handle(&self) -> usize {
        todo!()
    }
}

pub fn init() {
    set_address_space_factory(box || {
        Arc::new(Mutex::new(box UserlandAddressSpace {
            mem: unsafe {
                Box::from_raw(alloc_zeroed(
                    Layout::from_size_align(2048, 8).expect("Layout creation failed"),
                ) as *mut _)
            },
            owned_pages: vec![],
        }))
    });
    set_kernel_address_space(KernelAddressSpace {
        mem: unsafe {
            Box::from_raw((read_cr3().0 + crate::PHYSICAL_MEMORY_OFFSET.as_usize()) as *mut _)
        },
        owned_pages: vec![],
    });
}
