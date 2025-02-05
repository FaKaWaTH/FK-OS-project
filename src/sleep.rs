use core::task::Poll;
use crate::{io::rtc, task::executor::yield_now};



pub async fn sleep(secs: u8) {
    let (start_sec, _, _, _, _, _) = rtc::read_rtc();

    loop {
        core::future::poll_fn(|cx| {
            crate::task::executor::WAKE_RTC_TASK.register(cx.waker());
            Poll::Ready(())
        }).await;

        yield_now();

        let (cur_sec, _, _, _, _, _) = rtc::read_rtc();

        let elapsed = if cur_sec >= start_sec {
            cur_sec - start_sec
        } else {
            (60 - start_sec) + cur_sec
        };

        if elapsed >= secs {
            break;
        }
    }
}