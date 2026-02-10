use anchor_parser::declare_program;

declare_program!(meteora_damm_v2);

#[cfg(test)]
mod tests {
    use super::meteora_damm_v2;
    use solana_client::{
        rpc_config::RpcTransactionConfig,
        rpc_response::{UiInstruction, UiParsedInstruction},
    };
    use solana_sdk::pubkey::Pubkey;

    #[test]
    fn test_program_id() {
        assert_eq!(
            meteora_damm_v2::ID.to_string(),
            "cpamdpZCGKUy5JxQXB4dcpGPiikHawvSWAd6mEn1sGG"
        );
    }

    // ── Account discriminators ──────────────────────────────────────

    #[test]
    fn test_pool_discriminator() {
        use meteora_damm_v2::accounts::Pool;
        assert_eq!(Pool::DISCRIMINATOR, [241, 154, 109, 4, 17, 177, 109, 188]);
    }

    #[test]
    fn test_config_discriminator() {
        use meteora_damm_v2::accounts::Config;
        assert_eq!(
            Config::DISCRIMINATOR,
            [155, 12, 170, 224, 30, 250, 204, 130]
        );
    }

    #[test]
    fn test_position_discriminator() {
        use meteora_damm_v2::accounts::Position;
        assert_eq!(
            Position::DISCRIMINATOR,
            [170, 188, 143, 228, 122, 64, 247, 208]
        );
    }

    #[test]
    fn test_operator_discriminator() {
        use meteora_damm_v2::accounts::Operator;
        assert_eq!(
            Operator::DISCRIMINATOR,
            [219, 31, 188, 145, 69, 139, 204, 117]
        );
    }

    #[test]
    fn test_token_badge_discriminator() {
        use meteora_damm_v2::accounts::TokenBadge;
        assert_eq!(
            TokenBadge::DISCRIMINATOR,
            [116, 219, 204, 229, 249, 116, 255, 150]
        );
    }

    #[test]
    fn test_vesting_discriminator() {
        use meteora_damm_v2::accounts::Vesting;
        assert_eq!(
            Vesting::DISCRIMINATOR,
            [100, 149, 66, 138, 95, 200, 128, 241]
        );
    }

    // ── Bytemuck account deserialization ─────────────────────────────

    #[test]
    fn test_pool_bytemuck_deserialization() {
        use meteora_damm_v2::accounts::Pool;

        let disc = Pool::DISCRIMINATOR;
        let struct_size = std::mem::size_of::<Pool>();
        let mut data = Vec::with_capacity(8 + struct_size);
        data.extend_from_slice(&disc);
        data.extend_from_slice(&vec![0u8; struct_size]);

        let pool = Pool::from_account_data(&data).unwrap();
        assert_eq!(pool.liquidity, 0);
        assert_eq!(pool.sqrt_price, 0);
        assert_eq!(pool.pool_status, 0);
        assert_eq!(pool.token_a_mint, Pubkey::default());
    }

    #[test]
    fn test_pool_bad_discriminator() {
        let data = vec![0u8; 2048];
        assert!(meteora_damm_v2::accounts::Pool::from_account_data(&data).is_err());
    }

    #[test]
    fn test_pool_too_short() {
        let data = vec![241, 154, 109, 4, 17, 177, 109, 188, 0, 0];
        assert!(meteora_damm_v2::accounts::Pool::from_account_data(&data).is_err());
    }

    #[test]
    fn test_operator_bytemuck_deserialization() {
        use meteora_damm_v2::accounts::Operator;

        let disc = Operator::DISCRIMINATOR;
        let struct_size = std::mem::size_of::<Operator>();
        let mut data = Vec::with_capacity(8 + struct_size);
        data.extend_from_slice(&disc);
        data.extend_from_slice(&vec![0u8; struct_size]);

        let op = Operator::from_account_data(&data).unwrap();
        assert_eq!(op.whitelisted_address, Pubkey::default());
        assert_eq!(op.permission, 0);
    }

    #[test]
    fn test_token_badge_bytemuck_deserialization() {
        use meteora_damm_v2::accounts::TokenBadge;

        let disc = TokenBadge::DISCRIMINATOR;
        let struct_size = std::mem::size_of::<TokenBadge>();
        let mut data = Vec::with_capacity(8 + struct_size);
        data.extend_from_slice(&disc);
        data.extend_from_slice(&vec![0u8; struct_size]);

        let badge = TokenBadge::from_account_data(&data).unwrap();
        assert_eq!(badge.token_mint, Pubkey::default());
    }

    // ── Account enum (utils) ────────────────────────────────────────

    #[test]
    fn test_account_enum_parse_pool() {
        use meteora_damm_v2::accounts::Pool;
        use meteora_damm_v2::utils::Account;

        let struct_size = std::mem::size_of::<Pool>();
        let mut data = Vec::with_capacity(8 + struct_size);
        data.extend_from_slice(&Pool::DISCRIMINATOR);
        data.extend_from_slice(&vec![0u8; struct_size]);

        let parsed = Account::parse(&data).unwrap();
        match parsed {
            Account::Pool(pool) => {
                assert_eq!(pool.liquidity, 0);
            }
            _ => panic!("Expected Pool variant"),
        }
    }

    #[test]
    fn test_account_enum_unknown_discriminator() {
        use meteora_damm_v2::utils::Account;
        let data = vec![0u8; 2048];
        assert!(Account::parse(&data).is_err());
    }

    // ── Event discriminators ────────────────────────────────────────

    #[test]
    fn test_evt_swap2_discriminator() {
        use meteora_damm_v2::events::EvtSwap2;
        assert_eq!(
            EvtSwap2::DISCRIMINATOR,
            [189, 66, 51, 168, 38, 80, 117, 153]
        );
    }

    #[test]
    fn test_evt_initialize_pool_discriminator() {
        use meteora_damm_v2::events::EvtInitializePool;
        assert_eq!(
            EvtInitializePool::DISCRIMINATOR,
            [228, 50, 246, 85, 203, 66, 134, 37]
        );
    }

    #[test]
    fn test_evt_create_position_discriminator() {
        use meteora_damm_v2::events::EvtCreatePosition;
        assert_eq!(
            EvtCreatePosition::DISCRIMINATOR,
            [156, 15, 119, 198, 29, 181, 221, 55]
        );
    }

    #[test]
    fn test_evt_liquidity_change_discriminator() {
        use meteora_damm_v2::events::EvtLiquidityChange;
        assert_eq!(
            EvtLiquidityChange::DISCRIMINATOR,
            [197, 171, 78, 127, 224, 211, 87, 13]
        );
    }

    #[test]
    fn test_evt_claim_position_fee_discriminator() {
        use meteora_damm_v2::events::EvtClaimPositionFee;
        assert_eq!(
            EvtClaimPositionFee::DISCRIMINATOR,
            [198, 182, 183, 52, 97, 12, 49, 56]
        );
    }

    // ── Event from_logs ─────────────────────────────────────────────

    #[test]
    fn test_evt_create_position_from_logs() {
        use base64::Engine;
        use borsh::BorshSerialize;
        use meteora_damm_v2::events::EvtCreatePosition;

        let evt = EvtCreatePosition {
            pool: Pubkey::new_unique(),
            owner: Pubkey::new_unique(),
            position: Pubkey::new_unique(),
            position_nft_mint: Pubkey::new_unique(),
        };

        let mut event_data = Vec::new();
        event_data.extend_from_slice(&EvtCreatePosition::DISCRIMINATOR);
        evt.serialize(&mut event_data).unwrap();

        let encoded = base64::engine::general_purpose::STANDARD.encode(&event_data);
        let log_line = format!("Program data: {encoded}");

        let logs: Vec<&str> = vec![&log_line];
        let events = EvtCreatePosition::from_logs(&logs);
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].pool, evt.pool);
        assert_eq!(events[0].owner, evt.owner);
        assert_eq!(events[0].position, evt.position);
        assert_eq!(events[0].position_nft_mint, evt.position_nft_mint);
    }

    #[test]
    fn test_evt_close_position_from_logs() {
        use base64::Engine;
        use borsh::BorshSerialize;
        use meteora_damm_v2::events::EvtClosePosition;

        let evt = EvtClosePosition {
            pool: Pubkey::new_unique(),
            owner: Pubkey::new_unique(),
            position: Pubkey::new_unique(),
            position_nft_mint: Pubkey::new_unique(),
        };

        let mut event_data = Vec::new();
        event_data.extend_from_slice(&EvtClosePosition::DISCRIMINATOR);
        evt.serialize(&mut event_data).unwrap();

        let encoded = base64::engine::general_purpose::STANDARD.encode(&event_data);
        let log_line = format!("Program data: {encoded}");

        let events = EvtClosePosition::from_logs(&[log_line.as_str()]);
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].pool, evt.pool);
    }

    #[test]
    fn test_evt_claim_position_fee_from_logs() {
        use base64::Engine;
        use borsh::BorshSerialize;
        use meteora_damm_v2::events::EvtClaimPositionFee;

        let evt = EvtClaimPositionFee {
            pool: Pubkey::new_unique(),
            position: Pubkey::new_unique(),
            owner: Pubkey::new_unique(),
            fee_a_claimed: 1_000_000,
            fee_b_claimed: 2_000_000,
        };

        let mut event_data = Vec::new();
        event_data.extend_from_slice(&EvtClaimPositionFee::DISCRIMINATOR);
        evt.serialize(&mut event_data).unwrap();

        let encoded = base64::engine::general_purpose::STANDARD.encode(&event_data);
        let log_line = format!("Program data: {encoded}");

        let events = EvtClaimPositionFee::from_logs(&[log_line.as_str()]);
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].fee_a_claimed, 1_000_000);
        assert_eq!(events[0].fee_b_claimed, 2_000_000);
    }

    #[test]
    fn test_evt_set_pool_status_from_logs() {
        use base64::Engine;
        use borsh::BorshSerialize;
        use meteora_damm_v2::events::EvtSetPoolStatus;

        let evt = EvtSetPoolStatus {
            pool: Pubkey::new_unique(),
            status: 2,
        };

        let mut event_data = Vec::new();
        event_data.extend_from_slice(&EvtSetPoolStatus::DISCRIMINATOR);
        evt.serialize(&mut event_data).unwrap();

        let encoded = base64::engine::general_purpose::STANDARD.encode(&event_data);
        let log_line = format!("Program data: {encoded}");

        let events = EvtSetPoolStatus::from_logs(&[log_line.as_str()]);
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].status, 2);
    }

    #[test]
    fn test_evt_claim_partner_fee_from_logs() {
        use base64::Engine;
        use borsh::BorshSerialize;
        use meteora_damm_v2::events::EvtClaimPartnerFee;

        let evt = EvtClaimPartnerFee {
            pool: Pubkey::new_unique(),
            token_a_amount: 5_000_000,
            token_b_amount: 10_000_000,
        };

        let mut event_data = Vec::new();
        event_data.extend_from_slice(&EvtClaimPartnerFee::DISCRIMINATOR);
        evt.serialize(&mut event_data).unwrap();

        let encoded = base64::engine::general_purpose::STANDARD.encode(&event_data);
        let log_line = format!("Program data: {encoded}");

        let events = EvtClaimPartnerFee::from_logs(&[log_line.as_str()]);
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].token_a_amount, 5_000_000);
        assert_eq!(events[0].token_b_amount, 10_000_000);
    }

    #[test]
    fn test_event_from_logs_no_match() {
        use meteora_damm_v2::events::EvtSwap2;
        let logs: Vec<&str> = vec!["Program log: Hello"];
        assert!(EvtSwap2::from_logs(&logs).is_empty());
    }

    // ── Event enum (utils) ──────────────────────────────────────────

    #[test]
    fn test_event_enum_from_logs() {
        use base64::Engine;
        use borsh::BorshSerialize;
        use meteora_damm_v2::events::EvtCreatePosition;
        use meteora_damm_v2::utils::Event;

        let evt = EvtCreatePosition {
            pool: Pubkey::new_unique(),
            owner: Pubkey::new_unique(),
            position: Pubkey::new_unique(),
            position_nft_mint: Pubkey::new_unique(),
        };

        let mut event_data = Vec::new();
        event_data.extend_from_slice(&EvtCreatePosition::DISCRIMINATOR);
        evt.serialize(&mut event_data).unwrap();

        let encoded = base64::engine::general_purpose::STANDARD.encode(&event_data);
        let log_line = format!("Program data: {encoded}");

        let events = Event::from_logs(&[log_line.as_str()]);
        assert_eq!(events.len(), 1);
        match &events[0] {
            Event::EvtCreatePosition(e) => {
                assert_eq!(e.pool, evt.pool);
            }
            other => panic!("Expected EvtCreatePosition, got {:?}", other),
        }
    }

    #[test]
    fn test_event_enum_from_logs_multiple() {
        use base64::Engine;
        use borsh::BorshSerialize;
        use meteora_damm_v2::events::{EvtClaimPartnerFee, EvtSetPoolStatus};
        use meteora_damm_v2::utils::Event;

        let evt1 = EvtSetPoolStatus {
            pool: Pubkey::new_unique(),
            status: 1,
        };
        let mut d1 = Vec::new();
        d1.extend_from_slice(&EvtSetPoolStatus::DISCRIMINATOR);
        evt1.serialize(&mut d1).unwrap();
        let log1 = format!(
            "Program data: {}",
            base64::engine::general_purpose::STANDARD.encode(&d1)
        );

        let evt2 = EvtClaimPartnerFee {
            pool: Pubkey::new_unique(),
            token_a_amount: 100,
            token_b_amount: 200,
        };
        let mut d2 = Vec::new();
        d2.extend_from_slice(&EvtClaimPartnerFee::DISCRIMINATOR);
        evt2.serialize(&mut d2).unwrap();
        let log2 = format!(
            "Program data: {}",
            base64::engine::general_purpose::STANDARD.encode(&d2)
        );

        let logs: Vec<&str> = vec![&log1, &log2];
        let events = Event::from_logs(&logs);
        assert_eq!(events.len(), 2);
        assert!(matches!(&events[0], Event::EvtSetPoolStatus(_)));
        assert!(matches!(&events[1], Event::EvtClaimPartnerFee(_)));
    }

    #[test]
    fn test_event_enum_no_match() {
        use meteora_damm_v2::utils::Event;
        let logs: Vec<&str> = vec!["Program log: nothing"];
        assert!(Event::from_logs(&logs).is_empty());
    }

    // ── Constants ───────────────────────────────────────────────────

    #[test]
    fn test_constant_bin_step_bps_default() {
        assert_eq!(meteora_damm_v2::constants::BIN_STEP_BPS_DEFAULT, 1u16);
    }

    #[test]
    fn test_constant_fee_denominator() {
        assert_eq!(
            meteora_damm_v2::constants::FEE_DENOMINATOR,
            1_000_000_000u64
        );
    }

    #[test]
    fn test_constant_max_basis_point() {
        assert_eq!(meteora_damm_v2::constants::MAX_BASIS_POINT, 10_000u64);
    }

    #[test]
    fn test_constant_split_position_denominator() {
        assert_eq!(
            meteora_damm_v2::constants::SPLIT_POSITION_DENOMINATOR,
            1_000_000_000u32
        );
    }

    #[test]
    fn test_constant_pool_prefix() {
        assert_eq!(
            meteora_damm_v2::constants::POOL_PREFIX,
            &[112, 111, 111, 108]
        );
        assert_eq!(
            std::str::from_utf8(meteora_damm_v2::constants::POOL_PREFIX).unwrap(),
            "pool"
        );
    }

    #[test]
    fn test_constant_position_prefix() {
        assert_eq!(
            std::str::from_utf8(meteora_damm_v2::constants::POSITION_PREFIX).unwrap(),
            "position"
        );
    }

    #[test]
    fn test_constant_token_vault_prefix() {
        assert_eq!(
            std::str::from_utf8(meteora_damm_v2::constants::TOKEN_VAULT_PREFIX).unwrap(),
            "token_vault"
        );
    }

    #[test]
    fn test_constant_pool_authority_prefix() {
        assert_eq!(
            std::str::from_utf8(meteora_damm_v2::constants::POOL_AUTHORITY_PREFIX).unwrap(),
            "pool_authority"
        );
    }

    #[test]
    fn test_constant_customizable_pool_prefix() {
        assert_eq!(
            std::str::from_utf8(meteora_damm_v2::constants::CUSTOMIZABLE_POOL_PREFIX).unwrap(),
            "cpool"
        );
    }

    #[test]
    fn test_constant_bin_step_u128_default_le_bytes() {
        assert_eq!(
            meteora_damm_v2::constants::BIN_STEP_U128_DEFAULT_LE_BYTES,
            [203, 16, 199, 186, 184, 141, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0]
        );
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
    fn test_add_liquidity_instruction() {
        use meteora_damm_v2::instructions;
        #[allow(unused_imports)]
        use meteora_damm_v2::types;
        use solana_sdk::instruction::AccountMeta;

        let program_id = meteora_damm_v2::ID;
        let pool = Pubkey::from([1; 32]);
        let position = Pubkey::from([2; 32]);
        let token_a_account = Pubkey::from([3; 32]);
        let token_b_account = Pubkey::from([4; 32]);
        let token_a_vault = Pubkey::from([5; 32]);
        let token_b_vault = Pubkey::from([6; 32]);
        let token_a_mint = Pubkey::from([7; 32]);
        let token_b_mint = Pubkey::from([8; 32]);
        let position_nft_account = Pubkey::from([9; 32]);
        let owner = Pubkey::from([10; 32]);
        let token_a_program = Pubkey::from([11; 32]);
        let token_b_program = Pubkey::from([12; 32]);
        let event_authority = Pubkey::from([13; 32]);
        let program = Pubkey::from([14; 32]);
        let params = types::AddLiquidityParameters {
            liquidity_delta: 0u128,
            token_a_amount_threshold: 0u64,
            token_b_amount_threshold: 0u64,
        };

        let mut expected_data: Vec<u8> = vec![181, 157, 89, 67, 143, 182, 52, 72];
        borsh::BorshSerialize::serialize(&params, &mut expected_data).unwrap();

        let expected_accounts = vec![
            AccountMeta::new(pool, false),
            AccountMeta::new(position, false),
            AccountMeta::new(token_a_account, false),
            AccountMeta::new(token_b_account, false),
            AccountMeta::new(token_a_vault, false),
            AccountMeta::new(token_b_vault, false),
            AccountMeta::new_readonly(token_a_mint, false),
            AccountMeta::new_readonly(token_b_mint, false),
            AccountMeta::new_readonly(position_nft_account, false),
            AccountMeta::new_readonly(owner, true),
            AccountMeta::new_readonly(token_a_program, false),
            AccountMeta::new_readonly(token_b_program, false),
            AccountMeta::new_readonly(event_authority, false),
            AccountMeta::new_readonly(program, false),
        ];

        let gen_ix = instructions::add_liquidity(
            &program_id,
            &instructions::AddLiquidityAccounts {
                pool,
                position,
                token_a_account,
                token_b_account,
                token_a_vault,
                token_b_vault,
                token_a_mint,
                token_b_mint,
                position_nft_account,
                owner,
                token_a_program,
                token_b_program,
                event_authority,
                program,
            },
            params.clone(),
        );

        let expected_ix = solana_sdk::instruction::Instruction::new_with_bytes(
            program_id,
            &expected_data,
            expected_accounts,
        );
        assert_ix(&gen_ix, &expected_ix);
    }

    #[test]
    fn test_claim_partner_fee_instruction() {
        use meteora_damm_v2::instructions;
        #[allow(unused_imports)]
        use meteora_damm_v2::types;
        use solana_sdk::instruction::AccountMeta;

        let program_id = meteora_damm_v2::ID;
        let pool_authority = Pubkey::from([1; 32]);
        let pool = Pubkey::from([2; 32]);
        let token_a_account = Pubkey::from([3; 32]);
        let token_b_account = Pubkey::from([4; 32]);
        let token_a_vault = Pubkey::from([5; 32]);
        let token_b_vault = Pubkey::from([6; 32]);
        let token_a_mint = Pubkey::from([7; 32]);
        let token_b_mint = Pubkey::from([8; 32]);
        let partner = Pubkey::from([9; 32]);
        let token_a_program = Pubkey::from([10; 32]);
        let token_b_program = Pubkey::from([11; 32]);
        let event_authority = Pubkey::from([12; 32]);
        let program = Pubkey::from([13; 32]);
        let max_amount_a = 0u64;
        let max_amount_b = 0u64;

        let mut expected_data: Vec<u8> = vec![97, 206, 39, 105, 94, 94, 126, 148];
        borsh::BorshSerialize::serialize(&max_amount_a, &mut expected_data).unwrap();
        borsh::BorshSerialize::serialize(&max_amount_b, &mut expected_data).unwrap();

        let expected_accounts = vec![
            AccountMeta::new_readonly(pool_authority, false),
            AccountMeta::new(pool, false),
            AccountMeta::new(token_a_account, false),
            AccountMeta::new(token_b_account, false),
            AccountMeta::new(token_a_vault, false),
            AccountMeta::new(token_b_vault, false),
            AccountMeta::new_readonly(token_a_mint, false),
            AccountMeta::new_readonly(token_b_mint, false),
            AccountMeta::new_readonly(partner, true),
            AccountMeta::new_readonly(token_a_program, false),
            AccountMeta::new_readonly(token_b_program, false),
            AccountMeta::new_readonly(event_authority, false),
            AccountMeta::new_readonly(program, false),
        ];

        let gen_ix = instructions::claim_partner_fee(
            &program_id,
            &instructions::ClaimPartnerFeeAccounts {
                pool_authority,
                pool,
                token_a_account,
                token_b_account,
                token_a_vault,
                token_b_vault,
                token_a_mint,
                token_b_mint,
                partner,
                token_a_program,
                token_b_program,
                event_authority,
                program,
            },
            max_amount_a,
            max_amount_b,
        );

        let expected_ix = solana_sdk::instruction::Instruction::new_with_bytes(
            program_id,
            &expected_data,
            expected_accounts,
        );
        assert_ix(&gen_ix, &expected_ix);
    }

    #[test]
    fn test_claim_position_fee_instruction() {
        use meteora_damm_v2::instructions;
        use solana_sdk::instruction::AccountMeta;

        let program_id = meteora_damm_v2::ID;
        let pool_authority = Pubkey::from([1; 32]);
        let pool = Pubkey::from([2; 32]);
        let position = Pubkey::from([3; 32]);
        let token_a_account = Pubkey::from([4; 32]);
        let token_b_account = Pubkey::from([5; 32]);
        let token_a_vault = Pubkey::from([6; 32]);
        let token_b_vault = Pubkey::from([7; 32]);
        let token_a_mint = Pubkey::from([8; 32]);
        let token_b_mint = Pubkey::from([9; 32]);
        let position_nft_account = Pubkey::from([10; 32]);
        let owner = Pubkey::from([11; 32]);
        let token_a_program = Pubkey::from([12; 32]);
        let token_b_program = Pubkey::from([13; 32]);
        let event_authority = Pubkey::from([14; 32]);
        let program = Pubkey::from([15; 32]);

        let expected_data: Vec<u8> = vec![180, 38, 154, 17, 133, 33, 162, 211];

        let expected_accounts = vec![
            AccountMeta::new_readonly(pool_authority, false),
            AccountMeta::new_readonly(pool, false),
            AccountMeta::new(position, false),
            AccountMeta::new(token_a_account, false),
            AccountMeta::new(token_b_account, false),
            AccountMeta::new(token_a_vault, false),
            AccountMeta::new(token_b_vault, false),
            AccountMeta::new_readonly(token_a_mint, false),
            AccountMeta::new_readonly(token_b_mint, false),
            AccountMeta::new_readonly(position_nft_account, false),
            AccountMeta::new_readonly(owner, true),
            AccountMeta::new_readonly(token_a_program, false),
            AccountMeta::new_readonly(token_b_program, false),
            AccountMeta::new_readonly(event_authority, false),
            AccountMeta::new_readonly(program, false),
        ];

        let gen_ix = instructions::claim_position_fee(
            &program_id,
            &instructions::ClaimPositionFeeAccounts {
                pool_authority,
                pool,
                position,
                token_a_account,
                token_b_account,
                token_a_vault,
                token_b_vault,
                token_a_mint,
                token_b_mint,
                position_nft_account,
                owner,
                token_a_program,
                token_b_program,
                event_authority,
                program,
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
    fn test_claim_protocol_fee_instruction() {
        use meteora_damm_v2::instructions;
        #[allow(unused_imports)]
        use meteora_damm_v2::types;
        use solana_sdk::instruction::AccountMeta;

        let program_id = meteora_damm_v2::ID;
        let pool_authority = Pubkey::from([1; 32]);
        let pool = Pubkey::from([2; 32]);
        let token_a_vault = Pubkey::from([3; 32]);
        let token_b_vault = Pubkey::from([4; 32]);
        let token_a_mint = Pubkey::from([5; 32]);
        let token_b_mint = Pubkey::from([6; 32]);
        let token_a_account = Pubkey::from([7; 32]);
        let token_b_account = Pubkey::from([8; 32]);
        let operator = Pubkey::from([9; 32]);
        let signer = Pubkey::from([10; 32]);
        let token_a_program = Pubkey::from([11; 32]);
        let token_b_program = Pubkey::from([12; 32]);
        let event_authority = Pubkey::from([13; 32]);
        let program = Pubkey::from([14; 32]);
        let max_amount_a = 0u64;
        let max_amount_b = 0u64;

        let mut expected_data: Vec<u8> = vec![165, 228, 133, 48, 99, 249, 255, 33];
        borsh::BorshSerialize::serialize(&max_amount_a, &mut expected_data).unwrap();
        borsh::BorshSerialize::serialize(&max_amount_b, &mut expected_data).unwrap();

        let expected_accounts = vec![
            AccountMeta::new_readonly(pool_authority, false),
            AccountMeta::new(pool, false),
            AccountMeta::new(token_a_vault, false),
            AccountMeta::new(token_b_vault, false),
            AccountMeta::new_readonly(token_a_mint, false),
            AccountMeta::new_readonly(token_b_mint, false),
            AccountMeta::new(token_a_account, false),
            AccountMeta::new(token_b_account, false),
            AccountMeta::new_readonly(operator, false),
            AccountMeta::new_readonly(signer, true),
            AccountMeta::new_readonly(token_a_program, false),
            AccountMeta::new_readonly(token_b_program, false),
            AccountMeta::new_readonly(event_authority, false),
            AccountMeta::new_readonly(program, false),
        ];

        let gen_ix = instructions::claim_protocol_fee(
            &program_id,
            &instructions::ClaimProtocolFeeAccounts {
                pool_authority,
                pool,
                token_a_vault,
                token_b_vault,
                token_a_mint,
                token_b_mint,
                token_a_account,
                token_b_account,
                operator,
                signer,
                token_a_program,
                token_b_program,
                event_authority,
                program,
            },
            max_amount_a,
            max_amount_b,
        );

        let expected_ix = solana_sdk::instruction::Instruction::new_with_bytes(
            program_id,
            &expected_data,
            expected_accounts,
        );
        assert_ix(&gen_ix, &expected_ix);
    }

    #[test]
    fn test_claim_reward_instruction() {
        use meteora_damm_v2::instructions;
        #[allow(unused_imports)]
        use meteora_damm_v2::types;
        use solana_sdk::instruction::AccountMeta;

        let program_id = meteora_damm_v2::ID;
        let pool_authority = Pubkey::from([1; 32]);
        let pool = Pubkey::from([2; 32]);
        let position = Pubkey::from([3; 32]);
        let reward_vault = Pubkey::from([4; 32]);
        let reward_mint = Pubkey::from([5; 32]);
        let user_token_account = Pubkey::from([6; 32]);
        let position_nft_account = Pubkey::from([7; 32]);
        let owner = Pubkey::from([8; 32]);
        let token_program = Pubkey::from([9; 32]);
        let event_authority = Pubkey::from([10; 32]);
        let program = Pubkey::from([11; 32]);
        let reward_index = 0u8;
        let skip_reward = 0u8;

        let mut expected_data: Vec<u8> = vec![149, 95, 181, 242, 94, 90, 158, 162];
        borsh::BorshSerialize::serialize(&reward_index, &mut expected_data).unwrap();
        borsh::BorshSerialize::serialize(&skip_reward, &mut expected_data).unwrap();

        let expected_accounts = vec![
            AccountMeta::new_readonly(pool_authority, false),
            AccountMeta::new(pool, false),
            AccountMeta::new(position, false),
            AccountMeta::new(reward_vault, false),
            AccountMeta::new_readonly(reward_mint, false),
            AccountMeta::new(user_token_account, false),
            AccountMeta::new_readonly(position_nft_account, false),
            AccountMeta::new_readonly(owner, true),
            AccountMeta::new_readonly(token_program, false),
            AccountMeta::new_readonly(event_authority, false),
            AccountMeta::new_readonly(program, false),
        ];

        let gen_ix = instructions::claim_reward(
            &program_id,
            &instructions::ClaimRewardAccounts {
                pool_authority,
                pool,
                position,
                reward_vault,
                reward_mint,
                user_token_account,
                position_nft_account,
                owner,
                token_program,
                event_authority,
                program,
            },
            reward_index,
            skip_reward,
        );

        let expected_ix = solana_sdk::instruction::Instruction::new_with_bytes(
            program_id,
            &expected_data,
            expected_accounts,
        );
        assert_ix(&gen_ix, &expected_ix);
    }

    #[test]
    fn test_close_config_instruction() {
        use meteora_damm_v2::instructions;
        use solana_sdk::instruction::AccountMeta;

        let program_id = meteora_damm_v2::ID;
        let config = Pubkey::from([1; 32]);
        let operator = Pubkey::from([2; 32]);
        let signer = Pubkey::from([3; 32]);
        let rent_receiver = Pubkey::from([4; 32]);
        let event_authority = Pubkey::from([5; 32]);
        let program = Pubkey::from([6; 32]);

        let expected_data: Vec<u8> = vec![145, 9, 72, 157, 95, 125, 61, 85];

        let expected_accounts = vec![
            AccountMeta::new(config, false),
            AccountMeta::new_readonly(operator, false),
            AccountMeta::new_readonly(signer, true),
            AccountMeta::new(rent_receiver, false),
            AccountMeta::new_readonly(event_authority, false),
            AccountMeta::new_readonly(program, false),
        ];

        let gen_ix = instructions::close_config(
            &program_id,
            &instructions::CloseConfigAccounts {
                config,
                operator,
                signer,
                rent_receiver,
                event_authority,
                program,
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
    fn test_close_operator_account_instruction() {
        use meteora_damm_v2::instructions;
        use solana_sdk::instruction::AccountMeta;

        let program_id = meteora_damm_v2::ID;
        let operator = Pubkey::from([1; 32]);
        let signer = Pubkey::from([2; 32]);
        let rent_receiver = Pubkey::from([3; 32]);
        let event_authority = Pubkey::from([4; 32]);
        let program = Pubkey::from([5; 32]);

        let expected_data: Vec<u8> = vec![171, 9, 213, 74, 120, 23, 3, 29];

        let expected_accounts = vec![
            AccountMeta::new(operator, false),
            AccountMeta::new_readonly(signer, true),
            AccountMeta::new(rent_receiver, false),
            AccountMeta::new_readonly(event_authority, false),
            AccountMeta::new_readonly(program, false),
        ];

        let gen_ix = instructions::close_operator_account(
            &program_id,
            &instructions::CloseOperatorAccountAccounts {
                operator,
                signer,
                rent_receiver,
                event_authority,
                program,
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
    fn test_close_position_instruction() {
        use meteora_damm_v2::instructions;
        use solana_sdk::instruction::AccountMeta;

        let program_id = meteora_damm_v2::ID;
        let position_nft_mint = Pubkey::from([1; 32]);
        let position_nft_account = Pubkey::from([2; 32]);
        let pool = Pubkey::from([3; 32]);
        let position = Pubkey::from([4; 32]);
        let pool_authority = Pubkey::from([5; 32]);
        let rent_receiver = Pubkey::from([6; 32]);
        let owner = Pubkey::from([7; 32]);
        let token_program = Pubkey::from([8; 32]);
        let event_authority = Pubkey::from([9; 32]);
        let program = Pubkey::from([10; 32]);

        let expected_data: Vec<u8> = vec![123, 134, 81, 0, 49, 68, 98, 98];

        let expected_accounts = vec![
            AccountMeta::new(position_nft_mint, false),
            AccountMeta::new(position_nft_account, false),
            AccountMeta::new(pool, false),
            AccountMeta::new(position, false),
            AccountMeta::new_readonly(pool_authority, false),
            AccountMeta::new(rent_receiver, false),
            AccountMeta::new_readonly(owner, true),
            AccountMeta::new_readonly(token_program, false),
            AccountMeta::new_readonly(event_authority, false),
            AccountMeta::new_readonly(program, false),
        ];

        let gen_ix = instructions::close_position(
            &program_id,
            &instructions::ClosePositionAccounts {
                position_nft_mint,
                position_nft_account,
                pool,
                position,
                pool_authority,
                rent_receiver,
                owner,
                token_program,
                event_authority,
                program,
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
    fn test_close_token_badge_instruction() {
        use meteora_damm_v2::instructions;
        use solana_sdk::instruction::AccountMeta;

        let program_id = meteora_damm_v2::ID;
        let token_badge = Pubkey::from([1; 32]);
        let operator = Pubkey::from([2; 32]);
        let signer = Pubkey::from([3; 32]);
        let rent_receiver = Pubkey::from([4; 32]);
        let event_authority = Pubkey::from([5; 32]);
        let program = Pubkey::from([6; 32]);

        let expected_data: Vec<u8> = vec![108, 146, 86, 110, 179, 254, 10, 104];

        let expected_accounts = vec![
            AccountMeta::new(token_badge, false),
            AccountMeta::new_readonly(operator, false),
            AccountMeta::new_readonly(signer, true),
            AccountMeta::new(rent_receiver, false),
            AccountMeta::new_readonly(event_authority, false),
            AccountMeta::new_readonly(program, false),
        ];

        let gen_ix = instructions::close_token_badge(
            &program_id,
            &instructions::CloseTokenBadgeAccounts {
                token_badge,
                operator,
                signer,
                rent_receiver,
                event_authority,
                program,
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
    fn test_create_config_instruction() {
        use meteora_damm_v2::instructions;
        #[allow(unused_imports)]
        use meteora_damm_v2::types;
        use solana_sdk::instruction::AccountMeta;

        let program_id = meteora_damm_v2::ID;
        let config = Pubkey::from([1; 32]);
        let operator = Pubkey::from([2; 32]);
        let signer = Pubkey::from([3; 32]);
        let payer = Pubkey::from([4; 32]);
        let system_program = Pubkey::from([5; 32]);
        let event_authority = Pubkey::from([6; 32]);
        let program = Pubkey::from([7; 32]);
        let index = 0u64;
        let config_parameters = types::StaticConfigParameters {
            pool_fees: types::PoolFeeParameters {
                base_fee: types::BaseFeeParameters { data: [0u8; 30] },
                dynamic_fee: None,
            },
            sqrt_min_price: 0u128,
            sqrt_max_price: 0u128,
            vault_config_key: Pubkey::from([8; 32]),
            pool_creator_authority: Pubkey::from([9; 32]),
            activation_type: 0u8,
            collect_fee_mode: 0u8,
        };

        let mut expected_data: Vec<u8> = vec![201, 207, 243, 114, 75, 111, 47, 189];
        borsh::BorshSerialize::serialize(&index, &mut expected_data).unwrap();
        borsh::BorshSerialize::serialize(&config_parameters, &mut expected_data).unwrap();

        let expected_accounts = vec![
            AccountMeta::new(config, false),
            AccountMeta::new_readonly(operator, false),
            AccountMeta::new_readonly(signer, true),
            AccountMeta::new(payer, true),
            AccountMeta::new_readonly(system_program, false),
            AccountMeta::new_readonly(event_authority, false),
            AccountMeta::new_readonly(program, false),
        ];

        let gen_ix = instructions::create_config(
            &program_id,
            &instructions::CreateConfigAccounts {
                config,
                operator,
                signer,
                payer,
                system_program,
                event_authority,
                program,
            },
            index,
            config_parameters.clone(),
        );

        let expected_ix = solana_sdk::instruction::Instruction::new_with_bytes(
            program_id,
            &expected_data,
            expected_accounts,
        );
        assert_ix(&gen_ix, &expected_ix);
    }

    #[test]
    fn test_create_dynamic_config_instruction() {
        use meteora_damm_v2::instructions;
        #[allow(unused_imports)]
        use meteora_damm_v2::types;
        use solana_sdk::instruction::AccountMeta;

        let program_id = meteora_damm_v2::ID;
        let config = Pubkey::from([1; 32]);
        let operator = Pubkey::from([2; 32]);
        let signer = Pubkey::from([3; 32]);
        let payer = Pubkey::from([4; 32]);
        let system_program = Pubkey::from([5; 32]);
        let event_authority = Pubkey::from([6; 32]);
        let program = Pubkey::from([7; 32]);
        let index = 0u64;
        let config_parameters = types::DynamicConfigParameters {
            pool_creator_authority: Pubkey::from([8; 32]),
        };

        let mut expected_data: Vec<u8> = vec![81, 251, 122, 78, 66, 57, 208, 82];
        borsh::BorshSerialize::serialize(&index, &mut expected_data).unwrap();
        borsh::BorshSerialize::serialize(&config_parameters, &mut expected_data).unwrap();

        let expected_accounts = vec![
            AccountMeta::new(config, false),
            AccountMeta::new_readonly(operator, false),
            AccountMeta::new_readonly(signer, true),
            AccountMeta::new(payer, true),
            AccountMeta::new_readonly(system_program, false),
            AccountMeta::new_readonly(event_authority, false),
            AccountMeta::new_readonly(program, false),
        ];

        let gen_ix = instructions::create_dynamic_config(
            &program_id,
            &instructions::CreateDynamicConfigAccounts {
                config,
                operator,
                signer,
                payer,
                system_program,
                event_authority,
                program,
            },
            index,
            config_parameters.clone(),
        );

        let expected_ix = solana_sdk::instruction::Instruction::new_with_bytes(
            program_id,
            &expected_data,
            expected_accounts,
        );
        assert_ix(&gen_ix, &expected_ix);
    }

    #[test]
    fn test_create_operator_account_instruction() {
        use meteora_damm_v2::instructions;
        #[allow(unused_imports)]
        use meteora_damm_v2::types;
        use solana_sdk::instruction::AccountMeta;

        let program_id = meteora_damm_v2::ID;
        let operator = Pubkey::from([1; 32]);
        let whitelisted_address = Pubkey::from([2; 32]);
        let signer = Pubkey::from([3; 32]);
        let payer = Pubkey::from([4; 32]);
        let system_program = Pubkey::from([5; 32]);
        let event_authority = Pubkey::from([6; 32]);
        let program = Pubkey::from([7; 32]);
        let permission = 0u128;

        let mut expected_data: Vec<u8> = vec![221, 64, 246, 149, 240, 153, 229, 163];
        borsh::BorshSerialize::serialize(&permission, &mut expected_data).unwrap();

        let expected_accounts = vec![
            AccountMeta::new(operator, false),
            AccountMeta::new_readonly(whitelisted_address, false),
            AccountMeta::new_readonly(signer, true),
            AccountMeta::new(payer, true),
            AccountMeta::new_readonly(system_program, false),
            AccountMeta::new_readonly(event_authority, false),
            AccountMeta::new_readonly(program, false),
        ];

        let gen_ix = instructions::create_operator_account(
            &program_id,
            &instructions::CreateOperatorAccountAccounts {
                operator,
                whitelisted_address,
                signer,
                payer,
                system_program,
                event_authority,
                program,
            },
            permission,
        );

        let expected_ix = solana_sdk::instruction::Instruction::new_with_bytes(
            program_id,
            &expected_data,
            expected_accounts,
        );
        assert_ix(&gen_ix, &expected_ix);
    }

    #[test]
    fn test_create_position_instruction() {
        use meteora_damm_v2::instructions;
        use solana_sdk::instruction::AccountMeta;

        let program_id = meteora_damm_v2::ID;
        let owner = Pubkey::from([1; 32]);
        let position_nft_mint = Pubkey::from([2; 32]);
        let position_nft_account = Pubkey::from([3; 32]);
        let pool = Pubkey::from([4; 32]);
        let position = Pubkey::from([5; 32]);
        let pool_authority = Pubkey::from([6; 32]);
        let payer = Pubkey::from([7; 32]);
        let token_program = Pubkey::from([8; 32]);
        let system_program = Pubkey::from([9; 32]);
        let event_authority = Pubkey::from([10; 32]);
        let program = Pubkey::from([11; 32]);

        let expected_data: Vec<u8> = vec![48, 215, 197, 153, 96, 203, 180, 133];

        let expected_accounts = vec![
            AccountMeta::new_readonly(owner, false),
            AccountMeta::new(position_nft_mint, true),
            AccountMeta::new(position_nft_account, false),
            AccountMeta::new(pool, false),
            AccountMeta::new(position, false),
            AccountMeta::new_readonly(pool_authority, false),
            AccountMeta::new(payer, true),
            AccountMeta::new_readonly(token_program, false),
            AccountMeta::new_readonly(system_program, false),
            AccountMeta::new_readonly(event_authority, false),
            AccountMeta::new_readonly(program, false),
        ];

        let gen_ix = instructions::create_position(
            &program_id,
            &instructions::CreatePositionAccounts {
                owner,
                position_nft_mint,
                position_nft_account,
                pool,
                position,
                pool_authority,
                payer,
                token_program,
                system_program,
                event_authority,
                program,
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
    fn test_create_token_badge_instruction() {
        use meteora_damm_v2::instructions;
        use solana_sdk::instruction::AccountMeta;

        let program_id = meteora_damm_v2::ID;
        let token_badge = Pubkey::from([1; 32]);
        let token_mint = Pubkey::from([2; 32]);
        let operator = Pubkey::from([3; 32]);
        let signer = Pubkey::from([4; 32]);
        let payer = Pubkey::from([5; 32]);
        let system_program = Pubkey::from([6; 32]);
        let event_authority = Pubkey::from([7; 32]);
        let program = Pubkey::from([8; 32]);

        let expected_data: Vec<u8> = vec![88, 206, 0, 91, 60, 175, 151, 118];

        let expected_accounts = vec![
            AccountMeta::new(token_badge, false),
            AccountMeta::new_readonly(token_mint, false),
            AccountMeta::new_readonly(operator, false),
            AccountMeta::new_readonly(signer, true),
            AccountMeta::new(payer, true),
            AccountMeta::new_readonly(system_program, false),
            AccountMeta::new_readonly(event_authority, false),
            AccountMeta::new_readonly(program, false),
        ];

        let gen_ix = instructions::create_token_badge(
            &program_id,
            &instructions::CreateTokenBadgeAccounts {
                token_badge,
                token_mint,
                operator,
                signer,
                payer,
                system_program,
                event_authority,
                program,
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
    fn test_dummy_ix_instruction() {
        use meteora_damm_v2::instructions;
        #[allow(unused_imports)]
        use meteora_damm_v2::types;
        use solana_sdk::instruction::AccountMeta;

        let program_id = meteora_damm_v2::ID;
        let pod_aligned_fee_time_scheduler = Pubkey::from([1; 32]);
        let pod_aligned_fee_rate_limiter = Pubkey::from([2; 32]);
        let pod_aligned_fee_market_cap_scheduler = Pubkey::from([3; 32]);
        let _ixs = types::DummyParams {
            borsh_fee_time_scheduler_params: types::BorshFeeTimeScheduler {
                cliff_fee_numerator: 0u64,
                number_of_period: 0u16,
                period_frequency: 0u64,
                reduction_factor: 0u64,
                base_fee_mode: 0u8,
                padding: [0u8; 3],
            },
            borsh_fee_rate_limiter_params: types::BorshFeeRateLimiter {
                cliff_fee_numerator: 0u64,
                fee_increment_bps: 0u16,
                max_limiter_duration: 0u32,
                max_fee_bps: 0u32,
                reference_amount: 0u64,
                base_fee_mode: 0u8,
                padding: [0u8; 3],
            },
            borsh_fee_market_cap_scheduler_params: types::BorshFeeMarketCapScheduler {
                cliff_fee_numerator: 0u64,
                number_of_period: 0u16,
                sqrt_price_step_bps: 0u32,
                scheduler_expiration_duration: 0u32,
                reduction_factor: 0u64,
                base_fee_mode: 0u8,
                padding: [0u8; 3],
            },
        };

        let mut expected_data: Vec<u8> = vec![234, 95, 176, 185, 7, 42, 35, 159];
        borsh::BorshSerialize::serialize(&_ixs, &mut expected_data).unwrap();

        let expected_accounts = vec![
            AccountMeta::new_readonly(pod_aligned_fee_time_scheduler, false),
            AccountMeta::new_readonly(pod_aligned_fee_rate_limiter, false),
            AccountMeta::new_readonly(pod_aligned_fee_market_cap_scheduler, false),
        ];

        let gen_ix = instructions::dummy_ix(
            &program_id,
            &instructions::DummyIxAccounts {
                pod_aligned_fee_time_scheduler,
                pod_aligned_fee_rate_limiter,
                pod_aligned_fee_market_cap_scheduler,
            },
            _ixs.clone(),
        );

        let expected_ix = solana_sdk::instruction::Instruction::new_with_bytes(
            program_id,
            &expected_data,
            expected_accounts,
        );
        assert_ix(&gen_ix, &expected_ix);
    }

    #[test]
    fn test_fund_reward_instruction() {
        use meteora_damm_v2::instructions;
        #[allow(unused_imports)]
        use meteora_damm_v2::types;
        use solana_sdk::instruction::AccountMeta;

        let program_id = meteora_damm_v2::ID;
        let pool = Pubkey::from([1; 32]);
        let reward_vault = Pubkey::from([2; 32]);
        let reward_mint = Pubkey::from([3; 32]);
        let funder_token_account = Pubkey::from([4; 32]);
        let funder = Pubkey::from([5; 32]);
        let token_program = Pubkey::from([6; 32]);
        let event_authority = Pubkey::from([7; 32]);
        let program = Pubkey::from([8; 32]);
        let reward_index = 0u8;
        let amount = 0u64;
        let carry_forward = false;

        let mut expected_data: Vec<u8> = vec![188, 50, 249, 165, 93, 151, 38, 63];
        borsh::BorshSerialize::serialize(&reward_index, &mut expected_data).unwrap();
        borsh::BorshSerialize::serialize(&amount, &mut expected_data).unwrap();
        borsh::BorshSerialize::serialize(&carry_forward, &mut expected_data).unwrap();

        let expected_accounts = vec![
            AccountMeta::new(pool, false),
            AccountMeta::new(reward_vault, false),
            AccountMeta::new_readonly(reward_mint, false),
            AccountMeta::new(funder_token_account, false),
            AccountMeta::new_readonly(funder, true),
            AccountMeta::new_readonly(token_program, false),
            AccountMeta::new_readonly(event_authority, false),
            AccountMeta::new_readonly(program, false),
        ];

        let gen_ix = instructions::fund_reward(
            &program_id,
            &instructions::FundRewardAccounts {
                pool,
                reward_vault,
                reward_mint,
                funder_token_account,
                funder,
                token_program,
                event_authority,
                program,
            },
            reward_index,
            amount,
            carry_forward,
        );

        let expected_ix = solana_sdk::instruction::Instruction::new_with_bytes(
            program_id,
            &expected_data,
            expected_accounts,
        );
        assert_ix(&gen_ix, &expected_ix);
    }

    #[test]
    fn test_initialize_customizable_pool_instruction() {
        use meteora_damm_v2::instructions;
        #[allow(unused_imports)]
        use meteora_damm_v2::types;
        use solana_sdk::instruction::AccountMeta;

        let program_id = meteora_damm_v2::ID;
        let creator = Pubkey::from([1; 32]);
        let position_nft_mint = Pubkey::from([2; 32]);
        let position_nft_account = Pubkey::from([3; 32]);
        let payer = Pubkey::from([4; 32]);
        let pool_authority = Pubkey::from([5; 32]);
        let pool = Pubkey::from([6; 32]);
        let position = Pubkey::from([7; 32]);
        let token_a_mint = Pubkey::from([8; 32]);
        let token_b_mint = Pubkey::from([9; 32]);
        let token_a_vault = Pubkey::from([10; 32]);
        let token_b_vault = Pubkey::from([11; 32]);
        let payer_token_a = Pubkey::from([12; 32]);
        let payer_token_b = Pubkey::from([13; 32]);
        let token_a_program = Pubkey::from([14; 32]);
        let token_b_program = Pubkey::from([15; 32]);
        let token_2022_program = Pubkey::from([16; 32]);
        let system_program = Pubkey::from([17; 32]);
        let event_authority = Pubkey::from([18; 32]);
        let program = Pubkey::from([19; 32]);
        let params = types::InitializeCustomizablePoolParameters {
            pool_fees: types::PoolFeeParameters {
                base_fee: types::BaseFeeParameters { data: [0u8; 30] },
                dynamic_fee: None,
            },
            sqrt_min_price: 0u128,
            sqrt_max_price: 0u128,
            has_alpha_vault: false,
            liquidity: 0u128,
            sqrt_price: 0u128,
            activation_type: 0u8,
            collect_fee_mode: 0u8,
            activation_point: None,
        };

        let mut expected_data: Vec<u8> = vec![20, 161, 241, 24, 189, 221, 180, 2];
        borsh::BorshSerialize::serialize(&params, &mut expected_data).unwrap();

        let expected_accounts = vec![
            AccountMeta::new_readonly(creator, false),
            AccountMeta::new(position_nft_mint, true),
            AccountMeta::new(position_nft_account, false),
            AccountMeta::new(payer, true),
            AccountMeta::new_readonly(pool_authority, false),
            AccountMeta::new(pool, false),
            AccountMeta::new(position, false),
            AccountMeta::new_readonly(token_a_mint, false),
            AccountMeta::new_readonly(token_b_mint, false),
            AccountMeta::new(token_a_vault, false),
            AccountMeta::new(token_b_vault, false),
            AccountMeta::new(payer_token_a, false),
            AccountMeta::new(payer_token_b, false),
            AccountMeta::new_readonly(token_a_program, false),
            AccountMeta::new_readonly(token_b_program, false),
            AccountMeta::new_readonly(token_2022_program, false),
            AccountMeta::new_readonly(system_program, false),
            AccountMeta::new_readonly(event_authority, false),
            AccountMeta::new_readonly(program, false),
        ];

        let gen_ix = instructions::initialize_customizable_pool(
            &program_id,
            &instructions::InitializeCustomizablePoolAccounts {
                creator,
                position_nft_mint,
                position_nft_account,
                payer,
                pool_authority,
                pool,
                position,
                token_a_mint,
                token_b_mint,
                token_a_vault,
                token_b_vault,
                payer_token_a,
                payer_token_b,
                token_a_program,
                token_b_program,
                token_2022_program,
                system_program,
                event_authority,
                program,
            },
            params.clone(),
        );

        let expected_ix = solana_sdk::instruction::Instruction::new_with_bytes(
            program_id,
            &expected_data,
            expected_accounts,
        );
        assert_ix(&gen_ix, &expected_ix);
    }

    #[test]
    fn test_initialize_pool_instruction() {
        use meteora_damm_v2::instructions;
        #[allow(unused_imports)]
        use meteora_damm_v2::types;
        use solana_sdk::instruction::AccountMeta;

        let program_id = meteora_damm_v2::ID;
        let creator = Pubkey::from([1; 32]);
        let position_nft_mint = Pubkey::from([2; 32]);
        let position_nft_account = Pubkey::from([3; 32]);
        let payer = Pubkey::from([4; 32]);
        let config = Pubkey::from([5; 32]);
        let pool_authority = Pubkey::from([6; 32]);
        let pool = Pubkey::from([7; 32]);
        let position = Pubkey::from([8; 32]);
        let token_a_mint = Pubkey::from([9; 32]);
        let token_b_mint = Pubkey::from([10; 32]);
        let token_a_vault = Pubkey::from([11; 32]);
        let token_b_vault = Pubkey::from([12; 32]);
        let payer_token_a = Pubkey::from([13; 32]);
        let payer_token_b = Pubkey::from([14; 32]);
        let token_a_program = Pubkey::from([15; 32]);
        let token_b_program = Pubkey::from([16; 32]);
        let token_2022_program = Pubkey::from([17; 32]);
        let system_program = Pubkey::from([18; 32]);
        let event_authority = Pubkey::from([19; 32]);
        let program = Pubkey::from([20; 32]);
        let params = types::InitializePoolParameters {
            liquidity: 0u128,
            sqrt_price: 0u128,
            activation_point: None,
        };

        let mut expected_data: Vec<u8> = vec![95, 180, 10, 172, 84, 174, 232, 40];
        borsh::BorshSerialize::serialize(&params, &mut expected_data).unwrap();

        let expected_accounts = vec![
            AccountMeta::new_readonly(creator, false),
            AccountMeta::new(position_nft_mint, true),
            AccountMeta::new(position_nft_account, false),
            AccountMeta::new(payer, true),
            AccountMeta::new_readonly(config, false),
            AccountMeta::new_readonly(pool_authority, false),
            AccountMeta::new(pool, false),
            AccountMeta::new(position, false),
            AccountMeta::new_readonly(token_a_mint, false),
            AccountMeta::new_readonly(token_b_mint, false),
            AccountMeta::new(token_a_vault, false),
            AccountMeta::new(token_b_vault, false),
            AccountMeta::new(payer_token_a, false),
            AccountMeta::new(payer_token_b, false),
            AccountMeta::new_readonly(token_a_program, false),
            AccountMeta::new_readonly(token_b_program, false),
            AccountMeta::new_readonly(token_2022_program, false),
            AccountMeta::new_readonly(system_program, false),
            AccountMeta::new_readonly(event_authority, false),
            AccountMeta::new_readonly(program, false),
        ];

        let gen_ix = instructions::initialize_pool(
            &program_id,
            &instructions::InitializePoolAccounts {
                creator,
                position_nft_mint,
                position_nft_account,
                payer,
                config,
                pool_authority,
                pool,
                position,
                token_a_mint,
                token_b_mint,
                token_a_vault,
                token_b_vault,
                payer_token_a,
                payer_token_b,
                token_a_program,
                token_b_program,
                token_2022_program,
                system_program,
                event_authority,
                program,
            },
            params.clone(),
        );

        let expected_ix = solana_sdk::instruction::Instruction::new_with_bytes(
            program_id,
            &expected_data,
            expected_accounts,
        );
        assert_ix(&gen_ix, &expected_ix);
    }

    #[test]
    fn test_initialize_pool_with_dynamic_config_instruction() {
        use meteora_damm_v2::instructions;
        #[allow(unused_imports)]
        use meteora_damm_v2::types;
        use solana_sdk::instruction::AccountMeta;

        let program_id = meteora_damm_v2::ID;
        let creator = Pubkey::from([1; 32]);
        let position_nft_mint = Pubkey::from([2; 32]);
        let position_nft_account = Pubkey::from([3; 32]);
        let payer = Pubkey::from([4; 32]);
        let pool_creator_authority = Pubkey::from([5; 32]);
        let config = Pubkey::from([6; 32]);
        let pool_authority = Pubkey::from([7; 32]);
        let pool = Pubkey::from([8; 32]);
        let position = Pubkey::from([9; 32]);
        let token_a_mint = Pubkey::from([10; 32]);
        let token_b_mint = Pubkey::from([11; 32]);
        let token_a_vault = Pubkey::from([12; 32]);
        let token_b_vault = Pubkey::from([13; 32]);
        let payer_token_a = Pubkey::from([14; 32]);
        let payer_token_b = Pubkey::from([15; 32]);
        let token_a_program = Pubkey::from([16; 32]);
        let token_b_program = Pubkey::from([17; 32]);
        let token_2022_program = Pubkey::from([18; 32]);
        let system_program = Pubkey::from([19; 32]);
        let event_authority = Pubkey::from([20; 32]);
        let program = Pubkey::from([21; 32]);
        let params = types::InitializeCustomizablePoolParameters {
            pool_fees: types::PoolFeeParameters {
                base_fee: types::BaseFeeParameters { data: [0u8; 30] },
                dynamic_fee: None,
            },
            sqrt_min_price: 0u128,
            sqrt_max_price: 0u128,
            has_alpha_vault: false,
            liquidity: 0u128,
            sqrt_price: 0u128,
            activation_type: 0u8,
            collect_fee_mode: 0u8,
            activation_point: None,
        };

        let mut expected_data: Vec<u8> = vec![149, 82, 72, 197, 253, 252, 68, 15];
        borsh::BorshSerialize::serialize(&params, &mut expected_data).unwrap();

        let expected_accounts = vec![
            AccountMeta::new_readonly(creator, false),
            AccountMeta::new(position_nft_mint, true),
            AccountMeta::new(position_nft_account, false),
            AccountMeta::new(payer, true),
            AccountMeta::new_readonly(pool_creator_authority, true),
            AccountMeta::new_readonly(config, false),
            AccountMeta::new_readonly(pool_authority, false),
            AccountMeta::new(pool, false),
            AccountMeta::new(position, false),
            AccountMeta::new_readonly(token_a_mint, false),
            AccountMeta::new_readonly(token_b_mint, false),
            AccountMeta::new(token_a_vault, false),
            AccountMeta::new(token_b_vault, false),
            AccountMeta::new(payer_token_a, false),
            AccountMeta::new(payer_token_b, false),
            AccountMeta::new_readonly(token_a_program, false),
            AccountMeta::new_readonly(token_b_program, false),
            AccountMeta::new_readonly(token_2022_program, false),
            AccountMeta::new_readonly(system_program, false),
            AccountMeta::new_readonly(event_authority, false),
            AccountMeta::new_readonly(program, false),
        ];

        let gen_ix = instructions::initialize_pool_with_dynamic_config(
            &program_id,
            &instructions::InitializePoolWithDynamicConfigAccounts {
                creator,
                position_nft_mint,
                position_nft_account,
                payer,
                pool_creator_authority,
                config,
                pool_authority,
                pool,
                position,
                token_a_mint,
                token_b_mint,
                token_a_vault,
                token_b_vault,
                payer_token_a,
                payer_token_b,
                token_a_program,
                token_b_program,
                token_2022_program,
                system_program,
                event_authority,
                program,
            },
            params.clone(),
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
        use meteora_damm_v2::instructions;
        #[allow(unused_imports)]
        use meteora_damm_v2::types;
        use solana_sdk::instruction::AccountMeta;

        let program_id = meteora_damm_v2::ID;
        let pool_authority = Pubkey::from([1; 32]);
        let pool = Pubkey::from([2; 32]);
        let reward_vault = Pubkey::from([3; 32]);
        let reward_mint = Pubkey::from([4; 32]);
        let signer = Pubkey::from([5; 32]);
        let payer = Pubkey::from([6; 32]);
        let token_program = Pubkey::from([7; 32]);
        let system_program = Pubkey::from([8; 32]);
        let event_authority = Pubkey::from([9; 32]);
        let program = Pubkey::from([10; 32]);
        let reward_index = 0u8;
        let reward_duration = 0u64;
        let funder = Pubkey::from([11; 32]);

        let mut expected_data: Vec<u8> = vec![95, 135, 192, 196, 242, 129, 230, 68];
        borsh::BorshSerialize::serialize(&reward_index, &mut expected_data).unwrap();
        borsh::BorshSerialize::serialize(&reward_duration, &mut expected_data).unwrap();
        borsh::BorshSerialize::serialize(&funder, &mut expected_data).unwrap();

        let expected_accounts = vec![
            AccountMeta::new_readonly(pool_authority, false),
            AccountMeta::new(pool, false),
            AccountMeta::new(reward_vault, false),
            AccountMeta::new_readonly(reward_mint, false),
            AccountMeta::new_readonly(signer, true),
            AccountMeta::new(payer, true),
            AccountMeta::new_readonly(token_program, false),
            AccountMeta::new_readonly(system_program, false),
            AccountMeta::new_readonly(event_authority, false),
            AccountMeta::new_readonly(program, false),
        ];

        let gen_ix = instructions::initialize_reward(
            &program_id,
            &instructions::InitializeRewardAccounts {
                pool_authority,
                pool,
                reward_vault,
                reward_mint,
                signer,
                payer,
                token_program,
                system_program,
                event_authority,
                program,
            },
            reward_index,
            reward_duration,
            funder,
        );

        let expected_ix = solana_sdk::instruction::Instruction::new_with_bytes(
            program_id,
            &expected_data,
            expected_accounts,
        );
        assert_ix(&gen_ix, &expected_ix);
    }

    #[test]
    fn test_lock_position_instruction() {
        use meteora_damm_v2::instructions;
        #[allow(unused_imports)]
        use meteora_damm_v2::types;
        use solana_sdk::instruction::AccountMeta;

        let program_id = meteora_damm_v2::ID;
        let pool = Pubkey::from([1; 32]);
        let position = Pubkey::from([2; 32]);
        let vesting = Pubkey::from([3; 32]);
        let position_nft_account = Pubkey::from([4; 32]);
        let owner = Pubkey::from([5; 32]);
        let payer = Pubkey::from([6; 32]);
        let system_program = Pubkey::from([7; 32]);
        let event_authority = Pubkey::from([8; 32]);
        let program = Pubkey::from([9; 32]);
        let params = types::VestingParameters {
            cliff_point: None,
            period_frequency: 0u64,
            cliff_unlock_liquidity: 0u128,
            liquidity_per_period: 0u128,
            number_of_period: 0u16,
        };

        let mut expected_data: Vec<u8> = vec![227, 62, 2, 252, 247, 10, 171, 185];
        borsh::BorshSerialize::serialize(&params, &mut expected_data).unwrap();

        let expected_accounts = vec![
            AccountMeta::new_readonly(pool, false),
            AccountMeta::new(position, false),
            AccountMeta::new(vesting, true),
            AccountMeta::new_readonly(position_nft_account, false),
            AccountMeta::new_readonly(owner, true),
            AccountMeta::new(payer, true),
            AccountMeta::new_readonly(system_program, false),
            AccountMeta::new_readonly(event_authority, false),
            AccountMeta::new_readonly(program, false),
        ];

        let gen_ix = instructions::lock_position(
            &program_id,
            &instructions::LockPositionAccounts {
                pool,
                position,
                vesting,
                position_nft_account,
                owner,
                payer,
                system_program,
                event_authority,
                program,
            },
            params.clone(),
        );

        let expected_ix = solana_sdk::instruction::Instruction::new_with_bytes(
            program_id,
            &expected_data,
            expected_accounts,
        );
        assert_ix(&gen_ix, &expected_ix);
    }

    #[test]
    fn test_permanent_lock_position_instruction() {
        use meteora_damm_v2::instructions;
        #[allow(unused_imports)]
        use meteora_damm_v2::types;
        use solana_sdk::instruction::AccountMeta;

        let program_id = meteora_damm_v2::ID;
        let pool = Pubkey::from([1; 32]);
        let position = Pubkey::from([2; 32]);
        let position_nft_account = Pubkey::from([3; 32]);
        let owner = Pubkey::from([4; 32]);
        let event_authority = Pubkey::from([5; 32]);
        let program = Pubkey::from([6; 32]);
        let permanent_lock_liquidity = 0u128;

        let mut expected_data: Vec<u8> = vec![165, 176, 125, 6, 231, 171, 186, 213];
        borsh::BorshSerialize::serialize(&permanent_lock_liquidity, &mut expected_data).unwrap();

        let expected_accounts = vec![
            AccountMeta::new(pool, false),
            AccountMeta::new(position, false),
            AccountMeta::new_readonly(position_nft_account, false),
            AccountMeta::new_readonly(owner, true),
            AccountMeta::new_readonly(event_authority, false),
            AccountMeta::new_readonly(program, false),
        ];

        let gen_ix = instructions::permanent_lock_position(
            &program_id,
            &instructions::PermanentLockPositionAccounts {
                pool,
                position,
                position_nft_account,
                owner,
                event_authority,
                program,
            },
            permanent_lock_liquidity,
        );

        let expected_ix = solana_sdk::instruction::Instruction::new_with_bytes(
            program_id,
            &expected_data,
            expected_accounts,
        );
        assert_ix(&gen_ix, &expected_ix);
    }

    #[test]
    fn test_refresh_vesting_instruction() {
        use meteora_damm_v2::instructions;
        use solana_sdk::instruction::AccountMeta;

        let program_id = meteora_damm_v2::ID;
        let pool = Pubkey::from([1; 32]);
        let position = Pubkey::from([2; 32]);
        let position_nft_account = Pubkey::from([3; 32]);
        let owner = Pubkey::from([4; 32]);

        let expected_data: Vec<u8> = vec![9, 94, 216, 14, 116, 204, 247, 0];

        let expected_accounts = vec![
            AccountMeta::new_readonly(pool, false),
            AccountMeta::new(position, false),
            AccountMeta::new_readonly(position_nft_account, false),
            AccountMeta::new_readonly(owner, false),
        ];

        let gen_ix = instructions::refresh_vesting(
            &program_id,
            &instructions::RefreshVestingAccounts {
                pool,
                position,
                position_nft_account,
                owner,
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
    fn test_remove_all_liquidity_instruction() {
        use meteora_damm_v2::instructions;
        #[allow(unused_imports)]
        use meteora_damm_v2::types;
        use solana_sdk::instruction::AccountMeta;

        let program_id = meteora_damm_v2::ID;
        let pool_authority = Pubkey::from([1; 32]);
        let pool = Pubkey::from([2; 32]);
        let position = Pubkey::from([3; 32]);
        let token_a_account = Pubkey::from([4; 32]);
        let token_b_account = Pubkey::from([5; 32]);
        let token_a_vault = Pubkey::from([6; 32]);
        let token_b_vault = Pubkey::from([7; 32]);
        let token_a_mint = Pubkey::from([8; 32]);
        let token_b_mint = Pubkey::from([9; 32]);
        let position_nft_account = Pubkey::from([10; 32]);
        let owner = Pubkey::from([11; 32]);
        let token_a_program = Pubkey::from([12; 32]);
        let token_b_program = Pubkey::from([13; 32]);
        let event_authority = Pubkey::from([14; 32]);
        let program = Pubkey::from([15; 32]);
        let token_a_amount_threshold = 0u64;
        let token_b_amount_threshold = 0u64;

        let mut expected_data: Vec<u8> = vec![10, 51, 61, 35, 112, 105, 24, 85];
        borsh::BorshSerialize::serialize(&token_a_amount_threshold, &mut expected_data).unwrap();
        borsh::BorshSerialize::serialize(&token_b_amount_threshold, &mut expected_data).unwrap();

        let expected_accounts = vec![
            AccountMeta::new_readonly(pool_authority, false),
            AccountMeta::new(pool, false),
            AccountMeta::new(position, false),
            AccountMeta::new(token_a_account, false),
            AccountMeta::new(token_b_account, false),
            AccountMeta::new(token_a_vault, false),
            AccountMeta::new(token_b_vault, false),
            AccountMeta::new_readonly(token_a_mint, false),
            AccountMeta::new_readonly(token_b_mint, false),
            AccountMeta::new_readonly(position_nft_account, false),
            AccountMeta::new_readonly(owner, true),
            AccountMeta::new_readonly(token_a_program, false),
            AccountMeta::new_readonly(token_b_program, false),
            AccountMeta::new_readonly(event_authority, false),
            AccountMeta::new_readonly(program, false),
        ];

        let gen_ix = instructions::remove_all_liquidity(
            &program_id,
            &instructions::RemoveAllLiquidityAccounts {
                pool_authority,
                pool,
                position,
                token_a_account,
                token_b_account,
                token_a_vault,
                token_b_vault,
                token_a_mint,
                token_b_mint,
                position_nft_account,
                owner,
                token_a_program,
                token_b_program,
                event_authority,
                program,
            },
            token_a_amount_threshold,
            token_b_amount_threshold,
        );

        let expected_ix = solana_sdk::instruction::Instruction::new_with_bytes(
            program_id,
            &expected_data,
            expected_accounts,
        );
        assert_ix(&gen_ix, &expected_ix);
    }

    #[test]
    fn test_remove_liquidity_instruction() {
        use meteora_damm_v2::instructions;
        #[allow(unused_imports)]
        use meteora_damm_v2::types;
        use solana_sdk::instruction::AccountMeta;

        let program_id = meteora_damm_v2::ID;
        let pool_authority = Pubkey::from([1; 32]);
        let pool = Pubkey::from([2; 32]);
        let position = Pubkey::from([3; 32]);
        let token_a_account = Pubkey::from([4; 32]);
        let token_b_account = Pubkey::from([5; 32]);
        let token_a_vault = Pubkey::from([6; 32]);
        let token_b_vault = Pubkey::from([7; 32]);
        let token_a_mint = Pubkey::from([8; 32]);
        let token_b_mint = Pubkey::from([9; 32]);
        let position_nft_account = Pubkey::from([10; 32]);
        let owner = Pubkey::from([11; 32]);
        let token_a_program = Pubkey::from([12; 32]);
        let token_b_program = Pubkey::from([13; 32]);
        let event_authority = Pubkey::from([14; 32]);
        let program = Pubkey::from([15; 32]);
        let params = types::RemoveLiquidityParameters {
            liquidity_delta: 0u128,
            token_a_amount_threshold: 0u64,
            token_b_amount_threshold: 0u64,
        };

        let mut expected_data: Vec<u8> = vec![80, 85, 209, 72, 24, 206, 177, 108];
        borsh::BorshSerialize::serialize(&params, &mut expected_data).unwrap();

        let expected_accounts = vec![
            AccountMeta::new_readonly(pool_authority, false),
            AccountMeta::new(pool, false),
            AccountMeta::new(position, false),
            AccountMeta::new(token_a_account, false),
            AccountMeta::new(token_b_account, false),
            AccountMeta::new(token_a_vault, false),
            AccountMeta::new(token_b_vault, false),
            AccountMeta::new_readonly(token_a_mint, false),
            AccountMeta::new_readonly(token_b_mint, false),
            AccountMeta::new_readonly(position_nft_account, false),
            AccountMeta::new_readonly(owner, true),
            AccountMeta::new_readonly(token_a_program, false),
            AccountMeta::new_readonly(token_b_program, false),
            AccountMeta::new_readonly(event_authority, false),
            AccountMeta::new_readonly(program, false),
        ];

        let gen_ix = instructions::remove_liquidity(
            &program_id,
            &instructions::RemoveLiquidityAccounts {
                pool_authority,
                pool,
                position,
                token_a_account,
                token_b_account,
                token_a_vault,
                token_b_vault,
                token_a_mint,
                token_b_mint,
                position_nft_account,
                owner,
                token_a_program,
                token_b_program,
                event_authority,
                program,
            },
            params.clone(),
        );

        let expected_ix = solana_sdk::instruction::Instruction::new_with_bytes(
            program_id,
            &expected_data,
            expected_accounts,
        );
        assert_ix(&gen_ix, &expected_ix);
    }

    #[test]
    fn test_set_pool_status_instruction() {
        use meteora_damm_v2::instructions;
        #[allow(unused_imports)]
        use meteora_damm_v2::types;
        use solana_sdk::instruction::AccountMeta;

        let program_id = meteora_damm_v2::ID;
        let pool = Pubkey::from([1; 32]);
        let operator = Pubkey::from([2; 32]);
        let signer = Pubkey::from([3; 32]);
        let event_authority = Pubkey::from([4; 32]);
        let program = Pubkey::from([5; 32]);
        let status = 0u8;

        let mut expected_data: Vec<u8> = vec![112, 87, 135, 223, 83, 204, 132, 53];
        borsh::BorshSerialize::serialize(&status, &mut expected_data).unwrap();

        let expected_accounts = vec![
            AccountMeta::new(pool, false),
            AccountMeta::new_readonly(operator, false),
            AccountMeta::new_readonly(signer, true),
            AccountMeta::new_readonly(event_authority, false),
            AccountMeta::new_readonly(program, false),
        ];

        let gen_ix = instructions::set_pool_status(
            &program_id,
            &instructions::SetPoolStatusAccounts {
                pool,
                operator,
                signer,
                event_authority,
                program,
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
    fn test_split_position_instruction() {
        use meteora_damm_v2::instructions;
        #[allow(unused_imports)]
        use meteora_damm_v2::types;
        use solana_sdk::instruction::AccountMeta;

        let program_id = meteora_damm_v2::ID;
        let pool = Pubkey::from([1; 32]);
        let first_position = Pubkey::from([2; 32]);
        let first_position_nft_account = Pubkey::from([3; 32]);
        let second_position = Pubkey::from([4; 32]);
        let second_position_nft_account = Pubkey::from([5; 32]);
        let first_owner = Pubkey::from([6; 32]);
        let second_owner = Pubkey::from([7; 32]);
        let event_authority = Pubkey::from([8; 32]);
        let program = Pubkey::from([9; 32]);
        let params = types::SplitPositionParameters {
            unlocked_liquidity_percentage: 0u8,
            permanent_locked_liquidity_percentage: 0u8,
            fee_a_percentage: 0u8,
            fee_b_percentage: 0u8,
            reward_0_percentage: 0u8,
            reward_1_percentage: 0u8,
            padding: [0u8; 16],
        };

        let mut expected_data: Vec<u8> = vec![172, 241, 221, 138, 161, 29, 253, 42];
        borsh::BorshSerialize::serialize(&params, &mut expected_data).unwrap();

        let expected_accounts = vec![
            AccountMeta::new(pool, false),
            AccountMeta::new(first_position, false),
            AccountMeta::new_readonly(first_position_nft_account, false),
            AccountMeta::new(second_position, false),
            AccountMeta::new_readonly(second_position_nft_account, false),
            AccountMeta::new_readonly(first_owner, true),
            AccountMeta::new_readonly(second_owner, true),
            AccountMeta::new_readonly(event_authority, false),
            AccountMeta::new_readonly(program, false),
        ];

        let gen_ix = instructions::split_position(
            &program_id,
            &instructions::SplitPositionAccounts {
                pool,
                first_position,
                first_position_nft_account,
                second_position,
                second_position_nft_account,
                first_owner,
                second_owner,
                event_authority,
                program,
            },
            params.clone(),
        );

        let expected_ix = solana_sdk::instruction::Instruction::new_with_bytes(
            program_id,
            &expected_data,
            expected_accounts,
        );
        assert_ix(&gen_ix, &expected_ix);
    }

    #[test]
    fn test_split_position2_instruction() {
        use meteora_damm_v2::instructions;
        #[allow(unused_imports)]
        use meteora_damm_v2::types;
        use solana_sdk::instruction::AccountMeta;

        let program_id = meteora_damm_v2::ID;
        let pool = Pubkey::from([1; 32]);
        let first_position = Pubkey::from([2; 32]);
        let first_position_nft_account = Pubkey::from([3; 32]);
        let second_position = Pubkey::from([4; 32]);
        let second_position_nft_account = Pubkey::from([5; 32]);
        let first_owner = Pubkey::from([6; 32]);
        let second_owner = Pubkey::from([7; 32]);
        let event_authority = Pubkey::from([8; 32]);
        let program = Pubkey::from([9; 32]);
        let numerator = 0u32;

        let mut expected_data: Vec<u8> = vec![221, 147, 228, 207, 140, 212, 17, 119];
        borsh::BorshSerialize::serialize(&numerator, &mut expected_data).unwrap();

        let expected_accounts = vec![
            AccountMeta::new(pool, false),
            AccountMeta::new(first_position, false),
            AccountMeta::new_readonly(first_position_nft_account, false),
            AccountMeta::new(second_position, false),
            AccountMeta::new_readonly(second_position_nft_account, false),
            AccountMeta::new_readonly(first_owner, true),
            AccountMeta::new_readonly(second_owner, true),
            AccountMeta::new_readonly(event_authority, false),
            AccountMeta::new_readonly(program, false),
        ];

        let gen_ix = instructions::split_position2(
            &program_id,
            &instructions::SplitPosition2Accounts {
                pool,
                first_position,
                first_position_nft_account,
                second_position,
                second_position_nft_account,
                first_owner,
                second_owner,
                event_authority,
                program,
            },
            numerator,
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
        use meteora_damm_v2::instructions;
        #[allow(unused_imports)]
        use meteora_damm_v2::types;
        use solana_sdk::instruction::AccountMeta;

        let program_id = meteora_damm_v2::ID;
        let pool_authority = Pubkey::from([1; 32]);
        let pool = Pubkey::from([2; 32]);
        let input_token_account = Pubkey::from([3; 32]);
        let output_token_account = Pubkey::from([4; 32]);
        let token_a_vault = Pubkey::from([5; 32]);
        let token_b_vault = Pubkey::from([6; 32]);
        let token_a_mint = Pubkey::from([7; 32]);
        let token_b_mint = Pubkey::from([8; 32]);
        let payer = Pubkey::from([9; 32]);
        let token_a_program = Pubkey::from([10; 32]);
        let token_b_program = Pubkey::from([11; 32]);
        let referral_token_account = Pubkey::from([12; 32]);
        let event_authority = Pubkey::from([13; 32]);
        let program = Pubkey::from([14; 32]);
        let _params = types::SwapParameters {
            amount_in: 0u64,
            minimum_amount_out: 0u64,
        };

        let mut expected_data: Vec<u8> = vec![248, 198, 158, 145, 225, 117, 135, 200];
        borsh::BorshSerialize::serialize(&_params, &mut expected_data).unwrap();

        let expected_accounts = vec![
            AccountMeta::new_readonly(pool_authority, false),
            AccountMeta::new(pool, false),
            AccountMeta::new(input_token_account, false),
            AccountMeta::new(output_token_account, false),
            AccountMeta::new(token_a_vault, false),
            AccountMeta::new(token_b_vault, false),
            AccountMeta::new_readonly(token_a_mint, false),
            AccountMeta::new_readonly(token_b_mint, false),
            AccountMeta::new_readonly(payer, true),
            AccountMeta::new_readonly(token_a_program, false),
            AccountMeta::new_readonly(token_b_program, false),
            AccountMeta::new(referral_token_account, false),
            AccountMeta::new_readonly(event_authority, false),
            AccountMeta::new_readonly(program, false),
        ];

        let gen_ix = instructions::swap(
            &program_id,
            &instructions::SwapAccounts {
                pool_authority,
                pool,
                input_token_account,
                output_token_account,
                token_a_vault,
                token_b_vault,
                token_a_mint,
                token_b_mint,
                payer,
                token_a_program,
                token_b_program,
                referral_token_account: Some(referral_token_account),
                event_authority,
                program,
            },
            _params.clone(),
        );

        let expected_ix = solana_sdk::instruction::Instruction::new_with_bytes(
            program_id,
            &expected_data,
            expected_accounts,
        );
        assert_ix(&gen_ix, &expected_ix);
    }

    #[test]
    fn test_swap2_instruction() {
        use meteora_damm_v2::instructions;
        #[allow(unused_imports)]
        use meteora_damm_v2::types;
        use solana_sdk::instruction::AccountMeta;

        let program_id = meteora_damm_v2::ID;
        let pool_authority = Pubkey::from([1; 32]);
        let pool = Pubkey::from([2; 32]);
        let input_token_account = Pubkey::from([3; 32]);
        let output_token_account = Pubkey::from([4; 32]);
        let token_a_vault = Pubkey::from([5; 32]);
        let token_b_vault = Pubkey::from([6; 32]);
        let token_a_mint = Pubkey::from([7; 32]);
        let token_b_mint = Pubkey::from([8; 32]);
        let payer = Pubkey::from([9; 32]);
        let token_a_program = Pubkey::from([10; 32]);
        let token_b_program = Pubkey::from([11; 32]);
        let referral_token_account = Pubkey::from([12; 32]);
        let event_authority = Pubkey::from([13; 32]);
        let program = Pubkey::from([14; 32]);
        let _params = types::SwapParameters2 {
            amount_0: 0u64,
            amount_1: 0u64,
            swap_mode: 0u8,
        };

        let mut expected_data: Vec<u8> = vec![65, 75, 63, 76, 235, 91, 91, 136];
        borsh::BorshSerialize::serialize(&_params, &mut expected_data).unwrap();

        let expected_accounts = vec![
            AccountMeta::new_readonly(pool_authority, false),
            AccountMeta::new(pool, false),
            AccountMeta::new(input_token_account, false),
            AccountMeta::new(output_token_account, false),
            AccountMeta::new(token_a_vault, false),
            AccountMeta::new(token_b_vault, false),
            AccountMeta::new_readonly(token_a_mint, false),
            AccountMeta::new_readonly(token_b_mint, false),
            AccountMeta::new_readonly(payer, true),
            AccountMeta::new_readonly(token_a_program, false),
            AccountMeta::new_readonly(token_b_program, false),
            AccountMeta::new(referral_token_account, false),
            AccountMeta::new_readonly(event_authority, false),
            AccountMeta::new_readonly(program, false),
        ];

        let gen_ix = instructions::swap2(
            &program_id,
            &instructions::Swap2Accounts {
                pool_authority,
                pool,
                input_token_account,
                output_token_account,
                token_a_vault,
                token_b_vault,
                token_a_mint,
                token_b_mint,
                payer,
                token_a_program,
                token_b_program,
                referral_token_account: Some(referral_token_account),
                event_authority,
                program,
            },
            _params.clone(),
        );

        let expected_ix = solana_sdk::instruction::Instruction::new_with_bytes(
            program_id,
            &expected_data,
            expected_accounts,
        );
        assert_ix(&gen_ix, &expected_ix);
    }

    #[test]
    fn test_update_pool_fees_instruction() {
        use meteora_damm_v2::instructions;
        #[allow(unused_imports)]
        use meteora_damm_v2::types;
        use solana_sdk::instruction::AccountMeta;

        let program_id = meteora_damm_v2::ID;
        let pool = Pubkey::from([1; 32]);
        let operator = Pubkey::from([2; 32]);
        let signer = Pubkey::from([3; 32]);
        let event_authority = Pubkey::from([4; 32]);
        let program = Pubkey::from([5; 32]);
        let params = types::UpdatePoolFeesParameters {
            cliff_fee_numerator: None,
            dynamic_fee: None,
        };

        let mut expected_data: Vec<u8> = vec![118, 217, 203, 179, 60, 8, 70, 89];
        borsh::BorshSerialize::serialize(&params, &mut expected_data).unwrap();

        let expected_accounts = vec![
            AccountMeta::new(pool, false),
            AccountMeta::new_readonly(operator, false),
            AccountMeta::new_readonly(signer, true),
            AccountMeta::new_readonly(event_authority, false),
            AccountMeta::new_readonly(program, false),
        ];

        let gen_ix = instructions::update_pool_fees(
            &program_id,
            &instructions::UpdatePoolFeesAccounts {
                pool,
                operator,
                signer,
                event_authority,
                program,
            },
            params.clone(),
        );

        let expected_ix = solana_sdk::instruction::Instruction::new_with_bytes(
            program_id,
            &expected_data,
            expected_accounts,
        );
        assert_ix(&gen_ix, &expected_ix);
    }

    #[test]
    fn test_update_reward_duration_instruction() {
        use meteora_damm_v2::instructions;
        #[allow(unused_imports)]
        use meteora_damm_v2::types;
        use solana_sdk::instruction::AccountMeta;

        let program_id = meteora_damm_v2::ID;
        let pool = Pubkey::from([1; 32]);
        let signer = Pubkey::from([2; 32]);
        let event_authority = Pubkey::from([3; 32]);
        let program = Pubkey::from([4; 32]);
        let reward_index = 0u8;
        let new_duration = 0u64;

        let mut expected_data: Vec<u8> = vec![138, 174, 196, 169, 213, 235, 254, 107];
        borsh::BorshSerialize::serialize(&reward_index, &mut expected_data).unwrap();
        borsh::BorshSerialize::serialize(&new_duration, &mut expected_data).unwrap();

        let expected_accounts = vec![
            AccountMeta::new(pool, false),
            AccountMeta::new_readonly(signer, true),
            AccountMeta::new_readonly(event_authority, false),
            AccountMeta::new_readonly(program, false),
        ];

        let gen_ix = instructions::update_reward_duration(
            &program_id,
            &instructions::UpdateRewardDurationAccounts {
                pool,
                signer,
                event_authority,
                program,
            },
            reward_index,
            new_duration,
        );

        let expected_ix = solana_sdk::instruction::Instruction::new_with_bytes(
            program_id,
            &expected_data,
            expected_accounts,
        );
        assert_ix(&gen_ix, &expected_ix);
    }

    #[test]
    fn test_update_reward_funder_instruction() {
        use meteora_damm_v2::instructions;
        #[allow(unused_imports)]
        use meteora_damm_v2::types;
        use solana_sdk::instruction::AccountMeta;

        let program_id = meteora_damm_v2::ID;
        let pool = Pubkey::from([1; 32]);
        let signer = Pubkey::from([2; 32]);
        let event_authority = Pubkey::from([3; 32]);
        let program = Pubkey::from([4; 32]);
        let reward_index = 0u8;
        let new_funder = Pubkey::from([5; 32]);

        let mut expected_data: Vec<u8> = vec![211, 28, 48, 32, 215, 160, 35, 23];
        borsh::BorshSerialize::serialize(&reward_index, &mut expected_data).unwrap();
        borsh::BorshSerialize::serialize(&new_funder, &mut expected_data).unwrap();

        let expected_accounts = vec![
            AccountMeta::new(pool, false),
            AccountMeta::new_readonly(signer, true),
            AccountMeta::new_readonly(event_authority, false),
            AccountMeta::new_readonly(program, false),
        ];

        let gen_ix = instructions::update_reward_funder(
            &program_id,
            &instructions::UpdateRewardFunderAccounts {
                pool,
                signer,
                event_authority,
                program,
            },
            reward_index,
            new_funder,
        );

        let expected_ix = solana_sdk::instruction::Instruction::new_with_bytes(
            program_id,
            &expected_data,
            expected_accounts,
        );
        assert_ix(&gen_ix, &expected_ix);
    }

    #[test]
    fn test_withdraw_ineligible_reward_instruction() {
        use meteora_damm_v2::instructions;
        #[allow(unused_imports)]
        use meteora_damm_v2::types;
        use solana_sdk::instruction::AccountMeta;

        let program_id = meteora_damm_v2::ID;
        let pool_authority = Pubkey::from([1; 32]);
        let pool = Pubkey::from([2; 32]);
        let reward_vault = Pubkey::from([3; 32]);
        let reward_mint = Pubkey::from([4; 32]);
        let funder_token_account = Pubkey::from([5; 32]);
        let funder = Pubkey::from([6; 32]);
        let token_program = Pubkey::from([7; 32]);
        let event_authority = Pubkey::from([8; 32]);
        let program = Pubkey::from([9; 32]);
        let reward_index = 0u8;

        let mut expected_data: Vec<u8> = vec![148, 206, 42, 195, 247, 49, 103, 8];
        borsh::BorshSerialize::serialize(&reward_index, &mut expected_data).unwrap();

        let expected_accounts = vec![
            AccountMeta::new_readonly(pool_authority, false),
            AccountMeta::new(pool, false),
            AccountMeta::new(reward_vault, false),
            AccountMeta::new_readonly(reward_mint, false),
            AccountMeta::new(funder_token_account, false),
            AccountMeta::new_readonly(funder, true),
            AccountMeta::new_readonly(token_program, false),
            AccountMeta::new_readonly(event_authority, false),
            AccountMeta::new_readonly(program, false),
        ];

        let gen_ix = instructions::withdraw_ineligible_reward(
            &program_id,
            &instructions::WithdrawIneligibleRewardAccounts {
                pool_authority,
                pool,
                reward_vault,
                reward_mint,
                funder_token_account,
                funder,
                token_program,
                event_authority,
                program,
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
    fn test_zap_protocol_fee_instruction() {
        use meteora_damm_v2::instructions;
        #[allow(unused_imports)]
        use meteora_damm_v2::types;
        use solana_sdk::instruction::AccountMeta;

        let program_id = meteora_damm_v2::ID;
        let pool_authority = Pubkey::from([1; 32]);
        let pool = Pubkey::from([2; 32]);
        let token_vault = Pubkey::from([3; 32]);
        let token_mint = Pubkey::from([4; 32]);
        let receiver_token = Pubkey::from([5; 32]);
        let operator = Pubkey::from([6; 32]);
        let signer = Pubkey::from([7; 32]);
        let token_program = Pubkey::from([8; 32]);
        let sysvar_instructions = Pubkey::from([9; 32]);
        let max_amount = 0u64;

        let mut expected_data: Vec<u8> = vec![213, 155, 187, 34, 56, 182, 91, 240];
        borsh::BorshSerialize::serialize(&max_amount, &mut expected_data).unwrap();

        let expected_accounts = vec![
            AccountMeta::new_readonly(pool_authority, false),
            AccountMeta::new(pool, false),
            AccountMeta::new(token_vault, false),
            AccountMeta::new_readonly(token_mint, false),
            AccountMeta::new(receiver_token, false),
            AccountMeta::new_readonly(operator, false),
            AccountMeta::new_readonly(signer, true),
            AccountMeta::new_readonly(token_program, false),
            AccountMeta::new_readonly(sysvar_instructions, false),
        ];

        let gen_ix = instructions::zap_protocol_fee(
            &program_id,
            &instructions::ZapProtocolFeeAccounts {
                pool_authority,
                pool,
                token_vault,
                token_mint,
                receiver_token,
                operator,
                signer,
                token_program,
                sysvar_instructions,
            },
            max_amount,
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
    fn test_swap_parameters2_type() {
        use meteora_damm_v2::types::SwapParameters2;
        let params = SwapParameters2 {
            amount_0: 1000,
            amount_1: 900,
            swap_mode: 0,
        };
        assert_eq!(params.amount_0, 1000);
        assert_eq!(params.amount_1, 900);
        assert_eq!(params.swap_mode, 0);
    }

    #[test]
    fn test_add_liquidity_parameters_type() {
        use meteora_damm_v2::types::AddLiquidityParameters;
        let params = AddLiquidityParameters {
            liquidity_delta: 1_000_000_000_000u128,
            token_a_amount_threshold: 500_000,
            token_b_amount_threshold: 500_000,
        };
        assert_eq!(params.liquidity_delta, 1_000_000_000_000);
    }

    #[test]
    fn test_initialize_pool_parameters_type() {
        use meteora_damm_v2::types::InitializePoolParameters;
        let params = InitializePoolParameters {
            liquidity: 1_000_000u128,
            sqrt_price: 1_000_000u128,
            activation_point: None,
        };
        assert!(params.activation_point.is_none());
    }

    // ── from_cpi_logs ───────────────────────────────────────────────

    // Real bs58-encoded inner instruction data from tx:
    // 26WqqRKocsJLvS1M8L6RAdqJXN7zGTphbYtPAv4RU2ad5HrtyS95iHzc9bztETHR446yTUGiTrun1eDUrtNCyM2p
    const REAL_CPI_BS58: &str = "EVM9wLnauu9H41GfKhTodovwvCUMboJZUghKDwVkY57zd9LqWRgdevKxet86mJEu36ekJHSoBYKwpTBrJbQ8CBs6BmqxTHiE35zKkQQvz9xUgXJevvxW6zjWr6VqMKFB6J6KUVRtqDnm3tvCH55Rbpe2eigSZ11BukRND7mdETTrYkjkWmCP8ySdiaAfMtqAMowfxA6PQyVhgJ1Xdo8oGA8XMY2ecPBz1JZp4Eke9meNdvgXydsXqPVs2zwtnnZuCcB9sj7vb4SB";

    #[test]
    fn test_evt_swap2_from_cpi_logs_real_data() {
        use meteora_damm_v2::events::EvtSwap2;

        let events = EvtSwap2::from_cpi_logs([REAL_CPI_BS58]);
        assert_eq!(events.len(), 1, "Should parse exactly one EvtSwap2 event");

        let evt = &events[0];
        assert_eq!(
            evt.pool.to_string(),
            "5gB4NPgFB3MHFHSeKN4sbaY6t9MB8ikCe9HyiKYid4Td"
        );
        assert_eq!(evt.trade_direction, 1);
        assert_eq!(evt.collect_fee_mode, 0);
        assert!(!evt.has_referral);
        assert_eq!(evt.params.amount_0, 37132);
        assert_eq!(evt.params.amount_1, 0);
        assert_eq!(evt.params.swap_mode, 0);
        assert_eq!(evt.swap_result.included_fee_input_amount, 37132);
        assert_eq!(evt.swap_result.excluded_fee_input_amount, 37132);
        assert_eq!(evt.swap_result.amount_left, 0);
        assert_eq!(evt.swap_result.output_amount, 32322);
        assert_eq!(evt.swap_result.trading_fee, 131);
        assert_eq!(evt.swap_result.protocol_fee, 32);
        assert_eq!(evt.swap_result.partner_fee, 0);
        assert_eq!(evt.swap_result.referral_fee, 0);
        assert_eq!(evt.included_transfer_fee_amount_in, 37132);
        assert_eq!(evt.included_transfer_fee_amount_out, 32322);
        assert_eq!(evt.excluded_transfer_fee_amount_out, 32322);
        assert_eq!(evt.current_timestamp, 1770654210);
        assert_eq!(evt.reserve_a_amount, 502460347958);
        assert_eq!(evt.reserve_b_amount, 256524161253);
    }

    #[test]
    fn test_evt_swap2_from_cpi_logs_via_event_enum() {
        use meteora_damm_v2::utils::Event;

        let events = Event::from_cpi_logs([REAL_CPI_BS58]);
        assert_eq!(events.len(), 1);
        match &events[0] {
            Event::EvtSwap2(evt) => {
                assert_eq!(
                    evt.pool.to_string(),
                    "5gB4NPgFB3MHFHSeKN4sbaY6t9MB8ikCe9HyiKYid4Td"
                );
                assert_eq!(evt.swap_result.output_amount, 32322);
                assert_eq!(evt.current_timestamp, 1770654210);
            }
            other => panic!("Expected EvtSwap2, got {:?}", other),
        }
    }

    #[test]
    fn test_from_cpi_logs_no_match() {
        use meteora_damm_v2::events::EvtCreatePosition;

        // The bs58 data encodes EvtSwap2, not EvtCreatePosition
        let events = EvtCreatePosition::from_cpi_logs([REAL_CPI_BS58]);
        assert!(events.is_empty());
    }

    #[test]
    fn test_from_cpi_logs_invalid_bs58() {
        use meteora_damm_v2::events::EvtSwap2;
        let events = EvtSwap2::from_cpi_logs(["not_valid_bs58!!!"]);
        assert!(events.is_empty());
    }

    #[test]
    fn test_from_logs_accepts_vec_string() {
        use meteora_damm_v2::events::EvtSwap2;
        // Verify the generic signature works with Vec<String>
        let logs: Vec<String> = vec!["Program log: nothing".to_string()];
        let events = EvtSwap2::from_logs(logs);
        assert!(events.is_empty());
    }

    #[test]
    fn test_from_logs_accepts_slice_of_strings() {
        use meteora_damm_v2::events::EvtSwap2;
        let logs = vec!["Program log: test".to_string()];
        let events = EvtSwap2::from_logs(&logs);
        assert!(events.is_empty());
    }

    #[tokio::test]
    async fn test_decode_meteora_swap_event_from_tx() {
        let client = solana_client::nonblocking::rpc_client::RpcClient::new(
            "https://api.mainnet-beta.solana.com".to_string(),
        );
        let signature = "Cq7xwxY4w5r5JikAjBHoPigbhoFomzB2RPHSn2g3JxgWuP3czDnNaiD7v7WWZMicmV19GM7i9b69GybuzudvBiL".parse().unwrap();
        let logs = client
            .get_transaction_with_config(
                &signature,
                RpcTransactionConfig {
                    max_supported_transaction_version: Some(0),
                    ..Default::default()
                },
            )
            .await
            .unwrap()
            .transaction
            .meta
            .unwrap()
            .inner_instructions
            .unwrap()
            .into_iter()
            .flat_map(|ui_ix| {
                ui_ix.instructions.into_iter().filter_map(|ix| match ix {
                    UiInstruction::Parsed(parsed) => match parsed {
                        UiParsedInstruction::PartiallyDecoded(pd) => Some(pd.data),
                        UiParsedInstruction::Parsed(_) => None,
                    },
                    UiInstruction::Compiled(compiled) => Some(compiled.data),
                })
            });

        let events = meteora_damm_v2::events::EvtSwap2::from_cpi_logs(logs);

        assert!(events.len() == 1);

        let swap_event = events[0];

        assert_eq!(swap_event.trade_direction, 0);
        assert_eq!(swap_event.collect_fee_mode, 0);
        assert_eq!(swap_event.has_referral, false);

        // --- params ---
        assert_eq!(swap_event.params.amount_0, 1_807_954_327);
        assert_eq!(swap_event.params.amount_1, 155_122_610);
        assert_eq!(swap_event.params.swap_mode, 0);

        assert_eq!(
            swap_event.swap_result.included_fee_input_amount,
            1_807_954_327
        );
        assert_eq!(
            swap_event.swap_result.excluded_fee_input_amount,
            1_807_954_327
        );
        assert_eq!(swap_event.swap_result.amount_left, 0);
        assert_eq!(swap_event.swap_result.output_amount, 155_124_941);
        assert_eq!(
            swap_event.swap_result.next_sqrt_price,
            5_404_274_234_843_416_104
        );
        assert_eq!(swap_event.swap_result.trading_fee, 49_660);
        assert_eq!(swap_event.swap_result.protocol_fee, 12_415);
        assert_eq!(swap_event.swap_result.partner_fee, 0);
        assert_eq!(swap_event.swap_result.referral_fee, 0);

        assert_eq!(swap_event.included_transfer_fee_amount_in, 1_807_954_327);
        assert_eq!(swap_event.included_transfer_fee_amount_out, 155_124_941);
        assert_eq!(swap_event.excluded_transfer_fee_amount_out, 155_124_941);

        assert_eq!(swap_event.current_timestamp, 1_770_656_918);
        assert_eq!(swap_event.reserve_a_amount, 13_441_336_246_749);
        assert_eq!(swap_event.reserve_b_amount, 200_238_277_714);
    }
}
