use {
    ddc::Edid,
    i2c_linux,
    std::{io, os::unix::ffi::OsStrExt},
    I2cDdc, I2cDeviceDdc,
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
/// extern crate ddc;
/// extern crate ddc_i2c;
///
/// use ddc::Ddc;
/// use ddc_i2c::I2cDeviceEnumerator;
///
/// # fn main() {
/// let displays = I2cDeviceEnumerator::new().unwrap();
/// for mut ddc in displays {
///     let mccs_version = ddc.get_vcp_feature(0xdf).unwrap();
///     println!("MCCS version: {:04x}", mccs_version.maximum());
/// }
/// # }
/// ```
///
/// # Dependency
///
/// Requires the `with-linux-enumerate` feature enabled to use.
pub struct Enumerator {
    inner: i2c_linux::Enumerator,
}

impl Enumerator {
    /// Create a new enumerator for available displays.
    pub fn new() -> io::Result<Self> {
        Ok(Enumerator {
            inner: i2c_linux::Enumerator::new()?,
        })
    }
}

impl Iterator for Enumerator {
    type Item = I2cDeviceDdc;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some((i2c, dev)) = self.inner.next() {
            let name = match dev.attribute_value("name") {
                Some(v) => v,
                None => continue,
            };

            // list stolen from ddcutil's ignorable_i2c_device_sysfs_name
            let skip_prefix = ["SMBus", "soc:i2cdsi", "smu", "mac-io", "u4"];

            if skip_prefix.iter().any(|p| name.as_bytes().starts_with(p.as_bytes())) {
                continue
            }

            let mut i2c = I2cDdc::new(i2c);

            if i2c.read_edid(0, &mut [0u8]).is_err() {
                continue
            }

            return Some(i2c)
        }

        None
    }
}
