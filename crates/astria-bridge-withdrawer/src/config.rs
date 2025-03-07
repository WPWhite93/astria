use serde::{
    Deserialize,
    Serialize,
};

// Allowed `struct_excessive_bools` because this is used as a container
// for deserialization. Making this a builder-pattern is not actionable.
#[allow(clippy::struct_excessive_bools)]
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
/// The single config for creating an astria-bridge service.
pub struct Config {
    // The cometbft rpc endpoint for submitting transactions to the sequencer.
    pub sequencer_cometbft_endpoint: String,
    // The chain id of the sequencer chain.
    pub sequencer_chain_id: String,
    // The path to the private key used to sign transactions submitted to the sequencer.
    pub sequencer_key_path: String,
    // The fee asset denomination to use for the bridge account's transactions.
    pub fee_asset_denomination: String,
    // The asset denomination being withdrawn from the rollup.
    pub rollup_asset_denomination: String,
    // The address of the AstriaWithdrawer contract on the evm rollup.
    pub ethereum_contract_address: String,
    // The rpc endpoint of the evm rollup.
    pub ethereum_rpc_endpoint: String,
    // The socket address at which the bridge service will server healthz, readyz, and status
    // calls.
    pub api_addr: String,
    pub log: String,
    /// Forces writing trace data to stdout no matter if connected to a tty or not.
    pub force_stdout: bool,
    /// Disables writing trace data to an opentelemetry endpoint.
    pub no_otel: bool,
    /// Set to true to disable the metrics server
    pub no_metrics: bool,
    /// The endpoint which will be listened on for serving prometheus metrics
    pub metrics_http_listener_addr: String,
    /// Writes a human readable format to stdout instead of JSON formatted OTEL trace data.
    pub pretty_print: bool,
}

impl config::Config for Config {
    const PREFIX: &'static str = "ASTRIA_BRIDGE_WITHDRAWER_";
}

#[cfg(test)]
mod tests {
    use super::Config;

    const EXAMPLE_ENV: &str = include_str!("../local.env.example");

    #[test]
    fn example_env_config_is_up_to_date() {
        config::tests::example_env_config_is_up_to_date::<Config>(EXAMPLE_ENV);
    }
}
