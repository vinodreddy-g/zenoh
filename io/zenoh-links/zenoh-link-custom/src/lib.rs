use std::net::SocketAddr;

use async_trait::async_trait;
use zenoh_core::zconfigurable;
use zenoh_link_commons::LocatorInspector;
use zenoh_protocol::{
    core::{endpoint::Address, Locator},
    transport::BatchSize,
};
use zenoh_result::{zerror, ZResult};

mod unicast;
pub use unicast::*;

// Default MTU (TCP PDU) in bytes.
// NOTE: Since TCP is a byte-stream oriented transport, theoretically it has
//       no limit regarding the MTU. However, given the batching strategy
//       adopted in Zenoh and the usage of 16 bits in Zenoh to encode the
//       payload length in byte-streamed, the TCP MTU is constrained to
//       2^16 - 1 bytes (i.e., 65535).
const CUSTOM_MAX_MTU: BatchSize = BatchSize::MAX;

pub const CUSTOM_LOCATOR_PREFIX: &str = "custom";

#[derive(Default, Clone, Copy)]
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

zconfigurable! {
    // Default MTU (CUSTOM PDU) in bytes.
    static ref CUSTOM_DEFAULT_MTU: BatchSize = CUSTOM_MAX_MTU;
    // The LINGER option causes the shutdown() call to block until (1) all application data is delivered
    // to the remote end or (2) a timeout expires. The timeout is expressed in seconds.
    // More info on the LINGER option and its dynamics can be found at:
    // https://blog.netherlabs.nl/articles/2009/01/18/the-ultimate-so_linger-page-or-why-is-my-tcp-not-reliable
    static ref CUSTOM_LINGER_TIMEOUT: i32 = 10;
    // Amount of time in microseconds to throttle the accept loop upon an error.
    // Default set to 100 ms.
    static ref CUSTOM_ACCEPT_THROTTLE_TIME: u64 = 100_000;
}

pub async fn get_tcp_addrs(address: Address<'_>) -> ZResult<impl Iterator<Item = SocketAddr>> {
    let iter = tokio::net::lookup_host(address.as_str().to_string())
        .await
        .map_err(|e| zerror!("{}", e))?
        .filter(|x| !x.ip().is_multicast());
    Ok(iter)
}
