use crate::command::{Command, CliOptions};
use crate::command::utils::build;
use crate::config::Config;

#[derive(Debug, Clone)]
pub struct Debug;

impl From<CliOptions> for Debug {
    fn from(cmd: CliOptions) -> Self {
        match cmd {
            CliOptions::Debug => Debug,
            _ => unreachable!(),
        }
    }
}

impl Command for Debug {
    fn run(self) -> anyhow::Result<()> {
        // 1. Package.toml 読み込み
        let config = Config::load("Package.toml")?;

        // 2. ビルド
        let (data, inst) = build(&config)?;
        let data = convert_to_u8(data.split("\n"));
        let inst = convert_to_u8(inst.split("\n"));

        // 3. デバッガ起動
        sb_dbg_tui::run(0, &data, &inst)?;

        Ok(())
    }
}

fn convert_to_u8<'a>(elems: impl Iterator<Item = &'a str>) -> Vec<u8> {
    elems
        .filter(|elem| !elem.is_empty())
        .map(|elem| u8::from_str_radix(elem, 16).unwrap())
        .collect()
}
