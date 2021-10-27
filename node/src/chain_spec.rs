use cumulus_primitives_core::ParaId;
use hex_literal::hex;
use development_runtime::{AccountId, AuraId, Signature};
use sc_chain_spec::{ChainSpecExtension, ChainSpecGroup};
use std::str::FromStr;
use sc_service::{ChainType, Properties};
use sc_telemetry::TelemetryEndpoints;
use serde::{Deserialize, Serialize};

use sp_core::{crypto::UncheckedInto, sr25519, Pair, Public};
use sp_runtime::{AccountId32, traits::{IdentifyAccount, Verify}};
use pichiu_runtime::constants::currency::PCHU;


/// Properties for Kylin.
pub fn kylin_properties() -> Properties {
	let mut properties = Properties::new();
	properties.insert("ss58Format".into(), 31.into());
	properties.insert("tokenDecimals".into(), 12.into());
	properties.insert("tokenSymbol".into(), "KYL".into());
	properties
}

/// Specialized `ChainSpec` for the normal parachain runtime.
pub type DevelopmentChainSpec = sc_service::GenericChainSpec<development_runtime::GenesisConfig>;

pub type PichiuChainSpec = sc_service::GenericChainSpec<pichiu_runtime::GenesisConfig>;

/// Specialized `ChainSpec` for the shell parachain runtime.
pub type ShellChainSpec = sc_service::GenericChainSpec<shell_runtime::GenesisConfig, Extensions>;


const POLKADOT_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";

/// Helper function to generate a crypto pair from seed
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}

/// The extensions for the [`ChainSpec`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ChainSpecGroup, ChainSpecExtension)]
#[serde(deny_unknown_fields)]
pub struct Extensions {
	/// The relay chain of the Parachain.
	pub relay_chain: String,
	/// The id of the Parachain.
	pub para_id: u32,
}

impl Extensions {
	/// Try to get the extension from the given `ChainSpec`.
	pub fn try_get(chain_spec: &dyn sc_service::ChainSpec) -> Option<&Self> {
		sc_chain_spec::get_extension(chain_spec.extensions())
	}
}

type AccountPublic = <Signature as Verify>::Signer;

/// Helper function to generate an account ID from seed
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
	where
		AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
	AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

fn shell_testnet_genesis(parachain_id: ParaId) -> shell_runtime::GenesisConfig {
	shell_runtime::GenesisConfig {
		system: shell_runtime::SystemConfig {
			code: shell_runtime::WASM_BINARY
				.expect("WASM binary was not build, please build it!")
				.to_vec(),
			changes_trie_config: Default::default(),
		},
		parachain_info: shell_runtime::ParachainInfoConfig { parachain_id },
		parachain_system: Default::default(),
	}
}


pub fn get_shell_chain_spec(id: ParaId) -> ShellChainSpec {
	ShellChainSpec::from_genesis(
		"Shell Local Testnet",
		"shell_local_testnet",
		ChainType::Local,
		move || shell_testnet_genesis(id),
		vec![],
		None,
		Some("Kylin"),
		Some(kylin_properties()),
		Extensions {
			relay_chain: "westend".into(),
			para_id: id.into(),
		},
	)
}

pub fn development_local_config(id: ParaId, environment: &str) -> DevelopmentChainSpec {
	DevelopmentChainSpec::from_genesis(
		// Name
		format!("kylin {} testnet", environment).as_str(),
		// ID
		format!("kylin-{}-testnet", environment).as_str(),
		ChainType::Development,
		move || {
			development_genesis(
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				vec![
					get_from_seed::<AuraId>("Alice"),
					get_from_seed::<AuraId>("Bob"),
					hex!["7c11cea2901e72fe525d7335e99d48bdf8dea2a983ac92fa3ab20508a438af73"]
					.unchecked_into(),
					hex!["287f278af79ef7f1b2a2b3d5a7c76a047e248232d13f0a5ec744789a96dc824d"]
					.unchecked_into()
				],
				vec![
					AccountId32::from_str("5Gn1igfpf4hP7iG1Gsm1AbwPBCpR8BmHK4b6i2VrGHQS1kAJ").unwrap(),
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					get_account_id_from_seed::<sr25519::Public>("Bob"),
					get_account_id_from_seed::<sr25519::Public>("Charlie"),
					get_account_id_from_seed::<sr25519::Public>("Dave"),
					get_account_id_from_seed::<sr25519::Public>("Eve"),
					get_account_id_from_seed::<sr25519::Public>("Ferdie"),
					get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
					get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
					get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
					get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
					get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
					get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
				],
				id,
			)
		},
		Vec::new(),
		None,
		Some("Kylin"),
		Some(kylin_properties()),
		Default::default()
	)
}

pub fn development_environment_config(id: ParaId,environment: &str) -> DevelopmentChainSpec {
	DevelopmentChainSpec::from_genesis(
		format!("{} testnet", environment).as_str(),
		// ID
		format!("{}-testnet", environment).as_str(),
		ChainType::Live,
		move || {
			development_genesis(
				AccountId32::from_str("5Gn1igfpf4hP7iG1Gsm1AbwPBCpR8BmHK4b6i2VrGHQS1kAJ").unwrap(),
				vec![
					hex!["7c11cea2901e72fe525d7335e99d48bdf8dea2a983ac92fa3ab20508a438af73"]
						.unchecked_into(),
					hex!["287f278af79ef7f1b2a2b3d5a7c76a047e248232d13f0a5ec744789a96dc824d"]
						.unchecked_into(),
				],
				vec![
					AccountId32::from_str("5Gn1igfpf4hP7iG1Gsm1AbwPBCpR8BmHK4b6i2VrGHQS1kAJ").unwrap(),
					get_account_id_from_seed::<sr25519::Public>("Alice"),
				],
				id,
			)
		},
		Vec::new(),
		Some(
			TelemetryEndpoints::new(vec![(POLKADOT_TELEMETRY_URL.to_string(), 0)])
				.expect("Polkadot telemetry url is valid; qed"),
		),
		Some("Kylin"),
		Some(kylin_properties()),
		Default::default()
	)
}

pub fn pichiu_local_network(para_id: ParaId) -> PichiuChainSpec {
	let mut properties = Properties::new();
	properties.insert("ss58Format".into(), 31.into());
	properties.insert("tokenSymbol".into(), "PCHU".into());
	properties.insert("tokenDecimals".into(), 18.into());

	PichiuChainSpec::from_genesis(
		"Pichiu Local Testnet",
		"pichiu_local_testnet",
		ChainType::Development,
		move || {
			pichiu_genesis(
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				vec![
					get_from_seed::<AuraId>("Alice"),
					get_from_seed::<AuraId>("Bob"),
					hex!["7c11cea2901e72fe525d7335e99d48bdf8dea2a983ac92fa3ab20508a438af73"]
					.unchecked_into(),
					hex!["287f278af79ef7f1b2a2b3d5a7c76a047e248232d13f0a5ec744789a96dc824d"]
					.unchecked_into()
				],
				endowed_accounts(),
				Some(50000000 * PCHU),
				para_id,
			)
		},
		Vec::new(),
		None,
		Some("Pichiu"),
		Some(properties),
		Default::default(),
	)
}

pub fn pichiu_development_network(para_id: ParaId) -> PichiuChainSpec {
	let mut properties = Properties::new();
	properties.insert("tokenSymbol".into(), "PCHU".into());
	properties.insert("tokenDecimals".into(), 18.into());

	PichiuChainSpec::from_genesis(
		"Pichiu dev",
		"pichiu_dev",
		ChainType::Live,
		move || {
			pichiu_genesis(
				AccountId32::from_str("5Gn1igfpf4hP7iG1Gsm1AbwPBCpR8BmHK4b6i2VrGHQS1kAJ").unwrap(),
				vec![
					get_from_seed::<AuraId>("Alice"),
					get_from_seed::<AuraId>("Bob"),
					hex!["7c11cea2901e72fe525d7335e99d48bdf8dea2a983ac92fa3ab20508a438af73"]
					.unchecked_into(),
					hex!["287f278af79ef7f1b2a2b3d5a7c76a047e248232d13f0a5ec744789a96dc824d"]
					.unchecked_into()
				],
				endowed_accounts(),
				Some(50000000 * PCHU),
				para_id,
			)
		},
		vec![],
		Some(
			TelemetryEndpoints::new(vec![(POLKADOT_TELEMETRY_URL.to_string(), 0)])
				.expect("Polkadot telemetry url is valid; qed"),
		),
		Some("Pichiu"),
		Some(properties),
		Default::default(),
	)
}

pub fn pichiu_network(para_id: ParaId) -> PichiuChainSpec {
	let mut properties = Properties::new();
	properties.insert("tokenSymbol".into(), "PCHU".into());
	properties.insert("tokenDecimals".into(), 18.into());

	PichiuChainSpec::from_genesis(
		"Pichiu Network",
		"pichiu_Network",
		ChainType::Live,
		move || {
			pichiu_genesis(
				AccountId32::from_str("5Gn1igfpf4hP7iG1Gsm1AbwPBCpR8BmHK4b6i2VrGHQS1kAJ").unwrap(),
				vec![
					hex!["7c11cea2901e72fe525d7335e99d48bdf8dea2a983ac92fa3ab20508a438af73"]
					.unchecked_into(),
					hex!["287f278af79ef7f1b2a2b3d5a7c76a047e248232d13f0a5ec744789a96dc824d"]
					.unchecked_into()
				],
				endowed_accounts(),
				Some(50000000 * PCHU),
				para_id,
			)
		},
		vec![],
		Some(
			TelemetryEndpoints::new(vec![(POLKADOT_TELEMETRY_URL.to_string(), 0)])
				.expect("Polkadot telemetry url is valid; qed"),
		),
		None,
		Some(properties),
		Default::default(),
	)
}

fn endowed_accounts() -> Vec<AccountId> {
	vec![
		get_account_id_from_seed::<sr25519::Public>("Alice"),
		get_account_id_from_seed::<sr25519::Public>("Bob"),
		get_account_id_from_seed::<sr25519::Public>("Charlie"),
		get_account_id_from_seed::<sr25519::Public>("Dave"),
		get_account_id_from_seed::<sr25519::Public>("Eve"),
		get_account_id_from_seed::<sr25519::Public>("Ferdie"),
		get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
		get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
		get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
		get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
		get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
		get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
	]
}

fn pichiu_genesis(
	root_key: AccountId,
	initial_authorities: Vec<pichiu_runtime::AuraId>,
	endowed_accounts: Vec<pichiu_runtime::AccountId>,
	total_issuance: Option<pichiu_runtime::Balance>,
	id: ParaId,
) -> pichiu_runtime::GenesisConfig {
	let num_endowed_accounts = endowed_accounts.len();
	let balances = match total_issuance {
		Some(total_issuance) => {
			let balance_per_endowed = total_issuance
				.checked_div(num_endowed_accounts as pichiu_runtime::Balance)
				.unwrap_or(0 as pichiu_runtime::Balance);
			endowed_accounts
				.iter()
				.cloned()
				.map(|k| (k, balance_per_endowed))
				.collect()
		}
		None => vec![],
	};

	pichiu_runtime::GenesisConfig {
		system: pichiu_runtime::SystemConfig {
			code: pichiu_runtime::WASM_BINARY
				.expect("WASM binary was not build, please build it!")
				.to_vec(),
			changes_trie_config: Default::default(),
		},
		balances: pichiu_runtime::BalancesConfig { balances },
		// elections: pichiu_runtime::ElectionsConfig { members: vec![] },
		// council: pichiu_runtime::CouncilConfig {
		// 	members: Default::default(),
		// 	phantom: Default::default(),
		// },
		// fees: pichiu_runtime::FeesConfig {
		// 	initial_fees: vec![(
		// 		// Anchoring state rent fee per day
		// 		// pre-image: 0xdb4faa73ca6d2016e53c7156087c176b79b169c409b8a0063a07964f3187f9e9
		// 		// hash   : 0x11da6d1f761ddf9bdb4c9d6e5303ebd41f61858d0a5647a1a7bfe089bf921be9
		// 		Hash::from(&[
		// 			17, 218, 109, 31, 118, 29, 223, 155, 219, 76, 157, 110, 83, 3, 235, 212, 31,
		// 			97, 133, 141, 10, 86, 71, 161, 167, 191, 224, 137, 191, 146, 27, 233,
		// 		]),
		// 		// Daily state rent, defined such that it will amount to 0.00259.. RAD (2_590_000_000_000_040) over
		// 		// 3 years, which is the expected average anchor duration. The other fee components for anchors amount
		// 		// to about 0.00041.. RAD (410_000_000_000_000), such that the total anchor price for 3 years will be
		// 		// 0.003.. RAD
		// 		2_365_296_803_653,
		// 	)],
		// },
		// vesting: Default::default(),
		sudo: pichiu_runtime::SudoConfig { key: root_key },
		parachain_info: pichiu_runtime::ParachainInfoConfig { parachain_id: id },
		// session: pichiu_runtime::SessionConfig {
		// 	keys: initial_authorities
		// 		.iter()
		// 		.cloned()
		// 		.map(|(acc, aura)| {
		// 			(
		// 				acc.clone(),                   // account id
		// 				acc.clone(),                   // validator id
		// 				get_altair_session_keys(aura), // session keys
		// 			)
		// 		})
		// 		.collect(),
		// },
		aura_ext: Default::default(),
		aura: pichiu_runtime::AuraConfig {
			authorities: initial_authorities,
		},
		// democracy: Default::default(),
		parachain_system: Default::default(),
	}
}
fn development_genesis(
	root_key: AccountId,
	initial_authorities: Vec<AuraId>,
	endowed_accounts: Vec<AccountId>,
	id: ParaId,
) -> development_runtime::GenesisConfig {
	let _initial_balance: u128 = 1_000_000_000 * 1000;

	development_runtime::GenesisConfig {
		system: development_runtime::SystemConfig {
			code: development_runtime::WASM_BINARY
				.expect("WASM binary was not build, please build it!")
				.to_vec(),
			changes_trie_config: Default::default(),
		},
		balances: development_runtime::BalancesConfig {
			balances: endowed_accounts
				.iter()
				.cloned()
				.map(|k| (k, 10 << 60))
				.collect(),
		},
		sudo: development_runtime::SudoConfig { key: root_key },
		parachain_info: development_runtime::ParachainInfoConfig { parachain_id: id },
		aura: development_runtime::AuraConfig {
			authorities: initial_authorities,
		},
	
		aura_ext: Default::default(),
		parachain_system: Default::default(),
	}
}
