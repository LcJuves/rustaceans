#[macro_export(local_inner_macros)]
macro_rules! time_millis_string {
    () => {
        (|| -> Result<String, std::time::SystemTimeError> {
            use std::time::{SystemTime, UNIX_EPOCH};
            Ok((SystemTime::now().duration_since(UNIX_EPOCH)?).as_millis().to_string())
        })()?
    };
}

#[macro_export(local_inner_macros)]
macro_rules! seeval {
    ($val:expr) => {
        #[cfg(debug_assertions)]
        std::println!("{} >>> {:?}", core::stringify!($val), $val);
    };
}

#[allow(unused_macros)]
#[macro_export(local_inner_macros)]
macro_rules! pass {
    () => {
        std::println!("\x1b[91m{}\x1b[0m", ">>> PASS");
    };
}
