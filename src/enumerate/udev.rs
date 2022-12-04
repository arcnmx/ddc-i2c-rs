use {
    crate::{I2cDdc, I2cDeviceDdc},
    ddc::Edid,
    i2c_linux::enumerate::{EnumeratedDevice, UdevDevice},
    std::{ffi::OsStr, io, os::unix::ffi::OsStrExt, vec},
};

/// Enumerate all currently attached displays on the system.
///
/// The current detection approach only checks that a monitor is on the I2C bus
/// with a reachable EDID/EEPROM. DDC/CI communication may not be available if
/// the display does not support it, or if the active input is controlled by
/// another host device.
///
/// # Example
///
/// ```rust,no_run
/// use ddc::Ddc;
/// use ddc_i2c::UdevEnumerator;
///
/// # fn main() {
/// let displays = UdevEnumerator::new().unwrap();
/// for ddc in displays {
///     let mut ddc = match ddc.open_checked() {
///         Ok(ddc) => ddc,
///         Err(e) => {
///             println!("Failed to open {:?}: {e:?}", ddc.name());
///             continue
///         },
///     };
///     let mccs_version = ddc.get_vcp_feature(0xdf).unwrap();
///     println!("MCCS version: {:04x}", mccs_version.maximum());
/// }
/// # }
/// ```
pub struct Enumerator {
    inner: vec::IntoIter<EnumeratedDevice>,
}

impl Enumerator {
    /// Create a new enumerator for available displays.
    pub fn new() -> io::Result<Self> {
        Ok(Enumerator {
            inner: i2c_linux::Enumerator::new()?.iter()?.collect::<Vec<_>>().into_iter(),
        })
    }
}

impl Iterator for Enumerator {
    type Item = EnumeratedI2cDevice;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(dev) = self.inner.next() {
            let name = match dev.udev_device().attribute_value("name") {
                Some(v) => v,
                None => continue,
            };

            // list stolen from ddcutil's ignorable_i2c_device_sysfs_name
            // https://github.com/rockowitz/ddcutil/blob/eff87a481587a03ebf0489aa2d7c897f0fe2d47f/src/util/sysfs_i2c_util.c#L196-L204
            let skip_prefix = [
                "SMBus",
                "Synopsys DesignWare",
                "soc:i2cdsi",
                "smu",
                "mac-io",
                "u4",
                "AMDGPU SMU",
            ];

            if !skip_prefix.iter().any(|p| name.as_bytes().starts_with(p.as_bytes())) {
                return Some(EnumeratedI2cDevice::new(dev))
            }
        }

        None
    }
}

/// A discovered I2C device from a [Udev Enumerator](Enumerator).
pub struct EnumeratedI2cDevice {
    device: EnumeratedDevice,
}

impl EnumeratedI2cDevice {
    /// Manually construct a device.
    pub fn new(device: EnumeratedDevice) -> Self {
        Self { device }
    }

    /// Information about the underlying device.
    pub fn device(&self) -> &EnumeratedDevice {
        &self.device
    }

    /// Information about the underlying device.
    pub fn udev_device(&self) -> &UdevDevice {
        self.device.udev_device()
    }

    /// The human-readable name of the device.
    ///
    /// This usually contains the name of the driver exposing the I2C device.
    pub fn name(&self) -> &OsStr {
        self.udev_device()
            .attribute_value("name")
            .expect("nameless i2c devices are already filtered out")
    }

    /// Open a handle to this device.
    pub fn open(&self) -> io::Result<I2cDeviceDdc> {
        self.device.open().map(I2cDdc::new)
    }

    /// [Open](open) a handle to this device.
    ///
    /// This performs a read to check whether there's a DDC device on the other end.
    pub fn open_checked(&self) -> io::Result<I2cDeviceDdc> {
        let mut i2c = self.open()?;
        let _ = i2c.read_edid(0, &mut [0u8; 0x80])?;
        Ok(i2c)
    }
}
