extern crate ddc;
extern crate ddc_i2c;
extern crate edid;

use std::path::Path;
use std::env::args;
use std::io;
use ddc::Edid;

#[cfg(feature = "i2c-linux")]
fn edid<P: AsRef<Path>>(path: P) -> io::Result<()> {
    let path = path.as_ref();

    println!("Opening {}", path.display());

    ddc(ddc_i2c::from_i2c_device(path)?)
}

#[cfg(not(feature = "i2c-linux"))]
fn edid<P: AsRef<Path>>(_path: P) -> io::Result<()> {
    unimplemented!()
}

fn ddc<D: Edid>(mut ddc: D) -> Result<(), D::EdidError> where
    D::EdidError: ::std::fmt::Debug,
    D::EdidError: From<io::Error>,
{
    let mut edid = [0u8; 0x80];
    let len = ddc.read_edid(0, &mut edid)?;

    let edid = edid::parse(&edid[..len]).to_result()
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))?;
    println!("EDID: {:#?}", edid);

    Ok(())
}

fn main() {
    //::env_logger::init();

    let path = args().nth(1);

    match path {
        Some(path) => edid(path).expect("failed to get EDID"),
        #[cfg(feature = "with-linux-enumerate")]
        None => ddc_i2c::I2cDeviceEnumerator::new().expect("failed to enumerate DDC devices").for_each(|i2c| match ddc(i2c) {
            Ok(()) => (),
            Err(e) => println!("Failure: {:?}", e),
        }),
        #[cfg(not(feature = "with-linux-enumerate"))]
        None => panic!("argument: i2c device path"),
    }
}
