anchor_gen::generate_cpi_crate!("idl.json");

anchor_lang::declare_id!("JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4");

use rand::{
    distributions::{Distribution, Uniform},
    seq::IteratorRandom,
};
use solana_program::pubkey;

// Now, we only support up to 8 authorities between [0, 1, 2, 3, 4, 5, 6, 7]. To create more authorities, we need to
// add them in the monorepo. We can use from 0 up to 255 in order to prevent hot accounts.
pub const AUTHORITY_COUNT: usize = 8;
pub const AUTHORITY_SEED: &[u8] = b"authority";

pub fn find_authorities() -> Vec<Pubkey> {
    (0..AUTHORITY_COUNT)
        .map(|authority_id| find_jupiter_program_authority(authority_id as u8))
        .collect()
}

pub fn find_event_authority() -> Pubkey {
    Pubkey::find_program_address(&[b"__event_authority"], &crate::ID).0
}

pub fn find_jupiter_program_authority_id() -> u8 {
    let mut rng = rand::thread_rng();
    let ids = Uniform::from(0..(AUTHORITY_COUNT as u8));
    ids.sample(&mut rng)
}

pub fn find_jupiter_program_authority(id: u8) -> Pubkey {
    Pubkey::find_program_address(&[AUTHORITY_SEED, &[id]], &crate::ID).0
}

pub fn find_jupiter_token_ledger() -> Pubkey {
    let mut rng = rand::thread_rng();
    let token_ledgers = [
        pubkey!("HtncvpUBGhSrs48KtC58ntJcTDw53sn78Lpq71zVwiez"),
        pubkey!("HxTk98CmBcxmtkrBWqRszYxrnDpqAsbitQBc2QjVBG3j"),
        pubkey!("CnUPHtfUVw3D2s4FB8H6QBuLwoes8YxauVgDtFybm7rz"),
        pubkey!("FhLPkpFmszHtSyyayj7KsXNZeBTqfQbUPmvgWAyJHBXh"),
    ];
    let token_ledger = token_ledgers.iter().choose(&mut rng);
    *token_ledger.unwrap()
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
