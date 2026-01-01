mod dbus;
mod duration;
mod pacman;
mod random;
mod sentences;

use std::error::Error;
use std::thread;

fn main() -> Result<(), Box<dyn Error>> {
    let welcome_message = sentences::welcome_message();
    println!("Welcome back. {welcome_message}\n");

    let pacman_handle = thread::spawn(|| pacman::time_since_last_pacman_update());
    let dbus_handle = thread::spawn(|| dbus::retrieve_boot_time());

    match pacman_handle.join().expect("pacman thread panicked") {
        Err(e) => eprintln!("checking system updates: {}", e),
        Ok(d) => println!("Last system update: {} ago.", duration::HumanDuration(d)),
    }

    match dbus_handle.join().expect("dbus thread panicked") {
        Err(e) => eprintln!("retrieving boot time: {}", e),
        Ok(d) => println!("boot time: {}.", d),
    }

    Ok(())
}
