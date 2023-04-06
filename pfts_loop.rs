use std::thread;
use std::time::Duration;

pub fn pfts_loop(process: fn() -> i32, max_sleep_time: u64, min_sleep_time: u64)
{
    let mut retval: i32;
    loop
    {
        let sleep_thread = thread::spawn(move || { thread::sleep(Duration::from_millis(max_sleep_time)); });
        retval = process();
        thread::sleep(Duration::from_millis(min_sleep_time));
        sleep_thread.join().unwrap();
        if retval == 0
        {
            break;
        }
    }
}