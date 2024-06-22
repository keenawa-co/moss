use cocoa::{
    appkit::{
        NSApplication, NSApplicationActivationPolicy::NSApplicationActivationPolicyRegular,
        NSEventModifierFlags, NSMenu, NSMenuItem, NSModalResponse, NSOpenPanel, NSPasteboard,
        NSPasteboardTypeString, NSSavePanel, NSWindow,
    },
    base::{id, nil, selector, BOOL, YES},
    foundation::{
        NSArray, NSAutoreleasePool, NSBundle, NSData, NSInteger, NSProcessInfo, NSString,
        NSUInteger, NSURL,
    },
};
use ptr::null_mut;

use objc::{
    class,
    declare::ClassDecl,
    msg_send,
    runtime::{Class, Object, Sel},
    sel, sel_impl,
};
use parking_lot::Mutex;
use std::{
    cell::Cell,
    convert::TryInto,
    ffi::{c_void, CStr, OsStr},
    os::{raw::c_char, unix::ffi::OsStrExt},
    path::{Path, PathBuf},
    process::Command,
    ptr,
    rc::Rc,
    slice, str,
    sync::Arc,
};

use crate::{
    executor::{BackgroundTaskExecutor, ForegroundTaskExecutor},
    platform::Platform,
};

use super::dispatch::MacDispatcher;

pub struct MacPlatformState {
    background_task_executor: BackgroundTaskExecutor,
    foreground_task_executor: ForegroundTaskExecutor,
    finish_launching: Option<Box<dyn FnOnce()>>,
}

pub(crate) struct MacPlatform(Mutex<MacPlatformState>);

impl Default for MacPlatform {
    fn default() -> Self {
        Self::new()
    }
}

const MAC_PLATFORM_IVAR: &str = "platform";
static mut APP_CLASS: *const Class = ptr::null();
static mut APP_DELEGATE_CLASS: *const Class = ptr::null();

#[ctor::ctor]
unsafe fn build_classes() {
    APP_CLASS = {
        let mut decl = ClassDecl::new("GPUIApplication", class!(NSApplication)).unwrap();
        decl.add_ivar::<*mut c_void>(MAC_PLATFORM_IVAR);
        decl.register()
    };

    APP_DELEGATE_CLASS = {
        let mut decl = ClassDecl::new("GPUIApplicationDelegate", class!(NSResponder)).unwrap();
        decl.add_ivar::<*mut c_void>(MAC_PLATFORM_IVAR);
        decl.add_method(
            sel!(applicationDidFinishLaunching:),
            did_finish_launching as extern "C" fn(&mut Object, Sel, id),
        );
        decl.register()
    }
}

unsafe fn get_mac_platform(object: &mut Object) -> &MacPlatform {
    let platform_ptr: *mut c_void = *object.get_ivar(MAC_PLATFORM_IVAR);
    assert!(!platform_ptr.is_null());
    &*(platform_ptr as *const MacPlatform)
}

extern "C" fn did_finish_launching(this: &mut Object, _: Sel, _: id) {
    unsafe {
        let app: id = msg_send![APP_CLASS, sharedApplication];
        app.setActivationPolicy_(NSApplicationActivationPolicyRegular);
        let platform = get_mac_platform(this);
        let callback = platform.0.lock().finish_launching.take();
        if let Some(callback) = callback {
            callback();
        }
    }
}

impl Platform for MacPlatform {
    fn run(&self, on_finish_launching: Box<dyn 'static + FnOnce()>) {
        self.0.lock().finish_launching = Some(on_finish_launching);

        unsafe {
            let app: id = msg_send![APP_CLASS, sharedApplication];
            let app_delegate: id = msg_send![APP_DELEGATE_CLASS, new];
            app.setDelegate_(app_delegate);

            let self_ptr = self as *const Self as *const c_void;
            (*app).set_ivar(MAC_PLATFORM_IVAR, self_ptr);
            (*app_delegate).set_ivar(MAC_PLATFORM_IVAR, self_ptr);

            let pool = NSAutoreleasePool::new(nil);
            app.run();
            pool.drain();

            (*app).set_ivar(MAC_PLATFORM_IVAR, null_mut::<c_void>());
            (*app.delegate()).set_ivar(MAC_PLATFORM_IVAR, null_mut::<c_void>());
        }
    }

    fn quit(&self) {
        unimplemented!()
    }

    fn background_task_executor(&self) -> BackgroundTaskExecutor {
        self.0.lock().background_task_executor.clone()
    }

    fn foreground_task_executor(&self) -> ForegroundTaskExecutor {
        self.0.lock().foreground_task_executor.clone()
    }
}

impl MacPlatform {
    pub(crate) fn new() -> Self {
        let dispatcher = Arc::new(MacDispatcher::new());
        Self(Mutex::new(MacPlatformState {
            background_task_executor: BackgroundTaskExecutor::new(dispatcher.clone()),
            foreground_task_executor: ForegroundTaskExecutor::new(dispatcher),
            finish_launching: None,
        }))
    }
}
