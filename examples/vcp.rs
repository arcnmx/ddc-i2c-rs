extern crate ddc;
extern crate ddc_i2c;

use std::env::args;
use ddc::Ddc;

fn ddc<D: Ddc>(mut ddc: D) where
    D::Error: ::std::fmt::Debug,
{
    let mccs_ver = ddc.get_vcp_feature(0xdf).expect("failed to read VCP value");
    println!("MCCS version is {:04x}", mccs_ver.maximum());

    let input = ddc.get_vcp_feature(0x60).expect("failed to read VCP value");
    println!("input is {:?}", input);
}

#[cfg(feature = "i2c-linux")]
fn main() {
    //::env_logger::init();

    let path = args().nth(1).expect("argument: i2c device path");

    ddc(ddc_i2c::from_i2c_device(path).expect("failed to open i2c device"))
}

#[cfg(not(feature = "i2c-linux"))]
fn main() {
    unimplemented!()
}
