use lazy_static::lazy_static;
use tokio::runtime::Runtime;

lazy_static! {
    pub static ref TOKIO_RT: Result<Runtime, std::io::Error> = Runtime::new();
}

#[macro_export(local_inner_macros)]
macro_rules! future_block_on {
    ($coroutine:expr) => {
        ($crate::TOKIO_RT.as_ref()?).block_on($coroutine)
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
