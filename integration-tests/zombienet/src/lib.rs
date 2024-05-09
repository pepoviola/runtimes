use zombienet_sdk::{NetworkConfig, NetworkConfigBuilder};
use anyhow::anyhow;

pub mod environment;

#[subxt::subxt(runtime_metadata_path = "artifacts/polkadot_metadata_small.scale")]
pub mod polkadot {}

pub type Error = Box<dyn std::error::Error>;

const CMD_TPL: &str = "chain-spec-generator {{chainName}}";

pub fn small_network() -> Result<NetworkConfig, Error> {
    let images = environment::get_images_from_env();
    let config = NetworkConfigBuilder::new()
        .with_relaychain(|r| {
            r.with_chain("polkadot-local")
                .with_default_command("polkadot")
                .with_default_image(images.polkadot.as_str())
                .with_chain_spec_command(CMD_TPL)
                .chain_spec_command_is_local(true)
                .with_node(|node| node.with_name("alice"))
                .with_node(|node| node.with_name("bob"))
        })
        .with_parachain(|p| {
            p.with_id(2000).cumulus_based(true).with_collator(|n| {
                n.with_name("collator")
                    .with_command("polkadot-parachain")
                    .with_image(images.cumulus.as_str())
            })
        })
        .build()
        .map_err(|errs| {
            let e = errs.iter().fold("".to_string(), |memo, err| {
                format!("{memo} \n {err}")
            });
            anyhow!(e)
        })?;

        Ok(config)
}