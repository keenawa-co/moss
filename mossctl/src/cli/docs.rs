use clap::{Args, Subcommand};
use fs::FS;
use futures::io::Cursor;
use graphql_parser::{
    schema::{parse_schema, Document},
    Style,
};
// use moss_net::sdl;
use std::{path::PathBuf, pin::Pin};

#[derive(Debug, Subcommand)]
pub enum DocsCommandList {
    Schema(SchemaCmdArgs),
}

#[derive(Args, Debug)]
pub struct SchemaCmdArgs {
    /// Specifies the directory and filename where the generated GraphQL schema file should be saved.
    /// This path represents where the file will be stored on the filesystem.
    /// For example, if you specify the default directory 'docs', the schema will be saved
    /// in the 'docs' directory with the filename 'schema.graphql'. You can specify a different directory
    /// according to your file organization requirements.
    #[arg(
        default_value = "docs",
        long = "path",
        help = "The directory and filename where the generated GraphQL schema file should be saved."
    )]
    path: PathBuf,

    /// Specifies the filename for the generated GraphQL schema file. This option allows you to define
    /// a custom filename for the schema file. By default, the filename is 'schema.graphql', but you can
    /// choose any name that suits your project's naming conventions.
    /// For instance, if you are generating a schema for a specific version of your API, you might name
    /// it 'schema_v2.graphql'.
    #[arg(
        default_value = "schema",
        long = "filename",
        help = "The filename for the generated GraphQL schema file, e.g., 'schema_v2.graphql'."
    )]
    filename: String,
}

pub async fn cmd_graphql_schema(
    SchemaCmdArgs { path, filename }: SchemaCmdArgs,
) -> anyhow::Result<()> {
    // let formatted_sdl = {
    //     let raw_sdl = sdl();
    //     let parsed_sdl: Document<String> = parse_schema(&raw_sdl)?;
    //     parsed_sdl.format(&{
    //         let mut s = Style::default();
    //         s.multiline_arguments(false);
    //         s
    //     })
    // };

    // let mut reader = Cursor::new(formatted_sdl.as_bytes());
    // let content_pin = Pin::new(&mut reader);
    // let realfs = fs::real::FileSystem::new();

    // realfs
    //     .create_file_with(&path.join(format!("{filename}.graphql")), content_pin)
    //     .await?;

    Ok(())
}
