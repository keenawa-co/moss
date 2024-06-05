use anyhow::Result;
use parking_lot::Mutex;
use std::{any::TypeId, future::Future, pin::Pin, ptr::NonNull, sync::Arc};

use super::event::Event;

struct Abstract(());

#[derive(Clone)]
pub(super) struct AsyncHook {
    data: NonNull<Abstract>,
    call: unsafe fn(
        NonNull<Abstract>,
        NonNull<Abstract>,
        Option<NonNull<Abstract>>,
    ) -> Pin<Box<dyn Future<Output = Result<()>> + Send>>,
}

unsafe impl Sync for AsyncHook {}
unsafe impl Send for AsyncHook {}

impl AsyncHook {
    pub fn new<E, F, Fut>(hook: F) -> Self
    where
        E: Event + 'static,
        F: Fn(Arc<Mutex<E>>) -> Fut + 'static + Send + Sync,
        Fut: Future<Output = Result<()>> + 'static + Send,
    {
        unsafe fn call<E, F, Fut>(
            hook: NonNull<Abstract>,
            event: NonNull<Abstract>,
            _result: Option<NonNull<Abstract>>,
        ) -> Pin<Box<dyn Future<Output = Result<()>> + Send>>
        where
            E: Event + 'static,
            F: Fn(Arc<Mutex<E>>) -> Fut + 'static + Send + Sync,
            Fut: Future<Output = Result<()>> + 'static + Send,
        {
            let hook: NonNull<F> = hook.cast();
            let event: NonNull<Arc<Mutex<E>>> = event.cast();
            let hook: &F = hook.as_ref();
            let event = event.as_ref().clone();
            Box::pin(hook(event))
        }

        unsafe {
            AsyncHook {
                data: NonNull::new_unchecked(Box::into_raw(Box::new(hook)) as *mut Abstract),
                call: call::<E, F, Fut>,
            }
        }
    }

    pub async fn call<E>(&self, event: Arc<Mutex<E>>) -> Result<()>
    where
        E: Event + 'static,
    {
        unsafe {
            (self.call)(
                self.data,
                NonNull::from(&event).cast(),
                None, // Pass None since we don't need the result
            )
            .await
        }
    }
}

#[derive(Clone)]
pub(super) struct Hook {
    data: NonNull<Abstract>,
    call: unsafe fn(NonNull<Abstract>, NonNull<Abstract>, NonNull<Abstract>),
}

unsafe impl Sync for Hook {}
unsafe impl Send for Hook {}

impl Hook {
    pub fn new<E, F>(hook: F) -> Self
    where
        E: Event + 'static,
        F: Fn(&mut E) -> Result<()>,
    {
        unsafe fn call<E, F>(
            hook: NonNull<Abstract>,
            event: NonNull<Abstract>,
            result: NonNull<Abstract>,
        ) where
            E: Event + 'static,
            F: Fn(&mut E) -> Result<()>,
        {
            let hook: NonNull<F> = hook.cast();
            let mut event: NonNull<E> = event.cast();
            let result: NonNull<Result<()>> = result.cast();
            let hook: &F = hook.as_ref();

            std::ptr::write(result.as_ptr(), hook(event.as_mut()))
        }

        unsafe {
            Hook {
                data: NonNull::new_unchecked(Box::into_raw(Box::new(hook)) as *mut Abstract),
                call: call::<E, F>,
            }
        }
    }

    pub fn call<E: Event>(&self, event: &mut E) -> Result<()> {
        let mut result = Ok(());

        unsafe {
            (self.call)(
                self.data,
                NonNull::from(event).cast(),
                NonNull::from(&mut result).cast(),
            );

            result
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone)]
    struct MyEvent<'a> {
        user: &'a str,
    }

    unsafe impl<'a> super::Event for MyEvent<'a> {
        const TYPE_NAME: &'static str = "MyEvent";
    }

    #[test]
    fn hook_test() {
        let hook_fn = |e: &mut MyEvent| -> Result<()> {
            println!("{}", e.user);

            Ok(())
        };

        let mut test_event = MyEvent { user: "g10z3r" };

        let hook = Hook::new(hook_fn);
        hook.call(&mut test_event).unwrap();
    }

    #[tokio::test]
    async fn async_hooks_run_concurrently() {
        use tokio::time::{sleep, Duration};

        let (tx, rx) = smol::channel::bounded(2);
        let tx1 = tx.clone();
        let tx2 = tx.clone();

        let hook_fn1 = move |_: Arc<Mutex<MyEvent>>| {
            let tx = tx1.clone();
            async move {
                sleep(Duration::from_secs(5)).await;
                tx.send(()).await.unwrap();
                println!("hook_fn1 is done at {}", chrono::Utc::now());
                Ok(())
            }
        };

        let hook_fn2 = move |_: Arc<Mutex<MyEvent>>| {
            let tx = tx2.clone();
            async move {
                sleep(Duration::from_secs(1)).await;
                tx.send(()).await.unwrap();
                println!("hook_fn2 is done at {}", chrono::Utc::now());
                Ok(())
            }
        };

        let test_event1 = Arc::new(Mutex::new(MyEvent { user: "user1" }));
        let test_event2 = Arc::new(Mutex::new(MyEvent { user: "user2" }));

        let hook1 = Arc::new(AsyncHook::new(hook_fn1));
        let hook2 = Arc::new(AsyncHook::new(hook_fn2));

        let hook1_clone = hook1.clone();
        let hook2_clone = hook2.clone();

        let event1_clone = Arc::clone(&test_event1);
        let event2_clone = Arc::clone(&test_event2);

        let handle1 = tokio::spawn(async move {
            hook1_clone.call(event1_clone).await.unwrap();
        });

        let handle2 = tokio::spawn(async move {
            hook2_clone.call(event2_clone).await.unwrap();
        });

        let _ = rx.recv().await;
        let _ = rx.recv().await;

        handle1.await.unwrap();
        handle2.await.unwrap();
    }
}
