use {
    ddc::Edid,
    std::{env::args, io, path::Path},
};

#[cfg(all(target_os = "linux", feature = "i2c-linux"))]
fn edid<P: AsRef<Path>>(path: P) -> io::Result<()> {
    let path = path.as_ref();

    println!("Opening {}", path.display());

    ddc(ddc_i2c::open_linux_device(path)?)
}

#[cfg(not(all(target_os = "linux", feature = "i2c-linux")))]
fn edid<P: AsRef<Path>>(_path: P) -> io::Result<()> {
    unimplemented!()
}

fn ddc<D: Edid>(mut ddc: D) -> Result<(), D::EdidError>
where
    D::EdidError: ::std::fmt::Debug,
    D::EdidError: From<io::Error>,
{
    let mut edid = [0u8; 0x80];
    let len = ddc.read_edid(0, &mut edid)?;

    let edid = edid::parse(&edid[..len])
        .to_result()
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))?;
    println!("EDID: {:#?}", edid);

    Ok(())
}

fn main() {
    //::env_logger::init();

    let path = args().nth(1);

    match path {
        Some(path) => edid(path).expect("failed to get EDID"),
        #[cfg(all(target_os = "linux", feature = "enumerate-udev"))]
        None => ddc_i2c::Enumerator::new()
            .expect("failed to enumerate DDC devices")
            .for_each(|i2c| match i2c.open().and_then(|i2c| ddc(i2c)) {
                Ok(()) => (),
                Err(e) => println!("Failure: {:?}", e),
            }),
        #[allow(unreachable_patterns)]
        _ => panic!("argument: i2c device path"),
    }
}
