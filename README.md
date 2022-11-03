# jupiter-cpi

CPI helpers for the [Jupiter](https://docs.jup.ag/notes/on-chain-program) program.

This crate is using [anchor-gen](https://github.com/saber-hq/anchor-gen) to automatically generate a crate from the Jupiter Anchor JSON IDL.

## How-to

```
let swap_ix = Instruction {
    program_id: jupiter_cpi::ID,
    accounts,
    data: jupiter_override::Route {
        swap_leg,
        in_amount: quote_response.in_amount,
        quoted_out_amount: 0,
        slippage_bps: 0,
        platform_fee_bps: 0,
    }
    .data(),
};
```

## License

Apache 2.0
