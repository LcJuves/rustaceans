/**
 * Created at 2020/11/16 22:49.
 *
 * @author Liangcheng Juves
 */
use std::thread::sleep;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

fn main() {
    loop {
        print!(
            "\r{}",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs()
        );
        sleep(Duration::new(0, 600));
    }
}
