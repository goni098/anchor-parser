use anchor_parser::declare_program;

declare_program!(meteora_dlmm);

fn main() {
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
            user: user,
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

    // Using the generic Event enum to parse any program event
    let sample_logs: Vec<&str> = vec![
        "Program LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo invoke [1]",
        "Program log: some log",
        "Program LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo success",
    ];

    let events = meteora_dlmm::utils::Event::from_logs(&sample_logs);
    println!("\nParsed {} events from logs", events.len());

    // ── Deserialize an account from raw data ────────────────────────

    // Each account type has a DISCRIMINATOR and from_account_data method
    println!("\nAccount discriminators:");
    println!(
        "  LbPair:    {:?}",
        meteora_dlmm::accounts::LbPair::DISCRIMINATOR
    );
    println!(
        "  Position:  {:?}",
        meteora_dlmm::accounts::Position::DISCRIMINATOR
    );
    println!(
        "  BinArray:  {:?}",
        meteora_dlmm::accounts::BinArray::DISCRIMINATOR
    );
    println!(
        "  Oracle:    {:?}",
        meteora_dlmm::accounts::Oracle::DISCRIMINATOR
    );

    println!("\nDone!");
}
