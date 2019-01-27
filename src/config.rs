//! Coroutine setup

const DEFAULT_STACK_SIZE: usize = 1 * 512 * 1024; // 512k

/// Resume Error
pub enum Error {
    /// Coroutine is panicked
    Panicked,

    /// Coroutine is panicking, carry with the parameter of `panic!()`
    Panicking(Box<Any + Send>),
}


impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Error::Panicked => write!(f, "Panicked"),
            &Error::Panicking(ref err) => {
                let msg = match err.downcast_ref::<&'static str>() {
                    Some(s) => *s,
                    None => {
                        match err.downcast_ref::<String>() {
                            Some(s) => &s[..],
                            None => "Box<Any>",
                        }
                    }
                };
                write!(f, "Panicking({})", msg)
            }
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", error::Error::description(self))
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match self {
            &Error::Panicked => "Panicked",
            &Error::Panicking(..) => "Panicking(..)",
        }
    }
}