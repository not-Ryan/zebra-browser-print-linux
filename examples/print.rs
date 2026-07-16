use cmd_lib::*;
use zebra_browser_print::*;

const ZPL: &str = include_str!("./test.zpl");

fn main() -> CmdResult {
    print_label("zebra_zt410", ZPL).unwrap();
    Ok(())
}
