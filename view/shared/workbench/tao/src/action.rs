use anyhow::Result;

pub trait AnyCommand {
    const ID: &'static str;

    fn execute() -> Result<()>;
}

pub struct HelloCommand {
    value: String,
}

impl AnyCommand for HelloCommand {
    const ID: &'static str = "extension.hello";

    fn execute() -> Result<()> {
        todo!()
    }
}
