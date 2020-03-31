//! Oasis runtime entry point.
extern crate oasis_runtime;

extern crate ethcore;
extern crate ethereum_types;
extern crate failure;
extern crate io_context;
extern crate oasis_core_keymanager_client;
extern crate oasis_core_runtime;
extern crate oasis_runtime_api;
extern crate oasis_runtime_common;
extern crate serde_bytes;

use std::sync::Arc;

use oasis_core_runtime::{
    common::version::Version, rak::RAK, version_from_cargo, Protocol, RpcDemux, RpcDispatcher,
    TxnDispatcher,
};
use oasis_runtime::dispatcher::Dispatcher;
use oasis_runtime_keymanager::trusted_policy_signers;

fn main() {
    // Initializer.
    let init = |protocol: &Arc<Protocol>,
                rak: &Arc<RAK>,
                _rpc_demux: &mut RpcDemux,
                _rpc: &mut RpcDispatcher|
     -> Option<Box<dyn TxnDispatcher>> {
        // Create the key manager client.
        let km_client = Arc::new(oasis_core_keymanager_client::RemoteClient::new_runtime(
            protocol.get_runtime_id(),
            protocol.clone(),
            rak.clone(),
            1024, // TODO: How big should this cache be?
            trusted_policy_signers(),
        ));

        Some(Box::new(Dispatcher::new(km_client)))
    };

    // Start the runtime.
    oasis_core_runtime::start_runtime(Box::new(init), version_from_cargo!());
}
