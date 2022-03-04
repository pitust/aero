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

use super::InterruptErrorStack;

use crate::arch::controlregs;
use crate::mem::PFCause;

use crate::unwind;
use crate::userland::scheduler;

macro interrupt_exception(fn $name:ident() => $message:expr) {
    pub fn $name(stack: &mut InterruptErrorStack) {
        unwind::prepare_panic();

        log::error!("EXCEPTION: {}", $message);
        log::error!("Stack: {:#x?}", stack);

        unwind::unwind_stack_trace();

        unsafe {
            loop {
                super::halt();
            }
        }
    }
}

interrupt_exception!(fn divide_by_zero() => "Division by zero");
interrupt_exception!(fn debug() => "Debug");
interrupt_exception!(fn non_maskable() => "Non Maskable");
interrupt_exception!(fn overflow() => "Stack Overflow");
interrupt_exception!(fn bound_range() => "Out of Bounds");
interrupt_exception!(fn invalid_opcode() => "Invalid Opcode");
interrupt_exception!(fn device_not_available() => "Device not Avaliable");
interrupt_exception!(fn double_fault() => "Double Fault");
interrupt_exception!(fn invalid_tss() => "Invalid TSS");
interrupt_exception!(fn segment_not_present() => "Segment not Present");
interrupt_exception!(fn stack_segment() => "Stack Segment Fault");
interrupt_exception!(fn protection() => "Protection Fault");
interrupt_exception!(fn fpu_fault() => "FPU floating point fault");
interrupt_exception!(fn alignment_check() => "Alignment check fault");
interrupt_exception!(fn machine_check() => "Machine check fault");
interrupt_exception!(fn simd() => "SIMD floating point fault");
interrupt_exception!(fn virtualization() => "Virtualization fault");
interrupt_exception!(fn security() => "Security exception");

pub fn breakpoint(stack: &mut InterruptErrorStack) {
    // We will need to prevent RIP from going out of sync with
    // instructions.
    //
    // So we will set the RIP to RIP - 1, pointing to the int3
    // instruction.
    (*stack).stack.iret.rip -= 1;
}

bitflags::bitflags! {
    /// Describes an page fault error code.
    #[repr(transparent)]
    pub struct PageFaultErrorCode: u64 {
        /// If this flag is set, the page fault was caused by a page-protection violation,
        /// else the page fault was caused by a not-present page.
        const PROTECTION_VIOLATION = 1;

        /// If this flag is set, the memory access that caused the page fault was a write.
        /// Else the access that caused the page fault is a memory read. This bit does not
        /// necessarily indicate the cause of the page fault was a read or write violation.
        const CAUSED_BY_WRITE = 1 << 1;

        /// If this flag is set, an access in user mode (CPL=3) caused the page fault. Else
        /// an access in supervisor mode (CPL=0, 1, or 2) caused the page fault. This bit
        /// does not necessarily indicate the cause of the page fault was a privilege violation.
        const USER_MODE = 1 << 2;

        /// If this flag is set, the page fault is a result of the processor reading a 1 from
        /// a reserved field within a page-translation-table entry.
        const MALFORMED_TABLE = 1 << 3;

        /// If this flag is set, it indicates that the access that caused the page fault was an
        /// instruction fetch.
        const INSTRUCTION_FETCH = 1 << 4;
    }
}

pub(super) fn page_fault(stack: &mut InterruptErrorStack) {
    let accessed_address = controlregs::read_cr2();
    let cause = PageFaultErrorCode::from_bits_truncate(stack.code);
    let cause = {
        let mut hl_cause = PFCause::Unknown;
        if !cause.contains(PageFaultErrorCode::USER_MODE) {
            hl_cause = PFCause::KernelFault;
        } else {
            if cause.contains(PageFaultErrorCode::PROTECTION_VIOLATION) {
                hl_cause = if cause.contains(PageFaultErrorCode::INSTRUCTION_FETCH) { PFCause::Execute } else { PFCause::Write };
            } else {
                hl_cause = PFCause::NotPresent;
            }
        }
        hl_cause
    };

    // We cannot directly check if we want to handle the page fault by checking
    // if the CS register contains the RPL_3 flag since, we also want to handle the
    // situation where we are trying to access a user provided buffer in the kernel and
    // its not mapped. So we handle the page fault if the accessed address is less then the
    // MAX userland address and we only signal kill the process if its trying to access
    // a non-mapped memory region while in RPL_3.
    let userland_last_address = super::super::task::userland_last_address();

    if accessed_address < userland_last_address && scheduler::is_initialized() {
        let signal = scheduler::get_scheduler()
            .current_task()
            .vm
            .handle_page_fault(todo!("what is the current task???"), cause, accessed_address);

        if !signal && stack.stack.iret.is_user() {
            log::error!("Segmentation fault");
            log::error!("");
            log::error!("accessed address: {:#x}", accessed_address);
            log::error!("reason: {:?}", cause);
            log::error!("");

            if stack.stack.iret.is_user() {
                let task = scheduler::get_scheduler().current_task();

                log::error!(
                    "process: (pid={}, pid={})",
                    task.tid().as_usize(),
                    task.pid().as_usize()
                );
            }

            log::error!("stack: {:#x?}", stack);

            scheduler::get_scheduler().current_task().vm.log();
            scheduler::get_scheduler().current_task().file_table.log();

            unwind::unwind_stack_trace();

            let task = scheduler::get_scheduler().current_task();
            task.signal(aero_syscall::signal::SIGSEGV);

            return;
        } else if !signal {
        } else {
            return;
        }
    }

    unwind::prepare_panic();

    log::error!("EXCEPTION: Page Fault");
    log::error!("");
    log::error!("Accessed Address: {:#x}", accessed_address);
    log::error!("Error: {:?}", cause);
    log::error!("");

    log::error!("Stack: {:#x?}", stack);

    unwind::unwind_stack_trace();

    unsafe {
        loop {
            super::halt();
        }
    }
}