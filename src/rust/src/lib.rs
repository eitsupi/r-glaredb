use datafusion_ext::vars::SessionVars;
use extendr_api::prelude::*;
use extendr_api::prelude::*;
use futures::lock::Mutex;
use sqlexec::{
    engine::{Engine, SessionStorageConfig},
    remote::client::RemoteClient,
};
use std::collections::HashMap;
use std::{
    fs,
    path::{Path, PathBuf},
    sync::Arc,
};
use url::Url;

/// Return string `"Hello world!"` to R.
/// @export
#[extendr]
fn hello_world() -> &'static str {
    "Hello world!"
}

// Macro to generate exports.
// This ensures exported functions are registered with R.
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod glaredb;
    fn hello_world;
}

#[derive(Debug, Clone)]
struct RSessionConf {
    /// Where to store both metastore and user data.
    data_dir: Option<PathBuf>,
    /// URL for cloud deployment to connect to.
    cloud_url: Option<Url>,
}

impl From<Option<String>> for RSessionConf {
    fn from(value: Option<String>) -> Self {
        match value {
            Some(s) => match Url::parse(&s) {
                Ok(u) => RSessionConf {
                    data_dir: None,
                    cloud_url: Some(u),
                },
                // Assume failing to parse a url just means the user provided a local path.
                Err(_) => RSessionConf {
                    data_dir: Some(PathBuf::from(s)),
                    cloud_url: None,
                },
            },
            None => RSessionConf {
                data_dir: None,
                cloud_url: None,
            },
        }
    }
}

// #[extendr(use_try_from = true)]
// pub fn connect(
//     data_dir_or_cloud_url: Option<String>,
//     spill_path: Option<String>,
//     disable_tls: bool,
//     cloud_addr: String,
//     location: Option<String>,
//     storage_options: Option<HashMap<String, String>>,
// )
