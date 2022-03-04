use crate::mem::VirtAddr;




pub fn userland_last_address() -> VirtAddr {
    // i don't even *want* to know if this is the highest address you can use
    VirtAddr::new(0x0000_ffff_0000_0000)
}