//! I2C device enumeration.

/// Enumerate Linux I2C devices using [Udev][i2c_linux::enumerate].
#[cfg(all(target_os = "linux", feature = "enumerate-udev"))]
#[cfg_attr(feature = "doc", doc(cfg(all(target_os = "linux", feature = "enumerate-udev"))))]
pub mod udev;

/// Alias for the default enumerator
#[cfg(all(target_os = "linux", feature = "enumerate-udev", feature = "enumerate"))]
pub type Enumerator = self::udev::Enumerator;
