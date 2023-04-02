
#[macro_export]
macro_rules! fail {
    ($templ:literal $(, $args:expr)*) => {
        {
            use ::log::warn;
            let msg = format!($templ, $($args, )*);
            warn!("fail at {}:{} - {}", file!(), line!(), msg);
            return Err(msg)
        }
    }
}

pub use fail;
