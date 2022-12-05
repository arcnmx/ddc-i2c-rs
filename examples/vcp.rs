use {ddc::Ddc, std::env::args};

fn ddc<D: Ddc>(mut ddc: D)
where
    D::Error: ::std::fmt::Debug,
{
    let mccs_ver = ddc.get_vcp_feature(0xdf).expect("failed to read VCP value");
    println!("MCCS version is {:04x}", mccs_ver.value());

    let input = ddc.get_vcp_feature(0x60).expect("failed to read VCP value");
    println!("input is {:?}", input);
}

#[cfg(all(target_os = "linux", feature = "i2c-linux"))]
fn main() {
    //::env_logger::init();

    let path = args().nth(1).expect("argument: i2c device path");

    ddc(ddc_i2c::from_i2c_device(path).expect("failed to open i2c device"))
}

#[cfg(not(all(target_os = "linux", feature = "i2c-linux")))]
fn main() {
    unimplemented!()
}
