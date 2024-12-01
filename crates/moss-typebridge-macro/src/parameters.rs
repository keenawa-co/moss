use proc_macro::TokenStream;
use std::path::PathBuf;
use std::str::FromStr;

#[derive(Debug)]
pub(crate) enum OutputLanguage {
    TypeScript,
}

#[derive(Debug)]
pub struct Parameters {
    pub(crate) language: OutputLanguage,
    pub(crate) output_path: PathBuf,
}

struct ParametersBuilder {
    language: Option<OutputLanguage>,
    output_path: Option<PathBuf>,
}

impl FromStr for OutputLanguage {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "typescript" => Ok(OutputLanguage::TypeScript),
            _ => Err(format!("unknown output language: {}", s)),
        }
    }
}

impl ParametersBuilder {
    fn new() -> ParametersBuilder {
        ParametersBuilder {
            language: None,
            output_path: None,
        }
    }

    fn build(self) -> Parameters {
        Parameters {
            language: self.language.unwrap_or(OutputLanguage::TypeScript),
            output_path: self.output_path.unwrap_or(PathBuf::new()),
        }
    }

    fn set_language(&mut self, language: OutputLanguage) {
        self.language = Some(language);
    }

    fn set_output_path(&mut self, output_path: PathBuf) {
        self.output_path = Some(output_path);
    }
}

pub(crate) fn parse_macro_args(args: TokenStream) -> Parameters {
    let args_list = args
        .to_string()
        .split(",")
        .map(|x| x.to_string())
        .collect::<Vec<String>>();
    //println!("{:#?}", args_list);
    let mut parameters_builder = ParametersBuilder::new();
    for arg in args_list {
        let parts = arg
            .split("=")
            .map(|s| s.trim().trim_matches(|c| c == '\'' || c == '"').to_string())
            .collect::<Vec<String>>();
        if parts.len() != 2 {
            continue;
        }
        let keyword = parts.get(0).unwrap().as_str();
        let value = parts.get(1).unwrap().as_str();
        //println!("{} {}", keyword, value);
        match keyword {
            "language" => parameters_builder.set_language(OutputLanguage::from_str(value).unwrap()),
            "output_path" => parameters_builder.set_output_path(PathBuf::from(value)),
            _ => continue,
        }
    }
    parameters_builder.build()
}
