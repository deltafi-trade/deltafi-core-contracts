#![cfg(feature = "test-bpf")]

mod utils;

use deltafi_swap::{
    math::{Decimal, TryDiv},
    processor::process,
    state::SwapType,
};

use solana_program_test::*;
use solana_sdk::{
    pubkey::Pubkey,
    signature::{Keypair, Signer},
};
use utils::*;

#[tokio::test]
async fn test_success() {
    let mut test = ProgramTest::new("deltafi_swap", deltafi_swap::id(), processor!(process));

    // limit to track compute unit increase
    test.set_bpf_compute_max_units(200_000);

    let swap_config = add_swap_config(&mut test);

    let sol_oracle = add_sol_oracle(&mut test);
    let srm_oracle = add_srm_oracle(&mut test);
    let srm_mint = add_srm_mint(&mut test);
    let user_account_owner = Keypair::new();
    let admin_account_owner = Keypair::new();

    let swap_info = add_swap_info(
        SwapType::Normal,
        &mut test,
        &swap_config,
        &user_account_owner,
        &admin_account_owner,
        AddSwapInfoArgs {
            token_a_mint: spl_token::native_mint::id(),
            token_b_mint: srm_mint.pubkey,
            token_a_amount: 42_000_000_000,
            token_b_amount: 800_000_000_000,
            oracle_a: sol_oracle.price_pubkey,
            oracle_b: srm_oracle.price_pubkey,
            market_price: sol_oracle.price.try_div(srm_oracle.price).unwrap(),
            slope: Decimal::one().try_div(2).unwrap(),
            last_market_price: sol_oracle.price.try_div(srm_oracle.price).unwrap(),
            last_valid_market_price_slot: 0,
            swap_out_limit_percentage: 10u8,
            ..AddSwapInfoArgs::default()
        },
    );

    let pool_owner = Keypair::new();
    // let farm_user = add_farm_user(&mut test, &pool_owner);

    let (mut banks_client, payer, _recent_blockhash) = test.start().await;

    let sol_deposit_account = create_and_mint_to_token_account(
        &mut banks_client,
        spl_token::native_mint::id(),
        None,
        &payer,
        pool_owner.pubkey(),
        10_000_000_000,
    )
    .await;

    let srm_deposit_account = create_and_mint_to_token_account(
        &mut banks_client,
        srm_mint.pubkey,
        Some(&srm_mint.authority),
        &payer,
        pool_owner.pubkey(),
        200_000_000_000,
    )
    .await;

    let pool_token_account = create_and_mint_to_token_account(
        &mut banks_client,
        swap_info.pool_mint,
        None,
        &payer,
        pool_owner.pubkey(),
        0,
    )
    .await;

    swap_info
        .deposit(
            SwapType::Normal,
            &mut banks_client,
            &pool_owner,
            sol_deposit_account,
            srm_deposit_account,
            pool_token_account,
            8_000_000_000,
            160_000_000_000,
            0,
            &payer,
        )
        .await;

    assert_eq!(
        get_token_balance(&mut banks_client, sol_deposit_account).await,
        2_000_000_000,
    );
    assert_eq!(
        get_token_balance(&mut banks_client, srm_deposit_account).await,
        // The deposit amount is 800/42*8_000_000_000 = 152_380_952_380
        200_000_000_000 - 152_380_952_380,
    );
    assert!(get_token_balance(&mut banks_client, pool_token_account).await > 0);
    assert_eq!(
        get_token_balance(&mut banks_client, swap_info.token_a).await,
        50_000_000_000,
    );
    assert_eq!(
        get_token_balance(&mut banks_client, swap_info.token_b).await,
        // The deposit amount is 800/42*8_000_000_000 = 152_380_952_380
        800_000_000_000 + 152_380_952_380,
    );
}
