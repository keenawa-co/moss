use app::{context::AppContext, App};

fn main() {
    return App::new().run(|ctx: &AppContext| {
        moss_cli::init(ctx);
    });
}
