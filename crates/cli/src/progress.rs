use std::io::{self, IsTerminal, Write};
use std::sync::mpsc::{self, Receiver, RecvTimeoutError, Sender};
use std::thread::{self, JoinHandle};
use std::time::Duration;

use anstream::{AutoStream, ColorChoice};
use anstyle::Style;

const INITIAL_DELAY: Duration = Duration::from_millis(150);
const FRAME_INTERVAL: Duration = Duration::from_millis(80);
const FRAMES: [char; 4] = ['|', '/', '-', '\\'];

/// A spinner shown on standard error while a slow operation runs.
///
/// It never appears when standard error is not a terminal, or when the
/// operation finishes before the initial delay.
#[derive(Debug)]
pub(crate) struct Progress {
    stop: Option<Sender<()>>,
    handle: Option<JoinHandle<()>>,
}

impl Progress {
    /// Start showing the spinner with the given label.
    #[must_use]
    pub(crate) fn start(label: String, color: ColorChoice) -> Self {
        if !io::stderr().is_terminal() {
            return Self {
                stop: None,
                handle: None,
            };
        }

        let (stop, receiver) = mpsc::channel();
        let handle = thread::spawn(move || show(&receiver, &label, color));

        Self {
            stop: Some(stop),
            handle: Some(handle),
        }
    }
}

impl Drop for Progress {
    fn drop(&mut self) {
        if let Some(stop) = self.stop.take() {
            let _send_result = stop.send(());
        }

        if let Some(handle) = self.handle.take() {
            let _join_result = handle.join();
        }
    }
}

fn show(receiver: &Receiver<()>, label: &str, color: ColorChoice) {
    if receiver.recv_timeout(INITIAL_DELAY).is_ok() {
        return;
    }

    let style = Style::new().dimmed();
    let clearance = " ".repeat(label.len() + 3);
    let mut stderr = AutoStream::new(io::stderr(), color).lock();

    for frame in FRAMES.iter().cycle() {
        match receiver.recv_timeout(FRAME_INTERVAL) {
            Ok(()) | Err(RecvTimeoutError::Disconnected) => break,
            Err(RecvTimeoutError::Timeout) => {
                let _write_result = write!(
                    stderr,
                    "\r{frame} {}{label}{}",
                    style.render(),
                    style.render_reset()
                );
                let _flush_result = stderr.flush();
            }
        }
    }

    let _clear_result = write!(stderr, "\r{clearance}\r");
    let _clear_flush_result = stderr.flush();
}
