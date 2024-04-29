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
