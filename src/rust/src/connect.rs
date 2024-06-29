use crate::connection::RGlareDbConnection;
use crate::environment::REnvironmentReader;
use crate::error::RGlareDbDatabaseError;
use crate::runtime::GLOBAL_RUNTIME;
use savvy::{savvy, EnvironmentSexp, StringSexp};
use std::collections::HashMap;
use std::sync::Arc;

#[savvy]
pub fn connect(
    cloud_addr: &str,
    disable_tls: bool,
    env: EnvironmentSexp,
    data_dir_or_cloud_url: Option<&str>,
    spill_path: Option<&str>,
    location: Option<&str>,
    storage_options: Option<StringSexp>,
) -> savvy::Result<RGlareDbConnection> {
    let data_dir_or_cloud_url = data_dir_or_cloud_url.map(|s| s.to_string());
    let spill_path = spill_path.map(|s| s.to_string());
    let cloud_addr = cloud_addr.to_string();
    let location = location.map(|s| s.to_string());
    let storage_options = storage_options
        .map(StrageOptions::try_from)
        .transpose()?
        .map(|s| s.options);

    GLOBAL_RUNTIME.0.block_on(async move {
        Ok(RGlareDbConnection {
            inner: Arc::new(
                glaredb::ConnectOptionsBuilder::default()
                    .connection_target(data_dir_or_cloud_url.clone())
                    .set_storage_options(storage_options)
                    .location(location)
                    .spill_path(spill_path)
                    .disable_tls(disable_tls)
                    .cloud_addr(cloud_addr)
                    .client_type(glaredb::ClientType::Rust)
                    .environment_reader(Arc::new(REnvironmentReader::new(env)))
                    .build()
                    .map_err(glaredb::DatabaseError::from)
                    .map_err(RGlareDbDatabaseError::from)?
                    .connect()
                    .await
                    .map_err(RGlareDbDatabaseError::from)?,
            ),
        })
    })
}

#[derive(Clone)]
struct StrageOptions {
    options: HashMap<String, String>,
}

impl TryFrom<StringSexp> for StrageOptions {
    type Error = String;

    fn try_from(chr: StringSexp) -> Result<Self, String> {
        let mut map = HashMap::new();
        let Some(names) = chr.get_names() else {
            return Err("`storage_options` must be a named character vector.".to_string());
        };
        let values = chr.to_vec();
        for (name, value) in names.iter().zip(values.iter()) {
            map.insert(name.to_string(), value.to_string());
        }
        Ok(StrageOptions { options: map })
    }
}
