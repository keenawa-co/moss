use std::time::Duration;

use app::{context::AppContext, context_compact::AppContextCompact, App, AppCompact};

fn main() {
    // return App::new().run(|ctx: &AppContext| {
    //     moss_cli::init(ctx);
    // });

    AppCompact::new().run(|ctx: &mut AppContextCompact| {
        moss_cli::init(ctx);
    })
}
