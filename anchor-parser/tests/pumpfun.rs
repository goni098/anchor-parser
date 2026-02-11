use anchor_parser::declare_program;

declare_program!(pumpfun);

#[cfg(test)]
mod tests {
    use crate::pumpfun::accounts::BondingCurve;

    use super::pumpfun;
    use solana_client::nonblocking::rpc_client::RpcClient;
    use solana_sdk::pubkey::Pubkey;

    #[test]
    fn test_program_id() {
        assert_eq!(
            pumpfun::ID.to_string(),
            "6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P"
        );
    }

    // ── Account discriminators ──────────────────────────────────────

    #[test]
    fn test_bonding_curve_discriminator() {
        use pumpfun::accounts::BondingCurve;
        assert_eq!(
            BondingCurve::DISCRIMINATOR,
            [23, 183, 248, 55, 96, 216, 172, 96]
        );
    }

    #[test]
    fn test_global_discriminator() {
        use pumpfun::accounts::Global;
        assert_eq!(
            Global::DISCRIMINATOR,
            [167, 232, 232, 177, 200, 108, 114, 127]
        );
    }

    #[test]
    fn test_fee_config_discriminator() {
        use pumpfun::accounts::FeeConfig;
        assert_eq!(
            FeeConfig::DISCRIMINATOR,
            [143, 52, 146, 187, 219, 123, 76, 155]
        );
    }

    // ── Account from_account_data ───────────────────────────────────

    #[test]
    fn test_bonding_curve_from_account_data() {
        use borsh::BorshSerialize;
        use pumpfun::accounts::BondingCurve;

        let bc = BondingCurve {
            virtual_token_reserves: 1_000_000_000,
            virtual_sol_reserves: 30_000_000_000,
            real_token_reserves: 793_100_000_000_000,
            real_sol_reserves: 0,
            token_total_supply: 1_000_000_000_000_000,
            complete: false,
            creator: Pubkey::new_unique(),
            is_mayhem_mode: false,
        };

        let mut data = Vec::new();
        data.extend_from_slice(&BondingCurve::DISCRIMINATOR);
        bc.serialize(&mut data).unwrap();

        let parsed = BondingCurve::from_account_data(&data).unwrap();
        assert_eq!(parsed.virtual_token_reserves, 1_000_000_000);
        assert_eq!(parsed.virtual_sol_reserves, 30_000_000_000);
        assert_eq!(parsed.real_token_reserves, 793_100_000_000_000);
        assert_eq!(parsed.token_total_supply, 1_000_000_000_000_000);
        assert!(!parsed.complete);
        assert_eq!(parsed.creator, bc.creator);
        assert!(!parsed.is_mayhem_mode);
    }

    #[test]
    fn test_bonding_curve_bad_discriminator() {
        let data = vec![0u8; 100];
        assert!(pumpfun::accounts::BondingCurve::from_account_data(&data).is_err());
    }

    #[test]
    fn test_bonding_curve_too_short() {
        let data = vec![23, 183, 248, 55];
        assert!(pumpfun::accounts::BondingCurve::from_account_data(&data).is_err());
    }

    // ── Account enum (utils) ────────────────────────────────────────

    #[test]
    fn test_account_enum_parse() {
        use borsh::BorshSerialize;
        use pumpfun::accounts::BondingCurve;
        use pumpfun::utils::Account;

        let bc = BondingCurve {
            virtual_token_reserves: 500,
            virtual_sol_reserves: 600,
            real_token_reserves: 700,
            real_sol_reserves: 800,
            token_total_supply: 900,
            complete: true,
            creator: Pubkey::new_unique(),
            is_mayhem_mode: false,
        };

        let mut data = Vec::new();
        data.extend_from_slice(&BondingCurve::DISCRIMINATOR);
        bc.serialize(&mut data).unwrap();

        let parsed = Account::parse(&data).unwrap();
        match parsed {
            Account::BondingCurve(inner) => {
                assert_eq!(inner.virtual_token_reserves, 500);
                assert!(inner.complete);
            }
            _ => panic!("Expected BondingCurve variant"),
        }
    }

    #[test]
    fn test_account_enum_unknown_discriminator() {
        use pumpfun::utils::Account;
        let data = vec![0u8; 100];
        assert!(Account::parse(&data).is_err());
    }

    // ── Event discriminators ────────────────────────────────────────

    #[test]
    fn test_trade_event_discriminator() {
        use pumpfun::events::TradeEvent;
        assert_eq!(
            TradeEvent::DISCRIMINATOR,
            [189, 219, 127, 211, 78, 230, 97, 238]
        );
    }

    #[test]
    fn test_create_event_discriminator() {
        use pumpfun::events::CreateEvent;
        assert_eq!(
            CreateEvent::DISCRIMINATOR,
            [27, 114, 169, 77, 222, 235, 99, 118]
        );
    }

    #[test]
    fn test_complete_event_discriminator() {
        use pumpfun::events::CompleteEvent;
        assert_eq!(
            CompleteEvent::DISCRIMINATOR,
            [95, 114, 97, 156, 212, 46, 152, 8]
        );
    }

    #[test]
    fn test_set_params_event_discriminator() {
        use pumpfun::events::SetParamsEvent;
        assert_eq!(
            SetParamsEvent::DISCRIMINATOR,
            [223, 195, 159, 246, 62, 48, 143, 131]
        );
    }

    // ── Event from_logs ─────────────────────────────────────────────

    #[test]
    fn test_trade_event_from_logs() {
        use base64::Engine;
        use borsh::BorshSerialize;
        use pumpfun::events::TradeEvent;

        let evt = TradeEvent {
            mint: Pubkey::new_unique(),
            sol_amount: 1_000_000_000,
            token_amount: 50_000_000_000,
            is_buy: true,
            user: Pubkey::new_unique(),
            timestamp: 1700000000,
            virtual_sol_reserves: 30_000_000_000,
            virtual_token_reserves: 1_000_000_000_000,
            real_sol_reserves: 1_000_000_000,
            real_token_reserves: 793_000_000_000_000,
            fee_recipient: Pubkey::new_unique(),
            fee_basis_points: 100,
            fee: 10_000_000,
            creator: Pubkey::new_unique(),
            creator_fee_basis_points: 50,
            creator_fee: 5_000_000,
            track_volume: true,
            total_unclaimed_tokens: 0,
            total_claimed_tokens: 0,
            current_sol_volume: 1_000_000_000,
            last_update_timestamp: 1700000000,
            ix_name: "buy".to_string(),
            mayhem_mode: false,
        };

        let mut event_data = Vec::new();
        event_data.extend_from_slice(&TradeEvent::DISCRIMINATOR);
        evt.serialize(&mut event_data).unwrap();

        let encoded = base64::engine::general_purpose::STANDARD.encode(&event_data);
        let log_line = format!("Program data: {encoded}");

        let logs: Vec<&str> = vec![
            "Program 6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P invoke [1]",
            &log_line,
            "Program 6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P success",
        ];

        let events = TradeEvent::from_logs(&logs);
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].sol_amount, 1_000_000_000);
        assert_eq!(events[0].token_amount, 50_000_000_000);
        assert!(events[0].is_buy);
        assert_eq!(events[0].timestamp, 1700000000);
    }

    #[test]
    fn test_create_event_from_logs() {
        use base64::Engine;
        use borsh::BorshSerialize;
        use pumpfun::events::CreateEvent;

        let evt = CreateEvent {
            name: "TestToken".to_string(),
            symbol: "TT".to_string(),
            uri: "https://example.com/meta.json".to_string(),
            mint: Pubkey::new_unique(),
            bonding_curve: Pubkey::new_unique(),
            user: Pubkey::new_unique(),
            creator: Pubkey::new_unique(),
            timestamp: 1700000000,
            virtual_token_reserves: 1_000_000_000_000,
            virtual_sol_reserves: 30_000_000_000,
            real_token_reserves: 793_100_000_000_000,
            token_total_supply: 1_000_000_000_000_000,
            token_program: Pubkey::new_unique(),
            is_mayhem_mode: false,
        };

        let mut event_data = Vec::new();
        event_data.extend_from_slice(&CreateEvent::DISCRIMINATOR);
        evt.serialize(&mut event_data).unwrap();

        let encoded = base64::engine::general_purpose::STANDARD.encode(&event_data);
        let log_line = format!("Program data: {encoded}");

        let logs: Vec<&str> = vec![&log_line];
        let events = CreateEvent::from_logs(&logs);
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].name, "TestToken");
        assert_eq!(events[0].symbol, "TT");
        assert_eq!(events[0].token_total_supply, 1_000_000_000_000_000);
    }

    #[test]
    fn test_event_from_logs_no_match() {
        use pumpfun::events::TradeEvent;
        let logs: Vec<&str> = vec!["Program log: Hello"];
        assert!(TradeEvent::from_logs(&logs).is_empty());
    }

    // ── Event enum (utils) ──────────────────────────────────────────

    #[test]
    fn test_event_enum_from_logs_trade() {
        use base64::Engine;
        use borsh::BorshSerialize;
        use pumpfun::events::TradeEvent;
        use pumpfun::utils::Event;

        let evt = TradeEvent {
            mint: Pubkey::new_unique(),
            sol_amount: 500_000_000,
            token_amount: 25_000_000_000,
            is_buy: false,
            user: Pubkey::new_unique(),
            timestamp: 1700000001,
            virtual_sol_reserves: 29_500_000_000,
            virtual_token_reserves: 1_025_000_000_000,
            real_sol_reserves: 500_000_000,
            real_token_reserves: 793_050_000_000_000,
            fee_recipient: Pubkey::new_unique(),
            fee_basis_points: 100,
            fee: 5_000_000,
            creator: Pubkey::new_unique(),
            creator_fee_basis_points: 50,
            creator_fee: 2_500_000,
            track_volume: false,
            total_unclaimed_tokens: 0,
            total_claimed_tokens: 0,
            current_sol_volume: 500_000_000,
            last_update_timestamp: 1700000001,
            ix_name: "sell".to_string(),
            mayhem_mode: false,
        };

        let mut event_data = Vec::new();
        event_data.extend_from_slice(&TradeEvent::DISCRIMINATOR);
        evt.serialize(&mut event_data).unwrap();

        let encoded = base64::engine::general_purpose::STANDARD.encode(&event_data);
        let log_line = format!("Program data: {encoded}");

        let logs: Vec<&str> = vec![&log_line];
        let events = Event::from_logs(&logs);
        assert_eq!(events.len(), 1);
        match &events[0] {
            Event::TradeEvent(e) => {
                assert_eq!(e.sol_amount, 500_000_000);
                assert!(!e.is_buy);
            }
            other => panic!("Expected TradeEvent, got {:?}", other),
        }
    }

    #[test]
    fn test_event_enum_from_logs_multiple() {
        use base64::Engine;
        use borsh::BorshSerialize;
        use pumpfun::events::{CompleteEvent, TradeEvent};
        use pumpfun::utils::Event;

        let trade = TradeEvent {
            mint: Pubkey::new_unique(),
            sol_amount: 100,
            token_amount: 200,
            is_buy: true,
            user: Pubkey::new_unique(),
            timestamp: 100,
            virtual_sol_reserves: 100,
            virtual_token_reserves: 100,
            real_sol_reserves: 100,
            real_token_reserves: 100,
            fee_recipient: Pubkey::new_unique(),
            fee_basis_points: 0,
            fee: 0,
            creator: Pubkey::new_unique(),
            creator_fee_basis_points: 0,
            creator_fee: 0,
            track_volume: false,
            total_unclaimed_tokens: 0,
            total_claimed_tokens: 0,
            current_sol_volume: 0,
            last_update_timestamp: 100,
            ix_name: "buy".to_string(),
            mayhem_mode: false,
        };
        let mut td = Vec::new();
        td.extend_from_slice(&TradeEvent::DISCRIMINATOR);
        trade.serialize(&mut td).unwrap();
        let trade_log = format!(
            "Program data: {}",
            base64::engine::general_purpose::STANDARD.encode(&td)
        );

        let complete = CompleteEvent {
            user: Pubkey::new_unique(),
            mint: Pubkey::new_unique(),
            bonding_curve: Pubkey::new_unique(),
            timestamp: 200,
        };
        let mut cd = Vec::new();
        cd.extend_from_slice(&CompleteEvent::DISCRIMINATOR);
        complete.serialize(&mut cd).unwrap();
        let complete_log = format!(
            "Program data: {}",
            base64::engine::general_purpose::STANDARD.encode(&cd)
        );

        let logs: Vec<&str> = vec![&trade_log, &complete_log];
        let events = Event::from_logs(&logs);
        assert_eq!(events.len(), 2);
        assert!(matches!(&events[0], Event::TradeEvent(_)));
        assert!(matches!(&events[1], Event::CompleteEvent(_)));
    }

    #[test]
    fn test_event_enum_no_match() {
        use pumpfun::utils::Event;
        let logs: Vec<&str> = vec!["Program log: nothing here"];
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
    fn test_admin_set_creator_instruction() {
        use pumpfun::instructions;
        #[allow(unused_imports)]
        use pumpfun::types;
        use solana_sdk::instruction::AccountMeta;

        let program_id = pumpfun::ID;
        let admin_set_creator_authority = Pubkey::from([1; 32]);
        let global = Pubkey::from([2; 32]);
        let mint = Pubkey::from([3; 32]);
        let bonding_curve = Pubkey::from([4; 32]);
        let event_authority = Pubkey::from([5; 32]);
        let program = Pubkey::from([6; 32]);
        let creator = Pubkey::from([7; 32]);

        let mut expected_data: Vec<u8> = vec![69, 25, 171, 142, 57, 239, 13, 4];
        borsh::BorshSerialize::serialize(&creator, &mut expected_data).unwrap();

        let expected_accounts = vec![
            AccountMeta::new_readonly(admin_set_creator_authority, true),
            AccountMeta::new_readonly(global, false),
            AccountMeta::new_readonly(mint, false),
            AccountMeta::new(bonding_curve, false),
            AccountMeta::new_readonly(event_authority, false),
            AccountMeta::new_readonly(program, false),
        ];

        let gen_ix = instructions::admin_set_creator(
            &program_id,
            &instructions::AdminSetCreatorAccounts {
                admin_set_creator_authority,
                global,
                mint,
                bonding_curve,
                event_authority,
                program,
            },
            creator,
        );

        let expected_ix = solana_sdk::instruction::Instruction::new_with_bytes(
            program_id,
            &expected_data,
            expected_accounts,
        );
        assert_ix(&gen_ix, &expected_ix);
    }

    #[test]
    fn test_admin_set_idl_authority_instruction() {
        use pumpfun::instructions;
        #[allow(unused_imports)]
        use pumpfun::types;
        use solana_sdk::instruction::AccountMeta;

        let program_id = pumpfun::ID;
        let authority = Pubkey::from([1; 32]);
        let global = Pubkey::from([2; 32]);
        let idl_account = Pubkey::from([3; 32]);
        let system_program = Pubkey::from([4; 32]);
        let program_signer = Pubkey::from([5; 32]);
        let event_authority = Pubkey::from([6; 32]);
        let program = Pubkey::from([7; 32]);
        let idl_authority = Pubkey::from([8; 32]);

        let mut expected_data: Vec<u8> = vec![8, 217, 96, 231, 144, 104, 192, 5];
        borsh::BorshSerialize::serialize(&idl_authority, &mut expected_data).unwrap();

        let expected_accounts = vec![
            AccountMeta::new_readonly(authority, true),
            AccountMeta::new_readonly(global, false),
            AccountMeta::new(idl_account, false),
            AccountMeta::new_readonly(system_program, false),
            AccountMeta::new_readonly(program_signer, false),
            AccountMeta::new_readonly(event_authority, false),
            AccountMeta::new_readonly(program, false),
        ];

        let gen_ix = instructions::admin_set_idl_authority(
            &program_id,
            &instructions::AdminSetIdlAuthorityAccounts {
                authority,
                global,
                idl_account,
                system_program,
                program_signer,
                event_authority,
                program,
            },
            idl_authority,
        );

        let expected_ix = solana_sdk::instruction::Instruction::new_with_bytes(
            program_id,
            &expected_data,
            expected_accounts,
        );
        assert_ix(&gen_ix, &expected_ix);
    }

    #[test]
    fn test_admin_update_token_incentives_instruction() {
        use pumpfun::instructions;
        #[allow(unused_imports)]
        use pumpfun::types;
        use solana_sdk::instruction::AccountMeta;

        let program_id = pumpfun::ID;
        let authority = Pubkey::from([1; 32]);
        let global = Pubkey::from([2; 32]);
        let global_volume_accumulator = Pubkey::from([3; 32]);
        let mint = Pubkey::from([4; 32]);
        let global_incentive_token_account = Pubkey::from([5; 32]);
        let associated_token_program = Pubkey::from([6; 32]);
        let system_program = Pubkey::from([7; 32]);
        let token_program = Pubkey::from([8; 32]);
        let event_authority = Pubkey::from([9; 32]);
        let program = Pubkey::from([10; 32]);
        let start_time = 0i64;
        let end_time = 0i64;
        let seconds_in_a_day = 0i64;
        let day_number = 0u64;
        let pump_token_supply_per_day = 0u64;

        let mut expected_data: Vec<u8> = vec![209, 11, 115, 87, 213, 23, 124, 204];
        borsh::BorshSerialize::serialize(&start_time, &mut expected_data).unwrap();
        borsh::BorshSerialize::serialize(&end_time, &mut expected_data).unwrap();
        borsh::BorshSerialize::serialize(&seconds_in_a_day, &mut expected_data).unwrap();
        borsh::BorshSerialize::serialize(&day_number, &mut expected_data).unwrap();
        borsh::BorshSerialize::serialize(&pump_token_supply_per_day, &mut expected_data).unwrap();

        let expected_accounts = vec![
            AccountMeta::new(authority, true),
            AccountMeta::new_readonly(global, false),
            AccountMeta::new(global_volume_accumulator, false),
            AccountMeta::new_readonly(mint, false),
            AccountMeta::new(global_incentive_token_account, false),
            AccountMeta::new_readonly(associated_token_program, false),
            AccountMeta::new_readonly(system_program, false),
            AccountMeta::new_readonly(token_program, false),
            AccountMeta::new_readonly(event_authority, false),
            AccountMeta::new_readonly(program, false),
        ];

        let gen_ix = instructions::admin_update_token_incentives(
            &program_id,
            &instructions::AdminUpdateTokenIncentivesAccounts {
                authority,
                global,
                global_volume_accumulator,
                mint,
                global_incentive_token_account,
                associated_token_program,
                system_program,
                token_program,
                event_authority,
                program,
            },
            start_time,
            end_time,
            seconds_in_a_day,
            day_number,
            pump_token_supply_per_day,
        );

        let expected_ix = solana_sdk::instruction::Instruction::new_with_bytes(
            program_id,
            &expected_data,
            expected_accounts,
        );
        assert_ix(&gen_ix, &expected_ix);
    }

    #[test]
    fn test_buy_instruction() {
        use pumpfun::instructions;
        #[allow(unused_imports)]
        use pumpfun::types;
        use solana_sdk::instruction::AccountMeta;

        let program_id = pumpfun::ID;
        let global = Pubkey::from([1; 32]);
        let fee_recipient = Pubkey::from([2; 32]);
        let mint = Pubkey::from([3; 32]);
        let bonding_curve = Pubkey::from([4; 32]);
        let associated_bonding_curve = Pubkey::from([5; 32]);
        let associated_user = Pubkey::from([6; 32]);
        let user = Pubkey::from([7; 32]);
        let system_program = Pubkey::from([8; 32]);
        let token_program = Pubkey::from([9; 32]);
        let creator_vault = Pubkey::from([10; 32]);
        let event_authority = Pubkey::from([11; 32]);
        let program = Pubkey::from([12; 32]);
        let global_volume_accumulator = Pubkey::from([13; 32]);
        let user_volume_accumulator = Pubkey::from([14; 32]);
        let fee_config = Pubkey::from([15; 32]);
        let fee_program = Pubkey::from([16; 32]);
        let amount = 0u64;
        let max_sol_cost = 0u64;
        let track_volume = types::OptionBool(false);

        let mut expected_data: Vec<u8> = vec![102, 6, 61, 18, 1, 218, 235, 234];
        borsh::BorshSerialize::serialize(&amount, &mut expected_data).unwrap();
        borsh::BorshSerialize::serialize(&max_sol_cost, &mut expected_data).unwrap();
        borsh::BorshSerialize::serialize(&track_volume, &mut expected_data).unwrap();

        let expected_accounts = vec![
            AccountMeta::new_readonly(global, false),
            AccountMeta::new(fee_recipient, false),
            AccountMeta::new_readonly(mint, false),
            AccountMeta::new(bonding_curve, false),
            AccountMeta::new(associated_bonding_curve, false),
            AccountMeta::new(associated_user, false),
            AccountMeta::new(user, true),
            AccountMeta::new_readonly(system_program, false),
            AccountMeta::new_readonly(token_program, false),
            AccountMeta::new(creator_vault, false),
            AccountMeta::new_readonly(event_authority, false),
            AccountMeta::new_readonly(program, false),
            AccountMeta::new_readonly(global_volume_accumulator, false),
            AccountMeta::new(user_volume_accumulator, false),
            AccountMeta::new_readonly(fee_config, false),
            AccountMeta::new_readonly(fee_program, false),
        ];

        let gen_ix = instructions::buy(
            &program_id,
            &instructions::BuyAccounts {
                global,
                fee_recipient,
                mint,
                bonding_curve,
                associated_bonding_curve,
                associated_user,
                user,
                system_program,
                token_program,
                creator_vault,
                event_authority,
                program,
                global_volume_accumulator,
                user_volume_accumulator,
                fee_config,
                fee_program,
            },
            amount,
            max_sol_cost,
            track_volume.clone(),
        );

        let expected_ix = solana_sdk::instruction::Instruction::new_with_bytes(
            program_id,
            &expected_data,
            expected_accounts,
        );
        assert_ix(&gen_ix, &expected_ix);
    }

    #[test]
    fn test_buy_exact_sol_in_instruction() {
        use pumpfun::instructions;
        #[allow(unused_imports)]
        use pumpfun::types;
        use solana_sdk::instruction::AccountMeta;

        let program_id = pumpfun::ID;
        let global = Pubkey::from([1; 32]);
        let fee_recipient = Pubkey::from([2; 32]);
        let mint = Pubkey::from([3; 32]);
        let bonding_curve = Pubkey::from([4; 32]);
        let associated_bonding_curve = Pubkey::from([5; 32]);
        let associated_user = Pubkey::from([6; 32]);
        let user = Pubkey::from([7; 32]);
        let system_program = Pubkey::from([8; 32]);
        let token_program = Pubkey::from([9; 32]);
        let creator_vault = Pubkey::from([10; 32]);
        let event_authority = Pubkey::from([11; 32]);
        let program = Pubkey::from([12; 32]);
        let global_volume_accumulator = Pubkey::from([13; 32]);
        let user_volume_accumulator = Pubkey::from([14; 32]);
        let fee_config = Pubkey::from([15; 32]);
        let fee_program = Pubkey::from([16; 32]);
        let spendable_sol_in = 0u64;
        let min_tokens_out = 0u64;
        let track_volume = types::OptionBool(false);

        let mut expected_data: Vec<u8> = vec![56, 252, 116, 8, 158, 223, 205, 95];
        borsh::BorshSerialize::serialize(&spendable_sol_in, &mut expected_data).unwrap();
        borsh::BorshSerialize::serialize(&min_tokens_out, &mut expected_data).unwrap();
        borsh::BorshSerialize::serialize(&track_volume, &mut expected_data).unwrap();

        let expected_accounts = vec![
            AccountMeta::new_readonly(global, false),
            AccountMeta::new(fee_recipient, false),
            AccountMeta::new_readonly(mint, false),
            AccountMeta::new(bonding_curve, false),
            AccountMeta::new(associated_bonding_curve, false),
            AccountMeta::new(associated_user, false),
            AccountMeta::new(user, true),
            AccountMeta::new_readonly(system_program, false),
            AccountMeta::new_readonly(token_program, false),
            AccountMeta::new(creator_vault, false),
            AccountMeta::new_readonly(event_authority, false),
            AccountMeta::new_readonly(program, false),
            AccountMeta::new_readonly(global_volume_accumulator, false),
            AccountMeta::new(user_volume_accumulator, false),
            AccountMeta::new_readonly(fee_config, false),
            AccountMeta::new_readonly(fee_program, false),
        ];

        let gen_ix = instructions::buy_exact_sol_in(
            &program_id,
            &instructions::BuyExactSolInAccounts {
                global,
                fee_recipient,
                mint,
                bonding_curve,
                associated_bonding_curve,
                associated_user,
                user,
                system_program,
                token_program,
                creator_vault,
                event_authority,
                program,
                global_volume_accumulator,
                user_volume_accumulator,
                fee_config,
                fee_program,
            },
            spendable_sol_in,
            min_tokens_out,
            track_volume.clone(),
        );

        let expected_ix = solana_sdk::instruction::Instruction::new_with_bytes(
            program_id,
            &expected_data,
            expected_accounts,
        );
        assert_ix(&gen_ix, &expected_ix);
    }

    #[test]
    fn test_claim_token_incentives_instruction() {
        use pumpfun::instructions;
        use solana_sdk::instruction::AccountMeta;

        let program_id = pumpfun::ID;
        let user = Pubkey::from([1; 32]);
        let user_ata = Pubkey::from([2; 32]);
        let global_volume_accumulator = Pubkey::from([3; 32]);
        let global_incentive_token_account = Pubkey::from([4; 32]);
        let user_volume_accumulator = Pubkey::from([5; 32]);
        let mint = Pubkey::from([6; 32]);
        let token_program = Pubkey::from([7; 32]);
        let system_program = Pubkey::from([8; 32]);
        let associated_token_program = Pubkey::from([9; 32]);
        let event_authority = Pubkey::from([10; 32]);
        let program = Pubkey::from([11; 32]);
        let payer = Pubkey::from([12; 32]);

        let expected_data: Vec<u8> = vec![16, 4, 71, 28, 204, 1, 40, 27];

        let expected_accounts = vec![
            AccountMeta::new_readonly(user, false),
            AccountMeta::new(user_ata, false),
            AccountMeta::new_readonly(global_volume_accumulator, false),
            AccountMeta::new(global_incentive_token_account, false),
            AccountMeta::new(user_volume_accumulator, false),
            AccountMeta::new_readonly(mint, false),
            AccountMeta::new_readonly(token_program, false),
            AccountMeta::new_readonly(system_program, false),
            AccountMeta::new_readonly(associated_token_program, false),
            AccountMeta::new_readonly(event_authority, false),
            AccountMeta::new_readonly(program, false),
            AccountMeta::new(payer, true),
        ];

        let gen_ix = instructions::claim_token_incentives(
            &program_id,
            &instructions::ClaimTokenIncentivesAccounts {
                user,
                user_ata,
                global_volume_accumulator,
                global_incentive_token_account,
                user_volume_accumulator,
                mint,
                token_program,
                system_program,
                associated_token_program,
                event_authority,
                program,
                payer,
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
    fn test_close_user_volume_accumulator_instruction() {
        use pumpfun::instructions;
        use solana_sdk::instruction::AccountMeta;

        let program_id = pumpfun::ID;
        let user = Pubkey::from([1; 32]);
        let user_volume_accumulator = Pubkey::from([2; 32]);
        let event_authority = Pubkey::from([3; 32]);
        let program = Pubkey::from([4; 32]);

        let expected_data: Vec<u8> = vec![249, 69, 164, 218, 150, 103, 84, 138];

        let expected_accounts = vec![
            AccountMeta::new(user, true),
            AccountMeta::new(user_volume_accumulator, false),
            AccountMeta::new_readonly(event_authority, false),
            AccountMeta::new_readonly(program, false),
        ];

        let gen_ix = instructions::close_user_volume_accumulator(
            &program_id,
            &instructions::CloseUserVolumeAccumulatorAccounts {
                user,
                user_volume_accumulator,
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
    fn test_collect_creator_fee_instruction() {
        use pumpfun::instructions;
        use solana_sdk::instruction::AccountMeta;

        let program_id = pumpfun::ID;
        let creator = Pubkey::from([1; 32]);
        let creator_vault = Pubkey::from([2; 32]);
        let system_program = Pubkey::from([3; 32]);
        let event_authority = Pubkey::from([4; 32]);
        let program = Pubkey::from([5; 32]);

        let expected_data: Vec<u8> = vec![20, 22, 86, 123, 198, 28, 219, 132];

        let expected_accounts = vec![
            AccountMeta::new(creator, false),
            AccountMeta::new(creator_vault, false),
            AccountMeta::new_readonly(system_program, false),
            AccountMeta::new_readonly(event_authority, false),
            AccountMeta::new_readonly(program, false),
        ];

        let gen_ix = instructions::collect_creator_fee(
            &program_id,
            &instructions::CollectCreatorFeeAccounts {
                creator,
                creator_vault,
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
    fn test_create_instruction() {
        use pumpfun::instructions;
        #[allow(unused_imports)]
        use pumpfun::types;
        use solana_sdk::instruction::AccountMeta;

        let program_id = pumpfun::ID;
        let mint = Pubkey::from([1; 32]);
        let mint_authority = Pubkey::from([2; 32]);
        let bonding_curve = Pubkey::from([3; 32]);
        let associated_bonding_curve = Pubkey::from([4; 32]);
        let global = Pubkey::from([5; 32]);
        let mpl_token_metadata = Pubkey::from([6; 32]);
        let metadata = Pubkey::from([7; 32]);
        let user = Pubkey::from([8; 32]);
        let system_program = Pubkey::from([9; 32]);
        let token_program = Pubkey::from([10; 32]);
        let associated_token_program = Pubkey::from([11; 32]);
        let rent = Pubkey::from([12; 32]);
        let event_authority = Pubkey::from([13; 32]);
        let program = Pubkey::from([14; 32]);
        let name = "test".to_string();
        let symbol = "test".to_string();
        let uri = "test".to_string();
        let creator = Pubkey::from([15; 32]);

        let mut expected_data: Vec<u8> = vec![24, 30, 200, 40, 5, 28, 7, 119];
        borsh::BorshSerialize::serialize(&name, &mut expected_data).unwrap();
        borsh::BorshSerialize::serialize(&symbol, &mut expected_data).unwrap();
        borsh::BorshSerialize::serialize(&uri, &mut expected_data).unwrap();
        borsh::BorshSerialize::serialize(&creator, &mut expected_data).unwrap();

        let expected_accounts = vec![
            AccountMeta::new(mint, true),
            AccountMeta::new_readonly(mint_authority, false),
            AccountMeta::new(bonding_curve, false),
            AccountMeta::new(associated_bonding_curve, false),
            AccountMeta::new_readonly(global, false),
            AccountMeta::new_readonly(mpl_token_metadata, false),
            AccountMeta::new(metadata, false),
            AccountMeta::new(user, true),
            AccountMeta::new_readonly(system_program, false),
            AccountMeta::new_readonly(token_program, false),
            AccountMeta::new_readonly(associated_token_program, false),
            AccountMeta::new_readonly(rent, false),
            AccountMeta::new_readonly(event_authority, false),
            AccountMeta::new_readonly(program, false),
        ];

        let gen_ix = instructions::create(
            &program_id,
            &instructions::CreateAccounts {
                mint,
                mint_authority,
                bonding_curve,
                associated_bonding_curve,
                global,
                mpl_token_metadata,
                metadata,
                user,
                system_program,
                token_program,
                associated_token_program,
                rent,
                event_authority,
                program,
            },
            name.clone(),
            symbol.clone(),
            uri.clone(),
            creator,
        );

        let expected_ix = solana_sdk::instruction::Instruction::new_with_bytes(
            program_id,
            &expected_data,
            expected_accounts,
        );
        assert_ix(&gen_ix, &expected_ix);
    }

    #[test]
    fn test_create_v2_instruction() {
        use pumpfun::instructions;
        #[allow(unused_imports)]
        use pumpfun::types;
        use solana_sdk::instruction::AccountMeta;

        let program_id = pumpfun::ID;
        let mint = Pubkey::from([1; 32]);
        let mint_authority = Pubkey::from([2; 32]);
        let bonding_curve = Pubkey::from([3; 32]);
        let associated_bonding_curve = Pubkey::from([4; 32]);
        let global = Pubkey::from([5; 32]);
        let user = Pubkey::from([6; 32]);
        let system_program = Pubkey::from([7; 32]);
        let token_program = Pubkey::from([8; 32]);
        let associated_token_program = Pubkey::from([9; 32]);
        let mayhem_program_id = Pubkey::from([10; 32]);
        let global_params = Pubkey::from([11; 32]);
        let sol_vault = Pubkey::from([12; 32]);
        let mayhem_state = Pubkey::from([13; 32]);
        let mayhem_token_vault = Pubkey::from([14; 32]);
        let event_authority = Pubkey::from([15; 32]);
        let program = Pubkey::from([16; 32]);
        let name = "test".to_string();
        let symbol = "test".to_string();
        let uri = "test".to_string();
        let creator = Pubkey::from([17; 32]);
        let is_mayhem_mode = false;

        let mut expected_data: Vec<u8> = vec![214, 144, 76, 236, 95, 139, 49, 180];
        borsh::BorshSerialize::serialize(&name, &mut expected_data).unwrap();
        borsh::BorshSerialize::serialize(&symbol, &mut expected_data).unwrap();
        borsh::BorshSerialize::serialize(&uri, &mut expected_data).unwrap();
        borsh::BorshSerialize::serialize(&creator, &mut expected_data).unwrap();
        borsh::BorshSerialize::serialize(&is_mayhem_mode, &mut expected_data).unwrap();

        let expected_accounts = vec![
            AccountMeta::new(mint, true),
            AccountMeta::new_readonly(mint_authority, false),
            AccountMeta::new(bonding_curve, false),
            AccountMeta::new(associated_bonding_curve, false),
            AccountMeta::new_readonly(global, false),
            AccountMeta::new(user, true),
            AccountMeta::new_readonly(system_program, false),
            AccountMeta::new_readonly(token_program, false),
            AccountMeta::new_readonly(associated_token_program, false),
            AccountMeta::new(mayhem_program_id, false),
            AccountMeta::new_readonly(global_params, false),
            AccountMeta::new(sol_vault, false),
            AccountMeta::new(mayhem_state, false),
            AccountMeta::new(mayhem_token_vault, false),
            AccountMeta::new_readonly(event_authority, false),
            AccountMeta::new_readonly(program, false),
        ];

        let gen_ix = instructions::create_v2(
            &program_id,
            &instructions::CreateV2Accounts {
                mint,
                mint_authority,
                bonding_curve,
                associated_bonding_curve,
                global,
                user,
                system_program,
                token_program,
                associated_token_program,
                mayhem_program_id,
                global_params,
                sol_vault,
                mayhem_state,
                mayhem_token_vault,
                event_authority,
                program,
            },
            name.clone(),
            symbol.clone(),
            uri.clone(),
            creator,
            is_mayhem_mode,
        );

        let expected_ix = solana_sdk::instruction::Instruction::new_with_bytes(
            program_id,
            &expected_data,
            expected_accounts,
        );
        assert_ix(&gen_ix, &expected_ix);
    }

    #[test]
    fn test_distribute_creator_fees_instruction() {
        use pumpfun::instructions;
        use solana_sdk::instruction::AccountMeta;

        let program_id = pumpfun::ID;
        let mint = Pubkey::from([1; 32]);
        let bonding_curve = Pubkey::from([2; 32]);
        let sharing_config = Pubkey::from([3; 32]);
        let creator_vault = Pubkey::from([4; 32]);
        let system_program = Pubkey::from([5; 32]);
        let event_authority = Pubkey::from([6; 32]);
        let program = Pubkey::from([7; 32]);

        let expected_data: Vec<u8> = vec![165, 114, 103, 0, 121, 206, 247, 81];

        let expected_accounts = vec![
            AccountMeta::new_readonly(mint, false),
            AccountMeta::new_readonly(bonding_curve, false),
            AccountMeta::new_readonly(sharing_config, false),
            AccountMeta::new(creator_vault, false),
            AccountMeta::new_readonly(system_program, false),
            AccountMeta::new_readonly(event_authority, false),
            AccountMeta::new_readonly(program, false),
        ];

        let gen_ix = instructions::distribute_creator_fees(
            &program_id,
            &instructions::DistributeCreatorFeesAccounts {
                mint,
                bonding_curve,
                sharing_config,
                creator_vault,
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
    fn test_extend_account_instruction() {
        use pumpfun::instructions;
        use solana_sdk::instruction::AccountMeta;

        let program_id = pumpfun::ID;
        let account = Pubkey::from([1; 32]);
        let user = Pubkey::from([2; 32]);
        let system_program = Pubkey::from([3; 32]);
        let event_authority = Pubkey::from([4; 32]);
        let program = Pubkey::from([5; 32]);

        let expected_data: Vec<u8> = vec![234, 102, 194, 203, 150, 72, 62, 229];

        let expected_accounts = vec![
            AccountMeta::new(account, false),
            AccountMeta::new_readonly(user, true),
            AccountMeta::new_readonly(system_program, false),
            AccountMeta::new_readonly(event_authority, false),
            AccountMeta::new_readonly(program, false),
        ];

        let gen_ix = instructions::extend_account(
            &program_id,
            &instructions::ExtendAccountAccounts {
                account,
                user,
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
    fn test_get_minimum_distributable_fee_instruction() {
        use pumpfun::instructions;
        use solana_sdk::instruction::AccountMeta;

        let program_id = pumpfun::ID;
        let mint = Pubkey::from([1; 32]);
        let bonding_curve = Pubkey::from([2; 32]);
        let sharing_config = Pubkey::from([3; 32]);
        let creator_vault = Pubkey::from([4; 32]);

        let expected_data: Vec<u8> = vec![117, 225, 127, 202, 134, 95, 68, 35];

        let expected_accounts = vec![
            AccountMeta::new_readonly(mint, false),
            AccountMeta::new_readonly(bonding_curve, false),
            AccountMeta::new_readonly(sharing_config, false),
            AccountMeta::new_readonly(creator_vault, false),
        ];

        let gen_ix = instructions::get_minimum_distributable_fee(
            &program_id,
            &instructions::GetMinimumDistributableFeeAccounts {
                mint,
                bonding_curve,
                sharing_config,
                creator_vault,
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
    fn test_init_user_volume_accumulator_instruction() {
        use pumpfun::instructions;
        use solana_sdk::instruction::AccountMeta;

        let program_id = pumpfun::ID;
        let payer = Pubkey::from([1; 32]);
        let user = Pubkey::from([2; 32]);
        let user_volume_accumulator = Pubkey::from([3; 32]);
        let system_program = Pubkey::from([4; 32]);
        let event_authority = Pubkey::from([5; 32]);
        let program = Pubkey::from([6; 32]);

        let expected_data: Vec<u8> = vec![94, 6, 202, 115, 255, 96, 232, 183];

        let expected_accounts = vec![
            AccountMeta::new(payer, true),
            AccountMeta::new_readonly(user, false),
            AccountMeta::new(user_volume_accumulator, false),
            AccountMeta::new_readonly(system_program, false),
            AccountMeta::new_readonly(event_authority, false),
            AccountMeta::new_readonly(program, false),
        ];

        let gen_ix = instructions::init_user_volume_accumulator(
            &program_id,
            &instructions::InitUserVolumeAccumulatorAccounts {
                payer,
                user,
                user_volume_accumulator,
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
    fn test_initialize_instruction() {
        use pumpfun::instructions;
        use solana_sdk::instruction::AccountMeta;

        let program_id = pumpfun::ID;
        let global = Pubkey::from([1; 32]);
        let user = Pubkey::from([2; 32]);
        let system_program = Pubkey::from([3; 32]);

        let expected_data: Vec<u8> = vec![175, 175, 109, 31, 13, 152, 155, 237];

        let expected_accounts = vec![
            AccountMeta::new(global, false),
            AccountMeta::new(user, true),
            AccountMeta::new_readonly(system_program, false),
        ];

        let gen_ix = instructions::initialize(
            &program_id,
            &instructions::InitializeAccounts {
                global,
                user,
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
    fn test_migrate_instruction() {
        use pumpfun::instructions;
        use solana_sdk::instruction::AccountMeta;

        let program_id = pumpfun::ID;
        let global = Pubkey::from([1; 32]);
        let withdraw_authority = Pubkey::from([2; 32]);
        let mint = Pubkey::from([3; 32]);
        let bonding_curve = Pubkey::from([4; 32]);
        let associated_bonding_curve = Pubkey::from([5; 32]);
        let user = Pubkey::from([6; 32]);
        let system_program = Pubkey::from([7; 32]);
        let token_program = Pubkey::from([8; 32]);
        let pump_amm = Pubkey::from([9; 32]);
        let pool = Pubkey::from([10; 32]);
        let pool_authority = Pubkey::from([11; 32]);
        let pool_authority_mint_account = Pubkey::from([12; 32]);
        let pool_authority_wsol_account = Pubkey::from([13; 32]);
        let amm_global_config = Pubkey::from([14; 32]);
        let wsol_mint = Pubkey::from([15; 32]);
        let lp_mint = Pubkey::from([16; 32]);
        let user_pool_token_account = Pubkey::from([17; 32]);
        let pool_base_token_account = Pubkey::from([18; 32]);
        let pool_quote_token_account = Pubkey::from([19; 32]);
        let token_2022_program = Pubkey::from([20; 32]);
        let associated_token_program = Pubkey::from([21; 32]);
        let pump_amm_event_authority = Pubkey::from([22; 32]);
        let event_authority = Pubkey::from([23; 32]);
        let program = Pubkey::from([24; 32]);

        let expected_data: Vec<u8> = vec![155, 234, 231, 146, 236, 158, 162, 30];

        let expected_accounts = vec![
            AccountMeta::new_readonly(global, false),
            AccountMeta::new(withdraw_authority, false),
            AccountMeta::new_readonly(mint, false),
            AccountMeta::new(bonding_curve, false),
            AccountMeta::new(associated_bonding_curve, false),
            AccountMeta::new_readonly(user, true),
            AccountMeta::new_readonly(system_program, false),
            AccountMeta::new_readonly(token_program, false),
            AccountMeta::new_readonly(pump_amm, false),
            AccountMeta::new(pool, false),
            AccountMeta::new(pool_authority, false),
            AccountMeta::new(pool_authority_mint_account, false),
            AccountMeta::new(pool_authority_wsol_account, false),
            AccountMeta::new_readonly(amm_global_config, false),
            AccountMeta::new_readonly(wsol_mint, false),
            AccountMeta::new(lp_mint, false),
            AccountMeta::new(user_pool_token_account, false),
            AccountMeta::new(pool_base_token_account, false),
            AccountMeta::new(pool_quote_token_account, false),
            AccountMeta::new_readonly(token_2022_program, false),
            AccountMeta::new_readonly(associated_token_program, false),
            AccountMeta::new_readonly(pump_amm_event_authority, false),
            AccountMeta::new_readonly(event_authority, false),
            AccountMeta::new_readonly(program, false),
        ];

        let gen_ix = instructions::migrate(
            &program_id,
            &instructions::MigrateAccounts {
                global,
                withdraw_authority,
                mint,
                bonding_curve,
                associated_bonding_curve,
                user,
                system_program,
                token_program,
                pump_amm,
                pool,
                pool_authority,
                pool_authority_mint_account,
                pool_authority_wsol_account,
                amm_global_config,
                wsol_mint,
                lp_mint,
                user_pool_token_account,
                pool_base_token_account,
                pool_quote_token_account,
                token_2022_program,
                associated_token_program,
                pump_amm_event_authority,
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
    fn test_migrate_bonding_curve_creator_instruction() {
        use pumpfun::instructions;
        use solana_sdk::instruction::AccountMeta;

        let program_id = pumpfun::ID;
        let mint = Pubkey::from([1; 32]);
        let bonding_curve = Pubkey::from([2; 32]);
        let sharing_config = Pubkey::from([3; 32]);
        let event_authority = Pubkey::from([4; 32]);
        let program = Pubkey::from([5; 32]);

        let expected_data: Vec<u8> = vec![87, 124, 52, 191, 52, 38, 214, 232];

        let expected_accounts = vec![
            AccountMeta::new_readonly(mint, false),
            AccountMeta::new(bonding_curve, false),
            AccountMeta::new_readonly(sharing_config, false),
            AccountMeta::new_readonly(event_authority, false),
            AccountMeta::new_readonly(program, false),
        ];

        let gen_ix = instructions::migrate_bonding_curve_creator(
            &program_id,
            &instructions::MigrateBondingCurveCreatorAccounts {
                mint,
                bonding_curve,
                sharing_config,
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
    fn test_sell_instruction() {
        use pumpfun::instructions;
        #[allow(unused_imports)]
        use pumpfun::types;
        use solana_sdk::instruction::AccountMeta;

        let program_id = pumpfun::ID;
        let global = Pubkey::from([1; 32]);
        let fee_recipient = Pubkey::from([2; 32]);
        let mint = Pubkey::from([3; 32]);
        let bonding_curve = Pubkey::from([4; 32]);
        let associated_bonding_curve = Pubkey::from([5; 32]);
        let associated_user = Pubkey::from([6; 32]);
        let user = Pubkey::from([7; 32]);
        let system_program = Pubkey::from([8; 32]);
        let creator_vault = Pubkey::from([9; 32]);
        let token_program = Pubkey::from([10; 32]);
        let event_authority = Pubkey::from([11; 32]);
        let program = Pubkey::from([12; 32]);
        let fee_config = Pubkey::from([13; 32]);
        let fee_program = Pubkey::from([14; 32]);
        let amount = 0u64;
        let min_sol_output = 0u64;

        let mut expected_data: Vec<u8> = vec![51, 230, 133, 164, 1, 127, 131, 173];
        borsh::BorshSerialize::serialize(&amount, &mut expected_data).unwrap();
        borsh::BorshSerialize::serialize(&min_sol_output, &mut expected_data).unwrap();

        let expected_accounts = vec![
            AccountMeta::new_readonly(global, false),
            AccountMeta::new(fee_recipient, false),
            AccountMeta::new_readonly(mint, false),
            AccountMeta::new(bonding_curve, false),
            AccountMeta::new(associated_bonding_curve, false),
            AccountMeta::new(associated_user, false),
            AccountMeta::new(user, true),
            AccountMeta::new_readonly(system_program, false),
            AccountMeta::new(creator_vault, false),
            AccountMeta::new_readonly(token_program, false),
            AccountMeta::new_readonly(event_authority, false),
            AccountMeta::new_readonly(program, false),
            AccountMeta::new_readonly(fee_config, false),
            AccountMeta::new_readonly(fee_program, false),
        ];

        let gen_ix = instructions::sell(
            &program_id,
            &instructions::SellAccounts {
                global,
                fee_recipient,
                mint,
                bonding_curve,
                associated_bonding_curve,
                associated_user,
                user,
                system_program,
                creator_vault,
                token_program,
                event_authority,
                program,
                fee_config,
                fee_program,
            },
            amount,
            min_sol_output,
        );

        let expected_ix = solana_sdk::instruction::Instruction::new_with_bytes(
            program_id,
            &expected_data,
            expected_accounts,
        );
        assert_ix(&gen_ix, &expected_ix);
    }

    #[test]
    fn test_set_creator_instruction() {
        use pumpfun::instructions;
        #[allow(unused_imports)]
        use pumpfun::types;
        use solana_sdk::instruction::AccountMeta;

        let program_id = pumpfun::ID;
        let set_creator_authority = Pubkey::from([1; 32]);
        let global = Pubkey::from([2; 32]);
        let mint = Pubkey::from([3; 32]);
        let metadata = Pubkey::from([4; 32]);
        let bonding_curve = Pubkey::from([5; 32]);
        let event_authority = Pubkey::from([6; 32]);
        let program = Pubkey::from([7; 32]);
        let creator = Pubkey::from([8; 32]);

        let mut expected_data: Vec<u8> = vec![254, 148, 255, 112, 207, 142, 170, 165];
        borsh::BorshSerialize::serialize(&creator, &mut expected_data).unwrap();

        let expected_accounts = vec![
            AccountMeta::new_readonly(set_creator_authority, true),
            AccountMeta::new_readonly(global, false),
            AccountMeta::new_readonly(mint, false),
            AccountMeta::new_readonly(metadata, false),
            AccountMeta::new(bonding_curve, false),
            AccountMeta::new_readonly(event_authority, false),
            AccountMeta::new_readonly(program, false),
        ];

        let gen_ix = instructions::set_creator(
            &program_id,
            &instructions::SetCreatorAccounts {
                set_creator_authority,
                global,
                mint,
                metadata,
                bonding_curve,
                event_authority,
                program,
            },
            creator,
        );

        let expected_ix = solana_sdk::instruction::Instruction::new_with_bytes(
            program_id,
            &expected_data,
            expected_accounts,
        );
        assert_ix(&gen_ix, &expected_ix);
    }

    #[test]
    fn test_set_mayhem_virtual_params_instruction() {
        use pumpfun::instructions;
        use solana_sdk::instruction::AccountMeta;

        let program_id = pumpfun::ID;
        let sol_vault_authority = Pubkey::from([1; 32]);
        let mayhem_token_vault = Pubkey::from([2; 32]);
        let mint = Pubkey::from([3; 32]);
        let global = Pubkey::from([4; 32]);
        let bonding_curve = Pubkey::from([5; 32]);
        let token_program = Pubkey::from([6; 32]);
        let event_authority = Pubkey::from([7; 32]);
        let program = Pubkey::from([8; 32]);

        let expected_data: Vec<u8> = vec![61, 169, 188, 191, 153, 149, 42, 97];

        let expected_accounts = vec![
            AccountMeta::new(sol_vault_authority, true),
            AccountMeta::new(mayhem_token_vault, false),
            AccountMeta::new_readonly(mint, false),
            AccountMeta::new_readonly(global, false),
            AccountMeta::new(bonding_curve, false),
            AccountMeta::new_readonly(token_program, false),
            AccountMeta::new_readonly(event_authority, false),
            AccountMeta::new_readonly(program, false),
        ];

        let gen_ix = instructions::set_mayhem_virtual_params(
            &program_id,
            &instructions::SetMayhemVirtualParamsAccounts {
                sol_vault_authority,
                mayhem_token_vault,
                mint,
                global,
                bonding_curve,
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
    fn test_set_metaplex_creator_instruction() {
        use pumpfun::instructions;
        use solana_sdk::instruction::AccountMeta;

        let program_id = pumpfun::ID;
        let mint = Pubkey::from([1; 32]);
        let metadata = Pubkey::from([2; 32]);
        let bonding_curve = Pubkey::from([3; 32]);
        let event_authority = Pubkey::from([4; 32]);
        let program = Pubkey::from([5; 32]);

        let expected_data: Vec<u8> = vec![138, 96, 174, 217, 48, 85, 197, 246];

        let expected_accounts = vec![
            AccountMeta::new_readonly(mint, false),
            AccountMeta::new_readonly(metadata, false),
            AccountMeta::new(bonding_curve, false),
            AccountMeta::new_readonly(event_authority, false),
            AccountMeta::new_readonly(program, false),
        ];

        let gen_ix = instructions::set_metaplex_creator(
            &program_id,
            &instructions::SetMetaplexCreatorAccounts {
                mint,
                metadata,
                bonding_curve,
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
    fn test_set_params_instruction() {
        use pumpfun::instructions;
        #[allow(unused_imports)]
        use pumpfun::types;
        use solana_sdk::instruction::AccountMeta;

        let program_id = pumpfun::ID;
        let global = Pubkey::from([1; 32]);
        let authority = Pubkey::from([2; 32]);
        let event_authority = Pubkey::from([3; 32]);
        let program = Pubkey::from([4; 32]);
        let initial_virtual_token_reserves = 0u64;
        let initial_virtual_sol_reserves = 0u64;
        let initial_real_token_reserves = 0u64;
        let token_total_supply = 0u64;
        let fee_basis_points = 0u64;
        let withdraw_authority = Pubkey::from([5; 32]);
        let enable_migrate = false;
        let pool_migration_fee = 0u64;
        let creator_fee_basis_points = 0u64;
        let set_creator_authority = Pubkey::from([6; 32]);
        let admin_set_creator_authority = Pubkey::from([7; 32]);

        let mut expected_data: Vec<u8> = vec![27, 234, 178, 52, 147, 2, 187, 141];
        borsh::BorshSerialize::serialize(&initial_virtual_token_reserves, &mut expected_data)
            .unwrap();
        borsh::BorshSerialize::serialize(&initial_virtual_sol_reserves, &mut expected_data)
            .unwrap();
        borsh::BorshSerialize::serialize(&initial_real_token_reserves, &mut expected_data).unwrap();
        borsh::BorshSerialize::serialize(&token_total_supply, &mut expected_data).unwrap();
        borsh::BorshSerialize::serialize(&fee_basis_points, &mut expected_data).unwrap();
        borsh::BorshSerialize::serialize(&withdraw_authority, &mut expected_data).unwrap();
        borsh::BorshSerialize::serialize(&enable_migrate, &mut expected_data).unwrap();
        borsh::BorshSerialize::serialize(&pool_migration_fee, &mut expected_data).unwrap();
        borsh::BorshSerialize::serialize(&creator_fee_basis_points, &mut expected_data).unwrap();
        borsh::BorshSerialize::serialize(&set_creator_authority, &mut expected_data).unwrap();
        borsh::BorshSerialize::serialize(&admin_set_creator_authority, &mut expected_data).unwrap();

        let expected_accounts = vec![
            AccountMeta::new(global, false),
            AccountMeta::new(authority, true),
            AccountMeta::new_readonly(event_authority, false),
            AccountMeta::new_readonly(program, false),
        ];

        let gen_ix = instructions::set_params(
            &program_id,
            &instructions::SetParamsAccounts {
                global,
                authority,
                event_authority,
                program,
            },
            initial_virtual_token_reserves,
            initial_virtual_sol_reserves,
            initial_real_token_reserves,
            token_total_supply,
            fee_basis_points,
            withdraw_authority,
            enable_migrate,
            pool_migration_fee,
            creator_fee_basis_points,
            set_creator_authority,
            admin_set_creator_authority,
        );

        let expected_ix = solana_sdk::instruction::Instruction::new_with_bytes(
            program_id,
            &expected_data,
            expected_accounts,
        );
        assert_ix(&gen_ix, &expected_ix);
    }

    #[test]
    fn test_set_reserved_fee_recipients_instruction() {
        use pumpfun::instructions;
        #[allow(unused_imports)]
        use pumpfun::types;
        use solana_sdk::instruction::AccountMeta;

        let program_id = pumpfun::ID;
        let global = Pubkey::from([1; 32]);
        let authority = Pubkey::from([2; 32]);
        let event_authority = Pubkey::from([3; 32]);
        let program = Pubkey::from([4; 32]);
        let whitelist_pda = Pubkey::from([5; 32]);

        let mut expected_data: Vec<u8> = vec![111, 172, 162, 232, 114, 89, 213, 142];
        borsh::BorshSerialize::serialize(&whitelist_pda, &mut expected_data).unwrap();

        let expected_accounts = vec![
            AccountMeta::new(global, false),
            AccountMeta::new_readonly(authority, true),
            AccountMeta::new_readonly(event_authority, false),
            AccountMeta::new_readonly(program, false),
        ];

        let gen_ix = instructions::set_reserved_fee_recipients(
            &program_id,
            &instructions::SetReservedFeeRecipientsAccounts {
                global,
                authority,
                event_authority,
                program,
            },
            whitelist_pda,
        );

        let expected_ix = solana_sdk::instruction::Instruction::new_with_bytes(
            program_id,
            &expected_data,
            expected_accounts,
        );
        assert_ix(&gen_ix, &expected_ix);
    }

    #[test]
    fn test_sync_user_volume_accumulator_instruction() {
        use pumpfun::instructions;
        use solana_sdk::instruction::AccountMeta;

        let program_id = pumpfun::ID;
        let user = Pubkey::from([1; 32]);
        let global_volume_accumulator = Pubkey::from([2; 32]);
        let user_volume_accumulator = Pubkey::from([3; 32]);
        let event_authority = Pubkey::from([4; 32]);
        let program = Pubkey::from([5; 32]);

        let expected_data: Vec<u8> = vec![86, 31, 192, 87, 163, 87, 79, 238];

        let expected_accounts = vec![
            AccountMeta::new_readonly(user, false),
            AccountMeta::new_readonly(global_volume_accumulator, false),
            AccountMeta::new(user_volume_accumulator, false),
            AccountMeta::new_readonly(event_authority, false),
            AccountMeta::new_readonly(program, false),
        ];

        let gen_ix = instructions::sync_user_volume_accumulator(
            &program_id,
            &instructions::SyncUserVolumeAccumulatorAccounts {
                user,
                global_volume_accumulator,
                user_volume_accumulator,
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
    fn test_toggle_create_v2_instruction() {
        use pumpfun::instructions;
        #[allow(unused_imports)]
        use pumpfun::types;
        use solana_sdk::instruction::AccountMeta;

        let program_id = pumpfun::ID;
        let global = Pubkey::from([1; 32]);
        let authority = Pubkey::from([2; 32]);
        let event_authority = Pubkey::from([3; 32]);
        let program = Pubkey::from([4; 32]);
        let enabled = false;

        let mut expected_data: Vec<u8> = vec![28, 255, 230, 240, 172, 107, 203, 171];
        borsh::BorshSerialize::serialize(&enabled, &mut expected_data).unwrap();

        let expected_accounts = vec![
            AccountMeta::new(global, false),
            AccountMeta::new(authority, true),
            AccountMeta::new_readonly(event_authority, false),
            AccountMeta::new_readonly(program, false),
        ];

        let gen_ix = instructions::toggle_create_v2(
            &program_id,
            &instructions::ToggleCreateV2Accounts {
                global,
                authority,
                event_authority,
                program,
            },
            enabled,
        );

        let expected_ix = solana_sdk::instruction::Instruction::new_with_bytes(
            program_id,
            &expected_data,
            expected_accounts,
        );
        assert_ix(&gen_ix, &expected_ix);
    }

    #[test]
    fn test_toggle_mayhem_mode_instruction() {
        use pumpfun::instructions;
        #[allow(unused_imports)]
        use pumpfun::types;
        use solana_sdk::instruction::AccountMeta;

        let program_id = pumpfun::ID;
        let global = Pubkey::from([1; 32]);
        let authority = Pubkey::from([2; 32]);
        let event_authority = Pubkey::from([3; 32]);
        let program = Pubkey::from([4; 32]);
        let enabled = false;

        let mut expected_data: Vec<u8> = vec![1, 9, 111, 208, 100, 31, 255, 163];
        borsh::BorshSerialize::serialize(&enabled, &mut expected_data).unwrap();

        let expected_accounts = vec![
            AccountMeta::new(global, false),
            AccountMeta::new(authority, true),
            AccountMeta::new_readonly(event_authority, false),
            AccountMeta::new_readonly(program, false),
        ];

        let gen_ix = instructions::toggle_mayhem_mode(
            &program_id,
            &instructions::ToggleMayhemModeAccounts {
                global,
                authority,
                event_authority,
                program,
            },
            enabled,
        );

        let expected_ix = solana_sdk::instruction::Instruction::new_with_bytes(
            program_id,
            &expected_data,
            expected_accounts,
        );
        assert_ix(&gen_ix, &expected_ix);
    }

    #[test]
    fn test_update_global_authority_instruction() {
        use pumpfun::instructions;
        use solana_sdk::instruction::AccountMeta;

        let program_id = pumpfun::ID;
        let global = Pubkey::from([1; 32]);
        let authority = Pubkey::from([2; 32]);
        let new_authority = Pubkey::from([3; 32]);
        let event_authority = Pubkey::from([4; 32]);
        let program = Pubkey::from([5; 32]);

        let expected_data: Vec<u8> = vec![227, 181, 74, 196, 208, 21, 97, 213];

        let expected_accounts = vec![
            AccountMeta::new(global, false),
            AccountMeta::new_readonly(authority, true),
            AccountMeta::new_readonly(new_authority, false),
            AccountMeta::new_readonly(event_authority, false),
            AccountMeta::new_readonly(program, false),
        ];

        let gen_ix = instructions::update_global_authority(
            &program_id,
            &instructions::UpdateGlobalAuthorityAccounts {
                global,
                authority,
                new_authority,
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

    // ── Types ───────────────────────────────────────────────────────

    #[test]
    fn test_option_bool_type() {
        use pumpfun::types::OptionBool;
        let ob = OptionBool(true);
        assert!(ob.0);
    }

    // - onchain fetch
    #[tokio::test]
    async fn test_fetch_bonding_curve() {
        let client = RpcClient::new("https://api.mainnet-beta.solana.com".to_string());
        let bonding_curve_address =
            solana_sdk::pubkey!("4cKijT6TkfwP7KBQ5R6ZcE6GGdaGRqCA5boTnvw61cWH");

        let bonding_curve_data = client
            .get_account_data(&bonding_curve_address)
            .await
            .unwrap();

        let bonding_curve = BondingCurve::from_account_data(&bonding_curve_data).unwrap();

        assert_eq!(
            bonding_curve.creator,
            solana_sdk::pubkey!("HeFU8pcfNU3UaN4MmpXBgPhTmxx9CKsnvsxgfp7vJmnn")
        );
    }
}
