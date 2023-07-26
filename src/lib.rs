anchor_gen::generate_cpi_crate!("idl.json");

anchor_lang::declare_id!("JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4");

use rand::distributions::{Distribution, Uniform};
use rand::seq::IteratorRandom;
use solana_sdk::pubkey;

// Now, we only support up to 2 Authorities between [0, 1]. To create more authorities, we need to
// add them in the monorepo. We can use from 0 up to 255 in order to prevent hot accounts.
pub const PROGRAM_AUTHORITY_ID_MAX: u8 = 1;
pub const AUTHORITY_SEED: &[u8] = b"authority";

pub fn find_jupiter_program_authority_id() -> u8 {
    let mut rng = rand::thread_rng();
    let ids = Uniform::from(0..(PROGRAM_AUTHORITY_ID_MAX + 1));
    ids.sample(&mut rng)
}

pub fn find_jupiter_program_authority(id: u8) -> Pubkey {
    Pubkey::find_program_address(&[&AUTHORITY_SEED, &[id]], &crate::ID).0
}

pub fn find_jupiter_token_ledger() -> Pubkey {
    let mut rng = rand::thread_rng();
    let token_ledgers = vec![
        pubkey!("CqdGCCCirfgx87nmsJyWG6845dAVqWYeun11zVqvdBM1"),
        pubkey!("DWoxXsgpCehmefg1MLR5dapwrTFYhikDRrekKTQeUToa"),
        pubkey!("8fz7UjjbdGAdiue65NirjEYxRH4qA7WAVBvkQYC7bVUs"),
        pubkey!("ECzAqoWaGtGdpcoEmZ1DrkXqbUK5LnbxRE35xuFMrwdB"),
    ];
    let token_ledger = token_ledgers.iter().choose(&mut rng);
    token_ledger.unwrap().clone()
}

pub fn find_jupiter_open_orders(market: &Pubkey, authority: &Pubkey) -> Pubkey {
    Pubkey::find_program_address(
        &[b"open_orders".as_ref(), market.as_ref(), authority.as_ref()],
        &self::ID,
    )
    .0
}

fn pubkey_or_none(pubkey: Pubkey) -> Option<Pubkey> {
    // Anchor doesn't support Option<Pubkey> yet so if the pubkey is the program key itself
    // it is a None.
    if pubkey == self::ID {
        None
    } else {
        Some(pubkey)
    }
}

impl From<Vec<AccountMeta>> for accounts::Route {
    fn from(accounts: Vec<AccountMeta>) -> accounts::Route {
        accounts::Route {
            token_program: accounts[0].pubkey,
            user_transfer_authority: accounts[1].pubkey,
            user_source_token_account: accounts[2].pubkey,
            user_destination_token_account: accounts[3].pubkey,
            destination_token_account: pubkey_or_none(accounts[4].pubkey),
            destination_mint: accounts[5].pubkey,
            platform_fee_account: pubkey_or_none(accounts[6].pubkey),
        }
    }
}

impl From<Vec<AccountMeta>> for accounts::RouteWithTokenLedger {
    fn from(accounts: Vec<AccountMeta>) -> accounts::RouteWithTokenLedger {
        accounts::RouteWithTokenLedger {
            token_program: accounts[0].pubkey,
            user_transfer_authority: accounts[1].pubkey,
            user_source_token_account: accounts[2].pubkey,
            user_destination_token_account: accounts[3].pubkey,
            destination_token_account: pubkey_or_none(accounts[4].pubkey),
            destination_mint: accounts[5].pubkey,
            platform_fee_account: pubkey_or_none(accounts[6].pubkey),
            token_ledger: accounts[7].pubkey,
        }
    }
}

impl From<Vec<AccountMeta>> for accounts::SharedAccountsRoute {
    fn from(accounts: Vec<AccountMeta>) -> accounts::SharedAccountsRoute {
        accounts::SharedAccountsRoute {
            token_program: accounts[0].pubkey,
            program_authority: accounts[1].pubkey,
            user_transfer_authority: accounts[2].pubkey,
            source_token_account: accounts[3].pubkey,
            program_source_token_account: accounts[4].pubkey,
            program_destination_token_account: accounts[5].pubkey,
            destination_token_account: accounts[6].pubkey,
            source_mint: accounts[7].pubkey,
            destination_mint: accounts[8].pubkey,
            platform_fee_account: pubkey_or_none(accounts[9].pubkey),
            token2022_program: pubkey_or_none(accounts[10].pubkey),
        }
    }
}

impl From<Vec<AccountMeta>> for accounts::SharedAccountsRouteWithTokenLedger {
    fn from(accounts: Vec<AccountMeta>) -> accounts::SharedAccountsRouteWithTokenLedger {
        accounts::SharedAccountsRouteWithTokenLedger {
            token_program: accounts[0].pubkey,
            program_authority: accounts[1].pubkey,
            user_transfer_authority: accounts[2].pubkey,
            source_token_account: accounts[3].pubkey,
            program_source_token_account: accounts[4].pubkey,
            program_destination_token_account: accounts[5].pubkey,
            destination_token_account: accounts[6].pubkey,
            source_mint: accounts[7].pubkey,
            destination_mint: accounts[8].pubkey,
            platform_fee_account: pubkey_or_none(accounts[9].pubkey),
            token2022_program: pubkey_or_none(accounts[10].pubkey),
            token_ledger: accounts[11].pubkey,
        }
    }
}

// Temporarily redefined it until solution is found
pub mod jupiter_override {
    use anchor_lang::InstructionData;
    use anchor_lang::{prelude::*, Discriminator};
    use jupiter_amm_interface::Swap as InterfaceSwap;

    #[derive(AnchorSerialize, Debug)]
    pub struct RoutePlanStep {
        pub swap: InterfaceSwap,
        pub percent: u8,
        pub input_index: u8,
        pub output_index: u8,
    }

    #[derive(AnchorSerialize)]
    pub struct Route {
        pub route_plan: Vec<RoutePlanStep>,
        pub in_amount: u64,
        pub quoted_out_amount: u64,
        pub slippage_bps: u16,
        pub platform_fee_bps: u8,
    }
    impl Discriminator for Route {
        const DISCRIMINATOR: [u8; 8] = [229, 23, 203, 151, 122, 227, 173, 42];
    }

    impl InstructionData for Route {}

    #[derive(AnchorSerialize)]
    pub struct RouteWithTokenLedger {
        pub route_plan: Vec<RoutePlanStep>,
        pub quoted_out_amount: u64,
        pub slippage_bps: u16,
        pub platform_fee_bps: u8,
    }
    impl Discriminator for RouteWithTokenLedger {
        const DISCRIMINATOR: [u8; 8] = [150, 86, 71, 116, 167, 93, 14, 104];
    }

    impl InstructionData for RouteWithTokenLedger {}

    #[derive(AnchorSerialize)]
    pub struct SharedAccountsRoute {
        pub id: u8,
        pub route_plan: Vec<RoutePlanStep>,
        pub in_amount: u64,
        pub quoted_out_amount: u64,
        pub slippage_bps: u16,
        pub platform_fee_bps: u8,
    }
    impl Discriminator for SharedAccountsRoute {
        const DISCRIMINATOR: [u8; 8] = [193, 32, 155, 51, 65, 214, 156, 129];
    }

    impl InstructionData for SharedAccountsRoute {}

    #[derive(AnchorSerialize)]
    pub struct SharedAccountsRouteWithTokenLedger {
        pub id: u8,
        pub route_plan: Vec<RoutePlanStep>,
        pub quoted_out_amount: u64,
        pub slippage_bps: u16,
        pub platform_fee_bps: u8,
    }
    impl Discriminator for SharedAccountsRouteWithTokenLedger {
        const DISCRIMINATOR: [u8; 8] = [230, 121, 143, 80, 119, 159, 106, 170];
    }

    impl InstructionData for SharedAccountsRouteWithTokenLedger {}
}
