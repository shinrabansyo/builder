use std::fs;
use std::fs::File;
use std::io::Write;

pub fn convert_raw() -> anyhow::Result<()> {
    fs::create_dir_all("./target/out/raw")?;

    let data = fs::read_to_string("./target/out/hex/data.hex")?;
    let data = data.split("\n").into_iter();
    save_as_binary("./target/out/raw/data.raw", data)?;

    let inst = fs::read_to_string("./target/out/hex/inst.hex")?;
    let inst = inst.split("\n").into_iter();
    save_as_binary("./target/out/raw/inst.raw", inst)?;

    Ok(())
}

fn save_as_binary<'a>(
    path: &str,
    elems: impl Iterator<Item = &'a str>,
) -> anyhow::Result<()> {
    let mut output = File::create(path)?;
    for elem in elems {
        if elem.is_empty() {
            continue;
        }
        let value = u8::from_str_radix(elem, 16).unwrap();
        output.write_all(&[value])?;
    }

    Ok(())
}
