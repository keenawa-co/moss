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
        let _defer = $crate::defer::defer(|| $e);
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
