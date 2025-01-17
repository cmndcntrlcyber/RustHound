//! Launch and end banners
use colored::*;
use crate::enums::date::{return_current_date,return_current_time};

/// Banner when RustHound start.
pub fn print_banner() {
    // https://docs.rs/colored/2.0.0/x86_64-pc-windows-msvc/colored/control/fn.set_virtual_terminal.html
    #[cfg(windows)]
    control::set_virtual_terminal(true).unwrap();

    // Banner for RustHound
    println!("{}","---------------------------------------------------".clear().bold());
    println!("Initializing {} at {} on {}",
        "RustHound".truecolor(247,76,0,),
        return_current_time(),
        return_current_date()
    );
    println!("Powered by g0h4n from {}","OpenCyber".truecolor(97,221,179));
    println!("{}\n","---------------------------------------------------".clear().bold());
}

/// Banner when RustHound finish.
pub fn print_end_banner() {
    // End banner for RustHound
    println!("\n{} Enumeration Completed at {} on {}! Happy Graphing!\n",
        "RustHound".truecolor(247,76,0,),
        return_current_time(),
        return_current_date()
    );
}