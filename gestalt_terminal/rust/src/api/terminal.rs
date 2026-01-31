use portable_pty::{CommandBuilder, MasterPty, NativePtySystem, PtySize, PtySystem};
use std::sync::Mutex;
use std::io::{Read, Write};
use crate::frb_generated::StreamSink;
use anyhow::Result;

static PTY_WRITER: std::sync::LazyLock<Mutex<Option<Box<dyn Write + Send>>>> = std::sync::LazyLock::new(|| Mutex::new(None));
static PTY_MASTER: std::sync::LazyLock<Mutex<Option<Box<dyn MasterPty + Send>>>> = std::sync::LazyLock::new(|| Mutex::new(None));

#[flutter_rust_bridge::frb(sync)]
pub fn resize_terminal(cols: u16, rows: u16) -> Result<()> {
    let mut guard = PTY_MASTER.lock().unwrap();
    if let Some(master) = guard.as_mut() {
        master.resize(PtySize {
            rows,
            cols,
            pixel_width: 0,
            pixel_height: 0,
        })?;
    }
    Ok(())
}

pub fn init_terminal(sink: StreamSink<String>) -> Result<()> {
    let pty_system = NativePtySystem::default();
    let pair = pty_system.openpty(PtySize {
        rows: 24,
        cols: 80,
        pixel_width: 0,
        pixel_height: 0,
    })?;

    #[cfg(target_os = "windows")]
    let cmd = CommandBuilder::new("cmd");
    #[cfg(not(target_os = "windows"))]
    let cmd = CommandBuilder::new("bash"); // Assumes bash is available

    let _child = pair.slave.spawn_command(cmd)?;

    let mut reader = pair.master.try_clone_reader()?;
    let writer = pair.master.take_writer()?;

    // Store master and writer
    {
        let mut guard_writer = PTY_WRITER.lock().unwrap();
        *guard_writer = Some(writer);

        let mut guard_master = PTY_MASTER.lock().unwrap();
        *guard_master = Some(pair.master);
    }

    // Spawn thread to read output
    std::thread::spawn(move || {
        let mut buffer = [0u8; 1024];
        loop {
             match reader.read(&mut buffer) {
                Ok(n) if n > 0 => {
                    let s = String::from_utf8_lossy(&buffer[..n]).to_string();
                    if sink.add(s).is_err() {
                        break;
                    }
                }
                _ => break,
             }
        }
    });

    Ok(())
}

#[flutter_rust_bridge::frb(sync)]
pub fn send_terminal_input(input: String) -> Result<()> {
    let mut guard = PTY_WRITER.lock().unwrap();
    if let Some(writer) = guard.as_mut() {
        write!(writer, "{}", input)?;
    }
    Ok(())
}
