use std::thread;
use std::time::Duration;

pub fn pfts_loop(process: fn() -> Option<()>, max_sleep_time: u64, min_sleep_time: u64) {
    loop {
        let sleep_thread = thread::spawn(move || {
            thread::sleep(Duration::from_millis(max_sleep_time));
        });

        let retval = process();
        thread::sleep(Duration::from_millis(min_sleep_time));
        sleep_thread.join().unwrap();

        if let None = retval {
            break;
        }
    }
}
