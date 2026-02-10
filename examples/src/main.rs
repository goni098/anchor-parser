use anchor_parser::AccountDeserialize;
use anchor_parser::declare_program;

declare_program!(meteora_dlmm);

#[tokio::main]
async fn main() {
    println!("Program ID: {}", meteora_dlmm::ID);

    // ── Build a swap instruction ────────────────────────────────────

    let user = solana_sdk::pubkey::Pubkey::from([1; 32]);
    let lb_pair = solana_sdk::pubkey::Pubkey::from([2; 32]);

    let ix = meteora_dlmm::instructions::swap(
        &meteora_dlmm::ID,
        &meteora_dlmm::instructions::SwapAccounts {
            lb_pair,
            bin_array_bitmap_extension: None,
            reserve_x: solana_sdk::pubkey::Pubkey::from([6; 32]),
            reserve_y: solana_sdk::pubkey::Pubkey::from([7; 32]),
            user_token_in: solana_sdk::pubkey::Pubkey::from([8; 32]),
            user_token_out: solana_sdk::pubkey::Pubkey::from([9; 32]),
            token_x_mint: solana_sdk::pubkey::Pubkey::from([10; 32]),
            token_y_mint: solana_sdk::pubkey::Pubkey::from([11; 32]),
            oracle: solana_sdk::pubkey::Pubkey::from([12; 32]),
            host_fee_in: None,
            user,
            token_x_program: solana_sdk::pubkey::Pubkey::from([13; 32]),
            token_y_program: solana_sdk::pubkey::Pubkey::from([14; 32]),
            event_authority: solana_sdk::pubkey::Pubkey::from([15; 32]),
            program: solana_sdk::pubkey::Pubkey::from([16; 32]),
        },
        1_000_000u64, // amount_in
        0u64,         // min_amount_out
    );

    println!("Swap instruction built:");
    println!("  program_id: {}", ix.program_id);
    println!("  data len:   {} bytes", ix.data.len());
    println!("  accounts:   {}", ix.accounts.len());

    // ── Parse events from transaction logs ──────────────────────────

    let sample_logs: Vec<&str> = vec![
        "Program LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo invoke [1]",
        "Program log: some log",
        "Program LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo success",
    ];

    let events = meteora_dlmm::utils::Event::from_logs(&sample_logs);
    println!("\nParsed {} events from logs", events.len());

    // ── Deserialize an account from raw data ────────────────────────

    println!("\nAccount discriminators:");
    println!(
        "  LbPair:    {:?}",
        meteora_dlmm::accounts::LbPair::DISCRIMINATOR
    );
    println!(
        "  Position:  {:?}",
        meteora_dlmm::accounts::Position::DISCRIMINATOR
    );

    // ── Fetch account via RPC (client feature) ──────────────────────

    let rpc = solana_client::nonblocking::rpc_client::RpcClient::new(
        "https://api.mainnet-beta.solana.com".to_string(),
    );

    // fetch / fetch_multiple are available via AccountDeserialize trait
    let address: solana_sdk::pubkey::Pubkey = "LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo"
        .parse()
        .unwrap();

    match meteora_dlmm::accounts::LbPair::fetch(&rpc, &address).await {
        Ok(pair) => println!("\nFetched LbPair: active_id={}", pair.active_id),
        Err(e) => println!("\nFetch failed (expected for program address): {e}"),
    }

    // Or use the standalone client functions:
    // let pair = anchor_parser::client::fetch_account::<meteora_dlmm::accounts::LbPair>(&rpc, &address).await?;

    println!("\nDone!");
}
