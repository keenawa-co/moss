// use platform_log::AnyLogger;

use platform_log::log_service::AnyLogger;

pub struct WorkspaceLogService {
    file_logger: Box<dyn AnyLogger>,
    buffer_logger: Box<dyn AnyLogger>,
    tao_logger: Box<dyn AnyLogger>,
}

impl WorkspaceLogService {
    pub fn new(
        file_logger: Box<dyn AnyLogger>, 
        buffer_logger: Box<dyn AnyLogger>, 
        tao_logger: Box<dyn AnyLogger>
    ) -> Self {
        Self {
            file_logger,
            buffer_logger,
            tao_logger,
        }
    }
}