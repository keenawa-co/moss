use app::{context::AppContext, App};

fn main() {
    return App::new().run(|ctx: &mut AppContext| {
        moss_cli::init(ctx);
    });
}
