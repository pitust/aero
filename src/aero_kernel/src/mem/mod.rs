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

pub mod alloc;
pub mod pti;

use core::alloc::Layout;
use core::fmt;
use core::iter::Step;
use core::ops::{Add, AddAssign, Sub, SubAssign};
use core::sync::atomic::{AtomicUsize, Ordering};

use ::alloc::boxed::Box;
use ::alloc::sync::Arc;
use aero_syscall::MMapProt;
use ::alloc::{vec::Vec, vec};
use spin::Once;

use crate::utils::sync::Mutex;

/// A 64-bit virtual memory address.
///
/// This is a wrapper type around an `u64`, so it is always 8 bytes, even when compiled
/// on non 64-bit systems. The
/// [`TryFrom`](https://doc.rust-lang.org/std/convert/trait.TryFrom.html) trait can be used for performing conversions
/// between `u64` and `usize`.

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct VirtAddr(usize);

/// Align address downwards.
///
/// Returns the greatest x with alignment `align` so that x <= addr. The alignment must be
///  a power of 2.
#[inline]
pub fn align_down<T: Into<usize>>(addr: T, align: usize) -> usize {
    assert!(align.is_power_of_two(), "`align` must be a power of two");
    addr.into() & !(align - 1)
}

/// Align address upwards.
///
/// Returns the smallest `x` with alignment `align` so that `x >= addr`.
///
/// Panics if the alignment is not a power of two. Without the `const_fn`
/// feature, the panic message will be "index out of bounds".
#[inline]
pub fn align_up(addr: usize, align: usize) -> usize {
    let align_mask = align - 1;

    if addr & align_mask == 0 {
        addr // already aligned
    } else {
        (addr | align_mask) + 1
    }
}

#[inline]
pub fn is_aligned(addr: usize, align: usize) -> bool {
    align_up(addr, align) == addr
}

impl VirtAddr {
    /// Creates a new virtual address.
    #[inline]
    pub const fn new_u64(addr: u64) -> VirtAddr {
        VirtAddr(addr as usize)
    }

    /// Creates a new virtual address.
    #[inline]
    pub const fn new(addr: usize) -> VirtAddr {
        VirtAddr(addr)
    }

    /// Creates a virtual address that points to `0`.
    #[inline]
    pub const fn zero() -> VirtAddr {
        VirtAddr(0)
    }

    /// Converts the address to a `u64`.
    #[inline]
    #[deprecated = "please use usize not u64"]
    pub const fn as_u64(self) -> u64 {
        self.0 as u64
    }

    /// Converts the address to a `usize`.
    #[inline]
    pub const fn as_usize(self) -> usize {
        self.0
    }

    /// Converts the address to a raw pointer.
    #[cfg(target_pointer_width = "64")]
    #[inline]
    pub fn as_ptr<T>(self) -> *const T {
        self.as_usize() as *const T
    }

    /// Converts the address to a mutable raw pointer.
    #[cfg(target_pointer_width = "64")]
    #[inline]
    pub fn as_mut_ptr<T>(self) -> *mut T {
        self.as_ptr::<T>() as *mut T
    }

    /// Aligns the virtual address downwards to the given alignment.
    ///
    /// See the `align_down` function for more information.
    #[inline]
    pub fn align_down<U>(self, align: U) -> Self
    where
        U: Into<usize>,
    {
        VirtAddr(align_down(self.0, align.into()))
    }

    pub fn is_aligned<U>(&self, align: U) -> bool
    where
        U: Into<usize>,
    {
        is_aligned(self.0, align.into())
    }

    /// Aligns the virtual address upwards to the given alignment.
    ///
    /// See the `align_up` function for more information.
    #[inline]
    pub fn align_up<U>(self, align: U) -> Self
    where
        U: Into<usize>,
    {
        VirtAddr(align_up(self.0, align.into()))
    }

    pub const fn const_sub_u64(self, other: u64) -> VirtAddr {
        VirtAddr(self.0 - (other as usize))
    }
}

impl fmt::Debug for VirtAddr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_tuple("VirtAddr")
            .field(&format_args!("{:#x}", self.0))
            .finish()
    }
}

impl fmt::Binary for VirtAddr {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Binary::fmt(&self.0, f)
    }
}

impl fmt::LowerHex for VirtAddr {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::LowerHex::fmt(&self.0, f)
    }
}

impl fmt::Octal for VirtAddr {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Octal::fmt(&self.0, f)
    }
}

impl fmt::UpperHex for VirtAddr {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::UpperHex::fmt(&self.0, f)
    }
}

impl fmt::Pointer for VirtAddr {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Pointer::fmt(&(self.0 as *const ()), f)
    }
}

impl Add<u64> for VirtAddr {
    type Output = Self;
    #[inline]
    fn add(self, rhs: u64) -> Self::Output {
        VirtAddr::new(self.0 + (rhs as usize))
    }
}

impl AddAssign<u64> for VirtAddr {
    #[inline]
    fn add_assign(&mut self, rhs: u64) {
        *self = *self + (rhs as usize);
    }
}

#[cfg(target_pointer_width = "64")]
impl Add<usize> for VirtAddr {
    type Output = Self;
    #[inline]
    fn add(self, rhs: usize) -> Self::Output {
        self + rhs as u64
    }
}

#[cfg(target_pointer_width = "64")]
impl AddAssign<usize> for VirtAddr {
    #[inline]
    fn add_assign(&mut self, rhs: usize) {
        self.add_assign(rhs as u64)
    }
}

impl Sub<u64> for VirtAddr {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: u64) -> Self::Output {
        VirtAddr::new(self.0.checked_sub(rhs as usize).unwrap())
    }
}

impl SubAssign<u64> for VirtAddr {
    #[inline]
    fn sub_assign(&mut self, rhs: u64) {
        *self = *self - rhs;
    }
}

#[cfg(target_pointer_width = "64")]
impl Sub<usize> for VirtAddr {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: usize) -> Self::Output {
        self - rhs as u64
    }
}

#[cfg(target_pointer_width = "64")]
impl SubAssign<usize> for VirtAddr {
    #[inline]
    fn sub_assign(&mut self, rhs: usize) {
        self.sub_assign(rhs as u64)
    }
}

impl Sub<VirtAddr> for VirtAddr {
    type Output = usize;
    #[inline]
    fn sub(self, rhs: VirtAddr) -> Self::Output {
        self.as_usize().checked_sub(rhs.as_usize()).unwrap()
    }
}

impl Step for VirtAddr {
    #[inline]
    fn steps_between(start: &Self, end: &Self) -> Option<usize> {
        if start < end {
            Some(end.as_usize() - start.as_usize())
        } else {
            None
        }
    }

    #[inline]
    fn forward_checked(start: Self, count: usize) -> Option<Self> {
        Some(start + count)
    }

    #[inline]
    fn backward_checked(start: Self, count: usize) -> Option<Self> {
        Some(start - count)
    }
}

pub enum VMSpaceKind {
    User,
    Kernel,
}
bitflags::bitflags! {
    pub struct MapFlags : u64 {
        const READ = 1 << 0;
        const WRITE = 1 << 1;
        const EXEC = 1 << 2;
        const NO_CACHE = 1 << 3;
        const KERNEL = 1 << 4;
        const USER = 1 << 5;
    }
}
impl From<MMapProt> for MapFlags {
    fn from(_: MMapProt) -> Self {
        todo!()
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum PFCause {
    Unknown,
    KernelFault,
    NotPresent,
    Write,
    Execute,
}
pub trait PageFaultHandler: Send + Sync {
    fn page_fault(&self, vmspace: &mut dyn AddressSpace, cause: PFCause, addr: VirtAddr);
}
pub type AddressSpaceFactory = Box<dyn Fn() -> Arc<Mutex<Box<dyn AddressSpace>>> + Send + Sync>;
pub trait AddressSpace: Send + Sync {
    fn map(
        &mut self,
        phys: usize,
        virt: VirtAddr,
        len: usize,
        flags: MapFlags,
    ) -> Result<(), &'static str>;
    fn unmap(&mut self, virt: VirtAddr, len: usize) -> Result<(), &'static str>;
    fn fork(&mut self) -> Result<AddressSpaceHandle, &'static str>;
    fn set_flags(
        &mut self,
        virt: VirtAddr,
        len: usize,
        flags: MapFlags,
    ) -> Result<(), &'static str>;
    fn lookup(&mut self, virt: VirtAddr) -> Option<(usize, MapFlags)>;
    fn set_fault_handler(&mut self, handler: Box<dyn PageFaultHandler>);
    fn select(&mut self);
    fn arch_get_handle(&self) -> usize;
}
#[derive(Clone)]
pub struct AddressSpaceHandle {
    space: Arc<Mutex<Box<dyn AddressSpace>>>,
}
impl AddressSpaceHandle {
    pub fn new(space: Arc<Mutex<Box<dyn AddressSpace>>>) -> AddressSpaceHandle {
        AddressSpaceHandle { space }
    }
    pub fn map(
        &self,
        phys: usize,
        virt: VirtAddr,
        len: usize,
        flags: MapFlags,
    ) -> Result<(), &'static str> {
        self.space.lock_irq().map(phys, virt, len, flags)
    }

    pub fn unmap(&self, virt: VirtAddr, len: usize) -> Result<(), &'static str> {
        self.space.lock_irq().unmap(virt, len)
    }

    pub fn fork(&self) -> Result<AddressSpaceHandle, &'static str> {
        self.space.lock_irq().fork()
    }

    pub fn set_flags(
        &self,
        virt: VirtAddr,
        len: usize,
        flags: MapFlags,
    ) -> Result<(), &'static str> {
        self.space.lock_irq().set_flags(virt, len, flags)
    }

    pub fn lookup(&self, virt: VirtAddr) -> Option<(usize, MapFlags)> {
        self.space.lock_irq().lookup(virt)
    }

    pub fn set_fault_handler(&self, handler: Box<dyn PageFaultHandler>) {
        todo!()
    }

    pub fn select(&self) {
        todo!()
    }

    pub fn arch_get_handle(&self) -> usize {
        todo!()
    }
}

static KERNEL_ADDRESS_SPACE: Once<Arc<Mutex<Box<dyn AddressSpace>>>> = Once::new();
static ADDRESS_SPACE_FACTORY: Once<Mutex<AddressSpaceFactory>> = Once::new();
pub fn set_address_space_factory(f: AddressSpaceFactory) {
    ADDRESS_SPACE_FACTORY.call_once(|| Mutex::new(f));
}
pub fn set_kernel_address_space<T: 'static + AddressSpace>(f: T) {
    KERNEL_ADDRESS_SPACE.call_once(|| Arc::new(Mutex::new(box f)));
}

pub fn kernel_address_space() -> AddressSpaceHandle {
    let space = KERNEL_ADDRESS_SPACE
        .get()
        .expect("Missing kernel address space");
    AddressSpaceHandle::new(Arc::clone(space))
}
pub fn new_address_space() -> AddressSpaceHandle {
    let factory = ADDRESS_SPACE_FACTORY
        .get()
        .expect("Missing address space factory")
        .lock_irq();

    AddressSpaceHandle::new(factory())
}
pub fn page_bits() -> usize {
    // TODO: pages aren't always 4k
    12
}
pub fn page_size() -> usize {
    1 << page_bits()
}
pub fn pmm_alloc_fixed(sz: usize) -> usize {
    (if sz == 0x1000 {
        let mut lk = PMM_SINGLE_LIST.get().unwrap().lock();
        match lk.pop() {
            Some(p) => p as usize,
            None => {
                drop(lk);
                let dbl = pmm_alloc_fixed(0x2000);
                pmm_free_fixed(dbl, 0x1000);
                pmm_free_fixed(dbl + 0x1000, 0x1000);
                PMM_SINGLE_LIST.get().unwrap().lock().pop().unwrap() as usize
            }
        }
    } else {
        assert!(sz == 0x2000);
        PMM_DOUBLE_LIST.get().unwrap().lock().pop().expect("OOM!") as usize
    }) - unsafe { crate::PHYSICAL_MEMORY_OFFSET.as_usize() }
}
pub fn pmm_alloc_page() -> usize {
    pmm_alloc_fixed(0x1000)
}
struct PointerChain {
    ptr: *mut usize,
}
unsafe impl Send for PointerChain {}
unsafe impl Sync for PointerChain {}
impl PointerChain {
    // SAFETY: the new pointer must be valid.
    pub unsafe fn push(&mut self, new: *mut usize) {
        *new = self.ptr as usize;
        self.ptr = new;
    }
    pub fn pop(&mut self) -> Option<*mut usize> {
        if !self.ptr.is_null() {
            Some(unsafe {
                let ptr = self.ptr;
                self.ptr = *ptr as _;
                *ptr = 0;
                ptr
            })
        } else {
            None
        }
    }
}
static PMM_DOUBLE_LIST: Once<Mutex<PointerChain>> = Once::new();
static PMM_SINGLE_LIST: Once<Mutex<PointerChain>> = Once::new();
pub fn pmm_free_fixed(addr: usize, sz: usize) {
    PMM_SINGLE_LIST.call_once(|| Mutex::new(PointerChain { ptr: 0 as _ }));
    PMM_DOUBLE_LIST.call_once(|| Mutex::new(PointerChain { ptr: 0 as _ }));
    assert!(sz == 0x1000 || sz == 0x2000);
    
    let addr = unsafe { crate::PHYSICAL_MEMORY_OFFSET.as_usize() + addr };

    if sz == 0x1000 {
        let mut list = PMM_SINGLE_LIST.get().unwrap().lock();
        unsafe { list.push(addr as *mut usize); }
    } else {
        let mut list = PMM_DOUBLE_LIST.get().unwrap().lock();
        unsafe { list.push(addr as *mut usize); }
    }
}
// TODO: this is quite slow, and should be faster
// TODO: does that matter? I (pitust) think it's quite significant, and I shouldn't be doing the pmm anyway
// TODO: yoink old PMM
pub fn pmm_add_region(mut base: usize, mut len: usize) {
    log::debug!("memory map: [{base:#16x?}; {len:#16x?}]");
    while len >= 0x1000 {
        if len >= 0x2000 {
            pmm_free_fixed(base, 0x2000);
            len -= 0x2000;
            base += 0x2000;
        } else {
            pmm_free_fixed(base, 0x1000);
            len -= 0x1000;
            base += 0x1000;
        }
    }
}
pub fn pmm_free_page(pg: usize) {
    pmm_free_fixed(pg, 4096)
}

pub struct VMFrame {
    rc: &'static AtomicUsize,
}
// NOTE: this seems like a race condition in the making
impl VMFrame {
    pub fn ref_count(&self) -> usize {
        // NOTE: is SeqCst correct for this?
        // SeqCst is the strongest model, but I (pitust) am unsure if this is needed.
        // Anyway, for consistency between arm and x86 SeqCst is used.
        self.rc.load(Ordering::SeqCst)
    }
    pub fn add_ref(&self) {
        self.rc.fetch_add(1, Ordering::SeqCst);
    }
    pub fn del_ref(&self) {
        self.rc.fetch_sub(1, Ordering::SeqCst);
    }
}
impl Clone for VMFrame {
    fn clone(&self) -> VMFrame {
        VMFrame { rc: self.rc }
    }
}

static VM_FRAMES: Once<Vec<VMFrame>> = Once::new();
pub fn vmframe_init(max_ptr: usize) {
    VM_FRAMES.call_once(|| {
        let nframe = max_ptr >> page_bits();
        let mut v = vec![];
        v.resize_with(nframe, || VMFrame { rc: Box::leak(box AtomicUsize::new(0)) });
        v
    });
}
pub fn vm_frame(addr: usize) -> Option<VMFrame> {
    let w = VM_FRAMES.get()?;
    if addr >> page_bits() < w.len() {
        Some(w[addr >> page_bits()].clone())
    } else {
        None
    }
}

pub fn alloc_boxed_buffer<T>(size: usize) -> Box<[T]> {
    if size == 0 {
        return <Box<[T]>>::default();
    }

    let layout = unsafe { Layout::from_size_align_unchecked(size, 8) };
    let ptr = unsafe { ::alloc::alloc::alloc_zeroed(layout) as *mut T };
    let slice_ptr = core::ptr::slice_from_raw_parts_mut(ptr, size);

    unsafe { Box::from_raw(slice_ptr) }
}

/// Creates a Rust string from the provided C string.
///
/// ## Safety
/// - The provided pointer must be valid.
/// - The provided pointer must point to a null-terminated C string.
/// - The returned lifetime is not guaranteed to be the actual lifetime
/// of `ptr`.
/// - It is not guaranteed that the memory pointed by `ptr` won’t change
/// before the Rust string has been destroyed.
pub unsafe fn c_str_as_str<'cstring>(ptr: *const u8) -> &'cstring str {
    let length = c_strlen(ptr);
    let slice = core::slice::from_raw_parts(ptr, length);

    core::str::from_utf8_unchecked(slice)
}

/// Determines the provided of the given C string.
///
/// ## Safety
/// - The provided pointer must be valid.
/// - The provided pointer must point to a null-terminated C string.
pub unsafe fn c_strlen(mut ptr: *const u8) -> usize {
    let mut length = 0;

    while *ptr != 0 {
        ptr = ptr.offset(1);
        length += 1;
    }

    length
}
