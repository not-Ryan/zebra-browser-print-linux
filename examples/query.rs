use cmd_lib::*;
use zebra_browser_print::*;
use zpl_toolchain_print_client::{Printer, PrinterConfig, StatusQuery, UsbPrinter};

const OFFSETS: &str = include_str!("./offsets.zpl");
const ZPL: &str = include_str!("./test.zpl");

fn main() -> CmdResult {
    let config = PrinterConfig::default();
    let mut printer = UsbPrinter::find_zebra(config).unwrap();
    dbg!(printer.query_status().unwrap());
    dbg!(printer.query_info().unwrap());

    Ok(())
}
