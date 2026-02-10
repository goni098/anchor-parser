use anchor_parser::declare_program;

declare_program!(raydium_clmm);

#[cfg(test)]
mod tests {
    use super::raydium_clmm;
    use solana_sdk::pubkey::Pubkey;

    #[test]
    fn test_program_id() {
        assert_eq!(
            raydium_clmm::ID.to_string(),
            "CAMMCzo5YL8w4VFF8KVHrK22GGUsp5VTaW7grrKgrWqK"
        );
    }

    // ── Account discriminators ──────────────────────────────────────

    #[test]
    fn test_amm_config_discriminator() {
        use raydium_clmm::accounts::AmmConfig;
        assert_eq!(
            AmmConfig::DISCRIMINATOR,
            [218, 244, 33, 104, 203, 203, 43, 111]
        );
    }

    #[test]
    fn test_pool_state_discriminator() {
        use raydium_clmm::accounts::PoolState;
        assert_eq!(
            PoolState::DISCRIMINATOR,
            [247, 237, 227, 245, 215, 195, 222, 70]
        );
    }

    #[test]
    fn test_personal_position_state_discriminator() {
        use raydium_clmm::accounts::PersonalPositionState;
        assert_eq!(
            PersonalPositionState::DISCRIMINATOR,
            [70, 111, 150, 126, 230, 15, 25, 117]
        );
    }

    #[test]
    fn test_protocol_position_state_discriminator() {
        use raydium_clmm::accounts::ProtocolPositionState;
        assert_eq!(
            ProtocolPositionState::DISCRIMINATOR,
            [100, 226, 145, 99, 146, 218, 160, 106]
        );
    }

    #[test]
    fn test_observation_state_discriminator() {
        use raydium_clmm::accounts::ObservationState;
        assert_eq!(
            ObservationState::DISCRIMINATOR,
            [122, 174, 197, 53, 129, 9, 165, 132]
        );
    }

    #[test]
    fn test_operation_state_discriminator() {
        use raydium_clmm::accounts::OperationState;
        assert_eq!(
            OperationState::DISCRIMINATOR,
            [19, 236, 58, 237, 81, 222, 183, 252]
        );
    }

    #[test]
    fn test_tick_array_state_discriminator() {
        use raydium_clmm::accounts::TickArrayState;
        assert_eq!(
            TickArrayState::DISCRIMINATOR,
            [192, 155, 85, 205, 49, 249, 129, 42]
        );
    }

    #[test]
    fn test_tick_array_bitmap_extension_discriminator() {
        use raydium_clmm::accounts::TickArrayBitmapExtension;
        assert_eq!(
            TickArrayBitmapExtension::DISCRIMINATOR,
            [60, 150, 36, 219, 97, 128, 139, 153]
        );
    }

    #[test]
    fn test_support_mint_associated_discriminator() {
        use raydium_clmm::accounts::SupportMintAssociated;
        assert_eq!(
            SupportMintAssociated::DISCRIMINATOR,
            [134, 40, 183, 79, 12, 112, 162, 53]
        );
    }

    // ── Borsh account from_account_data ─────────────────────────────

    #[test]
    fn test_amm_config_from_account_data() {
        use borsh::BorshSerialize;
        use raydium_clmm::accounts::AmmConfig;

        let config = AmmConfig {
            bump: 255,
            index: 0,
            owner: Pubkey::new_unique(),
            protocol_fee_rate: 1200,
            trade_fee_rate: 2500,
            tick_spacing: 1,
            fund_fee_rate: 400,
            padding_u32: 0,
            fund_owner: Pubkey::new_unique(),
            padding: [0u64; 3],
        };

        let mut data = Vec::new();
        data.extend_from_slice(&AmmConfig::DISCRIMINATOR);
        config.serialize(&mut data).unwrap();

        let parsed = AmmConfig::from_account_data(&data).unwrap();
        assert_eq!(parsed.bump, 255);
        assert_eq!(parsed.protocol_fee_rate, 1200);
        assert_eq!(parsed.trade_fee_rate, 2500);
        assert_eq!(parsed.tick_spacing, 1);
        assert_eq!(parsed.fund_fee_rate, 400);
        assert_eq!(parsed.owner, config.owner);
    }

    #[test]
    fn test_personal_position_state_from_account_data() {
        use borsh::BorshSerialize;
        use raydium_clmm::accounts::PersonalPositionState;
        use raydium_clmm::types::PositionRewardInfo;

        let pos = PersonalPositionState {
            bump: [1],
            nft_mint: Pubkey::new_unique(),
            pool_id: Pubkey::new_unique(),
            tick_lower_index: -100,
            tick_upper_index: 100,
            liquidity: 1_000_000_000_000u128,
            fee_growth_inside_0_last_x64: 0,
            fee_growth_inside_1_last_x64: 0,
            token_fees_owed_0: 500,
            token_fees_owed_1: 600,
            reward_infos: [
                PositionRewardInfo {
                    growth_inside_last_x64: 0,
                    reward_amount_owed: 0,
                },
                PositionRewardInfo {
                    growth_inside_last_x64: 0,
                    reward_amount_owed: 0,
                },
                PositionRewardInfo {
                    growth_inside_last_x64: 0,
                    reward_amount_owed: 0,
                },
            ],
            recent_epoch: 500,
            padding: [0u64; 7],
        };

        let mut data = Vec::new();
        data.extend_from_slice(&PersonalPositionState::DISCRIMINATOR);
        pos.serialize(&mut data).unwrap();

        let parsed = PersonalPositionState::from_account_data(&data).unwrap();
        assert_eq!(parsed.tick_lower_index, -100);
        assert_eq!(parsed.tick_upper_index, 100);
        assert_eq!(parsed.liquidity, 1_000_000_000_000u128);
        assert_eq!(parsed.token_fees_owed_0, 500);
        assert_eq!(parsed.recent_epoch, 500);
    }

    #[test]
    fn test_amm_config_bad_discriminator() {
        let data = vec![0u8; 200];
        assert!(raydium_clmm::accounts::AmmConfig::from_account_data(&data).is_err());
    }

    // ── Bytemuck account deserialization ─────────────────────────────

    #[test]
    fn test_pool_state_bytemuck_deserialization() {
        use raydium_clmm::accounts::PoolState;

        let struct_size = std::mem::size_of::<PoolState>();
        let mut data = Vec::with_capacity(8 + struct_size);
        data.extend_from_slice(&PoolState::DISCRIMINATOR);
        data.extend_from_slice(&vec![0u8; struct_size]);

        let pool = PoolState::from_account_data(&data).unwrap();
        let liquidity = pool.liquidity;
        let sqrt_price = pool.sqrt_price_x64;
        let tick = pool.tick_current;
        let mint0 = pool.token_mint_0;
        assert_eq!(liquidity, 0);
        assert_eq!(sqrt_price, 0);
        assert_eq!(tick, 0);
        assert_eq!(mint0, Pubkey::default());
    }

    #[test]
    fn test_pool_state_bad_discriminator() {
        let data = vec![0u8; 4096];
        assert!(raydium_clmm::accounts::PoolState::from_account_data(&data).is_err());
    }

    #[test]
    fn test_tick_array_bitmap_extension_bytemuck() {
        use raydium_clmm::accounts::TickArrayBitmapExtension;

        let struct_size = std::mem::size_of::<TickArrayBitmapExtension>();
        let mut data = Vec::with_capacity(8 + struct_size);
        data.extend_from_slice(&TickArrayBitmapExtension::DISCRIMINATOR);
        data.extend_from_slice(&vec![0u8; struct_size]);

        let ext = TickArrayBitmapExtension::from_account_data(&data).unwrap();
        assert_eq!(ext.pool_id, Pubkey::default());
    }

    // ── Account enum (utils) ────────────────────────────────────────

    #[test]
    fn test_account_enum_parse_amm_config() {
        use borsh::BorshSerialize;
        use raydium_clmm::accounts::AmmConfig;
        use raydium_clmm::utils::Account;

        let config = AmmConfig {
            bump: 1,
            index: 0,
            owner: Pubkey::new_unique(),
            protocol_fee_rate: 1200,
            trade_fee_rate: 2500,
            tick_spacing: 10,
            fund_fee_rate: 400,
            padding_u32: 0,
            fund_owner: Pubkey::new_unique(),
            padding: [0u64; 3],
        };

        let mut data = Vec::new();
        data.extend_from_slice(&AmmConfig::DISCRIMINATOR);
        config.serialize(&mut data).unwrap();

        let parsed = Account::parse(&data).unwrap();
        match parsed {
            Account::AmmConfig(c) => {
                assert_eq!(c.trade_fee_rate, 2500);
                assert_eq!(c.tick_spacing, 10);
            }
            _ => panic!("Expected AmmConfig variant"),
        }
    }

    #[test]
    fn test_account_enum_parse_pool_state() {
        use raydium_clmm::accounts::PoolState;
        use raydium_clmm::utils::Account;

        let struct_size = std::mem::size_of::<PoolState>();
        let mut data = Vec::with_capacity(8 + struct_size);
        data.extend_from_slice(&PoolState::DISCRIMINATOR);
        data.extend_from_slice(&vec![0u8; struct_size]);

        let parsed = Account::parse(&data).unwrap();
        assert!(matches!(parsed, Account::PoolState(_)));
    }

    #[test]
    fn test_account_enum_unknown_discriminator() {
        use raydium_clmm::utils::Account;
        let data = vec![0u8; 4096];
        assert!(Account::parse(&data).is_err());
    }

    // ── Event discriminators ────────────────────────────────────────

    #[test]
    fn test_swap_event_discriminator() {
        use raydium_clmm::events::SwapEvent;
        assert_eq!(
            SwapEvent::DISCRIMINATOR,
            [64, 198, 205, 232, 38, 8, 113, 226]
        );
    }

    #[test]
    fn test_pool_created_event_discriminator() {
        use raydium_clmm::events::PoolCreatedEvent;
        assert_eq!(
            PoolCreatedEvent::DISCRIMINATOR,
            [25, 94, 75, 47, 112, 99, 53, 63]
        );
    }

    #[test]
    fn test_create_personal_position_event_discriminator() {
        use raydium_clmm::events::CreatePersonalPositionEvent;
        assert_eq!(
            CreatePersonalPositionEvent::DISCRIMINATOR,
            [100, 30, 87, 249, 196, 223, 154, 206]
        );
    }

    #[test]
    fn test_increase_liquidity_event_discriminator() {
        use raydium_clmm::events::IncreaseLiquidityEvent;
        assert_eq!(
            IncreaseLiquidityEvent::DISCRIMINATOR,
            [49, 79, 105, 212, 32, 34, 30, 84]
        );
    }

    #[test]
    fn test_decrease_liquidity_event_discriminator() {
        use raydium_clmm::events::DecreaseLiquidityEvent;
        assert_eq!(
            DecreaseLiquidityEvent::DISCRIMINATOR,
            [58, 222, 86, 58, 68, 50, 85, 56]
        );
    }

    #[test]
    fn test_liquidity_change_event_discriminator() {
        use raydium_clmm::events::LiquidityChangeEvent;
        assert_eq!(
            LiquidityChangeEvent::DISCRIMINATOR,
            [126, 240, 175, 206, 158, 88, 153, 107]
        );
    }

    #[test]
    fn test_config_change_event_discriminator() {
        use raydium_clmm::events::ConfigChangeEvent;
        assert_eq!(
            ConfigChangeEvent::DISCRIMINATOR,
            [247, 189, 7, 119, 106, 112, 95, 151]
        );
    }

    // ── Event from_logs ─────────────────────────────────────────────

    #[test]
    fn test_swap_event_from_logs() {
        use base64::Engine;
        use borsh::BorshSerialize;
        use raydium_clmm::events::SwapEvent;

        let evt = SwapEvent {
            pool_state: Pubkey::new_unique(),
            sender: Pubkey::new_unique(),
            token_account_0: Pubkey::new_unique(),
            token_account_1: Pubkey::new_unique(),
            amount_0: 1_000_000,
            transfer_fee_0: 0,
            amount_1: 500_000,
            transfer_fee_1: 0,
            zero_for_one: true,
            sqrt_price_x64: 79228162514264337593543950336u128,
            liquidity: 1_000_000_000_000u128,
            tick: 0,
        };

        let mut event_data = Vec::new();
        event_data.extend_from_slice(&SwapEvent::DISCRIMINATOR);
        evt.serialize(&mut event_data).unwrap();

        let encoded = base64::engine::general_purpose::STANDARD.encode(&event_data);
        let log_line = format!("Program data: {encoded}");

        let logs: Vec<&str> = vec![&log_line];
        let events = SwapEvent::from_logs(&logs);
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].amount_0, 1_000_000);
        assert_eq!(events[0].amount_1, 500_000);
        assert!(events[0].zero_for_one);
        assert_eq!(events[0].tick, 0);
    }

    #[test]
    fn test_pool_created_event_from_logs() {
        use base64::Engine;
        use borsh::BorshSerialize;
        use raydium_clmm::events::PoolCreatedEvent;

        let evt = PoolCreatedEvent {
            token_mint_0: Pubkey::new_unique(),
            token_mint_1: Pubkey::new_unique(),
            tick_spacing: 10,
            pool_state: Pubkey::new_unique(),
            sqrt_price_x64: 79228162514264337593543950336u128,
            tick: 0,
            token_vault_0: Pubkey::new_unique(),
            token_vault_1: Pubkey::new_unique(),
        };

        let mut event_data = Vec::new();
        event_data.extend_from_slice(&PoolCreatedEvent::DISCRIMINATOR);
        evt.serialize(&mut event_data).unwrap();

        let encoded = base64::engine::general_purpose::STANDARD.encode(&event_data);
        let log_line = format!("Program data: {encoded}");

        let events = PoolCreatedEvent::from_logs(&[log_line.as_str()]);
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].tick_spacing, 10);
        assert_eq!(events[0].pool_state, evt.pool_state);
        assert_eq!(events[0].token_mint_0, evt.token_mint_0);
    }

    #[test]
    fn test_collect_personal_fee_from_logs() {
        use base64::Engine;
        use borsh::BorshSerialize;
        use raydium_clmm::events::CollectPersonalFeeEvent;

        let evt = CollectPersonalFeeEvent {
            position_nft_mint: Pubkey::new_unique(),
            recipient_token_account_0: Pubkey::new_unique(),
            recipient_token_account_1: Pubkey::new_unique(),
            amount_0: 100_000,
            amount_1: 200_000,
        };

        let mut event_data = Vec::new();
        event_data.extend_from_slice(&CollectPersonalFeeEvent::DISCRIMINATOR);
        evt.serialize(&mut event_data).unwrap();

        let encoded = base64::engine::general_purpose::STANDARD.encode(&event_data);
        let log_line = format!("Program data: {encoded}");

        let events = CollectPersonalFeeEvent::from_logs(&[log_line.as_str()]);
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].amount_0, 100_000);
        assert_eq!(events[0].amount_1, 200_000);
    }

    #[test]
    fn test_event_from_logs_no_match() {
        use raydium_clmm::events::SwapEvent;
        let logs: Vec<&str> = vec!["Program log: Hello"];
        assert!(SwapEvent::from_logs(&logs).is_empty());
    }

    // ── Event from_cpi_logs ─────────────────────────────────────────

    #[test]
    fn test_swap_event_from_cpi_logs() {
        use borsh::BorshSerialize;
        use raydium_clmm::events::SwapEvent;

        let evt = SwapEvent {
            pool_state: Pubkey::new_unique(),
            sender: Pubkey::new_unique(),
            token_account_0: Pubkey::new_unique(),
            token_account_1: Pubkey::new_unique(),
            amount_0: 2_000_000,
            transfer_fee_0: 100,
            amount_1: 1_000_000,
            transfer_fee_1: 50,
            zero_for_one: false,
            sqrt_price_x64: 100_000_000_000u128,
            liquidity: 500_000_000_000u128,
            tick: -10,
        };

        // Build bs58-encoded inner instruction data:
        // [8-byte CPI event tag][8-byte discriminator][borsh payload]
        let mut raw = vec![0u8; 8]; // CPI event tag (arbitrary for test)
        raw.extend_from_slice(&SwapEvent::DISCRIMINATOR);
        evt.serialize(&mut raw).unwrap();

        let bs58_data = bs58::encode(&raw).into_string();

        let events = SwapEvent::from_cpi_logs([bs58_data.as_str()]);
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].amount_0, 2_000_000);
        assert!(!events[0].zero_for_one);
    }

    #[test]
    fn test_from_cpi_logs_ignores_wrong_discriminator() {
        use borsh::BorshSerialize;
        use raydium_clmm::events::{CollectPersonalFeeEvent, SwapEvent};

        let fee = CollectPersonalFeeEvent {
            position_nft_mint: Pubkey::new_unique(),
            recipient_token_account_0: Pubkey::new_unique(),
            recipient_token_account_1: Pubkey::new_unique(),
            amount_0: 1_000,
            amount_1: 500,
        };

        let mut raw = vec![0u8; 8];
        raw.extend_from_slice(&CollectPersonalFeeEvent::DISCRIMINATOR);
        fee.serialize(&mut raw).unwrap();

        let bs58_data = bs58::encode(&raw).into_string();

        // SwapEvent should not match CollectPersonalFeeEvent data
        let events = SwapEvent::from_cpi_logs([bs58_data.as_str()]);
        assert!(events.is_empty());
    }

    #[test]
    fn test_from_cpi_logs_empty() {
        use raydium_clmm::events::SwapEvent;
        let events = SwapEvent::from_cpi_logs(Vec::<&str>::new());
        assert!(events.is_empty());
    }

    // ── Event enum (utils) ──────────────────────────────────────────

    #[test]
    fn test_event_enum_from_logs() {
        use base64::Engine;
        use borsh::BorshSerialize;
        use raydium_clmm::events::PoolCreatedEvent;
        use raydium_clmm::utils::Event;

        let evt = PoolCreatedEvent {
            token_mint_0: Pubkey::new_unique(),
            token_mint_1: Pubkey::new_unique(),
            tick_spacing: 60,
            pool_state: Pubkey::new_unique(),
            sqrt_price_x64: 1_000_000u128,
            tick: 42,
            token_vault_0: Pubkey::new_unique(),
            token_vault_1: Pubkey::new_unique(),
        };

        let mut event_data = Vec::new();
        event_data.extend_from_slice(&PoolCreatedEvent::DISCRIMINATOR);
        evt.serialize(&mut event_data).unwrap();

        let encoded = base64::engine::general_purpose::STANDARD.encode(&event_data);
        let log_line = format!("Program data: {encoded}");

        let events = Event::from_logs(&[log_line.as_str()]);
        assert_eq!(events.len(), 1);
        match &events[0] {
            Event::PoolCreatedEvent(e) => {
                assert_eq!(e.tick_spacing, 60);
                assert_eq!(e.tick, 42);
            }
            other => panic!("Expected PoolCreatedEvent, got {:?}", other),
        }
    }

    #[test]
    fn test_event_enum_from_cpi_logs() {
        use borsh::BorshSerialize;
        use raydium_clmm::events::SwapEvent;
        use raydium_clmm::utils::Event;

        let evt = SwapEvent {
            pool_state: Pubkey::new_unique(),
            sender: Pubkey::new_unique(),
            token_account_0: Pubkey::new_unique(),
            token_account_1: Pubkey::new_unique(),
            amount_0: 5_000_000,
            transfer_fee_0: 0,
            amount_1: 4_500_000,
            transfer_fee_1: 0,
            zero_for_one: true,
            sqrt_price_x64: 0,
            liquidity: 0,
            tick: 0,
        };

        // Build bs58-encoded inner instruction data
        let mut raw = vec![0u8; 8]; // CPI event tag
        raw.extend_from_slice(&SwapEvent::DISCRIMINATOR);
        evt.serialize(&mut raw).unwrap();

        let bs58_data = bs58::encode(&raw).into_string();

        let events = Event::from_cpi_logs([bs58_data.as_str()]);
        assert_eq!(events.len(), 1);
        assert!(matches!(&events[0], Event::SwapEvent(_)));
    }

    #[test]
    fn test_event_enum_from_logs_multiple() {
        use base64::Engine;
        use borsh::BorshSerialize;
        use raydium_clmm::events::{CollectPersonalFeeEvent, SwapEvent};
        use raydium_clmm::utils::Event;

        let swap = SwapEvent {
            pool_state: Pubkey::new_unique(),
            sender: Pubkey::new_unique(),
            token_account_0: Pubkey::new_unique(),
            token_account_1: Pubkey::new_unique(),
            amount_0: 100,
            transfer_fee_0: 0,
            amount_1: 200,
            transfer_fee_1: 0,
            zero_for_one: true,
            sqrt_price_x64: 0,
            liquidity: 0,
            tick: 0,
        };
        let mut sd = Vec::new();
        sd.extend_from_slice(&SwapEvent::DISCRIMINATOR);
        swap.serialize(&mut sd).unwrap();
        let swap_log = format!(
            "Program data: {}",
            base64::engine::general_purpose::STANDARD.encode(&sd)
        );

        let fee = CollectPersonalFeeEvent {
            position_nft_mint: Pubkey::new_unique(),
            recipient_token_account_0: Pubkey::new_unique(),
            recipient_token_account_1: Pubkey::new_unique(),
            amount_0: 10,
            amount_1: 20,
        };
        let mut fd = Vec::new();
        fd.extend_from_slice(&CollectPersonalFeeEvent::DISCRIMINATOR);
        fee.serialize(&mut fd).unwrap();
        let fee_log = format!(
            "Program data: {}",
            base64::engine::general_purpose::STANDARD.encode(&fd)
        );

        let logs: Vec<&str> = vec![&swap_log, &fee_log];
        let events = Event::from_logs(&logs);
        assert_eq!(events.len(), 2);
        assert!(matches!(&events[0], Event::SwapEvent(_)));
        assert!(matches!(&events[1], Event::CollectPersonalFeeEvent(_)));
    }

    #[test]
    fn test_event_enum_no_match() {
        use raydium_clmm::utils::Event;
        let logs: Vec<&str> = vec!["Program log: nothing"];
        assert!(Event::from_logs(&logs).is_empty());
    }

    fn assert_ix(
        actual: &solana_sdk::instruction::Instruction,
        expect: &solana_sdk::instruction::Instruction,
    ) {
        assert_eq!(actual.program_id, expect.program_id, "program_id mismatch");
        assert_eq!(actual.data, expect.data, "data mismatch");
        assert_eq!(
            actual.accounts.len(),
            expect.accounts.len(),
            "accounts len mismatch"
        );
        for (i, (g, e)) in actual
            .accounts
            .iter()
            .zip(expect.accounts.iter())
            .enumerate()
        {
            assert_eq!(g.pubkey, e.pubkey, "pubkey mismatch at account[{}]", i);
            assert_eq!(
                g.is_signer, e.is_signer,
                "is_signer mismatch at account[{}]",
                i
            );
            assert_eq!(
                g.is_writable, e.is_writable,
                "is_writable mismatch at account[{}]",
                i
            );
        }
    }

    // ── Instructions ────────────────────────────────────────────────

    #[test]
    fn test_close_position_instruction() {
        use raydium_clmm::instructions;
        use solana_sdk::instruction::AccountMeta;

        let program_id = raydium_clmm::ID;
        let nft_owner = Pubkey::from([1; 32]);
        let position_nft_mint = Pubkey::from([2; 32]);
        let position_nft_account = Pubkey::from([3; 32]);
        let personal_position = Pubkey::from([4; 32]);
        let system_program = Pubkey::from([5; 32]);
        let token_program = Pubkey::from([6; 32]);

        let expected_data: Vec<u8> = vec![123, 134, 81, 0, 49, 68, 98, 98];

        let expected_accounts = vec![
            AccountMeta::new(nft_owner, true),
            AccountMeta::new(position_nft_mint, false),
            AccountMeta::new(position_nft_account, false),
            AccountMeta::new(personal_position, false),
            AccountMeta::new_readonly(system_program, false),
            AccountMeta::new_readonly(token_program, false),
        ];

        let gen_ix = instructions::close_position(
            &program_id,
            &instructions::ClosePositionAccounts {
                nft_owner,
                position_nft_mint,
                position_nft_account,
                personal_position,
                system_program,
                token_program,
            },
        );

        let expected_ix = solana_sdk::instruction::Instruction::new_with_bytes(
            program_id,
            &expected_data,
            expected_accounts,
        );
        assert_ix(&gen_ix, &expected_ix);
    }

    #[test]
    fn test_collect_fund_fee_instruction() {
        use raydium_clmm::instructions;
        #[allow(unused_imports)]
        use raydium_clmm::types;
        use solana_sdk::instruction::AccountMeta;

        let program_id = raydium_clmm::ID;
        let owner = Pubkey::from([1; 32]);
        let pool_state = Pubkey::from([2; 32]);
        let amm_config = Pubkey::from([3; 32]);
        let token_vault_0 = Pubkey::from([4; 32]);
        let token_vault_1 = Pubkey::from([5; 32]);
        let vault_0_mint = Pubkey::from([6; 32]);
        let vault_1_mint = Pubkey::from([7; 32]);
        let recipient_token_account_0 = Pubkey::from([8; 32]);
        let recipient_token_account_1 = Pubkey::from([9; 32]);
        let token_program = Pubkey::from([10; 32]);
        let token_program_2022 = Pubkey::from([11; 32]);
        let amount_0_requested = 0u64;
        let amount_1_requested = 0u64;

        let mut expected_data: Vec<u8> = vec![167, 138, 78, 149, 223, 194, 6, 126];
        borsh::BorshSerialize::serialize(&amount_0_requested, &mut expected_data).unwrap();
        borsh::BorshSerialize::serialize(&amount_1_requested, &mut expected_data).unwrap();

        let expected_accounts = vec![
            AccountMeta::new_readonly(owner, true),
            AccountMeta::new(pool_state, false),
            AccountMeta::new_readonly(amm_config, false),
            AccountMeta::new(token_vault_0, false),
            AccountMeta::new(token_vault_1, false),
            AccountMeta::new_readonly(vault_0_mint, false),
            AccountMeta::new_readonly(vault_1_mint, false),
            AccountMeta::new(recipient_token_account_0, false),
            AccountMeta::new(recipient_token_account_1, false),
            AccountMeta::new_readonly(token_program, false),
            AccountMeta::new_readonly(token_program_2022, false),
        ];

        let gen_ix = instructions::collect_fund_fee(
            &program_id,
            &instructions::CollectFundFeeAccounts {
                owner,
                pool_state,
                amm_config,
                token_vault_0,
                token_vault_1,
                vault_0_mint,
                vault_1_mint,
                recipient_token_account_0,
                recipient_token_account_1,
                token_program,
                token_program_2022,
            },
            amount_0_requested,
            amount_1_requested,
        );

        let expected_ix = solana_sdk::instruction::Instruction::new_with_bytes(
            program_id,
            &expected_data,
            expected_accounts,
        );
        assert_ix(&gen_ix, &expected_ix);
    }

    #[test]
    fn test_collect_protocol_fee_instruction() {
        use raydium_clmm::instructions;
        #[allow(unused_imports)]
        use raydium_clmm::types;
        use solana_sdk::instruction::AccountMeta;

        let program_id = raydium_clmm::ID;
        let owner = Pubkey::from([1; 32]);
        let pool_state = Pubkey::from([2; 32]);
        let amm_config = Pubkey::from([3; 32]);
        let token_vault_0 = Pubkey::from([4; 32]);
        let token_vault_1 = Pubkey::from([5; 32]);
        let vault_0_mint = Pubkey::from([6; 32]);
        let vault_1_mint = Pubkey::from([7; 32]);
        let recipient_token_account_0 = Pubkey::from([8; 32]);
        let recipient_token_account_1 = Pubkey::from([9; 32]);
        let token_program = Pubkey::from([10; 32]);
        let token_program_2022 = Pubkey::from([11; 32]);
        let amount_0_requested = 0u64;
        let amount_1_requested = 0u64;

        let mut expected_data: Vec<u8> = vec![136, 136, 252, 221, 194, 66, 126, 89];
        borsh::BorshSerialize::serialize(&amount_0_requested, &mut expected_data).unwrap();
        borsh::BorshSerialize::serialize(&amount_1_requested, &mut expected_data).unwrap();

        let expected_accounts = vec![
            AccountMeta::new_readonly(owner, true),
            AccountMeta::new(pool_state, false),
            AccountMeta::new_readonly(amm_config, false),
            AccountMeta::new(token_vault_0, false),
            AccountMeta::new(token_vault_1, false),
            AccountMeta::new_readonly(vault_0_mint, false),
            AccountMeta::new_readonly(vault_1_mint, false),
            AccountMeta::new(recipient_token_account_0, false),
            AccountMeta::new(recipient_token_account_1, false),
            AccountMeta::new_readonly(token_program, false),
            AccountMeta::new_readonly(token_program_2022, false),
        ];

        let gen_ix = instructions::collect_protocol_fee(
            &program_id,
            &instructions::CollectProtocolFeeAccounts {
                owner,
                pool_state,
                amm_config,
                token_vault_0,
                token_vault_1,
                vault_0_mint,
                vault_1_mint,
                recipient_token_account_0,
                recipient_token_account_1,
                token_program,
                token_program_2022,
            },
            amount_0_requested,
            amount_1_requested,
        );

        let expected_ix = solana_sdk::instruction::Instruction::new_with_bytes(
            program_id,
            &expected_data,
            expected_accounts,
        );
        assert_ix(&gen_ix, &expected_ix);
    }

    #[test]
    fn test_collect_remaining_rewards_instruction() {
        use raydium_clmm::instructions;
        #[allow(unused_imports)]
        use raydium_clmm::types;
        use solana_sdk::instruction::AccountMeta;

        let program_id = raydium_clmm::ID;
        let reward_funder = Pubkey::from([1; 32]);
        let funder_token_account = Pubkey::from([2; 32]);
        let pool_state = Pubkey::from([3; 32]);
        let reward_token_vault = Pubkey::from([4; 32]);
        let reward_vault_mint = Pubkey::from([5; 32]);
        let token_program = Pubkey::from([6; 32]);
        let token_program_2022 = Pubkey::from([7; 32]);
        let memo_program = Pubkey::from([8; 32]);
        let reward_index = 0u8;

        let mut expected_data: Vec<u8> = vec![18, 237, 166, 197, 34, 16, 213, 144];
        borsh::BorshSerialize::serialize(&reward_index, &mut expected_data).unwrap();

        let expected_accounts = vec![
            AccountMeta::new_readonly(reward_funder, true),
            AccountMeta::new(funder_token_account, false),
            AccountMeta::new(pool_state, false),
            AccountMeta::new_readonly(reward_token_vault, false),
            AccountMeta::new_readonly(reward_vault_mint, false),
            AccountMeta::new_readonly(token_program, false),
            AccountMeta::new_readonly(token_program_2022, false),
            AccountMeta::new_readonly(memo_program, false),
        ];

        let gen_ix = instructions::collect_remaining_rewards(
            &program_id,
            &instructions::CollectRemainingRewardsAccounts {
                reward_funder,
                funder_token_account,
                pool_state,
                reward_token_vault,
                reward_vault_mint,
                token_program,
                token_program_2022,
                memo_program,
            },
            reward_index,
        );

        let expected_ix = solana_sdk::instruction::Instruction::new_with_bytes(
            program_id,
            &expected_data,
            expected_accounts,
        );
        assert_ix(&gen_ix, &expected_ix);
    }

    #[test]
    fn test_create_amm_config_instruction() {
        use raydium_clmm::instructions;
        #[allow(unused_imports)]
        use raydium_clmm::types;
        use solana_sdk::instruction::AccountMeta;

        let program_id = raydium_clmm::ID;
        let owner = Pubkey::from([1; 32]);
        let amm_config = Pubkey::from([2; 32]);
        let system_program = Pubkey::from([3; 32]);
        let index = 0u16;
        let tick_spacing = 0u16;
        let trade_fee_rate = 0u32;
        let protocol_fee_rate = 0u32;
        let fund_fee_rate = 0u32;

        let mut expected_data: Vec<u8> = vec![137, 52, 237, 212, 215, 117, 108, 104];
        borsh::BorshSerialize::serialize(&index, &mut expected_data).unwrap();
        borsh::BorshSerialize::serialize(&tick_spacing, &mut expected_data).unwrap();
        borsh::BorshSerialize::serialize(&trade_fee_rate, &mut expected_data).unwrap();
        borsh::BorshSerialize::serialize(&protocol_fee_rate, &mut expected_data).unwrap();
        borsh::BorshSerialize::serialize(&fund_fee_rate, &mut expected_data).unwrap();

        let expected_accounts = vec![
            AccountMeta::new(owner, true),
            AccountMeta::new(amm_config, false),
            AccountMeta::new_readonly(system_program, false),
        ];

        let gen_ix = instructions::create_amm_config(
            &program_id,
            &instructions::CreateAmmConfigAccounts {
                owner,
                amm_config,
                system_program,
            },
            index,
            tick_spacing,
            trade_fee_rate,
            protocol_fee_rate,
            fund_fee_rate,
        );

        let expected_ix = solana_sdk::instruction::Instruction::new_with_bytes(
            program_id,
            &expected_data,
            expected_accounts,
        );
        assert_ix(&gen_ix, &expected_ix);
    }

    #[test]
    fn test_create_operation_account_instruction() {
        use raydium_clmm::instructions;
        use solana_sdk::instruction::AccountMeta;

        let program_id = raydium_clmm::ID;
        let owner = Pubkey::from([1; 32]);
        let operation_state = Pubkey::from([2; 32]);
        let system_program = Pubkey::from([3; 32]);

        let expected_data: Vec<u8> = vec![63, 87, 148, 33, 109, 35, 8, 104];

        let expected_accounts = vec![
            AccountMeta::new(owner, true),
            AccountMeta::new(operation_state, false),
            AccountMeta::new_readonly(system_program, false),
        ];

        let gen_ix = instructions::create_operation_account(
            &program_id,
            &instructions::CreateOperationAccountAccounts {
                owner,
                operation_state,
                system_program,
            },
        );

        let expected_ix = solana_sdk::instruction::Instruction::new_with_bytes(
            program_id,
            &expected_data,
            expected_accounts,
        );
        assert_ix(&gen_ix, &expected_ix);
    }

    #[test]
    fn test_create_pool_instruction() {
        use raydium_clmm::instructions;
        #[allow(unused_imports)]
        use raydium_clmm::types;
        use solana_sdk::instruction::AccountMeta;

        let program_id = raydium_clmm::ID;
        let pool_creator = Pubkey::from([1; 32]);
        let amm_config = Pubkey::from([2; 32]);
        let pool_state = Pubkey::from([3; 32]);
        let token_mint_0 = Pubkey::from([4; 32]);
        let token_mint_1 = Pubkey::from([5; 32]);
        let token_vault_0 = Pubkey::from([6; 32]);
        let token_vault_1 = Pubkey::from([7; 32]);
        let observation_state = Pubkey::from([8; 32]);
        let tick_array_bitmap = Pubkey::from([9; 32]);
        let token_program_0 = Pubkey::from([10; 32]);
        let token_program_1 = Pubkey::from([11; 32]);
        let system_program = Pubkey::from([12; 32]);
        let rent = Pubkey::from([13; 32]);
        let sqrt_price_x64 = 0u128;
        let open_time = 0u64;

        let mut expected_data: Vec<u8> = vec![233, 146, 209, 142, 207, 104, 64, 188];
        borsh::BorshSerialize::serialize(&sqrt_price_x64, &mut expected_data).unwrap();
        borsh::BorshSerialize::serialize(&open_time, &mut expected_data).unwrap();

        let expected_accounts = vec![
            AccountMeta::new(pool_creator, true),
            AccountMeta::new_readonly(amm_config, false),
            AccountMeta::new(pool_state, false),
            AccountMeta::new_readonly(token_mint_0, false),
            AccountMeta::new_readonly(token_mint_1, false),
            AccountMeta::new(token_vault_0, false),
            AccountMeta::new(token_vault_1, false),
            AccountMeta::new(observation_state, false),
            AccountMeta::new(tick_array_bitmap, false),
            AccountMeta::new_readonly(token_program_0, false),
            AccountMeta::new_readonly(token_program_1, false),
            AccountMeta::new_readonly(system_program, false),
            AccountMeta::new_readonly(rent, false),
        ];

        let gen_ix = instructions::create_pool(
            &program_id,
            &instructions::CreatePoolAccounts {
                pool_creator,
                amm_config,
                pool_state,
                token_mint_0,
                token_mint_1,
                token_vault_0,
                token_vault_1,
                observation_state,
                tick_array_bitmap,
                token_program_0,
                token_program_1,
                system_program,
                rent,
            },
            sqrt_price_x64,
            open_time,
        );

        let expected_ix = solana_sdk::instruction::Instruction::new_with_bytes(
            program_id,
            &expected_data,
            expected_accounts,
        );
        assert_ix(&gen_ix, &expected_ix);
    }

    #[test]
    fn test_create_support_mint_associated_instruction() {
        use raydium_clmm::instructions;
        use solana_sdk::instruction::AccountMeta;

        let program_id = raydium_clmm::ID;
        let owner = Pubkey::from([1; 32]);
        let token_mint = Pubkey::from([2; 32]);
        let support_mint_associated = Pubkey::from([3; 32]);
        let system_program = Pubkey::from([4; 32]);

        let expected_data: Vec<u8> = vec![17, 251, 65, 92, 136, 242, 14, 169];

        let expected_accounts = vec![
            AccountMeta::new(owner, true),
            AccountMeta::new_readonly(token_mint, false),
            AccountMeta::new(support_mint_associated, false),
            AccountMeta::new_readonly(system_program, false),
        ];

        let gen_ix = instructions::create_support_mint_associated(
            &program_id,
            &instructions::CreateSupportMintAssociatedAccounts {
                owner,
                token_mint,
                support_mint_associated,
                system_program,
            },
        );

        let expected_ix = solana_sdk::instruction::Instruction::new_with_bytes(
            program_id,
            &expected_data,
            expected_accounts,
        );
        assert_ix(&gen_ix, &expected_ix);
    }

    #[test]
    fn test_decrease_liquidity_instruction() {
        use raydium_clmm::instructions;
        #[allow(unused_imports)]
        use raydium_clmm::types;
        use solana_sdk::instruction::AccountMeta;

        let program_id = raydium_clmm::ID;
        let nft_owner = Pubkey::from([1; 32]);
        let nft_account = Pubkey::from([2; 32]);
        let personal_position = Pubkey::from([3; 32]);
        let pool_state = Pubkey::from([4; 32]);
        let protocol_position = Pubkey::from([5; 32]);
        let token_vault_0 = Pubkey::from([6; 32]);
        let token_vault_1 = Pubkey::from([7; 32]);
        let tick_array_lower = Pubkey::from([8; 32]);
        let tick_array_upper = Pubkey::from([9; 32]);
        let recipient_token_account_0 = Pubkey::from([10; 32]);
        let recipient_token_account_1 = Pubkey::from([11; 32]);
        let token_program = Pubkey::from([12; 32]);
        let liquidity = 0u128;
        let amount_0_min = 0u64;
        let amount_1_min = 0u64;

        let mut expected_data: Vec<u8> = vec![160, 38, 208, 111, 104, 91, 44, 1];
        borsh::BorshSerialize::serialize(&liquidity, &mut expected_data).unwrap();
        borsh::BorshSerialize::serialize(&amount_0_min, &mut expected_data).unwrap();
        borsh::BorshSerialize::serialize(&amount_1_min, &mut expected_data).unwrap();

        let expected_accounts = vec![
            AccountMeta::new_readonly(nft_owner, true),
            AccountMeta::new_readonly(nft_account, false),
            AccountMeta::new(personal_position, false),
            AccountMeta::new(pool_state, false),
            AccountMeta::new(protocol_position, false),
            AccountMeta::new(token_vault_0, false),
            AccountMeta::new(token_vault_1, false),
            AccountMeta::new(tick_array_lower, false),
            AccountMeta::new(tick_array_upper, false),
            AccountMeta::new(recipient_token_account_0, false),
            AccountMeta::new(recipient_token_account_1, false),
            AccountMeta::new_readonly(token_program, false),
        ];

        let gen_ix = instructions::decrease_liquidity(
            &program_id,
            &instructions::DecreaseLiquidityAccounts {
                nft_owner,
                nft_account,
                personal_position,
                pool_state,
                protocol_position,
                token_vault_0,
                token_vault_1,
                tick_array_lower,
                tick_array_upper,
                recipient_token_account_0,
                recipient_token_account_1,
                token_program,
            },
            liquidity,
            amount_0_min,
            amount_1_min,
        );

        let expected_ix = solana_sdk::instruction::Instruction::new_with_bytes(
            program_id,
            &expected_data,
            expected_accounts,
        );
        assert_ix(&gen_ix, &expected_ix);
    }

    #[test]
    fn test_decrease_liquidity_v2_instruction() {
        use raydium_clmm::instructions;
        #[allow(unused_imports)]
        use raydium_clmm::types;
        use solana_sdk::instruction::AccountMeta;

        let program_id = raydium_clmm::ID;
        let nft_owner = Pubkey::from([1; 32]);
        let nft_account = Pubkey::from([2; 32]);
        let personal_position = Pubkey::from([3; 32]);
        let pool_state = Pubkey::from([4; 32]);
        let protocol_position = Pubkey::from([5; 32]);
        let token_vault_0 = Pubkey::from([6; 32]);
        let token_vault_1 = Pubkey::from([7; 32]);
        let tick_array_lower = Pubkey::from([8; 32]);
        let tick_array_upper = Pubkey::from([9; 32]);
        let recipient_token_account_0 = Pubkey::from([10; 32]);
        let recipient_token_account_1 = Pubkey::from([11; 32]);
        let token_program = Pubkey::from([12; 32]);
        let token_program_2022 = Pubkey::from([13; 32]);
        let memo_program = Pubkey::from([14; 32]);
        let vault_0_mint = Pubkey::from([15; 32]);
        let vault_1_mint = Pubkey::from([16; 32]);
        let liquidity = 0u128;
        let amount_0_min = 0u64;
        let amount_1_min = 0u64;

        let mut expected_data: Vec<u8> = vec![58, 127, 188, 62, 79, 82, 196, 96];
        borsh::BorshSerialize::serialize(&liquidity, &mut expected_data).unwrap();
        borsh::BorshSerialize::serialize(&amount_0_min, &mut expected_data).unwrap();
        borsh::BorshSerialize::serialize(&amount_1_min, &mut expected_data).unwrap();

        let expected_accounts = vec![
            AccountMeta::new_readonly(nft_owner, true),
            AccountMeta::new_readonly(nft_account, false),
            AccountMeta::new(personal_position, false),
            AccountMeta::new(pool_state, false),
            AccountMeta::new(protocol_position, false),
            AccountMeta::new(token_vault_0, false),
            AccountMeta::new(token_vault_1, false),
            AccountMeta::new(tick_array_lower, false),
            AccountMeta::new(tick_array_upper, false),
            AccountMeta::new(recipient_token_account_0, false),
            AccountMeta::new(recipient_token_account_1, false),
            AccountMeta::new_readonly(token_program, false),
            AccountMeta::new_readonly(token_program_2022, false),
            AccountMeta::new_readonly(memo_program, false),
            AccountMeta::new_readonly(vault_0_mint, false),
            AccountMeta::new_readonly(vault_1_mint, false),
        ];

        let gen_ix = instructions::decrease_liquidity_v2(
            &program_id,
            &instructions::DecreaseLiquidityV2Accounts {
                nft_owner,
                nft_account,
                personal_position,
                pool_state,
                protocol_position,
                token_vault_0,
                token_vault_1,
                tick_array_lower,
                tick_array_upper,
                recipient_token_account_0,
                recipient_token_account_1,
                token_program,
                token_program_2022,
                memo_program,
                vault_0_mint,
                vault_1_mint,
            },
            liquidity,
            amount_0_min,
            amount_1_min,
        );

        let expected_ix = solana_sdk::instruction::Instruction::new_with_bytes(
            program_id,
            &expected_data,
            expected_accounts,
        );
        assert_ix(&gen_ix, &expected_ix);
    }

    #[test]
    fn test_increase_liquidity_instruction() {
        use raydium_clmm::instructions;
        #[allow(unused_imports)]
        use raydium_clmm::types;
        use solana_sdk::instruction::AccountMeta;

        let program_id = raydium_clmm::ID;
        let nft_owner = Pubkey::from([1; 32]);
        let nft_account = Pubkey::from([2; 32]);
        let pool_state = Pubkey::from([3; 32]);
        let protocol_position = Pubkey::from([4; 32]);
        let personal_position = Pubkey::from([5; 32]);
        let tick_array_lower = Pubkey::from([6; 32]);
        let tick_array_upper = Pubkey::from([7; 32]);
        let token_account_0 = Pubkey::from([8; 32]);
        let token_account_1 = Pubkey::from([9; 32]);
        let token_vault_0 = Pubkey::from([10; 32]);
        let token_vault_1 = Pubkey::from([11; 32]);
        let token_program = Pubkey::from([12; 32]);
        let liquidity = 0u128;
        let amount_0_max = 0u64;
        let amount_1_max = 0u64;

        let mut expected_data: Vec<u8> = vec![46, 156, 243, 118, 13, 205, 251, 178];
        borsh::BorshSerialize::serialize(&liquidity, &mut expected_data).unwrap();
        borsh::BorshSerialize::serialize(&amount_0_max, &mut expected_data).unwrap();
        borsh::BorshSerialize::serialize(&amount_1_max, &mut expected_data).unwrap();

        let expected_accounts = vec![
            AccountMeta::new_readonly(nft_owner, true),
            AccountMeta::new_readonly(nft_account, false),
            AccountMeta::new(pool_state, false),
            AccountMeta::new(protocol_position, false),
            AccountMeta::new(personal_position, false),
            AccountMeta::new(tick_array_lower, false),
            AccountMeta::new(tick_array_upper, false),
            AccountMeta::new(token_account_0, false),
            AccountMeta::new(token_account_1, false),
            AccountMeta::new(token_vault_0, false),
            AccountMeta::new(token_vault_1, false),
            AccountMeta::new_readonly(token_program, false),
        ];

        let gen_ix = instructions::increase_liquidity(
            &program_id,
            &instructions::IncreaseLiquidityAccounts {
                nft_owner,
                nft_account,
                pool_state,
                protocol_position,
                personal_position,
                tick_array_lower,
                tick_array_upper,
                token_account_0,
                token_account_1,
                token_vault_0,
                token_vault_1,
                token_program,
            },
            liquidity,
            amount_0_max,
            amount_1_max,
        );

        let expected_ix = solana_sdk::instruction::Instruction::new_with_bytes(
            program_id,
            &expected_data,
            expected_accounts,
        );
        assert_ix(&gen_ix, &expected_ix);
    }

    #[test]
    fn test_increase_liquidity_v2_instruction() {
        use raydium_clmm::instructions;
        #[allow(unused_imports)]
        use raydium_clmm::types;
        use solana_sdk::instruction::AccountMeta;

        let program_id = raydium_clmm::ID;
        let nft_owner = Pubkey::from([1; 32]);
        let nft_account = Pubkey::from([2; 32]);
        let pool_state = Pubkey::from([3; 32]);
        let protocol_position = Pubkey::from([4; 32]);
        let personal_position = Pubkey::from([5; 32]);
        let tick_array_lower = Pubkey::from([6; 32]);
        let tick_array_upper = Pubkey::from([7; 32]);
        let token_account_0 = Pubkey::from([8; 32]);
        let token_account_1 = Pubkey::from([9; 32]);
        let token_vault_0 = Pubkey::from([10; 32]);
        let token_vault_1 = Pubkey::from([11; 32]);
        let token_program = Pubkey::from([12; 32]);
        let token_program_2022 = Pubkey::from([13; 32]);
        let vault_0_mint = Pubkey::from([14; 32]);
        let vault_1_mint = Pubkey::from([15; 32]);
        let liquidity = 0u128;
        let amount_0_max = 0u64;
        let amount_1_max = 0u64;
        let base_flag = None;

        let mut expected_data: Vec<u8> = vec![133, 29, 89, 223, 69, 238, 176, 10];
        borsh::BorshSerialize::serialize(&liquidity, &mut expected_data).unwrap();
        borsh::BorshSerialize::serialize(&amount_0_max, &mut expected_data).unwrap();
        borsh::BorshSerialize::serialize(&amount_1_max, &mut expected_data).unwrap();
        borsh::BorshSerialize::serialize(&base_flag, &mut expected_data).unwrap();

        let expected_accounts = vec![
            AccountMeta::new_readonly(nft_owner, true),
            AccountMeta::new_readonly(nft_account, false),
            AccountMeta::new(pool_state, false),
            AccountMeta::new(protocol_position, false),
            AccountMeta::new(personal_position, false),
            AccountMeta::new(tick_array_lower, false),
            AccountMeta::new(tick_array_upper, false),
            AccountMeta::new(token_account_0, false),
            AccountMeta::new(token_account_1, false),
            AccountMeta::new(token_vault_0, false),
            AccountMeta::new(token_vault_1, false),
            AccountMeta::new_readonly(token_program, false),
            AccountMeta::new_readonly(token_program_2022, false),
            AccountMeta::new_readonly(vault_0_mint, false),
            AccountMeta::new_readonly(vault_1_mint, false),
        ];

        let gen_ix = instructions::increase_liquidity_v2(
            &program_id,
            &instructions::IncreaseLiquidityV2Accounts {
                nft_owner,
                nft_account,
                pool_state,
                protocol_position,
                personal_position,
                tick_array_lower,
                tick_array_upper,
                token_account_0,
                token_account_1,
                token_vault_0,
                token_vault_1,
                token_program,
                token_program_2022,
                vault_0_mint,
                vault_1_mint,
            },
            liquidity,
            amount_0_max,
            amount_1_max,
            base_flag,
        );

        let expected_ix = solana_sdk::instruction::Instruction::new_with_bytes(
            program_id,
            &expected_data,
            expected_accounts,
        );
        assert_ix(&gen_ix, &expected_ix);
    }

    #[test]
    fn test_initialize_reward_instruction() {
        use raydium_clmm::instructions;
        #[allow(unused_imports)]
        use raydium_clmm::types;
        use solana_sdk::instruction::AccountMeta;

        let program_id = raydium_clmm::ID;
        let reward_funder = Pubkey::from([1; 32]);
        let funder_token_account = Pubkey::from([2; 32]);
        let amm_config = Pubkey::from([3; 32]);
        let pool_state = Pubkey::from([4; 32]);
        let operation_state = Pubkey::from([5; 32]);
        let reward_token_mint = Pubkey::from([6; 32]);
        let reward_token_vault = Pubkey::from([7; 32]);
        let reward_token_program = Pubkey::from([8; 32]);
        let system_program = Pubkey::from([9; 32]);
        let rent = Pubkey::from([10; 32]);
        let param = types::InitializeRewardParam {
            open_time: 0u64,
            end_time: 0u64,
            emissions_per_second_x64: 0u128,
        };

        let mut expected_data: Vec<u8> = vec![95, 135, 192, 196, 242, 129, 230, 68];
        borsh::BorshSerialize::serialize(&param, &mut expected_data).unwrap();

        let expected_accounts = vec![
            AccountMeta::new(reward_funder, true),
            AccountMeta::new(funder_token_account, false),
            AccountMeta::new_readonly(amm_config, false),
            AccountMeta::new(pool_state, false),
            AccountMeta::new_readonly(operation_state, false),
            AccountMeta::new_readonly(reward_token_mint, false),
            AccountMeta::new(reward_token_vault, false),
            AccountMeta::new_readonly(reward_token_program, false),
            AccountMeta::new_readonly(system_program, false),
            AccountMeta::new_readonly(rent, false),
        ];

        let gen_ix = instructions::initialize_reward(
            &program_id,
            &instructions::InitializeRewardAccounts {
                reward_funder,
                funder_token_account,
                amm_config,
                pool_state,
                operation_state,
                reward_token_mint,
                reward_token_vault,
                reward_token_program,
                system_program,
                rent,
            },
            param.clone(),
        );

        let expected_ix = solana_sdk::instruction::Instruction::new_with_bytes(
            program_id,
            &expected_data,
            expected_accounts,
        );
        assert_ix(&gen_ix, &expected_ix);
    }

    #[test]
    fn test_open_position_instruction() {
        use raydium_clmm::instructions;
        #[allow(unused_imports)]
        use raydium_clmm::types;
        use solana_sdk::instruction::AccountMeta;

        let program_id = raydium_clmm::ID;
        let payer = Pubkey::from([1; 32]);
        let position_nft_owner = Pubkey::from([2; 32]);
        let position_nft_mint = Pubkey::from([3; 32]);
        let position_nft_account = Pubkey::from([4; 32]);
        let metadata_account = Pubkey::from([5; 32]);
        let pool_state = Pubkey::from([6; 32]);
        let protocol_position = Pubkey::from([7; 32]);
        let tick_array_lower = Pubkey::from([8; 32]);
        let tick_array_upper = Pubkey::from([9; 32]);
        let personal_position = Pubkey::from([10; 32]);
        let token_account_0 = Pubkey::from([11; 32]);
        let token_account_1 = Pubkey::from([12; 32]);
        let token_vault_0 = Pubkey::from([13; 32]);
        let token_vault_1 = Pubkey::from([14; 32]);
        let rent = Pubkey::from([15; 32]);
        let system_program = Pubkey::from([16; 32]);
        let token_program = Pubkey::from([17; 32]);
        let associated_token_program = Pubkey::from([18; 32]);
        let metadata_program = Pubkey::from([19; 32]);
        let tick_lower_index = 0i32;
        let tick_upper_index = 0i32;
        let tick_array_lower_start_index = 0i32;
        let tick_array_upper_start_index = 0i32;
        let liquidity = 0u128;
        let amount_0_max = 0u64;
        let amount_1_max = 0u64;

        let mut expected_data: Vec<u8> = vec![135, 128, 47, 77, 15, 152, 240, 49];
        borsh::BorshSerialize::serialize(&tick_lower_index, &mut expected_data).unwrap();
        borsh::BorshSerialize::serialize(&tick_upper_index, &mut expected_data).unwrap();
        borsh::BorshSerialize::serialize(&tick_array_lower_start_index, &mut expected_data)
            .unwrap();
        borsh::BorshSerialize::serialize(&tick_array_upper_start_index, &mut expected_data)
            .unwrap();
        borsh::BorshSerialize::serialize(&liquidity, &mut expected_data).unwrap();
        borsh::BorshSerialize::serialize(&amount_0_max, &mut expected_data).unwrap();
        borsh::BorshSerialize::serialize(&amount_1_max, &mut expected_data).unwrap();

        let expected_accounts = vec![
            AccountMeta::new(payer, true),
            AccountMeta::new_readonly(position_nft_owner, false),
            AccountMeta::new(position_nft_mint, true),
            AccountMeta::new(position_nft_account, false),
            AccountMeta::new(metadata_account, false),
            AccountMeta::new(pool_state, false),
            AccountMeta::new(protocol_position, false),
            AccountMeta::new(tick_array_lower, false),
            AccountMeta::new(tick_array_upper, false),
            AccountMeta::new(personal_position, false),
            AccountMeta::new(token_account_0, false),
            AccountMeta::new(token_account_1, false),
            AccountMeta::new(token_vault_0, false),
            AccountMeta::new(token_vault_1, false),
            AccountMeta::new_readonly(rent, false),
            AccountMeta::new_readonly(system_program, false),
            AccountMeta::new_readonly(token_program, false),
            AccountMeta::new_readonly(associated_token_program, false),
            AccountMeta::new_readonly(metadata_program, false),
        ];

        let gen_ix = instructions::open_position(
            &program_id,
            &instructions::OpenPositionAccounts {
                payer,
                position_nft_owner,
                position_nft_mint,
                position_nft_account,
                metadata_account,
                pool_state,
                protocol_position,
                tick_array_lower,
                tick_array_upper,
                personal_position,
                token_account_0,
                token_account_1,
                token_vault_0,
                token_vault_1,
                rent,
                system_program,
                token_program,
                associated_token_program,
                metadata_program,
            },
            tick_lower_index,
            tick_upper_index,
            tick_array_lower_start_index,
            tick_array_upper_start_index,
            liquidity,
            amount_0_max,
            amount_1_max,
        );

        let expected_ix = solana_sdk::instruction::Instruction::new_with_bytes(
            program_id,
            &expected_data,
            expected_accounts,
        );
        assert_ix(&gen_ix, &expected_ix);
    }

    #[test]
    fn test_open_position_v2_instruction() {
        use raydium_clmm::instructions;
        #[allow(unused_imports)]
        use raydium_clmm::types;
        use solana_sdk::instruction::AccountMeta;

        let program_id = raydium_clmm::ID;
        let payer = Pubkey::from([1; 32]);
        let position_nft_owner = Pubkey::from([2; 32]);
        let position_nft_mint = Pubkey::from([3; 32]);
        let position_nft_account = Pubkey::from([4; 32]);
        let metadata_account = Pubkey::from([5; 32]);
        let pool_state = Pubkey::from([6; 32]);
        let protocol_position = Pubkey::from([7; 32]);
        let tick_array_lower = Pubkey::from([8; 32]);
        let tick_array_upper = Pubkey::from([9; 32]);
        let personal_position = Pubkey::from([10; 32]);
        let token_account_0 = Pubkey::from([11; 32]);
        let token_account_1 = Pubkey::from([12; 32]);
        let token_vault_0 = Pubkey::from([13; 32]);
        let token_vault_1 = Pubkey::from([14; 32]);
        let rent = Pubkey::from([15; 32]);
        let system_program = Pubkey::from([16; 32]);
        let token_program = Pubkey::from([17; 32]);
        let associated_token_program = Pubkey::from([18; 32]);
        let metadata_program = Pubkey::from([19; 32]);
        let token_program_2022 = Pubkey::from([20; 32]);
        let vault_0_mint = Pubkey::from([21; 32]);
        let vault_1_mint = Pubkey::from([22; 32]);
        let tick_lower_index = 0i32;
        let tick_upper_index = 0i32;
        let tick_array_lower_start_index = 0i32;
        let tick_array_upper_start_index = 0i32;
        let liquidity = 0u128;
        let amount_0_max = 0u64;
        let amount_1_max = 0u64;
        let with_metadata = false;
        let base_flag = None;

        let mut expected_data: Vec<u8> = vec![77, 184, 74, 214, 112, 86, 241, 199];
        borsh::BorshSerialize::serialize(&tick_lower_index, &mut expected_data).unwrap();
        borsh::BorshSerialize::serialize(&tick_upper_index, &mut expected_data).unwrap();
        borsh::BorshSerialize::serialize(&tick_array_lower_start_index, &mut expected_data)
            .unwrap();
        borsh::BorshSerialize::serialize(&tick_array_upper_start_index, &mut expected_data)
            .unwrap();
        borsh::BorshSerialize::serialize(&liquidity, &mut expected_data).unwrap();
        borsh::BorshSerialize::serialize(&amount_0_max, &mut expected_data).unwrap();
        borsh::BorshSerialize::serialize(&amount_1_max, &mut expected_data).unwrap();
        borsh::BorshSerialize::serialize(&with_metadata, &mut expected_data).unwrap();
        borsh::BorshSerialize::serialize(&base_flag, &mut expected_data).unwrap();

        let expected_accounts = vec![
            AccountMeta::new(payer, true),
            AccountMeta::new_readonly(position_nft_owner, false),
            AccountMeta::new(position_nft_mint, true),
            AccountMeta::new(position_nft_account, false),
            AccountMeta::new(metadata_account, false),
            AccountMeta::new(pool_state, false),
            AccountMeta::new(protocol_position, false),
            AccountMeta::new(tick_array_lower, false),
            AccountMeta::new(tick_array_upper, false),
            AccountMeta::new(personal_position, false),
            AccountMeta::new(token_account_0, false),
            AccountMeta::new(token_account_1, false),
            AccountMeta::new(token_vault_0, false),
            AccountMeta::new(token_vault_1, false),
            AccountMeta::new_readonly(rent, false),
            AccountMeta::new_readonly(system_program, false),
            AccountMeta::new_readonly(token_program, false),
            AccountMeta::new_readonly(associated_token_program, false),
            AccountMeta::new_readonly(metadata_program, false),
            AccountMeta::new_readonly(token_program_2022, false),
            AccountMeta::new_readonly(vault_0_mint, false),
            AccountMeta::new_readonly(vault_1_mint, false),
        ];

        let gen_ix = instructions::open_position_v2(
            &program_id,
            &instructions::OpenPositionV2Accounts {
                payer,
                position_nft_owner,
                position_nft_mint,
                position_nft_account,
                metadata_account,
                pool_state,
                protocol_position,
                tick_array_lower,
                tick_array_upper,
                personal_position,
                token_account_0,
                token_account_1,
                token_vault_0,
                token_vault_1,
                rent,
                system_program,
                token_program,
                associated_token_program,
                metadata_program,
                token_program_2022,
                vault_0_mint,
                vault_1_mint,
            },
            tick_lower_index,
            tick_upper_index,
            tick_array_lower_start_index,
            tick_array_upper_start_index,
            liquidity,
            amount_0_max,
            amount_1_max,
            with_metadata,
            base_flag,
        );

        let expected_ix = solana_sdk::instruction::Instruction::new_with_bytes(
            program_id,
            &expected_data,
            expected_accounts,
        );
        assert_ix(&gen_ix, &expected_ix);
    }

    #[test]
    fn test_open_position_with_token22_nft_instruction() {
        use raydium_clmm::instructions;
        #[allow(unused_imports)]
        use raydium_clmm::types;
        use solana_sdk::instruction::AccountMeta;

        let program_id = raydium_clmm::ID;
        let payer = Pubkey::from([1; 32]);
        let position_nft_owner = Pubkey::from([2; 32]);
        let position_nft_mint = Pubkey::from([3; 32]);
        let position_nft_account = Pubkey::from([4; 32]);
        let pool_state = Pubkey::from([5; 32]);
        let protocol_position = Pubkey::from([6; 32]);
        let tick_array_lower = Pubkey::from([7; 32]);
        let tick_array_upper = Pubkey::from([8; 32]);
        let personal_position = Pubkey::from([9; 32]);
        let token_account_0 = Pubkey::from([10; 32]);
        let token_account_1 = Pubkey::from([11; 32]);
        let token_vault_0 = Pubkey::from([12; 32]);
        let token_vault_1 = Pubkey::from([13; 32]);
        let rent = Pubkey::from([14; 32]);
        let system_program = Pubkey::from([15; 32]);
        let token_program = Pubkey::from([16; 32]);
        let associated_token_program = Pubkey::from([17; 32]);
        let token_program_2022 = Pubkey::from([18; 32]);
        let vault_0_mint = Pubkey::from([19; 32]);
        let vault_1_mint = Pubkey::from([20; 32]);
        let tick_lower_index = 0i32;
        let tick_upper_index = 0i32;
        let tick_array_lower_start_index = 0i32;
        let tick_array_upper_start_index = 0i32;
        let liquidity = 0u128;
        let amount_0_max = 0u64;
        let amount_1_max = 0u64;
        let with_metadata = false;
        let base_flag = None;

        let mut expected_data: Vec<u8> = vec![77, 255, 174, 82, 125, 29, 201, 46];
        borsh::BorshSerialize::serialize(&tick_lower_index, &mut expected_data).unwrap();
        borsh::BorshSerialize::serialize(&tick_upper_index, &mut expected_data).unwrap();
        borsh::BorshSerialize::serialize(&tick_array_lower_start_index, &mut expected_data)
            .unwrap();
        borsh::BorshSerialize::serialize(&tick_array_upper_start_index, &mut expected_data)
            .unwrap();
        borsh::BorshSerialize::serialize(&liquidity, &mut expected_data).unwrap();
        borsh::BorshSerialize::serialize(&amount_0_max, &mut expected_data).unwrap();
        borsh::BorshSerialize::serialize(&amount_1_max, &mut expected_data).unwrap();
        borsh::BorshSerialize::serialize(&with_metadata, &mut expected_data).unwrap();
        borsh::BorshSerialize::serialize(&base_flag, &mut expected_data).unwrap();

        let expected_accounts = vec![
            AccountMeta::new(payer, true),
            AccountMeta::new_readonly(position_nft_owner, false),
            AccountMeta::new(position_nft_mint, true),
            AccountMeta::new(position_nft_account, false),
            AccountMeta::new(pool_state, false),
            AccountMeta::new(protocol_position, false),
            AccountMeta::new(tick_array_lower, false),
            AccountMeta::new(tick_array_upper, false),
            AccountMeta::new(personal_position, false),
            AccountMeta::new(token_account_0, false),
            AccountMeta::new(token_account_1, false),
            AccountMeta::new(token_vault_0, false),
            AccountMeta::new(token_vault_1, false),
            AccountMeta::new_readonly(rent, false),
            AccountMeta::new_readonly(system_program, false),
            AccountMeta::new_readonly(token_program, false),
            AccountMeta::new_readonly(associated_token_program, false),
            AccountMeta::new_readonly(token_program_2022, false),
            AccountMeta::new_readonly(vault_0_mint, false),
            AccountMeta::new_readonly(vault_1_mint, false),
        ];

        let gen_ix = instructions::open_position_with_token22_nft(
            &program_id,
            &instructions::OpenPositionWithToken22NftAccounts {
                payer,
                position_nft_owner,
                position_nft_mint,
                position_nft_account,
                pool_state,
                protocol_position,
                tick_array_lower,
                tick_array_upper,
                personal_position,
                token_account_0,
                token_account_1,
                token_vault_0,
                token_vault_1,
                rent,
                system_program,
                token_program,
                associated_token_program,
                token_program_2022,
                vault_0_mint,
                vault_1_mint,
            },
            tick_lower_index,
            tick_upper_index,
            tick_array_lower_start_index,
            tick_array_upper_start_index,
            liquidity,
            amount_0_max,
            amount_1_max,
            with_metadata,
            base_flag,
        );

        let expected_ix = solana_sdk::instruction::Instruction::new_with_bytes(
            program_id,
            &expected_data,
            expected_accounts,
        );
        assert_ix(&gen_ix, &expected_ix);
    }

    #[test]
    fn test_set_reward_params_instruction() {
        use raydium_clmm::instructions;
        #[allow(unused_imports)]
        use raydium_clmm::types;
        use solana_sdk::instruction::AccountMeta;

        let program_id = raydium_clmm::ID;
        let authority = Pubkey::from([1; 32]);
        let amm_config = Pubkey::from([2; 32]);
        let pool_state = Pubkey::from([3; 32]);
        let operation_state = Pubkey::from([4; 32]);
        let token_program = Pubkey::from([5; 32]);
        let token_program_2022 = Pubkey::from([6; 32]);
        let reward_index = 0u8;
        let emissions_per_second_x64 = 0u128;
        let open_time = 0u64;
        let end_time = 0u64;

        let mut expected_data: Vec<u8> = vec![112, 52, 167, 75, 32, 201, 211, 137];
        borsh::BorshSerialize::serialize(&reward_index, &mut expected_data).unwrap();
        borsh::BorshSerialize::serialize(&emissions_per_second_x64, &mut expected_data).unwrap();
        borsh::BorshSerialize::serialize(&open_time, &mut expected_data).unwrap();
        borsh::BorshSerialize::serialize(&end_time, &mut expected_data).unwrap();

        let expected_accounts = vec![
            AccountMeta::new_readonly(authority, true),
            AccountMeta::new_readonly(amm_config, false),
            AccountMeta::new(pool_state, false),
            AccountMeta::new_readonly(operation_state, false),
            AccountMeta::new_readonly(token_program, false),
            AccountMeta::new_readonly(token_program_2022, false),
        ];

        let gen_ix = instructions::set_reward_params(
            &program_id,
            &instructions::SetRewardParamsAccounts {
                authority,
                amm_config,
                pool_state,
                operation_state,
                token_program,
                token_program_2022,
            },
            reward_index,
            emissions_per_second_x64,
            open_time,
            end_time,
        );

        let expected_ix = solana_sdk::instruction::Instruction::new_with_bytes(
            program_id,
            &expected_data,
            expected_accounts,
        );
        assert_ix(&gen_ix, &expected_ix);
    }

    #[test]
    fn test_swap_instruction() {
        use raydium_clmm::instructions;
        #[allow(unused_imports)]
        use raydium_clmm::types;
        use solana_sdk::instruction::AccountMeta;

        let program_id = raydium_clmm::ID;
        let payer = Pubkey::from([1; 32]);
        let amm_config = Pubkey::from([2; 32]);
        let pool_state = Pubkey::from([3; 32]);
        let input_token_account = Pubkey::from([4; 32]);
        let output_token_account = Pubkey::from([5; 32]);
        let input_vault = Pubkey::from([6; 32]);
        let output_vault = Pubkey::from([7; 32]);
        let observation_state = Pubkey::from([8; 32]);
        let token_program = Pubkey::from([9; 32]);
        let tick_array = Pubkey::from([10; 32]);
        let amount = 0u64;
        let other_amount_threshold = 0u64;
        let sqrt_price_limit_x64 = 0u128;
        let is_base_input = false;

        let mut expected_data: Vec<u8> = vec![248, 198, 158, 145, 225, 117, 135, 200];
        borsh::BorshSerialize::serialize(&amount, &mut expected_data).unwrap();
        borsh::BorshSerialize::serialize(&other_amount_threshold, &mut expected_data).unwrap();
        borsh::BorshSerialize::serialize(&sqrt_price_limit_x64, &mut expected_data).unwrap();
        borsh::BorshSerialize::serialize(&is_base_input, &mut expected_data).unwrap();

        let expected_accounts = vec![
            AccountMeta::new_readonly(payer, true),
            AccountMeta::new_readonly(amm_config, false),
            AccountMeta::new(pool_state, false),
            AccountMeta::new(input_token_account, false),
            AccountMeta::new(output_token_account, false),
            AccountMeta::new(input_vault, false),
            AccountMeta::new(output_vault, false),
            AccountMeta::new(observation_state, false),
            AccountMeta::new_readonly(token_program, false),
            AccountMeta::new(tick_array, false),
        ];

        let gen_ix = instructions::swap(
            &program_id,
            &instructions::SwapAccounts {
                payer,
                amm_config,
                pool_state,
                input_token_account,
                output_token_account,
                input_vault,
                output_vault,
                observation_state,
                token_program,
                tick_array,
            },
            amount,
            other_amount_threshold,
            sqrt_price_limit_x64,
            is_base_input,
        );

        let expected_ix = solana_sdk::instruction::Instruction::new_with_bytes(
            program_id,
            &expected_data,
            expected_accounts,
        );
        assert_ix(&gen_ix, &expected_ix);
    }

    #[test]
    fn test_swap_router_base_in_instruction() {
        use raydium_clmm::instructions;
        #[allow(unused_imports)]
        use raydium_clmm::types;
        use solana_sdk::instruction::AccountMeta;

        let program_id = raydium_clmm::ID;
        let payer = Pubkey::from([1; 32]);
        let input_token_account = Pubkey::from([2; 32]);
        let input_token_mint = Pubkey::from([3; 32]);
        let token_program = Pubkey::from([4; 32]);
        let token_program_2022 = Pubkey::from([5; 32]);
        let memo_program = Pubkey::from([6; 32]);
        let amount_in = 0u64;
        let amount_out_minimum = 0u64;

        let mut expected_data: Vec<u8> = vec![69, 125, 115, 218, 245, 186, 242, 196];
        borsh::BorshSerialize::serialize(&amount_in, &mut expected_data).unwrap();
        borsh::BorshSerialize::serialize(&amount_out_minimum, &mut expected_data).unwrap();

        let expected_accounts = vec![
            AccountMeta::new_readonly(payer, true),
            AccountMeta::new(input_token_account, false),
            AccountMeta::new(input_token_mint, false),
            AccountMeta::new_readonly(token_program, false),
            AccountMeta::new_readonly(token_program_2022, false),
            AccountMeta::new_readonly(memo_program, false),
        ];

        let gen_ix = instructions::swap_router_base_in(
            &program_id,
            &instructions::SwapRouterBaseInAccounts {
                payer,
                input_token_account,
                input_token_mint,
                token_program,
                token_program_2022,
                memo_program,
            },
            amount_in,
            amount_out_minimum,
        );

        let expected_ix = solana_sdk::instruction::Instruction::new_with_bytes(
            program_id,
            &expected_data,
            expected_accounts,
        );
        assert_ix(&gen_ix, &expected_ix);
    }

    #[test]
    fn test_swap_v2_instruction() {
        use raydium_clmm::instructions;
        #[allow(unused_imports)]
        use raydium_clmm::types;
        use solana_sdk::instruction::AccountMeta;

        let program_id = raydium_clmm::ID;
        let payer = Pubkey::from([1; 32]);
        let amm_config = Pubkey::from([2; 32]);
        let pool_state = Pubkey::from([3; 32]);
        let input_token_account = Pubkey::from([4; 32]);
        let output_token_account = Pubkey::from([5; 32]);
        let input_vault = Pubkey::from([6; 32]);
        let output_vault = Pubkey::from([7; 32]);
        let observation_state = Pubkey::from([8; 32]);
        let token_program = Pubkey::from([9; 32]);
        let token_program_2022 = Pubkey::from([10; 32]);
        let memo_program = Pubkey::from([11; 32]);
        let input_vault_mint = Pubkey::from([12; 32]);
        let output_vault_mint = Pubkey::from([13; 32]);
        let amount = 0u64;
        let other_amount_threshold = 0u64;
        let sqrt_price_limit_x64 = 0u128;
        let is_base_input = false;

        let mut expected_data: Vec<u8> = vec![43, 4, 237, 11, 26, 201, 30, 98];
        borsh::BorshSerialize::serialize(&amount, &mut expected_data).unwrap();
        borsh::BorshSerialize::serialize(&other_amount_threshold, &mut expected_data).unwrap();
        borsh::BorshSerialize::serialize(&sqrt_price_limit_x64, &mut expected_data).unwrap();
        borsh::BorshSerialize::serialize(&is_base_input, &mut expected_data).unwrap();

        let expected_accounts = vec![
            AccountMeta::new_readonly(payer, true),
            AccountMeta::new_readonly(amm_config, false),
            AccountMeta::new(pool_state, false),
            AccountMeta::new(input_token_account, false),
            AccountMeta::new(output_token_account, false),
            AccountMeta::new(input_vault, false),
            AccountMeta::new(output_vault, false),
            AccountMeta::new(observation_state, false),
            AccountMeta::new_readonly(token_program, false),
            AccountMeta::new_readonly(token_program_2022, false),
            AccountMeta::new_readonly(memo_program, false),
            AccountMeta::new_readonly(input_vault_mint, false),
            AccountMeta::new_readonly(output_vault_mint, false),
        ];

        let gen_ix = instructions::swap_v2(
            &program_id,
            &instructions::SwapV2Accounts {
                payer,
                amm_config,
                pool_state,
                input_token_account,
                output_token_account,
                input_vault,
                output_vault,
                observation_state,
                token_program,
                token_program_2022,
                memo_program,
                input_vault_mint,
                output_vault_mint,
            },
            amount,
            other_amount_threshold,
            sqrt_price_limit_x64,
            is_base_input,
        );

        let expected_ix = solana_sdk::instruction::Instruction::new_with_bytes(
            program_id,
            &expected_data,
            expected_accounts,
        );
        assert_ix(&gen_ix, &expected_ix);
    }

    #[test]
    fn test_transfer_reward_owner_instruction() {
        use raydium_clmm::instructions;
        #[allow(unused_imports)]
        use raydium_clmm::types;
        use solana_sdk::instruction::AccountMeta;

        let program_id = raydium_clmm::ID;
        let authority = Pubkey::from([1; 32]);
        let pool_state = Pubkey::from([2; 32]);
        let new_owner = Pubkey::from([3; 32]);

        let mut expected_data: Vec<u8> = vec![7, 22, 12, 83, 242, 43, 48, 121];
        borsh::BorshSerialize::serialize(&new_owner, &mut expected_data).unwrap();

        let expected_accounts = vec![
            AccountMeta::new_readonly(authority, true),
            AccountMeta::new(pool_state, false),
        ];

        let gen_ix = instructions::transfer_reward_owner(
            &program_id,
            &instructions::TransferRewardOwnerAccounts {
                authority,
                pool_state,
            },
            new_owner,
        );

        let expected_ix = solana_sdk::instruction::Instruction::new_with_bytes(
            program_id,
            &expected_data,
            expected_accounts,
        );
        assert_ix(&gen_ix, &expected_ix);
    }

    #[test]
    fn test_update_amm_config_instruction() {
        use raydium_clmm::instructions;
        #[allow(unused_imports)]
        use raydium_clmm::types;
        use solana_sdk::instruction::AccountMeta;

        let program_id = raydium_clmm::ID;
        let owner = Pubkey::from([1; 32]);
        let amm_config = Pubkey::from([2; 32]);
        let param = 0u8;
        let value = 0u32;

        let mut expected_data: Vec<u8> = vec![49, 60, 174, 136, 154, 28, 116, 200];
        borsh::BorshSerialize::serialize(&param, &mut expected_data).unwrap();
        borsh::BorshSerialize::serialize(&value, &mut expected_data).unwrap();

        let expected_accounts = vec![
            AccountMeta::new_readonly(owner, true),
            AccountMeta::new(amm_config, false),
        ];

        let gen_ix = instructions::update_amm_config(
            &program_id,
            &instructions::UpdateAmmConfigAccounts { owner, amm_config },
            param,
            value,
        );

        let expected_ix = solana_sdk::instruction::Instruction::new_with_bytes(
            program_id,
            &expected_data,
            expected_accounts,
        );
        assert_ix(&gen_ix, &expected_ix);
    }

    #[test]
    fn test_update_operation_account_instruction() {
        use raydium_clmm::instructions;
        #[allow(unused_imports)]
        use raydium_clmm::types;
        use solana_sdk::instruction::AccountMeta;

        let program_id = raydium_clmm::ID;
        let owner = Pubkey::from([1; 32]);
        let operation_state = Pubkey::from([2; 32]);
        let system_program = Pubkey::from([3; 32]);
        let param = 0u8;
        let keys = vec![];

        let mut expected_data: Vec<u8> = vec![127, 70, 119, 40, 188, 227, 61, 7];
        borsh::BorshSerialize::serialize(&param, &mut expected_data).unwrap();
        borsh::BorshSerialize::serialize(&keys, &mut expected_data).unwrap();

        let expected_accounts = vec![
            AccountMeta::new_readonly(owner, true),
            AccountMeta::new(operation_state, false),
            AccountMeta::new_readonly(system_program, false),
        ];

        let gen_ix = instructions::update_operation_account(
            &program_id,
            &instructions::UpdateOperationAccountAccounts {
                owner,
                operation_state,
                system_program,
            },
            param,
            keys.clone(),
        );

        let expected_ix = solana_sdk::instruction::Instruction::new_with_bytes(
            program_id,
            &expected_data,
            expected_accounts,
        );
        assert_ix(&gen_ix, &expected_ix);
    }

    #[test]
    fn test_update_pool_status_instruction() {
        use raydium_clmm::instructions;
        #[allow(unused_imports)]
        use raydium_clmm::types;
        use solana_sdk::instruction::AccountMeta;

        let program_id = raydium_clmm::ID;
        let authority = Pubkey::from([1; 32]);
        let pool_state = Pubkey::from([2; 32]);
        let status = 0u8;

        let mut expected_data: Vec<u8> = vec![130, 87, 108, 6, 46, 224, 117, 123];
        borsh::BorshSerialize::serialize(&status, &mut expected_data).unwrap();

        let expected_accounts = vec![
            AccountMeta::new_readonly(authority, true),
            AccountMeta::new(pool_state, false),
        ];

        let gen_ix = instructions::update_pool_status(
            &program_id,
            &instructions::UpdatePoolStatusAccounts {
                authority,
                pool_state,
            },
            status,
        );

        let expected_ix = solana_sdk::instruction::Instruction::new_with_bytes(
            program_id,
            &expected_data,
            expected_accounts,
        );
        assert_ix(&gen_ix, &expected_ix);
    }

    #[test]
    fn test_update_reward_infos_instruction() {
        use raydium_clmm::instructions;
        use solana_sdk::instruction::AccountMeta;

        let program_id = raydium_clmm::ID;
        let pool_state = Pubkey::from([1; 32]);

        let expected_data: Vec<u8> = vec![163, 172, 224, 52, 11, 154, 106, 223];

        let expected_accounts = vec![AccountMeta::new(pool_state, false)];

        let gen_ix = instructions::update_reward_infos(
            &program_id,
            &instructions::UpdateRewardInfosAccounts { pool_state },
        );

        let expected_ix = solana_sdk::instruction::Instruction::new_with_bytes(
            program_id,
            &expected_data,
            expected_accounts,
        );
        assert_ix(&gen_ix, &expected_ix);
    }

    // ── Types ───────────────────────────────────────────────────────

    #[test]
    fn test_position_reward_info_type() {
        use raydium_clmm::types::PositionRewardInfo;
        let info = PositionRewardInfo {
            growth_inside_last_x64: 12345u128,
            reward_amount_owed: 6789,
        };
        assert_eq!(info.growth_inside_last_x64, 12345);
        assert_eq!(info.reward_amount_owed, 6789);
    }

    #[test]
    fn test_initialize_reward_param_type() {
        use raydium_clmm::types::InitializeRewardParam;
        let param = InitializeRewardParam {
            open_time: 1700000000,
            end_time: 1700100000,
            emissions_per_second_x64: 1_000_000_000_000u128,
        };
        assert_eq!(param.open_time, 1700000000);
        assert_eq!(param.end_time, 1700100000);
    }
}
