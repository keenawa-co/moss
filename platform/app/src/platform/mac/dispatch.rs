pub(crate) mod dispatch_sys {
    include!("dispatch_sys.rs");
}
use async_task::Runnable;
use objc::{
    class, msg_send,
    runtime::{BOOL, YES},
    sel, sel_impl,
};
use parking::{Parker, Unparker};
use parking_lot::Mutex;
use std::{
    ffi::c_void,
    ptr::{addr_of, NonNull},
    sync::Arc,
    time::Duration,
};

use dispatch_sys::*;

use crate::{executor::TaskLabel, platform::PlatformDispatcher};

extern "C" fn trampoline(runnable: *mut c_void) {
    let task = unsafe { Runnable::<()>::from_raw(NonNull::new_unchecked(runnable as *mut ())) };
    task.run();
}

pub(crate) struct MacDispatcher {
    parker: Arc<Mutex<Parker>>,
}

impl Default for MacDispatcher {
    fn default() -> Self {
        Self::new()
    }
}

impl MacDispatcher {
    pub fn new() -> Self {
        MacDispatcher {
            parker: Arc::new(Mutex::new(Parker::new())),
        }
    }
}

pub(crate) fn dispatch_get_main_queue() -> dispatch_queue_t {
    unsafe { addr_of!(_dispatch_main_q) as *const _ as dispatch_queue_t }
}

impl PlatformDispatcher for MacDispatcher {
    fn is_main_thread(&self) -> bool {
        let is_main_thread: BOOL = unsafe { msg_send![class!(NSThread), isMainThread] };
        is_main_thread == YES
    }

    fn dispatch(&self, runnable: Runnable, _: Option<TaskLabel>) {
        unsafe {
            dispatch_async_f(
                dispatch_get_global_queue(DISPATCH_QUEUE_PRIORITY_HIGH.try_into().unwrap(), 0),
                runnable.into_raw().as_ptr() as *mut c_void,
                Some(trampoline),
            );
        }
    }

    fn dispatch_on_main_thread(&self, runnable: Runnable) {
        let raw_ptr = runnable.into_raw().as_ptr() as *mut c_void;
        println!("Dispatching task to main thread: {:?}", raw_ptr);
        unsafe {
            dispatch_async_f(dispatch_get_main_queue(), raw_ptr, Some(trampoline));
        }
    }

    fn dispatch_after(&self, duration: Duration, runnable: Runnable) {
        unsafe {
            let queue =
                dispatch_get_global_queue(DISPATCH_QUEUE_PRIORITY_HIGH.try_into().unwrap(), 0);
            let when = dispatch_time(DISPATCH_TIME_NOW as u64, duration.as_nanos() as i64);
            dispatch_after_f(
                when,
                queue,
                runnable.into_raw().as_ptr() as *mut c_void,
                Some(trampoline),
            );
        }
    }

    fn park(&self, timeout: Option<Duration>) -> bool {
        if let Some(timeout) = timeout {
            self.parker.lock().park_timeout(timeout)
        } else {
            self.parker.lock().park();
            true
        }
    }

    fn unparker(&self) -> Unparker {
        self.parker.lock().unparker()
    }
}
