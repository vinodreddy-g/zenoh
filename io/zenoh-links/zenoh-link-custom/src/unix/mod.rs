pub mod unicast;

use async_trait::async_trait;
pub use unicast::*;
use zenoh_config::Config;
use zenoh_core::zconfigurable;
use zenoh_link_commons::{ConfigurationInspector, LocatorInspector};
use zenoh_protocol::core::{Locator, Parameters};
use zenoh_result::ZResult;

//pub const UNIXPIPE_LOCATOR_PREFIX: &str = "unixpipe";
pub const CUSTOM_LOCATOR_PREFIX: &str = "custom";

#[derive(Default, Clone, Copy)]
// pub struct UnixPipeLocatorInspector;
pub struct CustomLocatorInspector;

#[async_trait]
impl LocatorInspector for CustomLocatorInspector {
    fn protocol(&self) -> &str {
      CUSTOM_LOCATOR_PREFIX
    }

    async fn is_multicast(&self, _locator: &Locator) -> ZResult<bool> {
        Ok(false)
    }
}

#[derive(Default, Clone, Copy, Debug)]
pub struct CustomConfigurator;
// pub struct UnixPipeConfigurator;

impl ConfigurationInspector<Config> for CustomConfigurator {
    fn inspect_config(&self, config: &Config) -> ZResult<String> {
        let mut properties: Vec<(&str, &str)> = vec![];

        let c = config.transport().link().custom();
        let file_access_mask_;
        if let Some(file_access_mask) = c.file_access_mask() {
            file_access_mask_ = file_access_mask.to_string();
            properties.push((config::FILE_ACCESS_MASK, &file_access_mask_));
        }

        let s = Parameters::from_iter(properties.drain(..));

        Ok(s)
    }
}

zconfigurable! {
    // Default access mask for pipe files
    static ref FILE_ACCESS_MASK: u32 = config::FILE_ACCESS_MASK_DEFAULT;
}

pub mod config {
    pub const FILE_ACCESS_MASK: &str = "file_mask";
    pub const FILE_ACCESS_MASK_DEFAULT: u32 = 0o777;
}
