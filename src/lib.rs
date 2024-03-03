use std::ffi::CStr;
use std::process::{Command, Output};

static PLUGIN_NAME: &CStr = unsafe { CStr::from_bytes_with_nul_unchecked(b"apt\0") };

#[no_mangle]
pub extern "C" fn plugin_name() -> *const libc::c_char { PLUGIN_NAME.as_ptr() }

#[no_mangle]
pub extern "C" fn upgrade_all_api_version() -> u32 { 1 }

#[no_mangle]
pub extern "C" fn upgrade_all_api_v1() -> () {
    get_upgradable_packages();

    // TODO: get apt percentage as it's running and report it (or use a progress bar)
    let apt_upgrade = Command::new("apt").arg("upgrade")
        .output()
        .expect("Error: Could not run 'apt upgrade'!");

    verify_exit_code(apt_upgrade);
}

#[no_mangle]
pub extern "C" fn bulk_install_api_version () -> u32 { 1 }

#[no_mangle]
pub extern "C" fn bulk_install_api_v1(
    packages: *const *const libc::c_char, num_packages: usize
) -> bool {
    let mut command_base = Command::new("apt");
    let mut command = command_base.arg("install");
    
    for i in 0..num_packages { // FIXME - this is not getting the correct value (segfault)
        eprintln!("{i}");
        let package_c_str = unsafe { *packages.offset(i as isize) };
        let package = unsafe { CStr::from_ptr(package_c_str).to_string_lossy() };
        let package_str = package.as_ref();
        command = command.arg(package_str);
    }

    let command_string = format!("B {:?}", command);
    
    // Print the command string to stderr
    eprintln!("{}", command_string);

    if let Ok(_) = command.output() {
        true
    } else {
        false
    }
}

fn get_upgradable_packages() -> () {
    let apt_update = Command::new("apt").arg("update")
        .output()
        .expect("Error: Could not run 'apt update'");

    verify_exit_code(apt_update);

    // TODO
    //let apt_list_upgradable = Command::new("apt").arg("list").arg("--upgradable");
}

fn verify_exit_code(output: Output) -> () {
    let exit_code = output.status.code()
        .expect("Error: Could not find exit code of 'apt update'!");

    if exit_code != 0 {
        panic!("Error: command failed with status code {}", exit_code);
    }
}
