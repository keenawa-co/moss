use platform_core::platform::AnyPlatform;
use std::rc::Rc;

pub fn current_platform() -> Rc<dyn AnyPlatform> {
    use platform_core::platform::cross::client::CrossPlatformClient;

    Rc::new(CrossPlatformClient::new())
}
