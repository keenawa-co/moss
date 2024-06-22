use std::{borrow::Borrow, time::Duration};

use app::{
    context::AppContext, context_compact::AppContextCompact, context_model::ModelContext, App,
    AppCompact,
};
use project::Project;

fn main() {
    // return App::new().run(|ctx: &AppContext| {
    //     moss_cli::init(ctx);
    // });

    AppCompact::new().run(|ctx: &mut AppContextCompact| {
        moss_cli::init(ctx);
    })
}
