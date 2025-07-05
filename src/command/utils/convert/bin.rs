use std::fs;
use std::fs::File;
use std::io::Write;

pub fn convert_bin() -> anyhow::Result<()> {
    fs::create_dir_all("./target/out/bin")?;

    let data = fs::read_to_string("./target/out/hex/data.hex")?;
    let data = data
        .split("\n")
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>();

    let inst = fs::read_to_string("./target/out/hex/inst.hex")?;
    let inst = inst
        .split("\n")
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>();

    let mut bin_file = File::create("./target/out/bin/out.bin")?;
    write_header(&mut bin_file, data.len() as u32, inst.len() as u32)?;
    write_body(&mut bin_file, &data)?;
    write_body(&mut bin_file, &inst)?;

    Ok(())
}

fn write_header(
    f: &mut impl Write,
    data_size: u32,
    inst_size: u32,
) -> anyhow::Result<()> {
    // マジックナンバー
    f.write_all(&[0x53, 0x45, 0x4c, 0x46])?;

    // バージョン番号
    f.write_all(&[0x00, 0x00, 0x00, 0x00])?;

    // データ領域のサイズ
    println!("data_size: {}, {:?}", data_size, data_size.to_le_bytes());
    f.write_all(&data_size.to_le_bytes())?;

    // 命令領域のサイズ
    println!("inst_size: {}, {:?}", inst_size, inst_size.to_le_bytes());
    f.write_all(&inst_size.to_le_bytes())?;

    Ok(())
}

fn write_body(f: &mut impl Write, elems: &[&str]) -> anyhow::Result<()> {
    for elem in elems {
        let value = u8::from_str_radix(elem, 16).unwrap();
        f.write_all(&[value])?;
    }
    Ok(())
}
