use crate::connection::RGlareDbConnection;
use crate::environment::REnvironmentReader;
use crate::error::RGlareDbError;
use crate::runtime::GLOBAL_RUNTIME;
use savvy::savvy;
use std::sync::Arc;

// TODO: support optional arguments with savvy > 0.6.3
#[savvy]
pub fn connect(
    // data_dir_or_cloud_url: &str,
    // spill_path: &str,
    // disable_tls: bool,
    cloud_addr: &str,
    // location: &str,
    // storage_options: Option<HashMap<String, String>>, // TODO: support storage options
) -> savvy::Result<RGlareDbConnection> {
    // let data_dir_or_cloud_url = data_dir_or_cloud_url.to_string();
    // let spill_path = spill_path.to_string();
    let cloud_addr = cloud_addr.to_string();
    // let location = location.to_string();

    GLOBAL_RUNTIME.0.block_on(async move {
        Ok(RGlareDbConnection {
            inner: Arc::new(
                glaredb::ConnectOptionsBuilder::default()
                    .connection_target(None)
                    .location(None)
                    .spill_path(None)
                    .disable_tls(false)
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
