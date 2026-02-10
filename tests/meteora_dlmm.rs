use anchor_parser::declare_program;

declare_program!(meteora_dlmm);

#[cfg(test)]
mod tests {
    use super::meteora_dlmm;
    use solana_sdk::pubkey::Pubkey;

    #[test]
    fn test_program_id() {
        assert_eq!(
            meteora_dlmm::ID.to_string(),
            "LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo"
        );
    }

    // ── Account discriminators ──────────────────────────────────────

    #[test]
    fn test_bin_array_discriminator() {
        use meteora_dlmm::accounts::BinArray;
        assert_eq!(BinArray::DISCRIMINATOR, [92, 142, 92, 220, 5, 148, 70, 181]);
    }

    #[test]
    fn test_bin_array_bitmap_extension_discriminator() {
        use meteora_dlmm::accounts::BinArrayBitmapExtension;
        assert_eq!(BinArrayBitmapExtension::DISCRIMINATOR, [80, 111, 124, 113, 55, 237, 18, 5]);
    }

    #[test]
    fn test_claim_fee_operator_discriminator() {
        use meteora_dlmm::accounts::ClaimFeeOperator;
        assert_eq!(ClaimFeeOperator::DISCRIMINATOR, [166, 48, 134, 86, 34, 200, 188, 150]);
    }

    #[test]
    fn test_dummy_zc_account_discriminator() {
        use meteora_dlmm::accounts::DummyZcAccount;
        assert_eq!(DummyZcAccount::DISCRIMINATOR, [94, 107, 238, 80, 208, 48, 180, 8]);
    }

    #[test]
    fn test_lb_pair_discriminator() {
        use meteora_dlmm::accounts::LbPair;
        assert_eq!(LbPair::DISCRIMINATOR, [33, 11, 49, 98, 181, 101, 177, 13]);
    }

    #[test]
    fn test_operator_discriminator() {
        use meteora_dlmm::accounts::Operator;
        assert_eq!(Operator::DISCRIMINATOR, [219, 31, 188, 145, 69, 139, 204, 117]);
    }

    #[test]
    fn test_oracle_discriminator() {
        use meteora_dlmm::accounts::Oracle;
        assert_eq!(Oracle::DISCRIMINATOR, [139, 194, 131, 179, 140, 179, 229, 244]);
    }

    #[test]
    fn test_position_discriminator() {
        use meteora_dlmm::accounts::Position;
        assert_eq!(Position::DISCRIMINATOR, [170, 188, 143, 228, 122, 64, 247, 208]);
    }

    #[test]
    fn test_position_v2_discriminator() {
        use meteora_dlmm::accounts::PositionV2;
        assert_eq!(PositionV2::DISCRIMINATOR, [117, 176, 212, 199, 245, 180, 133, 182]);
    }

    #[test]
    fn test_preset_parameter_discriminator() {
        use meteora_dlmm::accounts::PresetParameter;
        assert_eq!(PresetParameter::DISCRIMINATOR, [242, 62, 244, 34, 181, 112, 58, 170]);
    }

    #[test]
    fn test_preset_parameter2_discriminator() {
        use meteora_dlmm::accounts::PresetParameter2;
        assert_eq!(PresetParameter2::DISCRIMINATOR, [171, 236, 148, 115, 162, 113, 222, 174]);
    }

    #[test]
    fn test_token_badge_discriminator() {
        use meteora_dlmm::accounts::TokenBadge;
        assert_eq!(TokenBadge::DISCRIMINATOR, [116, 219, 204, 229, 249, 116, 255, 150]);
    }

    // ── Event discriminators ────────────────────────────────────────

    #[test]
    fn test_add_liquidity_event_discriminator() {
        use meteora_dlmm::events::AddLiquidity;
        assert_eq!(AddLiquidity::DISCRIMINATOR, [31, 94, 125, 90, 227, 52, 61, 186]);
    }

    #[test]
    fn test_claim_fee_event_discriminator() {
        use meteora_dlmm::events::ClaimFee;
        assert_eq!(ClaimFee::DISCRIMINATOR, [75, 122, 154, 48, 140, 74, 123, 163]);
    }

    #[test]
    fn test_claim_fee2_event_discriminator() {
        use meteora_dlmm::events::ClaimFee2;
        assert_eq!(ClaimFee2::DISCRIMINATOR, [232, 171, 242, 97, 58, 77, 35, 45]);
    }

    #[test]
    fn test_claim_reward_event_discriminator() {
        use meteora_dlmm::events::ClaimReward;
        assert_eq!(ClaimReward::DISCRIMINATOR, [148, 116, 134, 204, 22, 171, 85, 95]);
    }

    #[test]
    fn test_claim_reward2_event_discriminator() {
        use meteora_dlmm::events::ClaimReward2;
        assert_eq!(ClaimReward2::DISCRIMINATOR, [27, 143, 244, 33, 80, 43, 110, 146]);
    }

    #[test]
    fn test_composition_fee_event_discriminator() {
        use meteora_dlmm::events::CompositionFee;
        assert_eq!(CompositionFee::DISCRIMINATOR, [128, 151, 123, 106, 17, 102, 113, 142]);
    }

    #[test]
    fn test_decrease_position_length_event_discriminator() {
        use meteora_dlmm::events::DecreasePositionLength;
        assert_eq!(DecreasePositionLength::DISCRIMINATOR, [52, 118, 235, 85, 172, 169, 15, 128]);
    }

    #[test]
    fn test_dynamic_fee_parameter_update_event_discriminator() {
        use meteora_dlmm::events::DynamicFeeParameterUpdate;
        assert_eq!(DynamicFeeParameterUpdate::DISCRIMINATOR, [88, 88, 178, 135, 194, 146, 91, 243]);
    }

    #[test]
    fn test_fee_parameter_update_event_discriminator() {
        use meteora_dlmm::events::FeeParameterUpdate;
        assert_eq!(FeeParameterUpdate::DISCRIMINATOR, [48, 76, 241, 117, 144, 215, 242, 44]);
    }

    #[test]
    fn test_fund_reward_event_discriminator() {
        use meteora_dlmm::events::FundReward;
        assert_eq!(FundReward::DISCRIMINATOR, [246, 228, 58, 130, 145, 170, 79, 204]);
    }

    #[test]
    fn test_go_to_a_bin_event_discriminator() {
        use meteora_dlmm::events::GoToABin;
        assert_eq!(GoToABin::DISCRIMINATOR, [59, 138, 76, 68, 138, 131, 176, 67]);
    }

    #[test]
    fn test_increase_observation_event_discriminator() {
        use meteora_dlmm::events::IncreaseObservation;
        assert_eq!(IncreaseObservation::DISCRIMINATOR, [99, 249, 17, 121, 166, 156, 207, 215]);
    }

    #[test]
    fn test_increase_position_length_event_discriminator() {
        use meteora_dlmm::events::IncreasePositionLength;
        assert_eq!(IncreasePositionLength::DISCRIMINATOR, [157, 239, 42, 204, 30, 56, 223, 46]);
    }

    #[test]
    fn test_initialize_reward_event_discriminator() {
        use meteora_dlmm::events::InitializeReward;
        assert_eq!(InitializeReward::DISCRIMINATOR, [211, 153, 88, 62, 149, 60, 177, 70]);
    }

    #[test]
    fn test_lb_pair_create_event_discriminator() {
        use meteora_dlmm::events::LbPairCreate;
        assert_eq!(LbPairCreate::DISCRIMINATOR, [185, 74, 252, 125, 27, 215, 188, 111]);
    }

    #[test]
    fn test_position_close_event_discriminator() {
        use meteora_dlmm::events::PositionClose;
        assert_eq!(PositionClose::DISCRIMINATOR, [255, 196, 16, 107, 28, 202, 53, 128]);
    }

    #[test]
    fn test_position_create_event_discriminator() {
        use meteora_dlmm::events::PositionCreate;
        assert_eq!(PositionCreate::DISCRIMINATOR, [144, 142, 252, 84, 157, 53, 37, 121]);
    }

    #[test]
    fn test_rebalancing_event_discriminator() {
        use meteora_dlmm::events::Rebalancing;
        assert_eq!(Rebalancing::DISCRIMINATOR, [0, 109, 117, 179, 61, 91, 199, 200]);
    }

    #[test]
    fn test_remove_liquidity_event_discriminator() {
        use meteora_dlmm::events::RemoveLiquidity;
        assert_eq!(RemoveLiquidity::DISCRIMINATOR, [116, 244, 97, 232, 103, 31, 152, 58]);
    }

    #[test]
    fn test_swap_event_discriminator() {
        use meteora_dlmm::events::Swap;
        assert_eq!(Swap::DISCRIMINATOR, [81, 108, 227, 190, 205, 208, 10, 196]);
    }

    #[test]
    fn test_update_position_lock_release_point_event_discriminator() {
        use meteora_dlmm::events::UpdatePositionLockReleasePoint;
        assert_eq!(UpdatePositionLockReleasePoint::DISCRIMINATOR, [133, 214, 66, 224, 64, 12, 7, 191]);
    }

    #[test]
    fn test_update_position_operator_event_discriminator() {
        use meteora_dlmm::events::UpdatePositionOperator;
        assert_eq!(UpdatePositionOperator::DISCRIMINATOR, [39, 115, 48, 204, 246, 47, 66, 57]);
    }

    #[test]
    fn test_update_reward_duration_event_discriminator() {
        use meteora_dlmm::events::UpdateRewardDuration;
        assert_eq!(UpdateRewardDuration::DISCRIMINATOR, [223, 245, 224, 153, 49, 29, 163, 172]);
    }

    #[test]
    fn test_update_reward_funder_event_discriminator() {
        use meteora_dlmm::events::UpdateRewardFunder;
        assert_eq!(UpdateRewardFunder::DISCRIMINATOR, [224, 178, 174, 74, 252, 165, 85, 180]);
    }

    #[test]
    fn test_withdraw_ineligible_reward_event_discriminator() {
        use meteora_dlmm::events::WithdrawIneligibleReward;
        assert_eq!(WithdrawIneligibleReward::DISCRIMINATOR, [231, 189, 65, 149, 102, 215, 154, 244]);
    }

    fn assert_ix(actual: &solana_sdk::instruction::Instruction, expect: &solana_sdk::instruction::Instruction) {
        assert_eq!(actual.program_id, expect.program_id, "program_id mismatch");
        assert_eq!(actual.data, expect.data, "data mismatch");
        assert_eq!(actual.accounts.len(), expect.accounts.len(), "accounts len mismatch");
        for (i, (g, e)) in actual.accounts.iter().zip(expect.accounts.iter()).enumerate() {
            assert_eq!(g.pubkey, e.pubkey, "pubkey mismatch at account[{}]", i);
            assert_eq!(g.is_signer, e.is_signer, "is_signer mismatch at account[{}]", i);
            assert_eq!(g.is_writable, e.is_writable, "is_writable mismatch at account[{}]", i);
        }
    }

    // ── Instructions ────────────────────────────────────────────────

    #[test]
    fn test_add_liquidity_instruction() {
        use meteora_dlmm::instructions;
        #[allow(unused_imports)]
        use meteora_dlmm::types;
        use solana_sdk::instruction::AccountMeta;

        let program_id = meteora_dlmm::ID;
        let position = Pubkey::from([1; 32]);
        let lb_pair = Pubkey::from([2; 32]);
        let user_token_x = Pubkey::from([3; 32]);
        let user_token_y = Pubkey::from([4; 32]);
        let reserve_x = Pubkey::from([5; 32]);
        let reserve_y = Pubkey::from([6; 32]);
        let token_x_mint = Pubkey::from([7; 32]);
        let token_y_mint = Pubkey::from([8; 32]);
        let bin_array_lower = Pubkey::from([9; 32]);
        let bin_array_upper = Pubkey::from([10; 32]);
        let sender = Pubkey::from([11; 32]);
        let token_x_program = Pubkey::from([12; 32]);
        let token_y_program = Pubkey::from([13; 32]);
        let event_authority = Pubkey::from([14; 32]);
        let program = Pubkey::from([15; 32]);
        let liquidity_parameter = types::LiquidityParameter { amount_x: 0u64, amount_y: 0u64, bin_liquidity_dist: vec![] };

        let mut expected_data: Vec<u8> = vec![181, 157, 89, 67, 143, 182, 52, 72];
        borsh::BorshSerialize::serialize(&liquidity_parameter.clone(), &mut expected_data).unwrap();

        let expected_accounts = vec![
            AccountMeta::new(position, false),
            AccountMeta::new(lb_pair, false),
            AccountMeta::new_readonly(program_id, false),
            AccountMeta::new(user_token_x, false),
            AccountMeta::new(user_token_y, false),
            AccountMeta::new(reserve_x, false),
            AccountMeta::new(reserve_y, false),
            AccountMeta::new_readonly(token_x_mint, false),
            AccountMeta::new_readonly(token_y_mint, false),
            AccountMeta::new(bin_array_lower, false),
            AccountMeta::new(bin_array_upper, false),
            AccountMeta::new_readonly(sender, true),
            AccountMeta::new_readonly(token_x_program, false),
            AccountMeta::new_readonly(token_y_program, false),
            AccountMeta::new_readonly(event_authority, false),
            AccountMeta::new_readonly(program, false),
        ];

        let gen_ix = instructions::add_liquidity(
            &program_id,
            &instructions::AddLiquidityAccounts {
                position,
                lb_pair,
                bin_array_bitmap_extension: None,
                user_token_x,
                user_token_y,
                reserve_x,
                reserve_y,
                token_x_mint,
                token_y_mint,
                bin_array_lower,
                bin_array_upper,
                sender,
                token_x_program,
                token_y_program,
                event_authority,
                program,
            },
            liquidity_parameter.clone(),
        );

        let expected_ix = solana_sdk::instruction::Instruction::new_with_bytes(
            program_id,
            &expected_data,
            expected_accounts,
        );
        assert_ix(&gen_ix, &expected_ix);
    }

    #[test]
    fn test_add_liquidity2_instruction() {
        use meteora_dlmm::instructions;
        #[allow(unused_imports)]
        use meteora_dlmm::types;
        use solana_sdk::instruction::AccountMeta;

        let program_id = meteora_dlmm::ID;
        let position = Pubkey::from([1; 32]);
        let lb_pair = Pubkey::from([2; 32]);
        let user_token_x = Pubkey::from([3; 32]);
        let user_token_y = Pubkey::from([4; 32]);
        let reserve_x = Pubkey::from([5; 32]);
        let reserve_y = Pubkey::from([6; 32]);
        let token_x_mint = Pubkey::from([7; 32]);
        let token_y_mint = Pubkey::from([8; 32]);
        let sender = Pubkey::from([9; 32]);
        let token_x_program = Pubkey::from([10; 32]);
        let token_y_program = Pubkey::from([11; 32]);
        let event_authority = Pubkey::from([12; 32]);
        let program = Pubkey::from([13; 32]);
        let liquidity_parameter = types::LiquidityParameter { amount_x: 0u64, amount_y: 0u64, bin_liquidity_dist: vec![] };
        let remaining_accounts_info = types::RemainingAccountsInfo { slices: vec![] };

        let mut expected_data: Vec<u8> = vec![228, 162, 78, 28, 70, 219, 116, 115];
        borsh::BorshSerialize::serialize(&liquidity_parameter.clone(), &mut expected_data).unwrap();
        borsh::BorshSerialize::serialize(&remaining_accounts_info.clone(), &mut expected_data).unwrap();

        let expected_accounts = vec![
            AccountMeta::new(position, false),
            AccountMeta::new(lb_pair, false),
            AccountMeta::new_readonly(program_id, false),
            AccountMeta::new(user_token_x, false),
            AccountMeta::new(user_token_y, false),
            AccountMeta::new(reserve_x, false),
            AccountMeta::new(reserve_y, false),
            AccountMeta::new_readonly(token_x_mint, false),
            AccountMeta::new_readonly(token_y_mint, false),
            AccountMeta::new_readonly(sender, true),
            AccountMeta::new_readonly(token_x_program, false),
            AccountMeta::new_readonly(token_y_program, false),
            AccountMeta::new_readonly(event_authority, false),
            AccountMeta::new_readonly(program, false),
        ];

        let gen_ix = instructions::add_liquidity2(
            &program_id,
            &instructions::AddLiquidity2Accounts {
                position,
                lb_pair,
                bin_array_bitmap_extension: None,
                user_token_x,
                user_token_y,
                reserve_x,
                reserve_y,
                token_x_mint,
                token_y_mint,
                sender,
                token_x_program,
                token_y_program,
                event_authority,
                program,
            },
            liquidity_parameter.clone(),
            remaining_accounts_info.clone(),
        );

        let expected_ix = solana_sdk::instruction::Instruction::new_with_bytes(
            program_id,
            &expected_data,
            expected_accounts,
        );
        assert_ix(&gen_ix, &expected_ix);
    }

    #[test]
    fn test_add_liquidity_by_strategy_instruction() {
        use meteora_dlmm::instructions;
        #[allow(unused_imports)]
        use meteora_dlmm::types;
        use solana_sdk::instruction::AccountMeta;

        let program_id = meteora_dlmm::ID;
        let position = Pubkey::from([1; 32]);
        let lb_pair = Pubkey::from([2; 32]);
        let user_token_x = Pubkey::from([3; 32]);
        let user_token_y = Pubkey::from([4; 32]);
        let reserve_x = Pubkey::from([5; 32]);
        let reserve_y = Pubkey::from([6; 32]);
        let token_x_mint = Pubkey::from([7; 32]);
        let token_y_mint = Pubkey::from([8; 32]);
        let bin_array_lower = Pubkey::from([9; 32]);
        let bin_array_upper = Pubkey::from([10; 32]);
        let sender = Pubkey::from([11; 32]);
        let token_x_program = Pubkey::from([12; 32]);
        let token_y_program = Pubkey::from([13; 32]);
        let event_authority = Pubkey::from([14; 32]);
        let program = Pubkey::from([15; 32]);
        let liquidity_parameter = types::LiquidityParameterByStrategy { amount_x: 0u64, amount_y: 0u64, active_id: 0i32, max_active_bin_slippage: 0i32, strategy_parameters: types::StrategyParameters { min_bin_id: 0i32, max_bin_id: 0i32, strategy_type: types::StrategyType::SpotOneSide, parameteres: [0u8; 64] } };

        let mut expected_data: Vec<u8> = vec![7, 3, 150, 127, 148, 40, 61, 200];
        borsh::BorshSerialize::serialize(&liquidity_parameter.clone(), &mut expected_data).unwrap();

        let expected_accounts = vec![
            AccountMeta::new(position, false),
            AccountMeta::new(lb_pair, false),
            AccountMeta::new_readonly(program_id, false),
            AccountMeta::new(user_token_x, false),
            AccountMeta::new(user_token_y, false),
            AccountMeta::new(reserve_x, false),
            AccountMeta::new(reserve_y, false),
            AccountMeta::new_readonly(token_x_mint, false),
            AccountMeta::new_readonly(token_y_mint, false),
            AccountMeta::new(bin_array_lower, false),
            AccountMeta::new(bin_array_upper, false),
            AccountMeta::new_readonly(sender, true),
            AccountMeta::new_readonly(token_x_program, false),
            AccountMeta::new_readonly(token_y_program, false),
            AccountMeta::new_readonly(event_authority, false),
            AccountMeta::new_readonly(program, false),
        ];

        let gen_ix = instructions::add_liquidity_by_strategy(
            &program_id,
            &instructions::AddLiquidityByStrategyAccounts {
                position,
                lb_pair,
                bin_array_bitmap_extension: None,
                user_token_x,
                user_token_y,
                reserve_x,
                reserve_y,
                token_x_mint,
                token_y_mint,
                bin_array_lower,
                bin_array_upper,
                sender,
                token_x_program,
                token_y_program,
                event_authority,
                program,
            },
            liquidity_parameter.clone(),
        );

        let expected_ix = solana_sdk::instruction::Instruction::new_with_bytes(
            program_id,
            &expected_data,
            expected_accounts,
        );
        assert_ix(&gen_ix, &expected_ix);
    }

    #[test]
    fn test_add_liquidity_by_strategy2_instruction() {
        use meteora_dlmm::instructions;
        #[allow(unused_imports)]
        use meteora_dlmm::types;
        use solana_sdk::instruction::AccountMeta;

        let program_id = meteora_dlmm::ID;
        let position = Pubkey::from([1; 32]);
        let lb_pair = Pubkey::from([2; 32]);
        let user_token_x = Pubkey::from([3; 32]);
        let user_token_y = Pubkey::from([4; 32]);
        let reserve_x = Pubkey::from([5; 32]);
        let reserve_y = Pubkey::from([6; 32]);
        let token_x_mint = Pubkey::from([7; 32]);
        let token_y_mint = Pubkey::from([8; 32]);
        let sender = Pubkey::from([9; 32]);
        let token_x_program = Pubkey::from([10; 32]);
        let token_y_program = Pubkey::from([11; 32]);
        let event_authority = Pubkey::from([12; 32]);
        let program = Pubkey::from([13; 32]);
        let liquidity_parameter = types::LiquidityParameterByStrategy { amount_x: 0u64, amount_y: 0u64, active_id: 0i32, max_active_bin_slippage: 0i32, strategy_parameters: types::StrategyParameters { min_bin_id: 0i32, max_bin_id: 0i32, strategy_type: types::StrategyType::SpotOneSide, parameteres: [0u8; 64] } };
        let remaining_accounts_info = types::RemainingAccountsInfo { slices: vec![] };

        let mut expected_data: Vec<u8> = vec![3, 221, 149, 218, 111, 141, 118, 213];
        borsh::BorshSerialize::serialize(&liquidity_parameter.clone(), &mut expected_data).unwrap();
        borsh::BorshSerialize::serialize(&remaining_accounts_info.clone(), &mut expected_data).unwrap();

        let expected_accounts = vec![
            AccountMeta::new(position, false),
            AccountMeta::new(lb_pair, false),
            AccountMeta::new_readonly(program_id, false),
            AccountMeta::new(user_token_x, false),
            AccountMeta::new(user_token_y, false),
            AccountMeta::new(reserve_x, false),
            AccountMeta::new(reserve_y, false),
            AccountMeta::new_readonly(token_x_mint, false),
            AccountMeta::new_readonly(token_y_mint, false),
            AccountMeta::new_readonly(sender, true),
            AccountMeta::new_readonly(token_x_program, false),
            AccountMeta::new_readonly(token_y_program, false),
            AccountMeta::new_readonly(event_authority, false),
            AccountMeta::new_readonly(program, false),
        ];

        let gen_ix = instructions::add_liquidity_by_strategy2(
            &program_id,
            &instructions::AddLiquidityByStrategy2Accounts {
                position,
                lb_pair,
                bin_array_bitmap_extension: None,
                user_token_x,
                user_token_y,
                reserve_x,
                reserve_y,
                token_x_mint,
                token_y_mint,
                sender,
                token_x_program,
                token_y_program,
                event_authority,
                program,
            },
            liquidity_parameter.clone(),
            remaining_accounts_info.clone(),
        );

        let expected_ix = solana_sdk::instruction::Instruction::new_with_bytes(
            program_id,
            &expected_data,
            expected_accounts,
        );
        assert_ix(&gen_ix, &expected_ix);
    }

    #[test]
    fn test_add_liquidity_by_strategy_one_side_instruction() {
        use meteora_dlmm::instructions;
        #[allow(unused_imports)]
        use meteora_dlmm::types;
        use solana_sdk::instruction::AccountMeta;

        let program_id = meteora_dlmm::ID;
        let position = Pubkey::from([1; 32]);
        let lb_pair = Pubkey::from([2; 32]);
        let user_token = Pubkey::from([3; 32]);
        let reserve = Pubkey::from([4; 32]);
        let token_mint = Pubkey::from([5; 32]);
        let bin_array_lower = Pubkey::from([6; 32]);
        let bin_array_upper = Pubkey::from([7; 32]);
        let sender = Pubkey::from([8; 32]);
        let token_program = Pubkey::from([9; 32]);
        let event_authority = Pubkey::from([10; 32]);
        let program = Pubkey::from([11; 32]);
        let liquidity_parameter = types::LiquidityParameterByStrategyOneSide { amount: 0u64, active_id: 0i32, max_active_bin_slippage: 0i32, strategy_parameters: types::StrategyParameters { min_bin_id: 0i32, max_bin_id: 0i32, strategy_type: types::StrategyType::SpotOneSide, parameteres: [0u8; 64] } };

        let mut expected_data: Vec<u8> = vec![41, 5, 238, 175, 100, 225, 6, 205];
        borsh::BorshSerialize::serialize(&liquidity_parameter.clone(), &mut expected_data).unwrap();

        let expected_accounts = vec![
            AccountMeta::new(position, false),
            AccountMeta::new(lb_pair, false),
            AccountMeta::new_readonly(program_id, false),
            AccountMeta::new(user_token, false),
            AccountMeta::new(reserve, false),
            AccountMeta::new_readonly(token_mint, false),
            AccountMeta::new(bin_array_lower, false),
            AccountMeta::new(bin_array_upper, false),
            AccountMeta::new_readonly(sender, true),
            AccountMeta::new_readonly(token_program, false),
            AccountMeta::new_readonly(event_authority, false),
            AccountMeta::new_readonly(program, false),
        ];

        let gen_ix = instructions::add_liquidity_by_strategy_one_side(
            &program_id,
            &instructions::AddLiquidityByStrategyOneSideAccounts {
                position,
                lb_pair,
                bin_array_bitmap_extension: None,
                user_token,
                reserve,
                token_mint,
                bin_array_lower,
                bin_array_upper,
                sender,
                token_program,
                event_authority,
                program,
            },
            liquidity_parameter.clone(),
        );

        let expected_ix = solana_sdk::instruction::Instruction::new_with_bytes(
            program_id,
            &expected_data,
            expected_accounts,
        );
        assert_ix(&gen_ix, &expected_ix);
    }

    #[test]
    fn test_add_liquidity_by_weight_instruction() {
        use meteora_dlmm::instructions;
        #[allow(unused_imports)]
        use meteora_dlmm::types;
        use solana_sdk::instruction::AccountMeta;

        let program_id = meteora_dlmm::ID;
        let position = Pubkey::from([1; 32]);
        let lb_pair = Pubkey::from([2; 32]);
        let user_token_x = Pubkey::from([3; 32]);
        let user_token_y = Pubkey::from([4; 32]);
        let reserve_x = Pubkey::from([5; 32]);
        let reserve_y = Pubkey::from([6; 32]);
        let token_x_mint = Pubkey::from([7; 32]);
        let token_y_mint = Pubkey::from([8; 32]);
        let bin_array_lower = Pubkey::from([9; 32]);
        let bin_array_upper = Pubkey::from([10; 32]);
        let sender = Pubkey::from([11; 32]);
        let token_x_program = Pubkey::from([12; 32]);
        let token_y_program = Pubkey::from([13; 32]);
        let event_authority = Pubkey::from([14; 32]);
        let program = Pubkey::from([15; 32]);
        let liquidity_parameter = types::LiquidityParameterByWeight { amount_x: 0u64, amount_y: 0u64, active_id: 0i32, max_active_bin_slippage: 0i32, bin_liquidity_dist: vec![] };

        let mut expected_data: Vec<u8> = vec![28, 140, 238, 99, 231, 162, 21, 149];
        borsh::BorshSerialize::serialize(&liquidity_parameter.clone(), &mut expected_data).unwrap();

        let expected_accounts = vec![
            AccountMeta::new(position, false),
            AccountMeta::new(lb_pair, false),
            AccountMeta::new_readonly(program_id, false),
            AccountMeta::new(user_token_x, false),
            AccountMeta::new(user_token_y, false),
            AccountMeta::new(reserve_x, false),
            AccountMeta::new(reserve_y, false),
            AccountMeta::new_readonly(token_x_mint, false),
            AccountMeta::new_readonly(token_y_mint, false),
            AccountMeta::new(bin_array_lower, false),
            AccountMeta::new(bin_array_upper, false),
            AccountMeta::new_readonly(sender, true),
            AccountMeta::new_readonly(token_x_program, false),
            AccountMeta::new_readonly(token_y_program, false),
            AccountMeta::new_readonly(event_authority, false),
            AccountMeta::new_readonly(program, false),
        ];

        let gen_ix = instructions::add_liquidity_by_weight(
            &program_id,
            &instructions::AddLiquidityByWeightAccounts {
                position,
                lb_pair,
                bin_array_bitmap_extension: None,
                user_token_x,
                user_token_y,
                reserve_x,
                reserve_y,
                token_x_mint,
                token_y_mint,
                bin_array_lower,
                bin_array_upper,
                sender,
                token_x_program,
                token_y_program,
                event_authority,
                program,
            },
            liquidity_parameter.clone(),
        );

        let expected_ix = solana_sdk::instruction::Instruction::new_with_bytes(
            program_id,
            &expected_data,
            expected_accounts,
        );
        assert_ix(&gen_ix, &expected_ix);
    }

    #[test]
    fn test_add_liquidity_one_side_instruction() {
        use meteora_dlmm::instructions;
        #[allow(unused_imports)]
        use meteora_dlmm::types;
        use solana_sdk::instruction::AccountMeta;

        let program_id = meteora_dlmm::ID;
        let position = Pubkey::from([1; 32]);
        let lb_pair = Pubkey::from([2; 32]);
        let user_token = Pubkey::from([3; 32]);
        let reserve = Pubkey::from([4; 32]);
        let token_mint = Pubkey::from([5; 32]);
        let bin_array_lower = Pubkey::from([6; 32]);
        let bin_array_upper = Pubkey::from([7; 32]);
        let sender = Pubkey::from([8; 32]);
        let token_program = Pubkey::from([9; 32]);
        let event_authority = Pubkey::from([10; 32]);
        let program = Pubkey::from([11; 32]);
        let liquidity_parameter = types::LiquidityOneSideParameter { amount: 0u64, active_id: 0i32, max_active_bin_slippage: 0i32, bin_liquidity_dist: vec![] };

        let mut expected_data: Vec<u8> = vec![94, 155, 103, 151, 70, 95, 220, 165];
        borsh::BorshSerialize::serialize(&liquidity_parameter.clone(), &mut expected_data).unwrap();

        let expected_accounts = vec![
            AccountMeta::new(position, false),
            AccountMeta::new(lb_pair, false),
            AccountMeta::new_readonly(program_id, false),
            AccountMeta::new(user_token, false),
            AccountMeta::new(reserve, false),
            AccountMeta::new_readonly(token_mint, false),
            AccountMeta::new(bin_array_lower, false),
            AccountMeta::new(bin_array_upper, false),
            AccountMeta::new_readonly(sender, true),
            AccountMeta::new_readonly(token_program, false),
            AccountMeta::new_readonly(event_authority, false),
            AccountMeta::new_readonly(program, false),
        ];

        let gen_ix = instructions::add_liquidity_one_side(
            &program_id,
            &instructions::AddLiquidityOneSideAccounts {
                position,
                lb_pair,
                bin_array_bitmap_extension: None,
                user_token,
                reserve,
                token_mint,
                bin_array_lower,
                bin_array_upper,
                sender,
                token_program,
                event_authority,
                program,
            },
            liquidity_parameter.clone(),
        );

        let expected_ix = solana_sdk::instruction::Instruction::new_with_bytes(
            program_id,
            &expected_data,
            expected_accounts,
        );
        assert_ix(&gen_ix, &expected_ix);
    }

    #[test]
    fn test_add_liquidity_one_side_precise_instruction() {
        use meteora_dlmm::instructions;
        #[allow(unused_imports)]
        use meteora_dlmm::types;
        use solana_sdk::instruction::AccountMeta;

        let program_id = meteora_dlmm::ID;
        let position = Pubkey::from([1; 32]);
        let lb_pair = Pubkey::from([2; 32]);
        let user_token = Pubkey::from([3; 32]);
        let reserve = Pubkey::from([4; 32]);
        let token_mint = Pubkey::from([5; 32]);
        let bin_array_lower = Pubkey::from([6; 32]);
        let bin_array_upper = Pubkey::from([7; 32]);
        let sender = Pubkey::from([8; 32]);
        let token_program = Pubkey::from([9; 32]);
        let event_authority = Pubkey::from([10; 32]);
        let program = Pubkey::from([11; 32]);
        let parameter = types::AddLiquiditySingleSidePreciseParameter { bins: vec![], decompress_multiplier: 0u64 };

        let mut expected_data: Vec<u8> = vec![161, 194, 103, 84, 171, 71, 250, 154];
        borsh::BorshSerialize::serialize(&parameter.clone(), &mut expected_data).unwrap();

        let expected_accounts = vec![
            AccountMeta::new(position, false),
            AccountMeta::new(lb_pair, false),
            AccountMeta::new_readonly(program_id, false),
            AccountMeta::new(user_token, false),
            AccountMeta::new(reserve, false),
            AccountMeta::new_readonly(token_mint, false),
            AccountMeta::new(bin_array_lower, false),
            AccountMeta::new(bin_array_upper, false),
            AccountMeta::new_readonly(sender, true),
            AccountMeta::new_readonly(token_program, false),
            AccountMeta::new_readonly(event_authority, false),
            AccountMeta::new_readonly(program, false),
        ];

        let gen_ix = instructions::add_liquidity_one_side_precise(
            &program_id,
            &instructions::AddLiquidityOneSidePreciseAccounts {
                position,
                lb_pair,
                bin_array_bitmap_extension: None,
                user_token,
                reserve,
                token_mint,
                bin_array_lower,
                bin_array_upper,
                sender,
                token_program,
                event_authority,
                program,
            },
            parameter.clone(),
        );

        let expected_ix = solana_sdk::instruction::Instruction::new_with_bytes(
            program_id,
            &expected_data,
            expected_accounts,
        );
        assert_ix(&gen_ix, &expected_ix);
    }

    #[test]
    fn test_add_liquidity_one_side_precise2_instruction() {
        use meteora_dlmm::instructions;
        #[allow(unused_imports)]
        use meteora_dlmm::types;
        use solana_sdk::instruction::AccountMeta;

        let program_id = meteora_dlmm::ID;
        let position = Pubkey::from([1; 32]);
        let lb_pair = Pubkey::from([2; 32]);
        let user_token = Pubkey::from([3; 32]);
        let reserve = Pubkey::from([4; 32]);
        let token_mint = Pubkey::from([5; 32]);
        let sender = Pubkey::from([6; 32]);
        let token_program = Pubkey::from([7; 32]);
        let event_authority = Pubkey::from([8; 32]);
        let program = Pubkey::from([9; 32]);
        let liquidity_parameter = types::AddLiquiditySingleSidePreciseParameter2 { bins: vec![], decompress_multiplier: 0u64, max_amount: 0u64 };
        let remaining_accounts_info = types::RemainingAccountsInfo { slices: vec![] };

        let mut expected_data: Vec<u8> = vec![33, 51, 163, 201, 117, 98, 125, 231];
        borsh::BorshSerialize::serialize(&liquidity_parameter.clone(), &mut expected_data).unwrap();
        borsh::BorshSerialize::serialize(&remaining_accounts_info.clone(), &mut expected_data).unwrap();

        let expected_accounts = vec![
            AccountMeta::new(position, false),
            AccountMeta::new(lb_pair, false),
            AccountMeta::new_readonly(program_id, false),
            AccountMeta::new(user_token, false),
            AccountMeta::new(reserve, false),
            AccountMeta::new_readonly(token_mint, false),
            AccountMeta::new_readonly(sender, true),
            AccountMeta::new_readonly(token_program, false),
            AccountMeta::new_readonly(event_authority, false),
            AccountMeta::new_readonly(program, false),
        ];

        let gen_ix = instructions::add_liquidity_one_side_precise2(
            &program_id,
            &instructions::AddLiquidityOneSidePrecise2Accounts {
                position,
                lb_pair,
                bin_array_bitmap_extension: None,
                user_token,
                reserve,
                token_mint,
                sender,
                token_program,
                event_authority,
                program,
            },
            liquidity_parameter.clone(),
            remaining_accounts_info.clone(),
        );

        let expected_ix = solana_sdk::instruction::Instruction::new_with_bytes(
            program_id,
            &expected_data,
            expected_accounts,
        );
        assert_ix(&gen_ix, &expected_ix);
    }

    #[test]
    fn test_claim_fee_instruction() {
        use meteora_dlmm::instructions;
        #[allow(unused_imports)]
        use meteora_dlmm::types;
        use solana_sdk::instruction::AccountMeta;

        let program_id = meteora_dlmm::ID;
        let lb_pair = Pubkey::from([1; 32]);
        let position = Pubkey::from([2; 32]);
        let bin_array_lower = Pubkey::from([3; 32]);
        let bin_array_upper = Pubkey::from([4; 32]);
        let sender = Pubkey::from([5; 32]);
        let reserve_x = Pubkey::from([6; 32]);
        let reserve_y = Pubkey::from([7; 32]);
        let user_token_x = Pubkey::from([8; 32]);
        let user_token_y = Pubkey::from([9; 32]);
        let token_x_mint = Pubkey::from([10; 32]);
        let token_y_mint = Pubkey::from([11; 32]);
        let token_program = Pubkey::from([12; 32]);
        let event_authority = Pubkey::from([13; 32]);
        let program = Pubkey::from([14; 32]);

        let expected_data: Vec<u8> = vec![169, 32, 79, 137, 136, 232, 70, 137];

        let expected_accounts = vec![
            AccountMeta::new(lb_pair, false),
            AccountMeta::new(position, false),
            AccountMeta::new(bin_array_lower, false),
            AccountMeta::new(bin_array_upper, false),
            AccountMeta::new_readonly(sender, true),
            AccountMeta::new(reserve_x, false),
            AccountMeta::new(reserve_y, false),
            AccountMeta::new(user_token_x, false),
            AccountMeta::new(user_token_y, false),
            AccountMeta::new_readonly(token_x_mint, false),
            AccountMeta::new_readonly(token_y_mint, false),
            AccountMeta::new_readonly(token_program, false),
            AccountMeta::new_readonly(event_authority, false),
            AccountMeta::new_readonly(program, false),
        ];

        let gen_ix = instructions::claim_fee(
            &program_id,
            &instructions::ClaimFeeAccounts {
                lb_pair,
                position,
                bin_array_lower,
                bin_array_upper,
                sender,
                reserve_x,
                reserve_y,
                user_token_x,
                user_token_y,
                token_x_mint,
                token_y_mint,
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
    fn test_claim_fee2_instruction() {
        use meteora_dlmm::instructions;
        #[allow(unused_imports)]
        use meteora_dlmm::types;
        use solana_sdk::instruction::AccountMeta;

        let program_id = meteora_dlmm::ID;
        let lb_pair = Pubkey::from([1; 32]);
        let position = Pubkey::from([2; 32]);
        let sender = Pubkey::from([3; 32]);
        let reserve_x = Pubkey::from([4; 32]);
        let reserve_y = Pubkey::from([5; 32]);
        let user_token_x = Pubkey::from([6; 32]);
        let user_token_y = Pubkey::from([7; 32]);
        let token_x_mint = Pubkey::from([8; 32]);
        let token_y_mint = Pubkey::from([9; 32]);
        let token_program_x = Pubkey::from([10; 32]);
        let token_program_y = Pubkey::from([11; 32]);
        let memo_program = Pubkey::from([12; 32]);
        let event_authority = Pubkey::from([13; 32]);
        let program = Pubkey::from([14; 32]);
        let min_bin_id = 0i32;
        let max_bin_id = 0i32;
        let remaining_accounts_info = types::RemainingAccountsInfo { slices: vec![] };

        let mut expected_data: Vec<u8> = vec![112, 191, 101, 171, 28, 144, 127, 187];
        borsh::BorshSerialize::serialize(&min_bin_id, &mut expected_data).unwrap();
        borsh::BorshSerialize::serialize(&max_bin_id, &mut expected_data).unwrap();
        borsh::BorshSerialize::serialize(&remaining_accounts_info.clone(), &mut expected_data).unwrap();

        let expected_accounts = vec![
            AccountMeta::new(lb_pair, false),
            AccountMeta::new(position, false),
            AccountMeta::new_readonly(sender, true),
            AccountMeta::new(reserve_x, false),
            AccountMeta::new(reserve_y, false),
            AccountMeta::new(user_token_x, false),
            AccountMeta::new(user_token_y, false),
            AccountMeta::new_readonly(token_x_mint, false),
            AccountMeta::new_readonly(token_y_mint, false),
            AccountMeta::new_readonly(token_program_x, false),
            AccountMeta::new_readonly(token_program_y, false),
            AccountMeta::new_readonly(memo_program, false),
            AccountMeta::new_readonly(event_authority, false),
            AccountMeta::new_readonly(program, false),
        ];

        let gen_ix = instructions::claim_fee2(
            &program_id,
            &instructions::ClaimFee2Accounts {
                lb_pair,
                position,
                sender,
                reserve_x,
                reserve_y,
                user_token_x,
                user_token_y,
                token_x_mint,
                token_y_mint,
                token_program_x,
                token_program_y,
                memo_program,
                event_authority,
                program,
            },
            min_bin_id,
            max_bin_id,
            remaining_accounts_info.clone(),
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
        use meteora_dlmm::instructions;
        #[allow(unused_imports)]
        use meteora_dlmm::types;
        use solana_sdk::instruction::AccountMeta;

        let program_id = meteora_dlmm::ID;
        let lb_pair = Pubkey::from([1; 32]);
        let position = Pubkey::from([2; 32]);
        let bin_array_lower = Pubkey::from([3; 32]);
        let bin_array_upper = Pubkey::from([4; 32]);
        let sender = Pubkey::from([5; 32]);
        let reward_vault = Pubkey::from([6; 32]);
        let reward_mint = Pubkey::from([7; 32]);
        let user_token_account = Pubkey::from([8; 32]);
        let token_program = Pubkey::from([9; 32]);
        let event_authority = Pubkey::from([10; 32]);
        let program = Pubkey::from([11; 32]);
        let reward_index = 0u64;

        let mut expected_data: Vec<u8> = vec![149, 95, 181, 242, 94, 90, 158, 162];
        borsh::BorshSerialize::serialize(&reward_index, &mut expected_data).unwrap();

        let expected_accounts = vec![
            AccountMeta::new(lb_pair, false),
            AccountMeta::new(position, false),
            AccountMeta::new(bin_array_lower, false),
            AccountMeta::new(bin_array_upper, false),
            AccountMeta::new_readonly(sender, true),
            AccountMeta::new(reward_vault, false),
            AccountMeta::new_readonly(reward_mint, false),
            AccountMeta::new(user_token_account, false),
            AccountMeta::new_readonly(token_program, false),
            AccountMeta::new_readonly(event_authority, false),
            AccountMeta::new_readonly(program, false),
        ];

        let gen_ix = instructions::claim_reward(
            &program_id,
            &instructions::ClaimRewardAccounts {
                lb_pair,
                position,
                bin_array_lower,
                bin_array_upper,
                sender,
                reward_vault,
                reward_mint,
                user_token_account,
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
    fn test_claim_reward2_instruction() {
        use meteora_dlmm::instructions;
        #[allow(unused_imports)]
        use meteora_dlmm::types;
        use solana_sdk::instruction::AccountMeta;

        let program_id = meteora_dlmm::ID;
        let lb_pair = Pubkey::from([1; 32]);
        let position = Pubkey::from([2; 32]);
        let sender = Pubkey::from([3; 32]);
        let reward_vault = Pubkey::from([4; 32]);
        let reward_mint = Pubkey::from([5; 32]);
        let user_token_account = Pubkey::from([6; 32]);
        let token_program = Pubkey::from([7; 32]);
        let memo_program = Pubkey::from([8; 32]);
        let event_authority = Pubkey::from([9; 32]);
        let program = Pubkey::from([10; 32]);
        let reward_index = 0u64;
        let min_bin_id = 0i32;
        let max_bin_id = 0i32;
        let remaining_accounts_info = types::RemainingAccountsInfo { slices: vec![] };

        let mut expected_data: Vec<u8> = vec![190, 3, 127, 119, 178, 87, 157, 183];
        borsh::BorshSerialize::serialize(&reward_index, &mut expected_data).unwrap();
        borsh::BorshSerialize::serialize(&min_bin_id, &mut expected_data).unwrap();
        borsh::BorshSerialize::serialize(&max_bin_id, &mut expected_data).unwrap();
        borsh::BorshSerialize::serialize(&remaining_accounts_info.clone(), &mut expected_data).unwrap();

        let expected_accounts = vec![
            AccountMeta::new(lb_pair, false),
            AccountMeta::new(position, false),
            AccountMeta::new_readonly(sender, true),
            AccountMeta::new(reward_vault, false),
            AccountMeta::new_readonly(reward_mint, false),
            AccountMeta::new(user_token_account, false),
            AccountMeta::new_readonly(token_program, false),
            AccountMeta::new_readonly(memo_program, false),
            AccountMeta::new_readonly(event_authority, false),
            AccountMeta::new_readonly(program, false),
        ];

        let gen_ix = instructions::claim_reward2(
            &program_id,
            &instructions::ClaimReward2Accounts {
                lb_pair,
                position,
                sender,
                reward_vault,
                reward_mint,
                user_token_account,
                token_program,
                memo_program,
                event_authority,
                program,
            },
            reward_index,
            min_bin_id,
            max_bin_id,
            remaining_accounts_info.clone(),
        );

        let expected_ix = solana_sdk::instruction::Instruction::new_with_bytes(
            program_id,
            &expected_data,
            expected_accounts,
        );
        assert_ix(&gen_ix, &expected_ix);
    }

    #[test]
    fn test_close_claim_fee_operator_account_instruction() {
        use meteora_dlmm::instructions;
        #[allow(unused_imports)]
        use meteora_dlmm::types;
        use solana_sdk::instruction::AccountMeta;

        let program_id = meteora_dlmm::ID;
        let claim_fee_operator = Pubkey::from([1; 32]);
        let rent_receiver = Pubkey::from([2; 32]);
        let signer = Pubkey::from([3; 32]);

        let expected_data: Vec<u8> = vec![184, 213, 88, 31, 179, 101, 130, 36];

        let expected_accounts = vec![
            AccountMeta::new(claim_fee_operator, false),
            AccountMeta::new(rent_receiver, false),
            AccountMeta::new_readonly(signer, true),
        ];

        let gen_ix = instructions::close_claim_fee_operator_account(
            &program_id,
            &instructions::CloseClaimFeeOperatorAccountAccounts {
                claim_fee_operator,
                rent_receiver,
                signer,
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
        use meteora_dlmm::instructions;
        #[allow(unused_imports)]
        use meteora_dlmm::types;
        use solana_sdk::instruction::AccountMeta;

        let program_id = meteora_dlmm::ID;
        let operator = Pubkey::from([1; 32]);
        let signer = Pubkey::from([2; 32]);
        let rent_receiver = Pubkey::from([3; 32]);

        let expected_data: Vec<u8> = vec![171, 9, 213, 74, 120, 23, 3, 29];

        let expected_accounts = vec![
            AccountMeta::new(operator, false),
            AccountMeta::new_readonly(signer, true),
            AccountMeta::new(rent_receiver, false),
        ];

        let gen_ix = instructions::close_operator_account(
            &program_id,
            &instructions::CloseOperatorAccountAccounts {
                operator,
                signer,
                rent_receiver,
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
        use meteora_dlmm::instructions;
        #[allow(unused_imports)]
        use meteora_dlmm::types;
        use solana_sdk::instruction::AccountMeta;

        let program_id = meteora_dlmm::ID;
        let position = Pubkey::from([1; 32]);
        let lb_pair = Pubkey::from([2; 32]);
        let bin_array_lower = Pubkey::from([3; 32]);
        let bin_array_upper = Pubkey::from([4; 32]);
        let sender = Pubkey::from([5; 32]);
        let rent_receiver = Pubkey::from([6; 32]);
        let event_authority = Pubkey::from([7; 32]);
        let program = Pubkey::from([8; 32]);

        let expected_data: Vec<u8> = vec![123, 134, 81, 0, 49, 68, 98, 98];

        let expected_accounts = vec![
            AccountMeta::new(position, false),
            AccountMeta::new(lb_pair, false),
            AccountMeta::new(bin_array_lower, false),
            AccountMeta::new(bin_array_upper, false),
            AccountMeta::new_readonly(sender, true),
            AccountMeta::new(rent_receiver, false),
            AccountMeta::new_readonly(event_authority, false),
            AccountMeta::new_readonly(program, false),
        ];

        let gen_ix = instructions::close_position(
            &program_id,
            &instructions::ClosePositionAccounts {
                position,
                lb_pair,
                bin_array_lower,
                bin_array_upper,
                sender,
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
    fn test_close_position2_instruction() {
        use meteora_dlmm::instructions;
        #[allow(unused_imports)]
        use meteora_dlmm::types;
        use solana_sdk::instruction::AccountMeta;

        let program_id = meteora_dlmm::ID;
        let position = Pubkey::from([1; 32]);
        let sender = Pubkey::from([2; 32]);
        let rent_receiver = Pubkey::from([3; 32]);
        let event_authority = Pubkey::from([4; 32]);
        let program = Pubkey::from([5; 32]);

        let expected_data: Vec<u8> = vec![174, 90, 35, 115, 186, 40, 147, 226];

        let expected_accounts = vec![
            AccountMeta::new(position, false),
            AccountMeta::new_readonly(sender, true),
            AccountMeta::new(rent_receiver, false),
            AccountMeta::new_readonly(event_authority, false),
            AccountMeta::new_readonly(program, false),
        ];

        let gen_ix = instructions::close_position2(
            &program_id,
            &instructions::ClosePosition2Accounts {
                position,
                sender,
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
    fn test_close_position_if_empty_instruction() {
        use meteora_dlmm::instructions;
        #[allow(unused_imports)]
        use meteora_dlmm::types;
        use solana_sdk::instruction::AccountMeta;

        let program_id = meteora_dlmm::ID;
        let position = Pubkey::from([1; 32]);
        let sender = Pubkey::from([2; 32]);
        let rent_receiver = Pubkey::from([3; 32]);
        let event_authority = Pubkey::from([4; 32]);
        let program = Pubkey::from([5; 32]);

        let expected_data: Vec<u8> = vec![59, 124, 212, 118, 91, 152, 110, 157];

        let expected_accounts = vec![
            AccountMeta::new(position, false),
            AccountMeta::new_readonly(sender, true),
            AccountMeta::new(rent_receiver, false),
            AccountMeta::new_readonly(event_authority, false),
            AccountMeta::new_readonly(program, false),
        ];

        let gen_ix = instructions::close_position_if_empty(
            &program_id,
            &instructions::ClosePositionIfEmptyAccounts {
                position,
                sender,
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
    fn test_close_preset_parameter_instruction() {
        use meteora_dlmm::instructions;
        #[allow(unused_imports)]
        use meteora_dlmm::types;
        use solana_sdk::instruction::AccountMeta;

        let program_id = meteora_dlmm::ID;
        let preset_parameter = Pubkey::from([1; 32]);
        let operator = Pubkey::from([2; 32]);
        let signer = Pubkey::from([3; 32]);
        let rent_receiver = Pubkey::from([4; 32]);

        let expected_data: Vec<u8> = vec![4, 148, 145, 100, 134, 26, 181, 61];

        let expected_accounts = vec![
            AccountMeta::new(preset_parameter, false),
            AccountMeta::new_readonly(operator, false),
            AccountMeta::new_readonly(signer, true),
            AccountMeta::new(rent_receiver, false),
        ];

        let gen_ix = instructions::close_preset_parameter(
            &program_id,
            &instructions::ClosePresetParameterAccounts {
                preset_parameter,
                operator,
                signer,
                rent_receiver,
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
    fn test_close_preset_parameter2_instruction() {
        use meteora_dlmm::instructions;
        #[allow(unused_imports)]
        use meteora_dlmm::types;
        use solana_sdk::instruction::AccountMeta;

        let program_id = meteora_dlmm::ID;
        let preset_parameter = Pubkey::from([1; 32]);
        let operator = Pubkey::from([2; 32]);
        let signer = Pubkey::from([3; 32]);
        let rent_receiver = Pubkey::from([4; 32]);

        let expected_data: Vec<u8> = vec![39, 25, 95, 107, 116, 17, 115, 28];

        let expected_accounts = vec![
            AccountMeta::new(preset_parameter, false),
            AccountMeta::new_readonly(operator, false),
            AccountMeta::new_readonly(signer, true),
            AccountMeta::new(rent_receiver, false),
        ];

        let gen_ix = instructions::close_preset_parameter2(
            &program_id,
            &instructions::ClosePresetParameter2Accounts {
                preset_parameter,
                operator,
                signer,
                rent_receiver,
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
        use meteora_dlmm::instructions;
        #[allow(unused_imports)]
        use meteora_dlmm::types;
        use solana_sdk::instruction::AccountMeta;

        let program_id = meteora_dlmm::ID;
        let token_badge = Pubkey::from([1; 32]);
        let rent_receiver = Pubkey::from([2; 32]);
        let operator = Pubkey::from([3; 32]);
        let signer = Pubkey::from([4; 32]);

        let expected_data: Vec<u8> = vec![108, 146, 86, 110, 179, 254, 10, 104];

        let expected_accounts = vec![
            AccountMeta::new(token_badge, false),
            AccountMeta::new(rent_receiver, false),
            AccountMeta::new_readonly(operator, false),
            AccountMeta::new_readonly(signer, true),
        ];

        let gen_ix = instructions::close_token_badge(
            &program_id,
            &instructions::CloseTokenBadgeAccounts {
                token_badge,
                rent_receiver,
                operator,
                signer,
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
    fn test_create_operator_account_instruction() {
        use meteora_dlmm::instructions;
        #[allow(unused_imports)]
        use meteora_dlmm::types;
        use solana_sdk::instruction::AccountMeta;

        let program_id = meteora_dlmm::ID;
        let operator = Pubkey::from([1; 32]);
        let whitelisted_signer = Pubkey::from([2; 32]);
        let signer = Pubkey::from([3; 32]);
        let payer = Pubkey::from([4; 32]);
        let system_program = Pubkey::from([5; 32]);
        let permission = 0u128;

        let mut expected_data: Vec<u8> = vec![221, 64, 246, 149, 240, 153, 229, 163];
        borsh::BorshSerialize::serialize(&permission, &mut expected_data).unwrap();

        let expected_accounts = vec![
            AccountMeta::new(operator, false),
            AccountMeta::new_readonly(whitelisted_signer, false),
            AccountMeta::new_readonly(signer, true),
            AccountMeta::new(payer, true),
            AccountMeta::new_readonly(system_program, false),
        ];

        let gen_ix = instructions::create_operator_account(
            &program_id,
            &instructions::CreateOperatorAccountAccounts {
                operator,
                whitelisted_signer,
                signer,
                payer,
                system_program,
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
    fn test_decrease_position_length_instruction() {
        use meteora_dlmm::instructions;
        #[allow(unused_imports)]
        use meteora_dlmm::types;
        use solana_sdk::instruction::AccountMeta;

        let program_id = meteora_dlmm::ID;
        let rent_receiver = Pubkey::from([1; 32]);
        let position = Pubkey::from([2; 32]);
        let owner = Pubkey::from([3; 32]);
        let system_program = Pubkey::from([4; 32]);
        let event_authority = Pubkey::from([5; 32]);
        let program = Pubkey::from([6; 32]);
        let length_to_remove = 0u16;
        let side = 0u8;

        let mut expected_data: Vec<u8> = vec![194, 219, 136, 32, 25, 96, 105, 37];
        borsh::BorshSerialize::serialize(&length_to_remove, &mut expected_data).unwrap();
        borsh::BorshSerialize::serialize(&side, &mut expected_data).unwrap();

        let expected_accounts = vec![
            AccountMeta::new(rent_receiver, false),
            AccountMeta::new(position, false),
            AccountMeta::new_readonly(owner, true),
            AccountMeta::new_readonly(system_program, false),
            AccountMeta::new_readonly(event_authority, false),
            AccountMeta::new_readonly(program, false),
        ];

        let gen_ix = instructions::decrease_position_length(
            &program_id,
            &instructions::DecreasePositionLengthAccounts {
                rent_receiver,
                position,
                owner,
                system_program,
                event_authority,
                program,
            },
            length_to_remove,
            side,
        );

        let expected_ix = solana_sdk::instruction::Instruction::new_with_bytes(
            program_id,
            &expected_data,
            expected_accounts,
        );
        assert_ix(&gen_ix, &expected_ix);
    }

    #[test]
    fn test_for_idl_type_generation_do_not_call_instruction() {
        use meteora_dlmm::instructions;
        #[allow(unused_imports)]
        use meteora_dlmm::types;
        use solana_sdk::instruction::AccountMeta;

        let program_id = meteora_dlmm::ID;
        let dummy_zc_account = Pubkey::from([1; 32]);
        let _ix = types::DummyIx { _pair_status: types::PairStatus::Enabled, _pair_type: types::PairType::Permissionless, _activation_type: types::ActivationType::Slot, _token_program_flag: types::TokenProgramFlags::TokenProgram, _resize_side: types::ResizeSide::Lower, _rounding: types::Rounding::Up };

        let mut expected_data: Vec<u8> = vec![180, 105, 69, 80, 95, 50, 73, 108];
        borsh::BorshSerialize::serialize(&_ix.clone(), &mut expected_data).unwrap();

        let expected_accounts = vec![
            AccountMeta::new_readonly(dummy_zc_account, false),
        ];

        let gen_ix = instructions::for_idl_type_generation_do_not_call(
            &program_id,
            &instructions::ForIdlTypeGenerationDoNotCallAccounts {
                dummy_zc_account,
            },
            _ix.clone(),
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
        use meteora_dlmm::instructions;
        #[allow(unused_imports)]
        use meteora_dlmm::types;
        use solana_sdk::instruction::AccountMeta;

        let program_id = meteora_dlmm::ID;
        let lb_pair = Pubkey::from([1; 32]);
        let reward_vault = Pubkey::from([2; 32]);
        let reward_mint = Pubkey::from([3; 32]);
        let funder_token_account = Pubkey::from([4; 32]);
        let funder = Pubkey::from([5; 32]);
        let bin_array = Pubkey::from([6; 32]);
        let token_program = Pubkey::from([7; 32]);
        let event_authority = Pubkey::from([8; 32]);
        let program = Pubkey::from([9; 32]);
        let reward_index = 0u64;
        let amount = 0u64;
        let carry_forward = false;
        let remaining_accounts_info = types::RemainingAccountsInfo { slices: vec![] };

        let mut expected_data: Vec<u8> = vec![188, 50, 249, 165, 93, 151, 38, 63];
        borsh::BorshSerialize::serialize(&reward_index, &mut expected_data).unwrap();
        borsh::BorshSerialize::serialize(&amount, &mut expected_data).unwrap();
        borsh::BorshSerialize::serialize(&carry_forward, &mut expected_data).unwrap();
        borsh::BorshSerialize::serialize(&remaining_accounts_info.clone(), &mut expected_data).unwrap();

        let expected_accounts = vec![
            AccountMeta::new(lb_pair, false),
            AccountMeta::new(reward_vault, false),
            AccountMeta::new_readonly(reward_mint, false),
            AccountMeta::new(funder_token_account, false),
            AccountMeta::new_readonly(funder, true),
            AccountMeta::new(bin_array, false),
            AccountMeta::new_readonly(token_program, false),
            AccountMeta::new_readonly(event_authority, false),
            AccountMeta::new_readonly(program, false),
        ];

        let gen_ix = instructions::fund_reward(
            &program_id,
            &instructions::FundRewardAccounts {
                lb_pair,
                reward_vault,
                reward_mint,
                funder_token_account,
                funder,
                bin_array,
                token_program,
                event_authority,
                program,
            },
            reward_index,
            amount,
            carry_forward,
            remaining_accounts_info.clone(),
        );

        let expected_ix = solana_sdk::instruction::Instruction::new_with_bytes(
            program_id,
            &expected_data,
            expected_accounts,
        );
        assert_ix(&gen_ix, &expected_ix);
    }

    #[test]
    fn test_go_to_a_bin_instruction() {
        use meteora_dlmm::instructions;
        #[allow(unused_imports)]
        use meteora_dlmm::types;
        use solana_sdk::instruction::AccountMeta;

        let program_id = meteora_dlmm::ID;
        let lb_pair = Pubkey::from([1; 32]);
        let event_authority = Pubkey::from([2; 32]);
        let program = Pubkey::from([3; 32]);
        let bin_id = 0i32;

        let mut expected_data: Vec<u8> = vec![146, 72, 174, 224, 40, 253, 84, 174];
        borsh::BorshSerialize::serialize(&bin_id, &mut expected_data).unwrap();

        let expected_accounts = vec![
            AccountMeta::new(lb_pair, false),
            AccountMeta::new_readonly(program_id, false),
            AccountMeta::new_readonly(program_id, false),
            AccountMeta::new_readonly(program_id, false),
            AccountMeta::new_readonly(event_authority, false),
            AccountMeta::new_readonly(program, false),
        ];

        let gen_ix = instructions::go_to_a_bin(
            &program_id,
            &instructions::GoToABinAccounts {
                lb_pair,
                bin_array_bitmap_extension: None,
                from_bin_array: None,
                to_bin_array: None,
                event_authority,
                program,
            },
            bin_id,
        );

        let expected_ix = solana_sdk::instruction::Instruction::new_with_bytes(
            program_id,
            &expected_data,
            expected_accounts,
        );
        assert_ix(&gen_ix, &expected_ix);
    }

    #[test]
    fn test_increase_oracle_length_instruction() {
        use meteora_dlmm::instructions;
        #[allow(unused_imports)]
        use meteora_dlmm::types;
        use solana_sdk::instruction::AccountMeta;

        let program_id = meteora_dlmm::ID;
        let oracle = Pubkey::from([1; 32]);
        let funder = Pubkey::from([2; 32]);
        let system_program = Pubkey::from([3; 32]);
        let event_authority = Pubkey::from([4; 32]);
        let program = Pubkey::from([5; 32]);
        let length_to_add = 0u64;

        let mut expected_data: Vec<u8> = vec![190, 61, 125, 87, 103, 79, 158, 173];
        borsh::BorshSerialize::serialize(&length_to_add, &mut expected_data).unwrap();

        let expected_accounts = vec![
            AccountMeta::new(oracle, false),
            AccountMeta::new(funder, true),
            AccountMeta::new_readonly(system_program, false),
            AccountMeta::new_readonly(event_authority, false),
            AccountMeta::new_readonly(program, false),
        ];

        let gen_ix = instructions::increase_oracle_length(
            &program_id,
            &instructions::IncreaseOracleLengthAccounts {
                oracle,
                funder,
                system_program,
                event_authority,
                program,
            },
            length_to_add,
        );

        let expected_ix = solana_sdk::instruction::Instruction::new_with_bytes(
            program_id,
            &expected_data,
            expected_accounts,
        );
        assert_ix(&gen_ix, &expected_ix);
    }

    #[test]
    fn test_increase_position_length_instruction() {
        use meteora_dlmm::instructions;
        #[allow(unused_imports)]
        use meteora_dlmm::types;
        use solana_sdk::instruction::AccountMeta;

        let program_id = meteora_dlmm::ID;
        let funder = Pubkey::from([1; 32]);
        let lb_pair = Pubkey::from([2; 32]);
        let position = Pubkey::from([3; 32]);
        let owner = Pubkey::from([4; 32]);
        let system_program = Pubkey::from([5; 32]);
        let event_authority = Pubkey::from([6; 32]);
        let program = Pubkey::from([7; 32]);
        let length_to_add = 0u16;
        let side = 0u8;

        let mut expected_data: Vec<u8> = vec![80, 83, 117, 211, 66, 13, 33, 149];
        borsh::BorshSerialize::serialize(&length_to_add, &mut expected_data).unwrap();
        borsh::BorshSerialize::serialize(&side, &mut expected_data).unwrap();

        let expected_accounts = vec![
            AccountMeta::new(funder, true),
            AccountMeta::new_readonly(lb_pair, false),
            AccountMeta::new(position, false),
            AccountMeta::new_readonly(owner, true),
            AccountMeta::new_readonly(system_program, false),
            AccountMeta::new_readonly(event_authority, false),
            AccountMeta::new_readonly(program, false),
        ];

        let gen_ix = instructions::increase_position_length(
            &program_id,
            &instructions::IncreasePositionLengthAccounts {
                funder,
                lb_pair,
                position,
                owner,
                system_program,
                event_authority,
                program,
            },
            length_to_add,
            side,
        );

        let expected_ix = solana_sdk::instruction::Instruction::new_with_bytes(
            program_id,
            &expected_data,
            expected_accounts,
        );
        assert_ix(&gen_ix, &expected_ix);
    }

    #[test]
    fn test_increase_position_length2_instruction() {
        use meteora_dlmm::instructions;
        #[allow(unused_imports)]
        use meteora_dlmm::types;
        use solana_sdk::instruction::AccountMeta;

        let program_id = meteora_dlmm::ID;
        let funder = Pubkey::from([1; 32]);
        let lb_pair = Pubkey::from([2; 32]);
        let position = Pubkey::from([3; 32]);
        let owner = Pubkey::from([4; 32]);
        let system_program = Pubkey::from([5; 32]);
        let event_authority = Pubkey::from([6; 32]);
        let program = Pubkey::from([7; 32]);
        let minimum_upper_bin_id = 0i32;

        let mut expected_data: Vec<u8> = vec![255, 210, 204, 71, 115, 137, 225, 113];
        borsh::BorshSerialize::serialize(&minimum_upper_bin_id, &mut expected_data).unwrap();

        let expected_accounts = vec![
            AccountMeta::new(funder, true),
            AccountMeta::new_readonly(lb_pair, false),
            AccountMeta::new(position, false),
            AccountMeta::new_readonly(owner, true),
            AccountMeta::new_readonly(system_program, false),
            AccountMeta::new_readonly(event_authority, false),
            AccountMeta::new_readonly(program, false),
        ];

        let gen_ix = instructions::increase_position_length2(
            &program_id,
            &instructions::IncreasePositionLength2Accounts {
                funder,
                lb_pair,
                position,
                owner,
                system_program,
                event_authority,
                program,
            },
            minimum_upper_bin_id,
        );

        let expected_ix = solana_sdk::instruction::Instruction::new_with_bytes(
            program_id,
            &expected_data,
            expected_accounts,
        );
        assert_ix(&gen_ix, &expected_ix);
    }

    #[test]
    fn test_initialize_bin_array_instruction() {
        use meteora_dlmm::instructions;
        #[allow(unused_imports)]
        use meteora_dlmm::types;
        use solana_sdk::instruction::AccountMeta;

        let program_id = meteora_dlmm::ID;
        let lb_pair = Pubkey::from([1; 32]);
        let bin_array = Pubkey::from([2; 32]);
        let funder = Pubkey::from([3; 32]);
        let system_program = Pubkey::from([4; 32]);
        let index = 0i64;

        let mut expected_data: Vec<u8> = vec![35, 86, 19, 185, 78, 212, 75, 211];
        borsh::BorshSerialize::serialize(&index, &mut expected_data).unwrap();

        let expected_accounts = vec![
            AccountMeta::new_readonly(lb_pair, false),
            AccountMeta::new(bin_array, false),
            AccountMeta::new(funder, true),
            AccountMeta::new_readonly(system_program, false),
        ];

        let gen_ix = instructions::initialize_bin_array(
            &program_id,
            &instructions::InitializeBinArrayAccounts {
                lb_pair,
                bin_array,
                funder,
                system_program,
            },
            index,
        );

        let expected_ix = solana_sdk::instruction::Instruction::new_with_bytes(
            program_id,
            &expected_data,
            expected_accounts,
        );
        assert_ix(&gen_ix, &expected_ix);
    }

    #[test]
    fn test_initialize_bin_array_bitmap_extension_instruction() {
        use meteora_dlmm::instructions;
        #[allow(unused_imports)]
        use meteora_dlmm::types;
        use solana_sdk::instruction::AccountMeta;

        let program_id = meteora_dlmm::ID;
        let lb_pair = Pubkey::from([1; 32]);
        let bin_array_bitmap_extension = Pubkey::from([2; 32]);
        let funder = Pubkey::from([3; 32]);
        let system_program = Pubkey::from([4; 32]);
        let rent = Pubkey::from([5; 32]);

        let expected_data: Vec<u8> = vec![47, 157, 226, 180, 12, 240, 33, 71];

        let expected_accounts = vec![
            AccountMeta::new_readonly(lb_pair, false),
            AccountMeta::new(bin_array_bitmap_extension, false),
            AccountMeta::new(funder, true),
            AccountMeta::new_readonly(system_program, false),
            AccountMeta::new_readonly(rent, false),
        ];

        let gen_ix = instructions::initialize_bin_array_bitmap_extension(
            &program_id,
            &instructions::InitializeBinArrayBitmapExtensionAccounts {
                lb_pair,
                bin_array_bitmap_extension,
                funder,
                system_program,
                rent,
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
    fn test_initialize_customizable_permissionless_lb_pair_instruction() {
        use meteora_dlmm::instructions;
        #[allow(unused_imports)]
        use meteora_dlmm::types;
        use solana_sdk::instruction::AccountMeta;

        let program_id = meteora_dlmm::ID;
        let lb_pair = Pubkey::from([1; 32]);
        let token_mint_x = Pubkey::from([2; 32]);
        let token_mint_y = Pubkey::from([3; 32]);
        let reserve_x = Pubkey::from([4; 32]);
        let reserve_y = Pubkey::from([5; 32]);
        let oracle = Pubkey::from([6; 32]);
        let user_token_x = Pubkey::from([7; 32]);
        let funder = Pubkey::from([8; 32]);
        let token_program = Pubkey::from([9; 32]);
        let system_program = Pubkey::from([10; 32]);
        let user_token_y = Pubkey::from([11; 32]);
        let event_authority = Pubkey::from([12; 32]);
        let program = Pubkey::from([13; 32]);
        let params = types::CustomizableParams { active_id: 0i32, bin_step: 0u16, base_factor: 0u16, activation_type: 0u8, has_alpha_vault: false, activation_point: None, creator_pool_on_off_control: false, base_fee_power_factor: 0u8, function_type: 0u8, padding: [0u8; 61] };

        let mut expected_data: Vec<u8> = vec![46, 39, 41, 135, 111, 183, 200, 64];
        borsh::BorshSerialize::serialize(&params.clone(), &mut expected_data).unwrap();

        let expected_accounts = vec![
            AccountMeta::new(lb_pair, false),
            AccountMeta::new_readonly(program_id, false),
            AccountMeta::new_readonly(token_mint_x, false),
            AccountMeta::new_readonly(token_mint_y, false),
            AccountMeta::new(reserve_x, false),
            AccountMeta::new(reserve_y, false),
            AccountMeta::new(oracle, false),
            AccountMeta::new_readonly(user_token_x, false),
            AccountMeta::new(funder, true),
            AccountMeta::new_readonly(token_program, false),
            AccountMeta::new_readonly(system_program, false),
            AccountMeta::new_readonly(user_token_y, false),
            AccountMeta::new_readonly(event_authority, false),
            AccountMeta::new_readonly(program, false),
        ];

        let gen_ix = instructions::initialize_customizable_permissionless_lb_pair(
            &program_id,
            &instructions::InitializeCustomizablePermissionlessLbPairAccounts {
                lb_pair,
                bin_array_bitmap_extension: None,
                token_mint_x,
                token_mint_y,
                reserve_x,
                reserve_y,
                oracle,
                user_token_x,
                funder,
                token_program,
                system_program,
                user_token_y,
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
    fn test_initialize_customizable_permissionless_lb_pair2_instruction() {
        use meteora_dlmm::instructions;
        #[allow(unused_imports)]
        use meteora_dlmm::types;
        use solana_sdk::instruction::AccountMeta;

        let program_id = meteora_dlmm::ID;
        let lb_pair = Pubkey::from([1; 32]);
        let token_mint_x = Pubkey::from([2; 32]);
        let token_mint_y = Pubkey::from([3; 32]);
        let reserve_x = Pubkey::from([4; 32]);
        let reserve_y = Pubkey::from([5; 32]);
        let oracle = Pubkey::from([6; 32]);
        let user_token_x = Pubkey::from([7; 32]);
        let funder = Pubkey::from([8; 32]);
        let token_program_x = Pubkey::from([9; 32]);
        let token_program_y = Pubkey::from([10; 32]);
        let system_program = Pubkey::from([11; 32]);
        let user_token_y = Pubkey::from([12; 32]);
        let event_authority = Pubkey::from([13; 32]);
        let program = Pubkey::from([14; 32]);
        let params = types::CustomizableParams { active_id: 0i32, bin_step: 0u16, base_factor: 0u16, activation_type: 0u8, has_alpha_vault: false, activation_point: None, creator_pool_on_off_control: false, base_fee_power_factor: 0u8, function_type: 0u8, padding: [0u8; 61] };

        let mut expected_data: Vec<u8> = vec![243, 73, 129, 126, 51, 19, 241, 107];
        borsh::BorshSerialize::serialize(&params.clone(), &mut expected_data).unwrap();

        let expected_accounts = vec![
            AccountMeta::new(lb_pair, false),
            AccountMeta::new_readonly(program_id, false),
            AccountMeta::new_readonly(token_mint_x, false),
            AccountMeta::new_readonly(token_mint_y, false),
            AccountMeta::new(reserve_x, false),
            AccountMeta::new(reserve_y, false),
            AccountMeta::new(oracle, false),
            AccountMeta::new_readonly(user_token_x, false),
            AccountMeta::new(funder, true),
            AccountMeta::new_readonly(program_id, false),
            AccountMeta::new_readonly(program_id, false),
            AccountMeta::new_readonly(token_program_x, false),
            AccountMeta::new_readonly(token_program_y, false),
            AccountMeta::new_readonly(system_program, false),
            AccountMeta::new_readonly(user_token_y, false),
            AccountMeta::new_readonly(event_authority, false),
            AccountMeta::new_readonly(program, false),
        ];

        let gen_ix = instructions::initialize_customizable_permissionless_lb_pair2(
            &program_id,
            &instructions::InitializeCustomizablePermissionlessLbPair2Accounts {
                lb_pair,
                bin_array_bitmap_extension: None,
                token_mint_x,
                token_mint_y,
                reserve_x,
                reserve_y,
                oracle,
                user_token_x,
                funder,
                token_badge_x: None,
                token_badge_y: None,
                token_program_x,
                token_program_y,
                system_program,
                user_token_y,
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
    fn test_initialize_lb_pair_instruction() {
        use meteora_dlmm::instructions;
        #[allow(unused_imports)]
        use meteora_dlmm::types;
        use solana_sdk::instruction::AccountMeta;

        let program_id = meteora_dlmm::ID;
        let lb_pair = Pubkey::from([1; 32]);
        let token_mint_x = Pubkey::from([2; 32]);
        let token_mint_y = Pubkey::from([3; 32]);
        let reserve_x = Pubkey::from([4; 32]);
        let reserve_y = Pubkey::from([5; 32]);
        let oracle = Pubkey::from([6; 32]);
        let preset_parameter = Pubkey::from([7; 32]);
        let funder = Pubkey::from([8; 32]);
        let token_program = Pubkey::from([9; 32]);
        let system_program = Pubkey::from([10; 32]);
        let rent = Pubkey::from([11; 32]);
        let event_authority = Pubkey::from([12; 32]);
        let program = Pubkey::from([13; 32]);
        let active_id = 0i32;
        let bin_step = 0u16;

        let mut expected_data: Vec<u8> = vec![45, 154, 237, 210, 221, 15, 166, 92];
        borsh::BorshSerialize::serialize(&active_id, &mut expected_data).unwrap();
        borsh::BorshSerialize::serialize(&bin_step, &mut expected_data).unwrap();

        let expected_accounts = vec![
            AccountMeta::new(lb_pair, false),
            AccountMeta::new_readonly(program_id, false),
            AccountMeta::new_readonly(token_mint_x, false),
            AccountMeta::new_readonly(token_mint_y, false),
            AccountMeta::new(reserve_x, false),
            AccountMeta::new(reserve_y, false),
            AccountMeta::new(oracle, false),
            AccountMeta::new_readonly(preset_parameter, false),
            AccountMeta::new(funder, true),
            AccountMeta::new_readonly(token_program, false),
            AccountMeta::new_readonly(system_program, false),
            AccountMeta::new_readonly(rent, false),
            AccountMeta::new_readonly(event_authority, false),
            AccountMeta::new_readonly(program, false),
        ];

        let gen_ix = instructions::initialize_lb_pair(
            &program_id,
            &instructions::InitializeLbPairAccounts {
                lb_pair,
                bin_array_bitmap_extension: None,
                token_mint_x,
                token_mint_y,
                reserve_x,
                reserve_y,
                oracle,
                preset_parameter,
                funder,
                token_program,
                system_program,
                rent,
                event_authority,
                program,
            },
            active_id,
            bin_step,
        );

        let expected_ix = solana_sdk::instruction::Instruction::new_with_bytes(
            program_id,
            &expected_data,
            expected_accounts,
        );
        assert_ix(&gen_ix, &expected_ix);
    }

    #[test]
    fn test_initialize_lb_pair2_instruction() {
        use meteora_dlmm::instructions;
        #[allow(unused_imports)]
        use meteora_dlmm::types;
        use solana_sdk::instruction::AccountMeta;

        let program_id = meteora_dlmm::ID;
        let lb_pair = Pubkey::from([1; 32]);
        let token_mint_x = Pubkey::from([2; 32]);
        let token_mint_y = Pubkey::from([3; 32]);
        let reserve_x = Pubkey::from([4; 32]);
        let reserve_y = Pubkey::from([5; 32]);
        let oracle = Pubkey::from([6; 32]);
        let preset_parameter = Pubkey::from([7; 32]);
        let funder = Pubkey::from([8; 32]);
        let token_program_x = Pubkey::from([9; 32]);
        let token_program_y = Pubkey::from([10; 32]);
        let system_program = Pubkey::from([11; 32]);
        let event_authority = Pubkey::from([12; 32]);
        let program = Pubkey::from([13; 32]);
        let params = types::InitializeLbPair2Params { active_id: 0i32, padding: [0u8; 96] };

        let mut expected_data: Vec<u8> = vec![73, 59, 36, 120, 237, 83, 108, 198];
        borsh::BorshSerialize::serialize(&params.clone(), &mut expected_data).unwrap();

        let expected_accounts = vec![
            AccountMeta::new(lb_pair, false),
            AccountMeta::new_readonly(program_id, false),
            AccountMeta::new_readonly(token_mint_x, false),
            AccountMeta::new_readonly(token_mint_y, false),
            AccountMeta::new(reserve_x, false),
            AccountMeta::new(reserve_y, false),
            AccountMeta::new(oracle, false),
            AccountMeta::new_readonly(preset_parameter, false),
            AccountMeta::new(funder, true),
            AccountMeta::new_readonly(program_id, false),
            AccountMeta::new_readonly(program_id, false),
            AccountMeta::new_readonly(token_program_x, false),
            AccountMeta::new_readonly(token_program_y, false),
            AccountMeta::new_readonly(system_program, false),
            AccountMeta::new_readonly(event_authority, false),
            AccountMeta::new_readonly(program, false),
        ];

        let gen_ix = instructions::initialize_lb_pair2(
            &program_id,
            &instructions::InitializeLbPair2Accounts {
                lb_pair,
                bin_array_bitmap_extension: None,
                token_mint_x,
                token_mint_y,
                reserve_x,
                reserve_y,
                oracle,
                preset_parameter,
                funder,
                token_badge_x: None,
                token_badge_y: None,
                token_program_x,
                token_program_y,
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
    fn test_initialize_permission_lb_pair_instruction() {
        use meteora_dlmm::instructions;
        #[allow(unused_imports)]
        use meteora_dlmm::types;
        use solana_sdk::instruction::AccountMeta;

        let program_id = meteora_dlmm::ID;
        let base = Pubkey::from([1; 32]);
        let lb_pair = Pubkey::from([2; 32]);
        let token_mint_x = Pubkey::from([3; 32]);
        let token_mint_y = Pubkey::from([4; 32]);
        let reserve_x = Pubkey::from([5; 32]);
        let reserve_y = Pubkey::from([6; 32]);
        let oracle = Pubkey::from([7; 32]);
        let payer = Pubkey::from([8; 32]);
        let operator = Pubkey::from([9; 32]);
        let signer = Pubkey::from([10; 32]);
        let token_program_x = Pubkey::from([11; 32]);
        let token_program_y = Pubkey::from([12; 32]);
        let system_program = Pubkey::from([13; 32]);
        let event_authority = Pubkey::from([14; 32]);
        let program = Pubkey::from([15; 32]);
        let ix_data = types::InitPermissionPairIx { active_id: 0i32, bin_step: 0u16, base_factor: 0u16, base_fee_power_factor: 0u8, activation_type: 0u8, protocol_share: 0u16 };

        let mut expected_data: Vec<u8> = vec![108, 102, 213, 85, 251, 3, 53, 21];
        borsh::BorshSerialize::serialize(&ix_data.clone(), &mut expected_data).unwrap();

        let expected_accounts = vec![
            AccountMeta::new_readonly(base, true),
            AccountMeta::new(lb_pair, false),
            AccountMeta::new_readonly(program_id, false),
            AccountMeta::new_readonly(token_mint_x, false),
            AccountMeta::new_readonly(token_mint_y, false),
            AccountMeta::new(reserve_x, false),
            AccountMeta::new(reserve_y, false),
            AccountMeta::new(oracle, false),
            AccountMeta::new(payer, true),
            AccountMeta::new_readonly(operator, false),
            AccountMeta::new_readonly(signer, true),
            AccountMeta::new_readonly(program_id, false),
            AccountMeta::new_readonly(program_id, false),
            AccountMeta::new_readonly(token_program_x, false),
            AccountMeta::new_readonly(token_program_y, false),
            AccountMeta::new_readonly(system_program, false),
            AccountMeta::new_readonly(event_authority, false),
            AccountMeta::new_readonly(program, false),
        ];

        let gen_ix = instructions::initialize_permission_lb_pair(
            &program_id,
            &instructions::InitializePermissionLbPairAccounts {
                base,
                lb_pair,
                bin_array_bitmap_extension: None,
                token_mint_x,
                token_mint_y,
                reserve_x,
                reserve_y,
                oracle,
                payer,
                operator,
                signer,
                token_badge_x: None,
                token_badge_y: None,
                token_program_x,
                token_program_y,
                system_program,
                event_authority,
                program,
            },
            ix_data.clone(),
        );

        let expected_ix = solana_sdk::instruction::Instruction::new_with_bytes(
            program_id,
            &expected_data,
            expected_accounts,
        );
        assert_ix(&gen_ix, &expected_ix);
    }

    #[test]
    fn test_initialize_position_instruction() {
        use meteora_dlmm::instructions;
        #[allow(unused_imports)]
        use meteora_dlmm::types;
        use solana_sdk::instruction::AccountMeta;

        let program_id = meteora_dlmm::ID;
        let payer = Pubkey::from([1; 32]);
        let position = Pubkey::from([2; 32]);
        let lb_pair = Pubkey::from([3; 32]);
        let owner = Pubkey::from([4; 32]);
        let system_program = Pubkey::from([5; 32]);
        let rent = Pubkey::from([6; 32]);
        let event_authority = Pubkey::from([7; 32]);
        let program = Pubkey::from([8; 32]);
        let lower_bin_id = 0i32;
        let width = 0i32;

        let mut expected_data: Vec<u8> = vec![219, 192, 234, 71, 190, 191, 102, 80];
        borsh::BorshSerialize::serialize(&lower_bin_id, &mut expected_data).unwrap();
        borsh::BorshSerialize::serialize(&width, &mut expected_data).unwrap();

        let expected_accounts = vec![
            AccountMeta::new(payer, true),
            AccountMeta::new(position, true),
            AccountMeta::new_readonly(lb_pair, false),
            AccountMeta::new_readonly(owner, true),
            AccountMeta::new_readonly(system_program, false),
            AccountMeta::new_readonly(rent, false),
            AccountMeta::new_readonly(event_authority, false),
            AccountMeta::new_readonly(program, false),
        ];

        let gen_ix = instructions::initialize_position(
            &program_id,
            &instructions::InitializePositionAccounts {
                payer,
                position,
                lb_pair,
                owner,
                system_program,
                rent,
                event_authority,
                program,
            },
            lower_bin_id,
            width,
        );

        let expected_ix = solana_sdk::instruction::Instruction::new_with_bytes(
            program_id,
            &expected_data,
            expected_accounts,
        );
        assert_ix(&gen_ix, &expected_ix);
    }

    #[test]
    fn test_initialize_position2_instruction() {
        use meteora_dlmm::instructions;
        #[allow(unused_imports)]
        use meteora_dlmm::types;
        use solana_sdk::instruction::AccountMeta;

        let program_id = meteora_dlmm::ID;
        let payer = Pubkey::from([1; 32]);
        let position = Pubkey::from([2; 32]);
        let lb_pair = Pubkey::from([3; 32]);
        let owner = Pubkey::from([4; 32]);
        let system_program = Pubkey::from([5; 32]);
        let event_authority = Pubkey::from([6; 32]);
        let program = Pubkey::from([7; 32]);
        let lower_bin_id = 0i32;
        let width = 0i32;

        let mut expected_data: Vec<u8> = vec![143, 19, 242, 145, 213, 15, 104, 115];
        borsh::BorshSerialize::serialize(&lower_bin_id, &mut expected_data).unwrap();
        borsh::BorshSerialize::serialize(&width, &mut expected_data).unwrap();

        let expected_accounts = vec![
            AccountMeta::new(payer, true),
            AccountMeta::new(position, true),
            AccountMeta::new_readonly(lb_pair, false),
            AccountMeta::new_readonly(owner, true),
            AccountMeta::new_readonly(system_program, false),
            AccountMeta::new_readonly(event_authority, false),
            AccountMeta::new_readonly(program, false),
        ];

        let gen_ix = instructions::initialize_position2(
            &program_id,
            &instructions::InitializePosition2Accounts {
                payer,
                position,
                lb_pair,
                owner,
                system_program,
                event_authority,
                program,
            },
            lower_bin_id,
            width,
        );

        let expected_ix = solana_sdk::instruction::Instruction::new_with_bytes(
            program_id,
            &expected_data,
            expected_accounts,
        );
        assert_ix(&gen_ix, &expected_ix);
    }

    #[test]
    fn test_initialize_position_by_operator_instruction() {
        use meteora_dlmm::instructions;
        #[allow(unused_imports)]
        use meteora_dlmm::types;
        use solana_sdk::instruction::AccountMeta;

        let program_id = meteora_dlmm::ID;
        let payer = Pubkey::from([1; 32]);
        let base = Pubkey::from([2; 32]);
        let position = Pubkey::from([3; 32]);
        let lb_pair = Pubkey::from([4; 32]);
        let owner = Pubkey::from([5; 32]);
        let operator = Pubkey::from([6; 32]);
        let operator_token_x = Pubkey::from([7; 32]);
        let owner_token_x = Pubkey::from([8; 32]);
        let system_program = Pubkey::from([9; 32]);
        let event_authority = Pubkey::from([10; 32]);
        let program = Pubkey::from([11; 32]);
        let lower_bin_id = 0i32;
        let width = 0i32;
        let fee_owner = Pubkey::from([0; 32]);
        let lock_release_point = 0u64;

        let mut expected_data: Vec<u8> = vec![251, 189, 190, 244, 117, 254, 35, 148];
        borsh::BorshSerialize::serialize(&lower_bin_id, &mut expected_data).unwrap();
        borsh::BorshSerialize::serialize(&width, &mut expected_data).unwrap();
        borsh::BorshSerialize::serialize(&fee_owner, &mut expected_data).unwrap();
        borsh::BorshSerialize::serialize(&lock_release_point, &mut expected_data).unwrap();

        let expected_accounts = vec![
            AccountMeta::new(payer, true),
            AccountMeta::new_readonly(base, true),
            AccountMeta::new(position, false),
            AccountMeta::new_readonly(lb_pair, false),
            AccountMeta::new_readonly(owner, false),
            AccountMeta::new_readonly(operator, true),
            AccountMeta::new_readonly(operator_token_x, false),
            AccountMeta::new_readonly(owner_token_x, false),
            AccountMeta::new_readonly(system_program, false),
            AccountMeta::new_readonly(event_authority, false),
            AccountMeta::new_readonly(program, false),
        ];

        let gen_ix = instructions::initialize_position_by_operator(
            &program_id,
            &instructions::InitializePositionByOperatorAccounts {
                payer,
                base,
                position,
                lb_pair,
                owner,
                operator,
                operator_token_x,
                owner_token_x,
                system_program,
                event_authority,
                program,
            },
            lower_bin_id,
            width,
            fee_owner,
            lock_release_point,
        );

        let expected_ix = solana_sdk::instruction::Instruction::new_with_bytes(
            program_id,
            &expected_data,
            expected_accounts,
        );
        assert_ix(&gen_ix, &expected_ix);
    }

    #[test]
    fn test_initialize_position_pda_instruction() {
        use meteora_dlmm::instructions;
        #[allow(unused_imports)]
        use meteora_dlmm::types;
        use solana_sdk::instruction::AccountMeta;

        let program_id = meteora_dlmm::ID;
        let payer = Pubkey::from([1; 32]);
        let base = Pubkey::from([2; 32]);
        let position = Pubkey::from([3; 32]);
        let lb_pair = Pubkey::from([4; 32]);
        let owner = Pubkey::from([5; 32]);
        let system_program = Pubkey::from([6; 32]);
        let rent = Pubkey::from([7; 32]);
        let event_authority = Pubkey::from([8; 32]);
        let program = Pubkey::from([9; 32]);
        let lower_bin_id = 0i32;
        let width = 0i32;

        let mut expected_data: Vec<u8> = vec![46, 82, 125, 146, 85, 141, 228, 153];
        borsh::BorshSerialize::serialize(&lower_bin_id, &mut expected_data).unwrap();
        borsh::BorshSerialize::serialize(&width, &mut expected_data).unwrap();

        let expected_accounts = vec![
            AccountMeta::new(payer, true),
            AccountMeta::new_readonly(base, true),
            AccountMeta::new(position, false),
            AccountMeta::new_readonly(lb_pair, false),
            AccountMeta::new_readonly(owner, true),
            AccountMeta::new_readonly(system_program, false),
            AccountMeta::new_readonly(rent, false),
            AccountMeta::new_readonly(event_authority, false),
            AccountMeta::new_readonly(program, false),
        ];

        let gen_ix = instructions::initialize_position_pda(
            &program_id,
            &instructions::InitializePositionPdaAccounts {
                payer,
                base,
                position,
                lb_pair,
                owner,
                system_program,
                rent,
                event_authority,
                program,
            },
            lower_bin_id,
            width,
        );

        let expected_ix = solana_sdk::instruction::Instruction::new_with_bytes(
            program_id,
            &expected_data,
            expected_accounts,
        );
        assert_ix(&gen_ix, &expected_ix);
    }

    #[test]
    fn test_initialize_preset_parameter_instruction() {
        use meteora_dlmm::instructions;
        #[allow(unused_imports)]
        use meteora_dlmm::types;
        use solana_sdk::instruction::AccountMeta;

        let program_id = meteora_dlmm::ID;
        let preset_parameter = Pubkey::from([1; 32]);
        let operator = Pubkey::from([2; 32]);
        let signer = Pubkey::from([3; 32]);
        let payer = Pubkey::from([4; 32]);
        let system_program = Pubkey::from([5; 32]);
        let ix = types::InitPresetParametersIx { index: 0u16, bin_step: 0u16, base_factor: 0u16, filter_period: 0u16, decay_period: 0u16, reduction_factor: 0u16, variable_fee_control: 0u32, max_volatility_accumulator: 0u32, protocol_share: 0u16, base_fee_power_factor: 0u8, function_type: 0u8 };

        let mut expected_data: Vec<u8> = vec![66, 188, 71, 211, 98, 109, 14, 186];
        borsh::BorshSerialize::serialize(&ix.clone(), &mut expected_data).unwrap();

        let expected_accounts = vec![
            AccountMeta::new(preset_parameter, false),
            AccountMeta::new_readonly(operator, false),
            AccountMeta::new_readonly(signer, true),
            AccountMeta::new(payer, true),
            AccountMeta::new_readonly(system_program, false),
        ];

        let gen_ix = instructions::initialize_preset_parameter(
            &program_id,
            &instructions::InitializePresetParameterAccounts {
                preset_parameter,
                operator,
                signer,
                payer,
                system_program,
            },
            ix.clone(),
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
        use meteora_dlmm::instructions;
        #[allow(unused_imports)]
        use meteora_dlmm::types;
        use solana_sdk::instruction::AccountMeta;

        let program_id = meteora_dlmm::ID;
        let lb_pair = Pubkey::from([1; 32]);
        let reward_vault = Pubkey::from([2; 32]);
        let reward_mint = Pubkey::from([3; 32]);
        let operator = Pubkey::from([4; 32]);
        let signer = Pubkey::from([5; 32]);
        let payer = Pubkey::from([6; 32]);
        let token_program = Pubkey::from([7; 32]);
        let system_program = Pubkey::from([8; 32]);
        let event_authority = Pubkey::from([9; 32]);
        let program = Pubkey::from([10; 32]);
        let reward_index = 0u64;
        let reward_duration = 0u64;
        let funder = Pubkey::from([0; 32]);

        let mut expected_data: Vec<u8> = vec![95, 135, 192, 196, 242, 129, 230, 68];
        borsh::BorshSerialize::serialize(&reward_index, &mut expected_data).unwrap();
        borsh::BorshSerialize::serialize(&reward_duration, &mut expected_data).unwrap();
        borsh::BorshSerialize::serialize(&funder, &mut expected_data).unwrap();

        let expected_accounts = vec![
            AccountMeta::new(lb_pair, false),
            AccountMeta::new(reward_vault, false),
            AccountMeta::new_readonly(reward_mint, false),
            AccountMeta::new_readonly(program_id, false),
            AccountMeta::new_readonly(operator, false),
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
                lb_pair,
                reward_vault,
                reward_mint,
                token_badge: None,
                operator,
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
    fn test_initialize_token_badge_instruction() {
        use meteora_dlmm::instructions;
        #[allow(unused_imports)]
        use meteora_dlmm::types;
        use solana_sdk::instruction::AccountMeta;

        let program_id = meteora_dlmm::ID;
        let token_mint = Pubkey::from([1; 32]);
        let token_badge = Pubkey::from([2; 32]);
        let operator = Pubkey::from([3; 32]);
        let signer = Pubkey::from([4; 32]);
        let payer = Pubkey::from([5; 32]);
        let system_program = Pubkey::from([6; 32]);

        let expected_data: Vec<u8> = vec![253, 77, 205, 95, 27, 224, 89, 223];

        let expected_accounts = vec![
            AccountMeta::new_readonly(token_mint, false),
            AccountMeta::new(token_badge, false),
            AccountMeta::new_readonly(operator, false),
            AccountMeta::new_readonly(signer, true),
            AccountMeta::new(payer, true),
            AccountMeta::new_readonly(system_program, false),
        ];

        let gen_ix = instructions::initialize_token_badge(
            &program_id,
            &instructions::InitializeTokenBadgeAccounts {
                token_mint,
                token_badge,
                operator,
                signer,
                payer,
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
    fn test_migrate_position_instruction() {
        use meteora_dlmm::instructions;
        #[allow(unused_imports)]
        use meteora_dlmm::types;
        use solana_sdk::instruction::AccountMeta;

        let program_id = meteora_dlmm::ID;
        let position_v2 = Pubkey::from([1; 32]);
        let position_v1 = Pubkey::from([2; 32]);
        let lb_pair = Pubkey::from([3; 32]);
        let bin_array_lower = Pubkey::from([4; 32]);
        let bin_array_upper = Pubkey::from([5; 32]);
        let signer_and_payer = Pubkey::from([6; 32]);
        let system_program = Pubkey::from([7; 32]);
        let rent_receiver = Pubkey::from([8; 32]);
        let event_authority = Pubkey::from([9; 32]);
        let program = Pubkey::from([10; 32]);

        let expected_data: Vec<u8> = vec![15, 132, 59, 50, 199, 6, 251, 46];

        let expected_accounts = vec![
            AccountMeta::new(position_v2, true),
            AccountMeta::new(position_v1, false),
            AccountMeta::new_readonly(lb_pair, false),
            AccountMeta::new(bin_array_lower, false),
            AccountMeta::new(bin_array_upper, false),
            AccountMeta::new(signer_and_payer, true),
            AccountMeta::new_readonly(system_program, false),
            AccountMeta::new(rent_receiver, false),
            AccountMeta::new_readonly(event_authority, false),
            AccountMeta::new_readonly(program, false),
        ];

        let gen_ix = instructions::migrate_position(
            &program_id,
            &instructions::MigratePositionAccounts {
                position_v2,
                position_v1,
                lb_pair,
                bin_array_lower,
                bin_array_upper,
                signer_and_payer,
                system_program,
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
    fn test_rebalance_liquidity_instruction() {
        use meteora_dlmm::instructions;
        #[allow(unused_imports)]
        use meteora_dlmm::types;
        use solana_sdk::instruction::AccountMeta;

        let program_id = meteora_dlmm::ID;
        let position = Pubkey::from([1; 32]);
        let lb_pair = Pubkey::from([2; 32]);
        let user_token_x = Pubkey::from([3; 32]);
        let user_token_y = Pubkey::from([4; 32]);
        let reserve_x = Pubkey::from([5; 32]);
        let reserve_y = Pubkey::from([6; 32]);
        let token_x_mint = Pubkey::from([7; 32]);
        let token_y_mint = Pubkey::from([8; 32]);
        let owner = Pubkey::from([9; 32]);
        let rent_payer = Pubkey::from([10; 32]);
        let token_x_program = Pubkey::from([11; 32]);
        let token_y_program = Pubkey::from([12; 32]);
        let memo_program = Pubkey::from([13; 32]);
        let system_program = Pubkey::from([14; 32]);
        let event_authority = Pubkey::from([15; 32]);
        let program = Pubkey::from([16; 32]);
        let params = types::RebalanceLiquidityParams { active_id: 0i32, max_active_bin_slippage: 0u16, should_claim_fee: false, should_claim_reward: false, min_withdraw_x_amount: 0u64, max_deposit_x_amount: 0u64, min_withdraw_y_amount: 0u64, max_deposit_y_amount: 0u64, shrink_mode: 0u8, padding: [0u8; 31], removes: vec![], adds: vec![] };
        let remaining_accounts_info = types::RemainingAccountsInfo { slices: vec![] };

        let mut expected_data: Vec<u8> = vec![92, 4, 176, 193, 119, 185, 83, 9];
        borsh::BorshSerialize::serialize(&params.clone(), &mut expected_data).unwrap();
        borsh::BorshSerialize::serialize(&remaining_accounts_info.clone(), &mut expected_data).unwrap();

        let expected_accounts = vec![
            AccountMeta::new(position, false),
            AccountMeta::new(lb_pair, false),
            AccountMeta::new_readonly(program_id, false),
            AccountMeta::new(user_token_x, false),
            AccountMeta::new(user_token_y, false),
            AccountMeta::new(reserve_x, false),
            AccountMeta::new(reserve_y, false),
            AccountMeta::new_readonly(token_x_mint, false),
            AccountMeta::new_readonly(token_y_mint, false),
            AccountMeta::new_readonly(owner, true),
            AccountMeta::new(rent_payer, true),
            AccountMeta::new_readonly(token_x_program, false),
            AccountMeta::new_readonly(token_y_program, false),
            AccountMeta::new_readonly(memo_program, false),
            AccountMeta::new_readonly(system_program, false),
            AccountMeta::new_readonly(event_authority, false),
            AccountMeta::new_readonly(program, false),
        ];

        let gen_ix = instructions::rebalance_liquidity(
            &program_id,
            &instructions::RebalanceLiquidityAccounts {
                position,
                lb_pair,
                bin_array_bitmap_extension: None,
                user_token_x,
                user_token_y,
                reserve_x,
                reserve_y,
                token_x_mint,
                token_y_mint,
                owner,
                rent_payer,
                token_x_program,
                token_y_program,
                memo_program,
                system_program,
                event_authority,
                program,
            },
            params.clone(),
            remaining_accounts_info.clone(),
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
        use meteora_dlmm::instructions;
        #[allow(unused_imports)]
        use meteora_dlmm::types;
        use solana_sdk::instruction::AccountMeta;

        let program_id = meteora_dlmm::ID;
        let position = Pubkey::from([1; 32]);
        let lb_pair = Pubkey::from([2; 32]);
        let user_token_x = Pubkey::from([3; 32]);
        let user_token_y = Pubkey::from([4; 32]);
        let reserve_x = Pubkey::from([5; 32]);
        let reserve_y = Pubkey::from([6; 32]);
        let token_x_mint = Pubkey::from([7; 32]);
        let token_y_mint = Pubkey::from([8; 32]);
        let bin_array_lower = Pubkey::from([9; 32]);
        let bin_array_upper = Pubkey::from([10; 32]);
        let sender = Pubkey::from([11; 32]);
        let token_x_program = Pubkey::from([12; 32]);
        let token_y_program = Pubkey::from([13; 32]);
        let event_authority = Pubkey::from([14; 32]);
        let program = Pubkey::from([15; 32]);

        let expected_data: Vec<u8> = vec![10, 51, 61, 35, 112, 105, 24, 85];

        let expected_accounts = vec![
            AccountMeta::new(position, false),
            AccountMeta::new(lb_pair, false),
            AccountMeta::new_readonly(program_id, false),
            AccountMeta::new(user_token_x, false),
            AccountMeta::new(user_token_y, false),
            AccountMeta::new(reserve_x, false),
            AccountMeta::new(reserve_y, false),
            AccountMeta::new_readonly(token_x_mint, false),
            AccountMeta::new_readonly(token_y_mint, false),
            AccountMeta::new(bin_array_lower, false),
            AccountMeta::new(bin_array_upper, false),
            AccountMeta::new_readonly(sender, true),
            AccountMeta::new_readonly(token_x_program, false),
            AccountMeta::new_readonly(token_y_program, false),
            AccountMeta::new_readonly(event_authority, false),
            AccountMeta::new_readonly(program, false),
        ];

        let gen_ix = instructions::remove_all_liquidity(
            &program_id,
            &instructions::RemoveAllLiquidityAccounts {
                position,
                lb_pair,
                bin_array_bitmap_extension: None,
                user_token_x,
                user_token_y,
                reserve_x,
                reserve_y,
                token_x_mint,
                token_y_mint,
                bin_array_lower,
                bin_array_upper,
                sender,
                token_x_program,
                token_y_program,
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
    fn test_remove_liquidity_instruction() {
        use meteora_dlmm::instructions;
        #[allow(unused_imports)]
        use meteora_dlmm::types;
        use solana_sdk::instruction::AccountMeta;

        let program_id = meteora_dlmm::ID;
        let position = Pubkey::from([1; 32]);
        let lb_pair = Pubkey::from([2; 32]);
        let user_token_x = Pubkey::from([3; 32]);
        let user_token_y = Pubkey::from([4; 32]);
        let reserve_x = Pubkey::from([5; 32]);
        let reserve_y = Pubkey::from([6; 32]);
        let token_x_mint = Pubkey::from([7; 32]);
        let token_y_mint = Pubkey::from([8; 32]);
        let bin_array_lower = Pubkey::from([9; 32]);
        let bin_array_upper = Pubkey::from([10; 32]);
        let sender = Pubkey::from([11; 32]);
        let token_x_program = Pubkey::from([12; 32]);
        let token_y_program = Pubkey::from([13; 32]);
        let event_authority = Pubkey::from([14; 32]);
        let program = Pubkey::from([15; 32]);
        let bin_liquidity_removal = vec![];

        let mut expected_data: Vec<u8> = vec![80, 85, 209, 72, 24, 206, 177, 108];
        borsh::BorshSerialize::serialize(&bin_liquidity_removal.clone(), &mut expected_data).unwrap();

        let expected_accounts = vec![
            AccountMeta::new(position, false),
            AccountMeta::new(lb_pair, false),
            AccountMeta::new_readonly(program_id, false),
            AccountMeta::new(user_token_x, false),
            AccountMeta::new(user_token_y, false),
            AccountMeta::new(reserve_x, false),
            AccountMeta::new(reserve_y, false),
            AccountMeta::new_readonly(token_x_mint, false),
            AccountMeta::new_readonly(token_y_mint, false),
            AccountMeta::new(bin_array_lower, false),
            AccountMeta::new(bin_array_upper, false),
            AccountMeta::new_readonly(sender, true),
            AccountMeta::new_readonly(token_x_program, false),
            AccountMeta::new_readonly(token_y_program, false),
            AccountMeta::new_readonly(event_authority, false),
            AccountMeta::new_readonly(program, false),
        ];

        let gen_ix = instructions::remove_liquidity(
            &program_id,
            &instructions::RemoveLiquidityAccounts {
                position,
                lb_pair,
                bin_array_bitmap_extension: None,
                user_token_x,
                user_token_y,
                reserve_x,
                reserve_y,
                token_x_mint,
                token_y_mint,
                bin_array_lower,
                bin_array_upper,
                sender,
                token_x_program,
                token_y_program,
                event_authority,
                program,
            },
            bin_liquidity_removal.clone(),
        );

        let expected_ix = solana_sdk::instruction::Instruction::new_with_bytes(
            program_id,
            &expected_data,
            expected_accounts,
        );
        assert_ix(&gen_ix, &expected_ix);
    }

    #[test]
    fn test_remove_liquidity2_instruction() {
        use meteora_dlmm::instructions;
        #[allow(unused_imports)]
        use meteora_dlmm::types;
        use solana_sdk::instruction::AccountMeta;

        let program_id = meteora_dlmm::ID;
        let position = Pubkey::from([1; 32]);
        let lb_pair = Pubkey::from([2; 32]);
        let user_token_x = Pubkey::from([3; 32]);
        let user_token_y = Pubkey::from([4; 32]);
        let reserve_x = Pubkey::from([5; 32]);
        let reserve_y = Pubkey::from([6; 32]);
        let token_x_mint = Pubkey::from([7; 32]);
        let token_y_mint = Pubkey::from([8; 32]);
        let sender = Pubkey::from([9; 32]);
        let token_x_program = Pubkey::from([10; 32]);
        let token_y_program = Pubkey::from([11; 32]);
        let memo_program = Pubkey::from([12; 32]);
        let event_authority = Pubkey::from([13; 32]);
        let program = Pubkey::from([14; 32]);
        let bin_liquidity_removal = vec![];
        let remaining_accounts_info = types::RemainingAccountsInfo { slices: vec![] };

        let mut expected_data: Vec<u8> = vec![230, 215, 82, 127, 241, 101, 227, 146];
        borsh::BorshSerialize::serialize(&bin_liquidity_removal.clone(), &mut expected_data).unwrap();
        borsh::BorshSerialize::serialize(&remaining_accounts_info.clone(), &mut expected_data).unwrap();

        let expected_accounts = vec![
            AccountMeta::new(position, false),
            AccountMeta::new(lb_pair, false),
            AccountMeta::new_readonly(program_id, false),
            AccountMeta::new(user_token_x, false),
            AccountMeta::new(user_token_y, false),
            AccountMeta::new(reserve_x, false),
            AccountMeta::new(reserve_y, false),
            AccountMeta::new_readonly(token_x_mint, false),
            AccountMeta::new_readonly(token_y_mint, false),
            AccountMeta::new_readonly(sender, true),
            AccountMeta::new_readonly(token_x_program, false),
            AccountMeta::new_readonly(token_y_program, false),
            AccountMeta::new_readonly(memo_program, false),
            AccountMeta::new_readonly(event_authority, false),
            AccountMeta::new_readonly(program, false),
        ];

        let gen_ix = instructions::remove_liquidity2(
            &program_id,
            &instructions::RemoveLiquidity2Accounts {
                position,
                lb_pair,
                bin_array_bitmap_extension: None,
                user_token_x,
                user_token_y,
                reserve_x,
                reserve_y,
                token_x_mint,
                token_y_mint,
                sender,
                token_x_program,
                token_y_program,
                memo_program,
                event_authority,
                program,
            },
            bin_liquidity_removal.clone(),
            remaining_accounts_info.clone(),
        );

        let expected_ix = solana_sdk::instruction::Instruction::new_with_bytes(
            program_id,
            &expected_data,
            expected_accounts,
        );
        assert_ix(&gen_ix, &expected_ix);
    }

    #[test]
    fn test_remove_liquidity_by_range_instruction() {
        use meteora_dlmm::instructions;
        #[allow(unused_imports)]
        use meteora_dlmm::types;
        use solana_sdk::instruction::AccountMeta;

        let program_id = meteora_dlmm::ID;
        let position = Pubkey::from([1; 32]);
        let lb_pair = Pubkey::from([2; 32]);
        let user_token_x = Pubkey::from([3; 32]);
        let user_token_y = Pubkey::from([4; 32]);
        let reserve_x = Pubkey::from([5; 32]);
        let reserve_y = Pubkey::from([6; 32]);
        let token_x_mint = Pubkey::from([7; 32]);
        let token_y_mint = Pubkey::from([8; 32]);
        let bin_array_lower = Pubkey::from([9; 32]);
        let bin_array_upper = Pubkey::from([10; 32]);
        let sender = Pubkey::from([11; 32]);
        let token_x_program = Pubkey::from([12; 32]);
        let token_y_program = Pubkey::from([13; 32]);
        let event_authority = Pubkey::from([14; 32]);
        let program = Pubkey::from([15; 32]);
        let from_bin_id = 0i32;
        let to_bin_id = 0i32;
        let bps_to_remove = 0u16;

        let mut expected_data: Vec<u8> = vec![26, 82, 102, 152, 240, 74, 105, 26];
        borsh::BorshSerialize::serialize(&from_bin_id, &mut expected_data).unwrap();
        borsh::BorshSerialize::serialize(&to_bin_id, &mut expected_data).unwrap();
        borsh::BorshSerialize::serialize(&bps_to_remove, &mut expected_data).unwrap();

        let expected_accounts = vec![
            AccountMeta::new(position, false),
            AccountMeta::new(lb_pair, false),
            AccountMeta::new_readonly(program_id, false),
            AccountMeta::new(user_token_x, false),
            AccountMeta::new(user_token_y, false),
            AccountMeta::new(reserve_x, false),
            AccountMeta::new(reserve_y, false),
            AccountMeta::new_readonly(token_x_mint, false),
            AccountMeta::new_readonly(token_y_mint, false),
            AccountMeta::new(bin_array_lower, false),
            AccountMeta::new(bin_array_upper, false),
            AccountMeta::new_readonly(sender, true),
            AccountMeta::new_readonly(token_x_program, false),
            AccountMeta::new_readonly(token_y_program, false),
            AccountMeta::new_readonly(event_authority, false),
            AccountMeta::new_readonly(program, false),
        ];

        let gen_ix = instructions::remove_liquidity_by_range(
            &program_id,
            &instructions::RemoveLiquidityByRangeAccounts {
                position,
                lb_pair,
                bin_array_bitmap_extension: None,
                user_token_x,
                user_token_y,
                reserve_x,
                reserve_y,
                token_x_mint,
                token_y_mint,
                bin_array_lower,
                bin_array_upper,
                sender,
                token_x_program,
                token_y_program,
                event_authority,
                program,
            },
            from_bin_id,
            to_bin_id,
            bps_to_remove,
        );

        let expected_ix = solana_sdk::instruction::Instruction::new_with_bytes(
            program_id,
            &expected_data,
            expected_accounts,
        );
        assert_ix(&gen_ix, &expected_ix);
    }

    #[test]
    fn test_remove_liquidity_by_range2_instruction() {
        use meteora_dlmm::instructions;
        #[allow(unused_imports)]
        use meteora_dlmm::types;
        use solana_sdk::instruction::AccountMeta;

        let program_id = meteora_dlmm::ID;
        let position = Pubkey::from([1; 32]);
        let lb_pair = Pubkey::from([2; 32]);
        let user_token_x = Pubkey::from([3; 32]);
        let user_token_y = Pubkey::from([4; 32]);
        let reserve_x = Pubkey::from([5; 32]);
        let reserve_y = Pubkey::from([6; 32]);
        let token_x_mint = Pubkey::from([7; 32]);
        let token_y_mint = Pubkey::from([8; 32]);
        let sender = Pubkey::from([9; 32]);
        let token_x_program = Pubkey::from([10; 32]);
        let token_y_program = Pubkey::from([11; 32]);
        let memo_program = Pubkey::from([12; 32]);
        let event_authority = Pubkey::from([13; 32]);
        let program = Pubkey::from([14; 32]);
        let from_bin_id = 0i32;
        let to_bin_id = 0i32;
        let bps_to_remove = 0u16;
        let remaining_accounts_info = types::RemainingAccountsInfo { slices: vec![] };

        let mut expected_data: Vec<u8> = vec![204, 2, 195, 145, 53, 145, 145, 205];
        borsh::BorshSerialize::serialize(&from_bin_id, &mut expected_data).unwrap();
        borsh::BorshSerialize::serialize(&to_bin_id, &mut expected_data).unwrap();
        borsh::BorshSerialize::serialize(&bps_to_remove, &mut expected_data).unwrap();
        borsh::BorshSerialize::serialize(&remaining_accounts_info.clone(), &mut expected_data).unwrap();

        let expected_accounts = vec![
            AccountMeta::new(position, false),
            AccountMeta::new(lb_pair, false),
            AccountMeta::new_readonly(program_id, false),
            AccountMeta::new(user_token_x, false),
            AccountMeta::new(user_token_y, false),
            AccountMeta::new(reserve_x, false),
            AccountMeta::new(reserve_y, false),
            AccountMeta::new_readonly(token_x_mint, false),
            AccountMeta::new_readonly(token_y_mint, false),
            AccountMeta::new_readonly(sender, true),
            AccountMeta::new_readonly(token_x_program, false),
            AccountMeta::new_readonly(token_y_program, false),
            AccountMeta::new_readonly(memo_program, false),
            AccountMeta::new_readonly(event_authority, false),
            AccountMeta::new_readonly(program, false),
        ];

        let gen_ix = instructions::remove_liquidity_by_range2(
            &program_id,
            &instructions::RemoveLiquidityByRange2Accounts {
                position,
                lb_pair,
                bin_array_bitmap_extension: None,
                user_token_x,
                user_token_y,
                reserve_x,
                reserve_y,
                token_x_mint,
                token_y_mint,
                sender,
                token_x_program,
                token_y_program,
                memo_program,
                event_authority,
                program,
            },
            from_bin_id,
            to_bin_id,
            bps_to_remove,
            remaining_accounts_info.clone(),
        );

        let expected_ix = solana_sdk::instruction::Instruction::new_with_bytes(
            program_id,
            &expected_data,
            expected_accounts,
        );
        assert_ix(&gen_ix, &expected_ix);
    }

    #[test]
    fn test_reset_bin_array_tombstone_fields_instruction() {
        use meteora_dlmm::instructions;
        #[allow(unused_imports)]
        use meteora_dlmm::types;
        use solana_sdk::instruction::AccountMeta;

        let program_id = meteora_dlmm::ID;
        let lb_pair = Pubkey::from([1; 32]);
        let bin_array = Pubkey::from([2; 32]);
        let operator = Pubkey::from([3; 32]);
        let signer = Pubkey::from([4; 32]);

        let expected_data: Vec<u8> = vec![54, 90, 252, 63, 41, 206, 63, 63];

        let expected_accounts = vec![
            AccountMeta::new_readonly(lb_pair, false),
            AccountMeta::new(bin_array, false),
            AccountMeta::new_readonly(operator, false),
            AccountMeta::new_readonly(signer, true),
        ];

        let gen_ix = instructions::reset_bin_array_tombstone_fields(
            &program_id,
            &instructions::ResetBinArrayTombstoneFieldsAccounts {
                lb_pair,
                bin_array,
                operator,
                signer,
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
    fn test_reset_pool_tombstone_fields_instruction() {
        use meteora_dlmm::instructions;
        #[allow(unused_imports)]
        use meteora_dlmm::types;
        use solana_sdk::instruction::AccountMeta;

        let program_id = meteora_dlmm::ID;
        let lb_pair = Pubkey::from([1; 32]);
        let operator = Pubkey::from([2; 32]);
        let signer = Pubkey::from([3; 32]);

        let expected_data: Vec<u8> = vec![246, 109, 19, 120, 108, 113, 68, 252];

        let expected_accounts = vec![
            AccountMeta::new(lb_pair, false),
            AccountMeta::new_readonly(operator, false),
            AccountMeta::new_readonly(signer, true),
        ];

        let gen_ix = instructions::reset_pool_tombstone_fields(
            &program_id,
            &instructions::ResetPoolTombstoneFieldsAccounts {
                lb_pair,
                operator,
                signer,
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
    fn test_reset_position_tombstone_fields_instruction() {
        use meteora_dlmm::instructions;
        #[allow(unused_imports)]
        use meteora_dlmm::types;
        use solana_sdk::instruction::AccountMeta;

        let program_id = meteora_dlmm::ID;
        let position = Pubkey::from([1; 32]);
        let operator = Pubkey::from([2; 32]);
        let signer = Pubkey::from([3; 32]);

        let expected_data: Vec<u8> = vec![206, 6, 51, 218, 211, 30, 159, 84];

        let expected_accounts = vec![
            AccountMeta::new(position, false),
            AccountMeta::new_readonly(operator, false),
            AccountMeta::new_readonly(signer, true),
        ];

        let gen_ix = instructions::reset_position_tombstone_fields(
            &program_id,
            &instructions::ResetPositionTombstoneFieldsAccounts {
                position,
                operator,
                signer,
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
    fn test_set_activation_point_instruction() {
        use meteora_dlmm::instructions;
        #[allow(unused_imports)]
        use meteora_dlmm::types;
        use solana_sdk::instruction::AccountMeta;

        let program_id = meteora_dlmm::ID;
        let lb_pair = Pubkey::from([1; 32]);
        let signer = Pubkey::from([2; 32]);
        let activation_point = 0u64;

        let mut expected_data: Vec<u8> = vec![91, 249, 15, 165, 26, 129, 254, 125];
        borsh::BorshSerialize::serialize(&activation_point, &mut expected_data).unwrap();

        let expected_accounts = vec![
            AccountMeta::new(lb_pair, false),
            AccountMeta::new_readonly(signer, true),
        ];

        let gen_ix = instructions::set_activation_point(
            &program_id,
            &instructions::SetActivationPointAccounts {
                lb_pair,
                signer,
            },
            activation_point,
        );

        let expected_ix = solana_sdk::instruction::Instruction::new_with_bytes(
            program_id,
            &expected_data,
            expected_accounts,
        );
        assert_ix(&gen_ix, &expected_ix);
    }

    #[test]
    fn test_set_pair_status_instruction() {
        use meteora_dlmm::instructions;
        #[allow(unused_imports)]
        use meteora_dlmm::types;
        use solana_sdk::instruction::AccountMeta;

        let program_id = meteora_dlmm::ID;
        let lb_pair = Pubkey::from([1; 32]);
        let operator = Pubkey::from([2; 32]);
        let signer = Pubkey::from([3; 32]);
        let status = 0u8;

        let mut expected_data: Vec<u8> = vec![67, 248, 231, 137, 154, 149, 217, 174];
        borsh::BorshSerialize::serialize(&status, &mut expected_data).unwrap();

        let expected_accounts = vec![
            AccountMeta::new(lb_pair, false),
            AccountMeta::new_readonly(operator, false),
            AccountMeta::new_readonly(signer, true),
        ];

        let gen_ix = instructions::set_pair_status(
            &program_id,
            &instructions::SetPairStatusAccounts {
                lb_pair,
                operator,
                signer,
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
    fn test_set_pair_status_permissionless_instruction() {
        use meteora_dlmm::instructions;
        #[allow(unused_imports)]
        use meteora_dlmm::types;
        use solana_sdk::instruction::AccountMeta;

        let program_id = meteora_dlmm::ID;
        let lb_pair = Pubkey::from([1; 32]);
        let signer = Pubkey::from([2; 32]);
        let status = 0u8;

        let mut expected_data: Vec<u8> = vec![78, 59, 152, 211, 70, 183, 46, 208];
        borsh::BorshSerialize::serialize(&status, &mut expected_data).unwrap();

        let expected_accounts = vec![
            AccountMeta::new(lb_pair, false),
            AccountMeta::new_readonly(signer, true),
        ];

        let gen_ix = instructions::set_pair_status_permissionless(
            &program_id,
            &instructions::SetPairStatusPermissionlessAccounts {
                lb_pair,
                signer,
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
    fn test_set_pre_activation_duration_instruction() {
        use meteora_dlmm::instructions;
        #[allow(unused_imports)]
        use meteora_dlmm::types;
        use solana_sdk::instruction::AccountMeta;

        let program_id = meteora_dlmm::ID;
        let lb_pair = Pubkey::from([1; 32]);
        let signer = Pubkey::from([2; 32]);
        let pre_activation_duration = 0u64;

        let mut expected_data: Vec<u8> = vec![165, 61, 201, 244, 130, 159, 22, 100];
        borsh::BorshSerialize::serialize(&pre_activation_duration, &mut expected_data).unwrap();

        let expected_accounts = vec![
            AccountMeta::new(lb_pair, false),
            AccountMeta::new_readonly(signer, true),
        ];

        let gen_ix = instructions::set_pre_activation_duration(
            &program_id,
            &instructions::SetPreActivationDurationAccounts {
                lb_pair,
                signer,
            },
            pre_activation_duration,
        );

        let expected_ix = solana_sdk::instruction::Instruction::new_with_bytes(
            program_id,
            &expected_data,
            expected_accounts,
        );
        assert_ix(&gen_ix, &expected_ix);
    }

    #[test]
    fn test_set_pre_activation_swap_address_instruction() {
        use meteora_dlmm::instructions;
        #[allow(unused_imports)]
        use meteora_dlmm::types;
        use solana_sdk::instruction::AccountMeta;

        let program_id = meteora_dlmm::ID;
        let lb_pair = Pubkey::from([1; 32]);
        let signer = Pubkey::from([2; 32]);
        let pre_activation_swap_address = Pubkey::from([0; 32]);

        let mut expected_data: Vec<u8> = vec![57, 139, 47, 123, 216, 80, 223, 10];
        borsh::BorshSerialize::serialize(&pre_activation_swap_address, &mut expected_data).unwrap();

        let expected_accounts = vec![
            AccountMeta::new(lb_pair, false),
            AccountMeta::new_readonly(signer, true),
        ];

        let gen_ix = instructions::set_pre_activation_swap_address(
            &program_id,
            &instructions::SetPreActivationSwapAddressAccounts {
                lb_pair,
                signer,
            },
            pre_activation_swap_address,
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
        use meteora_dlmm::instructions;
        #[allow(unused_imports)]
        use meteora_dlmm::types;
        use solana_sdk::instruction::AccountMeta;

        let program_id = meteora_dlmm::ID;
        let lb_pair = Pubkey::from([1; 32]);
        let reserve_x = Pubkey::from([2; 32]);
        let reserve_y = Pubkey::from([3; 32]);
        let user_token_in = Pubkey::from([4; 32]);
        let user_token_out = Pubkey::from([5; 32]);
        let token_x_mint = Pubkey::from([6; 32]);
        let token_y_mint = Pubkey::from([7; 32]);
        let oracle = Pubkey::from([8; 32]);
        let user = Pubkey::from([9; 32]);
        let token_x_program = Pubkey::from([10; 32]);
        let token_y_program = Pubkey::from([11; 32]);
        let event_authority = Pubkey::from([12; 32]);
        let program = Pubkey::from([13; 32]);
        let amount_in = 0u64;
        let min_amount_out = 0u64;

        let mut expected_data: Vec<u8> = vec![248, 198, 158, 145, 225, 117, 135, 200];
        borsh::BorshSerialize::serialize(&amount_in, &mut expected_data).unwrap();
        borsh::BorshSerialize::serialize(&min_amount_out, &mut expected_data).unwrap();

        let expected_accounts = vec![
            AccountMeta::new(lb_pair, false),
            AccountMeta::new_readonly(program_id, false),
            AccountMeta::new(reserve_x, false),
            AccountMeta::new(reserve_y, false),
            AccountMeta::new(user_token_in, false),
            AccountMeta::new(user_token_out, false),
            AccountMeta::new_readonly(token_x_mint, false),
            AccountMeta::new_readonly(token_y_mint, false),
            AccountMeta::new(oracle, false),
            AccountMeta::new_readonly(program_id, false),
            AccountMeta::new_readonly(user, true),
            AccountMeta::new_readonly(token_x_program, false),
            AccountMeta::new_readonly(token_y_program, false),
            AccountMeta::new_readonly(event_authority, false),
            AccountMeta::new_readonly(program, false),
        ];

        let gen_ix = instructions::swap(
            &program_id,
            &instructions::SwapAccounts {
                lb_pair,
                bin_array_bitmap_extension: None,
                reserve_x,
                reserve_y,
                user_token_in,
                user_token_out,
                token_x_mint,
                token_y_mint,
                oracle,
                host_fee_in: None,
                user,
                token_x_program,
                token_y_program,
                event_authority,
                program,
            },
            amount_in,
            min_amount_out,
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
        use meteora_dlmm::instructions;
        #[allow(unused_imports)]
        use meteora_dlmm::types;
        use solana_sdk::instruction::AccountMeta;

        let program_id = meteora_dlmm::ID;
        let lb_pair = Pubkey::from([1; 32]);
        let reserve_x = Pubkey::from([2; 32]);
        let reserve_y = Pubkey::from([3; 32]);
        let user_token_in = Pubkey::from([4; 32]);
        let user_token_out = Pubkey::from([5; 32]);
        let token_x_mint = Pubkey::from([6; 32]);
        let token_y_mint = Pubkey::from([7; 32]);
        let oracle = Pubkey::from([8; 32]);
        let user = Pubkey::from([9; 32]);
        let token_x_program = Pubkey::from([10; 32]);
        let token_y_program = Pubkey::from([11; 32]);
        let memo_program = Pubkey::from([12; 32]);
        let event_authority = Pubkey::from([13; 32]);
        let program = Pubkey::from([14; 32]);
        let amount_in = 0u64;
        let min_amount_out = 0u64;
        let remaining_accounts_info = types::RemainingAccountsInfo { slices: vec![] };

        let mut expected_data: Vec<u8> = vec![65, 75, 63, 76, 235, 91, 91, 136];
        borsh::BorshSerialize::serialize(&amount_in, &mut expected_data).unwrap();
        borsh::BorshSerialize::serialize(&min_amount_out, &mut expected_data).unwrap();
        borsh::BorshSerialize::serialize(&remaining_accounts_info.clone(), &mut expected_data).unwrap();

        let expected_accounts = vec![
            AccountMeta::new(lb_pair, false),
            AccountMeta::new_readonly(program_id, false),
            AccountMeta::new(reserve_x, false),
            AccountMeta::new(reserve_y, false),
            AccountMeta::new(user_token_in, false),
            AccountMeta::new(user_token_out, false),
            AccountMeta::new_readonly(token_x_mint, false),
            AccountMeta::new_readonly(token_y_mint, false),
            AccountMeta::new(oracle, false),
            AccountMeta::new_readonly(program_id, false),
            AccountMeta::new_readonly(user, true),
            AccountMeta::new_readonly(token_x_program, false),
            AccountMeta::new_readonly(token_y_program, false),
            AccountMeta::new_readonly(memo_program, false),
            AccountMeta::new_readonly(event_authority, false),
            AccountMeta::new_readonly(program, false),
        ];

        let gen_ix = instructions::swap2(
            &program_id,
            &instructions::Swap2Accounts {
                lb_pair,
                bin_array_bitmap_extension: None,
                reserve_x,
                reserve_y,
                user_token_in,
                user_token_out,
                token_x_mint,
                token_y_mint,
                oracle,
                host_fee_in: None,
                user,
                token_x_program,
                token_y_program,
                memo_program,
                event_authority,
                program,
            },
            amount_in,
            min_amount_out,
            remaining_accounts_info.clone(),
        );

        let expected_ix = solana_sdk::instruction::Instruction::new_with_bytes(
            program_id,
            &expected_data,
            expected_accounts,
        );
        assert_ix(&gen_ix, &expected_ix);
    }

    #[test]
    fn test_swap_exact_out_instruction() {
        use meteora_dlmm::instructions;
        #[allow(unused_imports)]
        use meteora_dlmm::types;
        use solana_sdk::instruction::AccountMeta;

        let program_id = meteora_dlmm::ID;
        let lb_pair = Pubkey::from([1; 32]);
        let reserve_x = Pubkey::from([2; 32]);
        let reserve_y = Pubkey::from([3; 32]);
        let user_token_in = Pubkey::from([4; 32]);
        let user_token_out = Pubkey::from([5; 32]);
        let token_x_mint = Pubkey::from([6; 32]);
        let token_y_mint = Pubkey::from([7; 32]);
        let oracle = Pubkey::from([8; 32]);
        let user = Pubkey::from([9; 32]);
        let token_x_program = Pubkey::from([10; 32]);
        let token_y_program = Pubkey::from([11; 32]);
        let event_authority = Pubkey::from([12; 32]);
        let program = Pubkey::from([13; 32]);
        let max_in_amount = 0u64;
        let out_amount = 0u64;

        let mut expected_data: Vec<u8> = vec![250, 73, 101, 33, 38, 207, 75, 184];
        borsh::BorshSerialize::serialize(&max_in_amount, &mut expected_data).unwrap();
        borsh::BorshSerialize::serialize(&out_amount, &mut expected_data).unwrap();

        let expected_accounts = vec![
            AccountMeta::new(lb_pair, false),
            AccountMeta::new_readonly(program_id, false),
            AccountMeta::new(reserve_x, false),
            AccountMeta::new(reserve_y, false),
            AccountMeta::new(user_token_in, false),
            AccountMeta::new(user_token_out, false),
            AccountMeta::new_readonly(token_x_mint, false),
            AccountMeta::new_readonly(token_y_mint, false),
            AccountMeta::new(oracle, false),
            AccountMeta::new_readonly(program_id, false),
            AccountMeta::new_readonly(user, true),
            AccountMeta::new_readonly(token_x_program, false),
            AccountMeta::new_readonly(token_y_program, false),
            AccountMeta::new_readonly(event_authority, false),
            AccountMeta::new_readonly(program, false),
        ];

        let gen_ix = instructions::swap_exact_out(
            &program_id,
            &instructions::SwapExactOutAccounts {
                lb_pair,
                bin_array_bitmap_extension: None,
                reserve_x,
                reserve_y,
                user_token_in,
                user_token_out,
                token_x_mint,
                token_y_mint,
                oracle,
                host_fee_in: None,
                user,
                token_x_program,
                token_y_program,
                event_authority,
                program,
            },
            max_in_amount,
            out_amount,
        );

        let expected_ix = solana_sdk::instruction::Instruction::new_with_bytes(
            program_id,
            &expected_data,
            expected_accounts,
        );
        assert_ix(&gen_ix, &expected_ix);
    }

    #[test]
    fn test_swap_exact_out2_instruction() {
        use meteora_dlmm::instructions;
        #[allow(unused_imports)]
        use meteora_dlmm::types;
        use solana_sdk::instruction::AccountMeta;

        let program_id = meteora_dlmm::ID;
        let lb_pair = Pubkey::from([1; 32]);
        let reserve_x = Pubkey::from([2; 32]);
        let reserve_y = Pubkey::from([3; 32]);
        let user_token_in = Pubkey::from([4; 32]);
        let user_token_out = Pubkey::from([5; 32]);
        let token_x_mint = Pubkey::from([6; 32]);
        let token_y_mint = Pubkey::from([7; 32]);
        let oracle = Pubkey::from([8; 32]);
        let user = Pubkey::from([9; 32]);
        let token_x_program = Pubkey::from([10; 32]);
        let token_y_program = Pubkey::from([11; 32]);
        let memo_program = Pubkey::from([12; 32]);
        let event_authority = Pubkey::from([13; 32]);
        let program = Pubkey::from([14; 32]);
        let max_in_amount = 0u64;
        let out_amount = 0u64;
        let remaining_accounts_info = types::RemainingAccountsInfo { slices: vec![] };

        let mut expected_data: Vec<u8> = vec![43, 215, 247, 132, 137, 60, 243, 81];
        borsh::BorshSerialize::serialize(&max_in_amount, &mut expected_data).unwrap();
        borsh::BorshSerialize::serialize(&out_amount, &mut expected_data).unwrap();
        borsh::BorshSerialize::serialize(&remaining_accounts_info.clone(), &mut expected_data).unwrap();

        let expected_accounts = vec![
            AccountMeta::new(lb_pair, false),
            AccountMeta::new_readonly(program_id, false),
            AccountMeta::new(reserve_x, false),
            AccountMeta::new(reserve_y, false),
            AccountMeta::new(user_token_in, false),
            AccountMeta::new(user_token_out, false),
            AccountMeta::new_readonly(token_x_mint, false),
            AccountMeta::new_readonly(token_y_mint, false),
            AccountMeta::new(oracle, false),
            AccountMeta::new_readonly(program_id, false),
            AccountMeta::new_readonly(user, true),
            AccountMeta::new_readonly(token_x_program, false),
            AccountMeta::new_readonly(token_y_program, false),
            AccountMeta::new_readonly(memo_program, false),
            AccountMeta::new_readonly(event_authority, false),
            AccountMeta::new_readonly(program, false),
        ];

        let gen_ix = instructions::swap_exact_out2(
            &program_id,
            &instructions::SwapExactOut2Accounts {
                lb_pair,
                bin_array_bitmap_extension: None,
                reserve_x,
                reserve_y,
                user_token_in,
                user_token_out,
                token_x_mint,
                token_y_mint,
                oracle,
                host_fee_in: None,
                user,
                token_x_program,
                token_y_program,
                memo_program,
                event_authority,
                program,
            },
            max_in_amount,
            out_amount,
            remaining_accounts_info.clone(),
        );

        let expected_ix = solana_sdk::instruction::Instruction::new_with_bytes(
            program_id,
            &expected_data,
            expected_accounts,
        );
        assert_ix(&gen_ix, &expected_ix);
    }

    #[test]
    fn test_swap_with_price_impact_instruction() {
        use meteora_dlmm::instructions;
        #[allow(unused_imports)]
        use meteora_dlmm::types;
        use solana_sdk::instruction::AccountMeta;

        let program_id = meteora_dlmm::ID;
        let lb_pair = Pubkey::from([1; 32]);
        let reserve_x = Pubkey::from([2; 32]);
        let reserve_y = Pubkey::from([3; 32]);
        let user_token_in = Pubkey::from([4; 32]);
        let user_token_out = Pubkey::from([5; 32]);
        let token_x_mint = Pubkey::from([6; 32]);
        let token_y_mint = Pubkey::from([7; 32]);
        let oracle = Pubkey::from([8; 32]);
        let user = Pubkey::from([9; 32]);
        let token_x_program = Pubkey::from([10; 32]);
        let token_y_program = Pubkey::from([11; 32]);
        let event_authority = Pubkey::from([12; 32]);
        let program = Pubkey::from([13; 32]);
        let amount_in = 0u64;
        let active_id = None;
        let max_price_impact_bps = 0u16;

        let mut expected_data: Vec<u8> = vec![56, 173, 230, 208, 173, 228, 156, 205];
        borsh::BorshSerialize::serialize(&amount_in, &mut expected_data).unwrap();
        borsh::BorshSerialize::serialize(&active_id, &mut expected_data).unwrap();
        borsh::BorshSerialize::serialize(&max_price_impact_bps, &mut expected_data).unwrap();

        let expected_accounts = vec![
            AccountMeta::new(lb_pair, false),
            AccountMeta::new_readonly(program_id, false),
            AccountMeta::new(reserve_x, false),
            AccountMeta::new(reserve_y, false),
            AccountMeta::new(user_token_in, false),
            AccountMeta::new(user_token_out, false),
            AccountMeta::new_readonly(token_x_mint, false),
            AccountMeta::new_readonly(token_y_mint, false),
            AccountMeta::new(oracle, false),
            AccountMeta::new_readonly(program_id, false),
            AccountMeta::new_readonly(user, true),
            AccountMeta::new_readonly(token_x_program, false),
            AccountMeta::new_readonly(token_y_program, false),
            AccountMeta::new_readonly(event_authority, false),
            AccountMeta::new_readonly(program, false),
        ];

        let gen_ix = instructions::swap_with_price_impact(
            &program_id,
            &instructions::SwapWithPriceImpactAccounts {
                lb_pair,
                bin_array_bitmap_extension: None,
                reserve_x,
                reserve_y,
                user_token_in,
                user_token_out,
                token_x_mint,
                token_y_mint,
                oracle,
                host_fee_in: None,
                user,
                token_x_program,
                token_y_program,
                event_authority,
                program,
            },
            amount_in,
            active_id,
            max_price_impact_bps,
        );

        let expected_ix = solana_sdk::instruction::Instruction::new_with_bytes(
            program_id,
            &expected_data,
            expected_accounts,
        );
        assert_ix(&gen_ix, &expected_ix);
    }

    #[test]
    fn test_swap_with_price_impact2_instruction() {
        use meteora_dlmm::instructions;
        #[allow(unused_imports)]
        use meteora_dlmm::types;
        use solana_sdk::instruction::AccountMeta;

        let program_id = meteora_dlmm::ID;
        let lb_pair = Pubkey::from([1; 32]);
        let reserve_x = Pubkey::from([2; 32]);
        let reserve_y = Pubkey::from([3; 32]);
        let user_token_in = Pubkey::from([4; 32]);
        let user_token_out = Pubkey::from([5; 32]);
        let token_x_mint = Pubkey::from([6; 32]);
        let token_y_mint = Pubkey::from([7; 32]);
        let oracle = Pubkey::from([8; 32]);
        let user = Pubkey::from([9; 32]);
        let token_x_program = Pubkey::from([10; 32]);
        let token_y_program = Pubkey::from([11; 32]);
        let memo_program = Pubkey::from([12; 32]);
        let event_authority = Pubkey::from([13; 32]);
        let program = Pubkey::from([14; 32]);
        let amount_in = 0u64;
        let active_id = None;
        let max_price_impact_bps = 0u16;
        let remaining_accounts_info = types::RemainingAccountsInfo { slices: vec![] };

        let mut expected_data: Vec<u8> = vec![74, 98, 192, 214, 177, 51, 75, 51];
        borsh::BorshSerialize::serialize(&amount_in, &mut expected_data).unwrap();
        borsh::BorshSerialize::serialize(&active_id, &mut expected_data).unwrap();
        borsh::BorshSerialize::serialize(&max_price_impact_bps, &mut expected_data).unwrap();
        borsh::BorshSerialize::serialize(&remaining_accounts_info.clone(), &mut expected_data).unwrap();

        let expected_accounts = vec![
            AccountMeta::new(lb_pair, false),
            AccountMeta::new_readonly(program_id, false),
            AccountMeta::new(reserve_x, false),
            AccountMeta::new(reserve_y, false),
            AccountMeta::new(user_token_in, false),
            AccountMeta::new(user_token_out, false),
            AccountMeta::new_readonly(token_x_mint, false),
            AccountMeta::new_readonly(token_y_mint, false),
            AccountMeta::new(oracle, false),
            AccountMeta::new_readonly(program_id, false),
            AccountMeta::new_readonly(user, true),
            AccountMeta::new_readonly(token_x_program, false),
            AccountMeta::new_readonly(token_y_program, false),
            AccountMeta::new_readonly(memo_program, false),
            AccountMeta::new_readonly(event_authority, false),
            AccountMeta::new_readonly(program, false),
        ];

        let gen_ix = instructions::swap_with_price_impact2(
            &program_id,
            &instructions::SwapWithPriceImpact2Accounts {
                lb_pair,
                bin_array_bitmap_extension: None,
                reserve_x,
                reserve_y,
                user_token_in,
                user_token_out,
                token_x_mint,
                token_y_mint,
                oracle,
                host_fee_in: None,
                user,
                token_x_program,
                token_y_program,
                memo_program,
                event_authority,
                program,
            },
            amount_in,
            active_id,
            max_price_impact_bps,
            remaining_accounts_info.clone(),
        );

        let expected_ix = solana_sdk::instruction::Instruction::new_with_bytes(
            program_id,
            &expected_data,
            expected_accounts,
        );
        assert_ix(&gen_ix, &expected_ix);
    }

    #[test]
    fn test_update_base_fee_parameters_instruction() {
        use meteora_dlmm::instructions;
        #[allow(unused_imports)]
        use meteora_dlmm::types;
        use solana_sdk::instruction::AccountMeta;

        let program_id = meteora_dlmm::ID;
        let lb_pair = Pubkey::from([1; 32]);
        let operator = Pubkey::from([2; 32]);
        let signer = Pubkey::from([3; 32]);
        let event_authority = Pubkey::from([4; 32]);
        let program = Pubkey::from([5; 32]);
        let fee_parameter = types::BaseFeeParameter { protocol_share: 0u16, base_factor: 0u16, base_fee_power_factor: 0u8 };

        let mut expected_data: Vec<u8> = vec![75, 168, 223, 161, 16, 195, 3, 47];
        borsh::BorshSerialize::serialize(&fee_parameter.clone(), &mut expected_data).unwrap();

        let expected_accounts = vec![
            AccountMeta::new(lb_pair, false),
            AccountMeta::new_readonly(operator, false),
            AccountMeta::new_readonly(signer, true),
            AccountMeta::new_readonly(event_authority, false),
            AccountMeta::new_readonly(program, false),
        ];

        let gen_ix = instructions::update_base_fee_parameters(
            &program_id,
            &instructions::UpdateBaseFeeParametersAccounts {
                lb_pair,
                operator,
                signer,
                event_authority,
                program,
            },
            fee_parameter.clone(),
        );

        let expected_ix = solana_sdk::instruction::Instruction::new_with_bytes(
            program_id,
            &expected_data,
            expected_accounts,
        );
        assert_ix(&gen_ix, &expected_ix);
    }

    #[test]
    fn test_update_dynamic_fee_parameters_instruction() {
        use meteora_dlmm::instructions;
        #[allow(unused_imports)]
        use meteora_dlmm::types;
        use solana_sdk::instruction::AccountMeta;

        let program_id = meteora_dlmm::ID;
        let lb_pair = Pubkey::from([1; 32]);
        let operator = Pubkey::from([2; 32]);
        let signer = Pubkey::from([3; 32]);
        let event_authority = Pubkey::from([4; 32]);
        let program = Pubkey::from([5; 32]);
        let fee_parameter = types::DynamicFeeParameter { filter_period: 0u16, decay_period: 0u16, reduction_factor: 0u16, variable_fee_control: 0u32, max_volatility_accumulator: 0u32 };

        let mut expected_data: Vec<u8> = vec![92, 161, 46, 246, 255, 189, 22, 22];
        borsh::BorshSerialize::serialize(&fee_parameter.clone(), &mut expected_data).unwrap();

        let expected_accounts = vec![
            AccountMeta::new(lb_pair, false),
            AccountMeta::new_readonly(operator, false),
            AccountMeta::new_readonly(signer, true),
            AccountMeta::new_readonly(event_authority, false),
            AccountMeta::new_readonly(program, false),
        ];

        let gen_ix = instructions::update_dynamic_fee_parameters(
            &program_id,
            &instructions::UpdateDynamicFeeParametersAccounts {
                lb_pair,
                operator,
                signer,
                event_authority,
                program,
            },
            fee_parameter.clone(),
        );

        let expected_ix = solana_sdk::instruction::Instruction::new_with_bytes(
            program_id,
            &expected_data,
            expected_accounts,
        );
        assert_ix(&gen_ix, &expected_ix);
    }

    #[test]
    fn test_update_fees_and_reward2_instruction() {
        use meteora_dlmm::instructions;
        #[allow(unused_imports)]
        use meteora_dlmm::types;
        use solana_sdk::instruction::AccountMeta;

        let program_id = meteora_dlmm::ID;
        let position = Pubkey::from([1; 32]);
        let lb_pair = Pubkey::from([2; 32]);
        let owner = Pubkey::from([3; 32]);
        let min_bin_id = 0i32;
        let max_bin_id = 0i32;

        let mut expected_data: Vec<u8> = vec![32, 142, 184, 154, 103, 65, 184, 88];
        borsh::BorshSerialize::serialize(&min_bin_id, &mut expected_data).unwrap();
        borsh::BorshSerialize::serialize(&max_bin_id, &mut expected_data).unwrap();

        let expected_accounts = vec![
            AccountMeta::new(position, false),
            AccountMeta::new(lb_pair, false),
            AccountMeta::new_readonly(owner, true),
        ];

        let gen_ix = instructions::update_fees_and_reward2(
            &program_id,
            &instructions::UpdateFeesAndReward2Accounts {
                position,
                lb_pair,
                owner,
            },
            min_bin_id,
            max_bin_id,
        );

        let expected_ix = solana_sdk::instruction::Instruction::new_with_bytes(
            program_id,
            &expected_data,
            expected_accounts,
        );
        assert_ix(&gen_ix, &expected_ix);
    }

    #[test]
    fn test_update_fees_and_rewards_instruction() {
        use meteora_dlmm::instructions;
        #[allow(unused_imports)]
        use meteora_dlmm::types;
        use solana_sdk::instruction::AccountMeta;

        let program_id = meteora_dlmm::ID;
        let position = Pubkey::from([1; 32]);
        let lb_pair = Pubkey::from([2; 32]);
        let bin_array_lower = Pubkey::from([3; 32]);
        let bin_array_upper = Pubkey::from([4; 32]);
        let owner = Pubkey::from([5; 32]);

        let expected_data: Vec<u8> = vec![154, 230, 250, 13, 236, 209, 75, 223];

        let expected_accounts = vec![
            AccountMeta::new(position, false),
            AccountMeta::new(lb_pair, false),
            AccountMeta::new(bin_array_lower, false),
            AccountMeta::new(bin_array_upper, false),
            AccountMeta::new_readonly(owner, true),
        ];

        let gen_ix = instructions::update_fees_and_rewards(
            &program_id,
            &instructions::UpdateFeesAndRewardsAccounts {
                position,
                lb_pair,
                bin_array_lower,
                bin_array_upper,
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
    fn test_update_position_operator_instruction() {
        use meteora_dlmm::instructions;
        #[allow(unused_imports)]
        use meteora_dlmm::types;
        use solana_sdk::instruction::AccountMeta;

        let program_id = meteora_dlmm::ID;
        let position = Pubkey::from([1; 32]);
        let owner = Pubkey::from([2; 32]);
        let event_authority = Pubkey::from([3; 32]);
        let program = Pubkey::from([4; 32]);
        let operator = Pubkey::from([0; 32]);

        let mut expected_data: Vec<u8> = vec![202, 184, 103, 143, 180, 191, 116, 217];
        borsh::BorshSerialize::serialize(&operator, &mut expected_data).unwrap();

        let expected_accounts = vec![
            AccountMeta::new(position, false),
            AccountMeta::new_readonly(owner, true),
            AccountMeta::new_readonly(event_authority, false),
            AccountMeta::new_readonly(program, false),
        ];

        let gen_ix = instructions::update_position_operator(
            &program_id,
            &instructions::UpdatePositionOperatorAccounts {
                position,
                owner,
                event_authority,
                program,
            },
            operator,
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
        use meteora_dlmm::instructions;
        #[allow(unused_imports)]
        use meteora_dlmm::types;
        use solana_sdk::instruction::AccountMeta;

        let program_id = meteora_dlmm::ID;
        let lb_pair = Pubkey::from([1; 32]);
        let operator = Pubkey::from([2; 32]);
        let signer = Pubkey::from([3; 32]);
        let bin_array = Pubkey::from([4; 32]);
        let event_authority = Pubkey::from([5; 32]);
        let program = Pubkey::from([6; 32]);
        let reward_index = 0u64;
        let new_duration = 0u64;

        let mut expected_data: Vec<u8> = vec![138, 174, 196, 169, 213, 235, 254, 107];
        borsh::BorshSerialize::serialize(&reward_index, &mut expected_data).unwrap();
        borsh::BorshSerialize::serialize(&new_duration, &mut expected_data).unwrap();

        let expected_accounts = vec![
            AccountMeta::new(lb_pair, false),
            AccountMeta::new_readonly(operator, false),
            AccountMeta::new_readonly(signer, true),
            AccountMeta::new(bin_array, false),
            AccountMeta::new_readonly(event_authority, false),
            AccountMeta::new_readonly(program, false),
        ];

        let gen_ix = instructions::update_reward_duration(
            &program_id,
            &instructions::UpdateRewardDurationAccounts {
                lb_pair,
                operator,
                signer,
                bin_array,
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
        use meteora_dlmm::instructions;
        #[allow(unused_imports)]
        use meteora_dlmm::types;
        use solana_sdk::instruction::AccountMeta;

        let program_id = meteora_dlmm::ID;
        let lb_pair = Pubkey::from([1; 32]);
        let operator = Pubkey::from([2; 32]);
        let signer = Pubkey::from([3; 32]);
        let event_authority = Pubkey::from([4; 32]);
        let program = Pubkey::from([5; 32]);
        let reward_index = 0u64;
        let new_funder = Pubkey::from([0; 32]);

        let mut expected_data: Vec<u8> = vec![211, 28, 48, 32, 215, 160, 35, 23];
        borsh::BorshSerialize::serialize(&reward_index, &mut expected_data).unwrap();
        borsh::BorshSerialize::serialize(&new_funder, &mut expected_data).unwrap();

        let expected_accounts = vec![
            AccountMeta::new(lb_pair, false),
            AccountMeta::new_readonly(operator, false),
            AccountMeta::new_readonly(signer, true),
            AccountMeta::new_readonly(event_authority, false),
            AccountMeta::new_readonly(program, false),
        ];

        let gen_ix = instructions::update_reward_funder(
            &program_id,
            &instructions::UpdateRewardFunderAccounts {
                lb_pair,
                operator,
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
        use meteora_dlmm::instructions;
        #[allow(unused_imports)]
        use meteora_dlmm::types;
        use solana_sdk::instruction::AccountMeta;

        let program_id = meteora_dlmm::ID;
        let lb_pair = Pubkey::from([1; 32]);
        let reward_vault = Pubkey::from([2; 32]);
        let reward_mint = Pubkey::from([3; 32]);
        let funder_token_account = Pubkey::from([4; 32]);
        let funder = Pubkey::from([5; 32]);
        let bin_array = Pubkey::from([6; 32]);
        let token_program = Pubkey::from([7; 32]);
        let memo_program = Pubkey::from([8; 32]);
        let event_authority = Pubkey::from([9; 32]);
        let program = Pubkey::from([10; 32]);
        let reward_index = 0u64;
        let remaining_accounts_info = types::RemainingAccountsInfo { slices: vec![] };

        let mut expected_data: Vec<u8> = vec![148, 206, 42, 195, 247, 49, 103, 8];
        borsh::BorshSerialize::serialize(&reward_index, &mut expected_data).unwrap();
        borsh::BorshSerialize::serialize(&remaining_accounts_info.clone(), &mut expected_data).unwrap();

        let expected_accounts = vec![
            AccountMeta::new(lb_pair, false),
            AccountMeta::new(reward_vault, false),
            AccountMeta::new_readonly(reward_mint, false),
            AccountMeta::new(funder_token_account, false),
            AccountMeta::new_readonly(funder, true),
            AccountMeta::new(bin_array, false),
            AccountMeta::new_readonly(token_program, false),
            AccountMeta::new_readonly(memo_program, false),
            AccountMeta::new_readonly(event_authority, false),
            AccountMeta::new_readonly(program, false),
        ];

        let gen_ix = instructions::withdraw_ineligible_reward(
            &program_id,
            &instructions::WithdrawIneligibleRewardAccounts {
                lb_pair,
                reward_vault,
                reward_mint,
                funder_token_account,
                funder,
                bin_array,
                token_program,
                memo_program,
                event_authority,
                program,
            },
            reward_index,
            remaining_accounts_info.clone(),
        );

        let expected_ix = solana_sdk::instruction::Instruction::new_with_bytes(
            program_id,
            &expected_data,
            expected_accounts,
        );
        assert_ix(&gen_ix, &expected_ix);
    }

    #[test]
    fn test_withdraw_protocol_fee_instruction() {
        use meteora_dlmm::instructions;
        #[allow(unused_imports)]
        use meteora_dlmm::types;
        use solana_sdk::instruction::AccountMeta;

        let program_id = meteora_dlmm::ID;
        let lb_pair = Pubkey::from([1; 32]);
        let reserve_x = Pubkey::from([2; 32]);
        let reserve_y = Pubkey::from([3; 32]);
        let token_x_mint = Pubkey::from([4; 32]);
        let token_y_mint = Pubkey::from([5; 32]);
        let receiver_token_x = Pubkey::from([6; 32]);
        let receiver_token_y = Pubkey::from([7; 32]);
        let operator = Pubkey::from([8; 32]);
        let signer = Pubkey::from([9; 32]);
        let token_x_program = Pubkey::from([10; 32]);
        let token_y_program = Pubkey::from([11; 32]);
        let max_amount_x = 0u64;
        let max_amount_y = 0u64;
        let remaining_accounts_info = types::RemainingAccountsInfo { slices: vec![] };

        let mut expected_data: Vec<u8> = vec![158, 201, 158, 189, 33, 93, 162, 103];
        borsh::BorshSerialize::serialize(&max_amount_x, &mut expected_data).unwrap();
        borsh::BorshSerialize::serialize(&max_amount_y, &mut expected_data).unwrap();
        borsh::BorshSerialize::serialize(&remaining_accounts_info.clone(), &mut expected_data).unwrap();

        let expected_accounts = vec![
            AccountMeta::new(lb_pair, false),
            AccountMeta::new(reserve_x, false),
            AccountMeta::new(reserve_y, false),
            AccountMeta::new_readonly(token_x_mint, false),
            AccountMeta::new_readonly(token_y_mint, false),
            AccountMeta::new(receiver_token_x, false),
            AccountMeta::new(receiver_token_y, false),
            AccountMeta::new_readonly(operator, false),
            AccountMeta::new_readonly(signer, true),
            AccountMeta::new_readonly(token_x_program, false),
            AccountMeta::new_readonly(token_y_program, false),
        ];

        let gen_ix = instructions::withdraw_protocol_fee(
            &program_id,
            &instructions::WithdrawProtocolFeeAccounts {
                lb_pair,
                reserve_x,
                reserve_y,
                token_x_mint,
                token_y_mint,
                receiver_token_x,
                receiver_token_y,
                operator,
                signer,
                token_x_program,
                token_y_program,
            },
            max_amount_x,
            max_amount_y,
            remaining_accounts_info.clone(),
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
        use meteora_dlmm::instructions;
        #[allow(unused_imports)]
        use meteora_dlmm::types;
        use solana_sdk::instruction::AccountMeta;

        let program_id = meteora_dlmm::ID;
        let lb_pair = Pubkey::from([1; 32]);
        let reserve = Pubkey::from([2; 32]);
        let token_mint = Pubkey::from([3; 32]);
        let receiver_token = Pubkey::from([4; 32]);
        let operator = Pubkey::from([5; 32]);
        let signer = Pubkey::from([6; 32]);
        let token_program = Pubkey::from([7; 32]);
        let sysvar_instructions = Pubkey::from([8; 32]);
        let max_amount = 0u64;
        let remaining_accounts_info = types::RemainingAccountsInfo { slices: vec![] };

        let mut expected_data: Vec<u8> = vec![213, 155, 187, 34, 56, 182, 91, 240];
        borsh::BorshSerialize::serialize(&max_amount, &mut expected_data).unwrap();
        borsh::BorshSerialize::serialize(&remaining_accounts_info.clone(), &mut expected_data).unwrap();

        let expected_accounts = vec![
            AccountMeta::new(lb_pair, false),
            AccountMeta::new(reserve, false),
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
                lb_pair,
                reserve,
                token_mint,
                receiver_token,
                operator,
                signer,
                token_program,
                sysvar_instructions,
            },
            max_amount,
            remaining_accounts_info.clone(),
        );

        let expected_ix = solana_sdk::instruction::Instruction::new_with_bytes(
            program_id,
            &expected_data,
            expected_accounts,
        );
        assert_ix(&gen_ix, &expected_ix);
    }

}
