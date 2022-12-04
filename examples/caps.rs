use {ddc::Ddc, std::str};

#[cfg(all(target_os = "linux", feature = "i2c-linux"))]
fn main() {
    //::env_logger::init();

    use std::env::args;

    let path = args().nth(1).expect("argument: i2c device path");

    ddc(ddc_i2c::open_linux_device(path).expect("failed to open i2c device"))
}

#[cfg(not(all(target_os = "linux", feature = "i2c-linux")))]
fn main() {
    unimplemented!()
}

fn ddc<D: Ddc>(mut ddc: D)
where
    D::Error: ::std::fmt::Debug,
{
    let caps = ddc.capabilities_string().expect("failed to read ddc capabilities");
    let caps = str::from_utf8(&caps).expect("caps was not a valid string");
    println!("got CAPS: {}", caps);
}
