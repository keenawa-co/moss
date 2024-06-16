use app::context::{App, AppContext};
use std::process::ExitCode;

// fn main() -> ExitCode {
//     let app = App::new();

//     return moss_cli::init(&app);
// }

fn main() {
    return App::new().run(|ctx: &mut AppContext| {
        moss_cli::init(ctx);
    });
}
