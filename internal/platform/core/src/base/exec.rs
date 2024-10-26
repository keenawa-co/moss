use core::mem::ManuallyDrop;

/// Executes a provided function when the returned `Defer` guard object is dropped.
/// This is typically used to ensure that cleanup code is executed when a scope is exited,
/// either normally or via a panic.
///
/// # Type Parameters
/// - `F`: The type of the closure to execute. Must implement `FnOnce()`.
///
/// # Parameters
/// - `f`: A closure of type `F` that is called when the `Defer` object is dropped.
///
/// # Returns
/// - A `Defer<F>` guard object that calls the provided closure upon being dropped.
///
/// # Examples
/// ```
/// use your_crate_name::defer;
///
/// fn main() {
///     let resource = "Resource allocation here";
///     let _defer = defer(|| println!("Cleanup code for {}", resource));
///     println!("Main logic here");
///     // When `_defer` is dropped at the end of `main`, the closure is called.
/// }
/// ```
///
/// The function ensures that the cleanup code is run no matter how the scope is exited,
/// which is particularly useful for managing resources or handling cleanup tasks.

pub fn defer<F>(f: F) -> impl Drop
where
    F: FnOnce(),
{
    struct Defer<F: FnOnce()>(ManuallyDrop<F>);

    impl<F: FnOnce()> Drop for Defer<F> {
        fn drop(&mut self) {
            let f: F = unsafe { ManuallyDrop::take(&mut self.0) };
            f();
        }
    }

    Defer(ManuallyDrop::new(f))
}

/// A convenience macro to create a `Defer` object for running cleanup or other
/// tasks when the current scope is exited. This macro simplifies the usage of
/// the `defer` function by automatically creating a `Defer` object.
///
/// # Syntax
/// `defer!(expression);`
///
/// # Parameters
/// - `expression`: An expression or closure to execute upon scope exit.
///
/// # Examples
/// ```
/// use your_crate_name::defer;
///
/// fn main() {
///     defer!(println!("This will be printed on scope exit"));
///     println!("This is printed first");
/// }
/// ```
///
/// Using the `defer!` macro, you can ensure that specific actions are taken
/// at the end of a scope, similar to destructors or finally blocks in other languages.

#[macro_export]
macro_rules! defer {
    ($e:expr) => {
        let _defer = $crate::base::exec::defer(|| $e);
    };
}

#[test]
fn test() {
    use core::cell::RefCell;

    let i = RefCell::new(0);

    {
        let _d = defer(|| *i.borrow_mut() += 1);
        assert_eq!(*i.borrow(), 0);
    }

    assert_eq!(*i.borrow(), 1);
}

#[test]
fn test_macro() {
    use core::cell::RefCell;

    let i = RefCell::new(0);
    let k = RefCell::new(0);

    {
        defer!(*i.borrow_mut() += 1);
        defer!(*k.borrow_mut() += 1);
        assert_eq!(*i.borrow(), 0);
        assert_eq!(*k.borrow(), 0);
    }

    assert_eq!(*i.borrow(), 1);
    assert_eq!(*k.borrow(), 1);
}

/// `maybe!` macro facilitates the creation and immediate invocation of a function expression.
/// This is particularly useful for scoping, utilizing the `?` operator in contexts where
/// it's typically not allowed, and simplifying asynchronous code execution.
///
/// ## Variants
/// - `maybe!($block)` — Immediately invokes a synchronous function block.
/// - `maybe!(async $block)` — Immediately invokes an asynchronous function block.
/// - `maybe!(async move $block)` — Immediately invokes an asynchronous function block
///    that takes ownership of the captured variables.
///
/// ## Examples
///
/// ### Synchronous Usage
/// You can use `maybe!` to scope variables and utilize the `?` operator within a context
/// that does not directly support it, like so:
///
/// ```
/// # fn function_that_might_fail() -> Result<i32, &'static str> { Ok(42) }
/// let result: Result<i32, &'static str> = maybe!({
///     let intermediate_result = function_that_might_fail()?;
///     Ok(intermediate_result * 2)
/// });
/// assert_eq!(result, Ok(84));
/// ```
///
/// ### Asynchronous Usage
/// For asynchronous code, `maybe!` allows the inclusion of `await` within contexts that are not
/// inherently asynchronous:
///
/// ```
/// # async fn fetch_data() -> Result<i32, &'static str> { Ok(42) }
/// # async fn async_main() -> Result<i32, &'static str> {
/// let result: Result<i32, &'static str> = maybe!(async {
///     let data = fetch_data().await?;
///     Ok(data * 2)
/// });
/// assert_eq!(result.await, Ok(84));
/// # Ok(())
/// # }
/// # async_main();
/// ```
///
/// ### Using `async move`
/// When you need to capture variables by value into the async block, `maybe!(async move { ... })`
/// can be used. This is useful when variables need to be moved rather than referenced:
///
/// ```
/// # async fn process_data(data: i32) -> Result<i32, &'static str> { Ok(data * 2) }
/// let value = 42;
/// let result: Result<i32, &'static str> = maybe!(async move {
///     process_data(value).await
/// });
/// # async fn assert_async() {
/// assert_eq!(result.await, Ok(84));
/// # }
/// # assert_async();
/// ```
///
/// This macro is a powerful tool in the Rust ecosystem for handling scopes, asynchronous code,
/// and error propagation cleanly and effectively within various coding contexts.
#[macro_export]
macro_rules! maybe {
    ($block:block) => {
        (|| $block)()
    };
    (async $block:block) => {
        (|| async $block)()
    };
    (async move $block:block) => {
        (|| async move $block)()
    };
}
