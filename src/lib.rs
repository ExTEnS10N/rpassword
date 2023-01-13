//! This library makes it easy to read passwords in a console application on all platforms, Unix,
//! Windows, WASM, etc.
//!
//! Here's how you can read a password:
//! 
//! ```no_run
//! let password = rpassword::read_password().unwrap();
//! println!("Your password is {}", password);
//! 
//! // It is recommended to secure clear your password if it's not needed anymore. 
//! // You can use crates like zeroize or secstr to achive this.
//! use zeroize::Zeroize;
//! password.zeroize();
//! ```
//! 
//! With prompt, retry and securely consume the password:
//! ```rust
//! use std::io::{Error, ErrorKind};
//! let res = rpassword::ask_password("Enter your password:", |password: &str| {
//!     // consume your password here, and make sure the code SHALL NOT PANIC here!

//!     if password.isEmpty() { // if password is wrong, return PermissionDenied to tell user to try again
//!         println!("Wrong password, please try again.");
//!         // This will cause password to be ask again automatically, and the wrong one will be securely zeroize too.
//!         Err(Error::from(ErrorKind::PermissionDenied)) 
//!     }

//!     Ok(()) // You can return any other thing you need other than the unit type ().
//! });
//! //Password has already securely zeroized here, no matter whether it is wrong or not, if nothing panic above.

//! match res {
//!     // if user has retried for 3 times, ask_password will not continue retrying and return this error.
//!     Err(error) if error.kind() == ErrorKind::PermissionDenied => { 
//!         panic!(); // now you can panic!() if needed.
//!     },
//!     Err(error) => {
//!         // Any other error will cause ask_password to exit without retry.
//!     }
//!     _ => {}
//! };
//! ```

use zeroize::Zeroize;

#[cfg(target_family = "unix")]
mod unix {
    use libc::{c_int, tcsetattr, termios, ECHO, ECHONL, TCSANOW};
    use zeroize::Zeroize;
    use std::io::{self, BufRead, Write, Error, ErrorKind};
    use std::mem;
    use std::os::unix::io::AsRawFd;

    /// Turns a C function return into an IO Result
    fn io_result(ret: c_int) -> std::io::Result<()> {
        match ret {
            0 => Ok(()),
            _ => Err(std::io::Error::last_os_error()),
        }
    }

    fn safe_tcgetattr(fd: c_int) -> std::io::Result<termios> {
        let mut term = mem::MaybeUninit::<termios>::uninit();
        io_result(unsafe { ::libc::tcgetattr(fd, term.as_mut_ptr()) })?;
        Ok(unsafe { term.assume_init() })
    }

    /// Reads a password from the TTY
    pub fn read_password() -> std::io::Result<String> {
        let tty = std::fs::File::open("/dev/tty")?;
        let fd = tty.as_raw_fd();
        let mut reader = io::BufReader::new(tty);

        let mut password = String::new();

        // Make two copies of the terminal settings. The first one will be modified
        // and the second one will act as a backup for when we want to set the
        // terminal back to its original state.
        let mut term = safe_tcgetattr(fd)?;
        let term_orig = safe_tcgetattr(fd)?;

        // Hide the password. This is what makes this function useful.
        term.c_lflag &= !ECHO;

        // But don't hide the NL character when the user hits ENTER.
        term.c_lflag |= ECHONL;

        // Save the settings for now.
        io_result(unsafe { tcsetattr(fd, TCSANOW, &term) })?;

        reader.read_line(&mut password)?;

        // Set the the mode back to normal
        unsafe { tcsetattr(fd, TCSANOW, &term_orig); }

        super::fix_line_issues(password)
    }

    pub fn ask_password<F, T>(prompt: &str, consume: F) -> Result<T, Error>
    where 
        F: Fn(&str) -> Result<T, Error>,
    {
        for _ in 0..3 {
            print!("{}", prompt);
            std::io::stdout().flush().ok();
            let read_result = read_password();
            match read_result {
                Ok(mut password) => {
                    let res = consume(&password);
                    password.zeroize();
                    match res {
                        Err(error) if error.kind() == ErrorKind::PermissionDenied => continue,
                        _ => {},
                    }
                    return res;
                },
                Err(_) => {continue;}
            }
        }
        return Err(Error::from(ErrorKind::PermissionDenied));
    }
}

#[cfg(target_family = "unix")]
pub use unix::*;

/// Normalizes the return of `read_line()` in the context of a CLI application
pub fn fix_line_issues(mut line: String) -> std::io::Result<String> {
    if !line.ends_with('\n') {
        line.zeroize();
        return Err(std::io::Error::from(
            std::io::ErrorKind::UnexpectedEof,
        ));
    }

    // Remove the \n from the line.
    line.pop();

    // Remove the \r from the line if present
    if line.ends_with('\r') {
        line.pop();
    }

    Ok(line)
}
