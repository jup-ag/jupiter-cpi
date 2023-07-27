anchor_gen::generate_cpi_crate!("idl.json");

anchor_lang::declare_id!("JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4");

use rand::seq::IteratorRandom;
use rand::Rng;
use solana_program::pubkey;

// Now, we only support up to 2 Authorities between [0, 1]. To create more authorities, we need to
// add them in the monorepo. We can use from 0 up to 255 in order to prevent hot accounts.
pub const PROGRAM_AUTHORITY_ID_MAX: u8 = 1;
pub const AUTHORITY_SEED: &[u8] = b"authority";

pub fn find_jupiter_program_authority_id() -> u8 {
    let mut rng = rand::thread_rng();
    rng.gen_range(0..=PROGRAM_AUTHORITY_ID_MAX)
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
        const DISCRIMINATOR: [u8; 8] = super::instruction::Route::DISCRIMINATOR;
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
        const DISCRIMINATOR: [u8; 8] = super::instruction::RouteWithTokenLedger::DISCRIMINATOR;
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
        const DISCRIMINATOR: [u8; 8] = super::instruction::SharedAccountsRoute::DISCRIMINATOR;
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
        const DISCRIMINATOR: [u8; 8] =
            super::instruction::SharedAccountsRouteWithTokenLedger::DISCRIMINATOR;
    }

    impl InstructionData for SharedAccountsRouteWithTokenLedger {}
}
