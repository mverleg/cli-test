
#[macro_export]
macro_rules! fail {
    ($templ:literal $(, $args:expr)*) => {
        {
            let msg = format!($templ, $($args, )*);
            eprintln!("failed at {}:{} - {}", file!(), line!(), msg);
            return Err(msg)
        }
    }
}

pub use fail;
