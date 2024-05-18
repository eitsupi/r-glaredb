use crate::connection::RGlareDbConnection;
use crate::environment::REnvironmentReader;
use crate::error::RGlareDbError;
use crate::runtime::GLOBAL_RUNTIME;
use savvy::savvy;
use std::sync::Arc;

#[savvy]
pub fn connect(
    cloud_addr: &str,
    disable_tls: bool,
    data_dir_or_cloud_url: Option<&str>,
    spill_path: Option<&str>,
    location: Option<&str>,
    // storage_options: Option<HashMap<String, String>>, // TODO: support storage options
) -> savvy::Result<RGlareDbConnection> {
    let data_dir_or_cloud_url = data_dir_or_cloud_url.map(|s| s.to_string());
    let spill_path = spill_path.map(|s| s.to_string());
    let cloud_addr = cloud_addr.to_string();
    let location = location.map(|s| s.to_string());

    GLOBAL_RUNTIME.0.block_on(async move {
        Ok(RGlareDbConnection {
            inner: Arc::new(
                glaredb::ConnectOptionsBuilder::default()
                    .connection_target(data_dir_or_cloud_url.clone())
                    .location(location)
                    .spill_path(spill_path)
                    .disable_tls(disable_tls)
                    .cloud_addr(cloud_addr)
                    .client_type(sqlexec::remote::client::RemoteClientType::Rust)
                    .environment_reader(Arc::new(REnvironmentReader))
                    .build()
                    .map_err(RGlareDbError::from)?
                    .connect()
                    .await
                    .map_err(RGlareDbError::from)?,
            ),
        })
    })
}
