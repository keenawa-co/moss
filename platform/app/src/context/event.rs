pub unsafe trait Event: Clone + Sized + Send {
    const TYPE_NAME: &'static str;
}
