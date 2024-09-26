use serde::{Deserialize};

#[derive(Deserialize, Debug)]
pub struct Config {
    pub jenkins: JenkinsConfig,
    pub apps: Vec<AppConfig>,
}

#[derive(Deserialize, Debug)]
pub struct JenkinsConfig {
    #[serde(rename = "jenkinsServerUrl")]
    pub jenkins_server_url: String,
    #[serde(rename = "jenkinsUserName")]
    pub jenkins_user_name: String,
    #[serde(rename = "jenkinsApiToken")]
    pub jenkins_api_token: String,
    #[serde(rename = "fileServerUrl")]
    pub file_server_url: String,
    #[serde(rename = "replaceFileParentDir")]
    pub replace_file_parent_dir: String,
}

#[derive(Deserialize, Debug)]
pub struct AppConfig {
    #[serde(rename = "exploedWarPath")]
    pub exploed_war_path: String,
    #[serde(rename = "homePath")]
    pub home_path: String,
    #[serde(rename = "name")]
    pub name: String,
}

