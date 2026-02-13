use std::fs;
use std::io::Write;

pub fn convert_hex_bank() -> anyhow::Result<()> {
    fs::create_dir_all("./target/out/hex_bank")?;

    let data = fs::read_to_string("./target/out/hex/data.hex")?;
    let data = data.split("\n").into_iter();
    save_banked::<4>("./target/out/hex_bank/data", data)?;

    let inst = fs::read_to_string("./target/out/hex/inst.hex")?;
    let inst = inst.split("\n").into_iter();
    save_banked::<6>("./target/out/hex_bank/inst", inst)?;

    Ok(())
}

fn save_banked<'a, const N: usize> (
    prefix: &str,
    elems: impl Iterator<Item = &'a str>,
) -> anyhow::Result<()> {
    let mut outputs = vec![];
    for idx in 0..N {
        let path = format!("{}_{}.hex", prefix, idx);
        outputs.push(fs::File::create(path)?);
    }

    for (idx, elem) in elems.enumerate() {
        outputs[idx % N].write(elem.as_bytes())?;
        outputs[idx % N].write(b"\n")?;
    }

    Ok(())
}
