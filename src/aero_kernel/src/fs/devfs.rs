use alloc::boxed::Box;

use hashbrown::HashMap;
use spin::Mutex;

use super::{install_filesystem, FileSystem};

const MAX_DEVICES: usize = 128;

lazy_static::lazy_static! {
    pub static ref DEVICES: Mutex<HashMap<&'static str, Box<dyn Device>>> = Mutex::new(HashMap::with_capacity(MAX_DEVICES));
}

pub trait Device: Send + Sync {}

pub struct DevFs;

impl FileSystem for DevFs {}

pub(super) fn install_device<D: 'static + Device>(signature: &'static str, device: Box<D>) {
    DEVICES.lock().insert(signature, device);
}

struct DevNull;
struct DevZero;
struct DevUrandom;

impl Device for DevNull {}
impl Device for DevZero {}
impl Device for DevUrandom {}

/// Initialize devfs and install it in the dyn filesystem hashmap.
pub(super) fn init() {
    let devfs = box DevFs;

    /*
     * First of all lets install all of the devices in devfs:
     *
     * /dev/null
     * /dev/zero
     * /dev/urandom
     */
    let null = box DevNull;
    install_device("null", null);

    log::debug!("Installed /dev/null");

    let zero = box DevZero;
    install_device("zero", zero);

    log::debug!("Installed /dev/zero");

    let urandom = box DevUrandom;
    install_device("urandom", urandom);

    log::debug!("Installed /dev/urandom");

    /*
     * Now after we have initialized devfs we are going to install it as a filesystem
     * in our dyn filesystems hashmap with "dev" as its signature.
     */
    install_filesystem("dev", devfs);

    log::debug!("Installed devfs");
}
