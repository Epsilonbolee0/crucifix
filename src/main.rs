#![no_std]
#![no_main]

mod vga_buffer;

use core::panic::PanicInfo;

static LOGO: &str = r#"
        _______  _______           _______  _     _______  _  
       (  ____ \(  ____ )|\     /|(  ____ \( )   (  ____ \( ) |\     /|
       | (    \/| (    )|| )   ( || (    \/| |   | (    \/| | ( \   / )
       | |      | (____)|| |   | || |    __| |__ | (__  __| |__\ (_) / 
       | |      |     __)| |   | || |   (__(@)__)|  __)(__(@)__)) _ (  
       | |      | (\ (   | |   | || |      | |   | (      | |  / ( ) \ 
       | (____/\| ) \ \__| (___) || (____/\| |   | )      | | ( /   \ )
       (_______/|/   \__/(_______)(_______/(_)   |/       (_) |/     \|
"#;
 

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

fn print_logo() {
    let shift: &str = "\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n";
    println!("{}{}", LOGO, shift)
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    print_logo();    
    loop {}
}
