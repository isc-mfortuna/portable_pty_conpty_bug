use portable_pty::{native_pty_system, Child, CommandBuilder, PtySize};
use std::{io::Read, io::Write};

fn main() {
    let (mut child, _reader, _writer) = spawn_dll_error();
    println!("Child Process Return: {:?}", child.wait());
    /* // Spawning and waiting on the child process in the same  block fixes the issue
        let pty_system = native_pty_system();
        // Create a new pty
        let mut pair = pty_system
            .openpty(PtySize {
                rows: 24,
                cols: 80,
                // Not all systems support pixel_width, pixel_height,
                // but it is good practice to set it to something
                // that matches the size of the selected font.  That
                // is more complex than can be shown here in this
                // brief example though!
                pixel_width: 0,
                pixel_height: 0,
            })
            .unwrap();
        let mut cmd = CommandBuilder::new("cmd");
        let mut child = pair.slave.spawn_command(cmd).unwrap();
        let mut reader = pair.master.try_clone_reader().unwrap();
        let mut writer = pair.master.take_writer().unwrap();
        (child, reader, writer)
        println!("Child Process Return: {:?}", child.wait());
    }
    */
}
fn spawn_dll_error() -> (
    Box<dyn Child + Send + Sync>,
    Box<dyn Read + Send>,
    Box<dyn Write + Send>,
) {
    let pty_system = native_pty_system();
    // Create a new pty
    let mut pair = pty_system
        .openpty(PtySize {
            rows: 24,
            cols: 80,
            // Not all systems support pixel_width, pixel_height,
            // but it is good practice to set it to something
            // that matches the size of the selected font.  That
            // is more complex than can be shown here in this
            // brief example though!
            pixel_width: 0,
            pixel_height: 0,
        })
        .unwrap();
    let mut cmd = CommandBuilder::new("cmd");
    let mut child = pair.slave.spawn_command(cmd).unwrap();
    let mut reader = pair.master.try_clone_reader().unwrap();
    let mut writer = pair.master.take_writer().unwrap();
    (child, reader, writer)
}
