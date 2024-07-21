use anyhow::{Context as AnyhowContext, Result};
use hashbrown::HashMap;
use serde_json::Value;
use std::{
    fs::{File, OpenOptions},
    io::{Read, Seek, SeekFrom, Write},
    path::{Path, PathBuf},
    sync::Arc,
};
use tokio::sync::mpsc::UnboundedSender;

use super::{
    configuration_default::DefaultConfiguration,
    configuration_model::{
        Configuration, ConfigurationModel, ConfigurationParser, UserConfiguration,
    },
    configuration_registry::ConfigurationRegistry,
    AbstractConfigurationService,
};

pub struct ConfigurationService {
    default_configuration: DefaultConfiguration,
    user_configuration: UserConfiguration,
    configuration: Configuration,
    configuration_editing: ConfigurationEditingService,
}

impl ConfigurationService {
    pub fn new(registry: Arc<ConfigurationRegistry>, config_file_path: &PathBuf) -> Result<Self> {
        let parser = ConfigurationParser::new(Arc::clone(&registry));
        let user_configuration = UserConfiguration::new(config_file_path, Arc::new(parser));

        let default_configuration = DefaultConfiguration::new(Arc::clone(&registry));
        default_configuration.initialize();

        let user_configuration_model = user_configuration
            .load_configuration()
            .context("failed to load user configuration model")?;
        let default_configuration_model = default_configuration
            .get_configuration_model()
            .context("failed to get default configuration model".to_string())
            .context("default was not initialized correctly")?;

        let configuration = Configuration::new(
            default_configuration_model,
            user_configuration_model,
            ConfigurationModel::empty(),
            ConfigurationModel::empty(),
        );

        let configuration_editing = ConfigurationEditingService::new(config_file_path.clone());

        Ok(Self {
            default_configuration,
            user_configuration,
            configuration,
            configuration_editing,
        })
    }
}

impl AbstractConfigurationService for ConfigurationService {
    fn get_value(&self, key: &str, overrider_identifier: Option<&str>) -> Option<Value> {
        self.configuration.get_value(key, overrider_identifier)
    }

    // TODO: use type Keyable for key
    fn update_value(&self, key: &str, value: serde_json::Value) -> Result<()> {
        Ok(self.configuration_editing.write(key.to_string(), value)?)
    }
}

#[derive(Debug)]
pub struct ConfigurationEditingService {
    edited_resource: PathBuf,
    write_queue: UnboundedSender<ConfigurationWriteJob>,
}

#[derive(Debug)]
pub struct ConfigurationWriteJob {
    path: String, // JSON Path
    value: serde_json::Value,
    resource: PathBuf,
}

impl ConfigurationEditingService {
    fn new(edited_resource: PathBuf) -> Self {
        let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<ConfigurationWriteJob>();

        // TODO: !!! GET RID OF THIS DISGRACE !!!
        // Replace with using an async job queue once it is implemented.
        tokio::spawn(async move {
            while let Some(job) = rx.recv().await {
                Self::do_write_job(job);
            }
        });

        Self {
            edited_resource,
            write_queue: tx,
        }
    }

    fn write(&self, path: String, value: serde_json::Value) -> Result<()> {
        Ok(self.write_queue.send(ConfigurationWriteJob {
            path,
            value,
            resource: self.edited_resource.clone(),
        })?)
    }

    fn do_write_job(job: ConfigurationWriteJob) {
        // TODO: Implement error handling when the event module is implemented
        if let Ok(mut file) = OpenOptions::new().read(true).write(true).open(job.resource) {
            let mut content = String::new();
            file.read_to_string(&mut content).expect("read file");

            let mut json: Value = serde_json::from_str(&content).expect("read content");
            if let Some(obj) = json.as_object_mut() {
                obj.insert(job.path, job.value);
            }

            file.seek(SeekFrom::Start(0)).expect("seek to start");
            file.set_len(0).expect("truncate file");

            let json_string = serde_json::to_string_pretty(&json).expect("write content");
            file.write_all(json_string.as_bytes()).expect("write");
        };
    }
}
