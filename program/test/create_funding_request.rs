#![cfg(feature = "test-sbf")]
mod create_funding_request {
    use assert_matches::assert_matches;
    use solana_program::instruction::{AccountMeta, Instruction};
    use solana_program::pubkey::Pubkey;
    use solana_program::{system_instruction, system_program};
    use solana_rpc_client::rpc_client::RpcClient;
    use solana_sdk::signer::keypair::Keypair;
    use solana_sdk::signer::Signer;
    use solana_sdk::transaction::Transaction;
    use solana_validator::test_validator::*;
    use std::str::FromStr;

    use crate::generated::entrypoint::process_instruction;
    use crate::generated::instructions::{ SafeCrowdfundingInstruction };
    use crate::test::create_account;

    #[test]
	fn test_transaction_is_ok() {
	    solana_logger::setup_with_default("solana_program_runtime=debug");
        let program_id = Pubkey::new_unique();

		let (test_validator, fee_payer_info) = TestValidatorGenesis::default()
			.add_program("safe_crowdfunding", program_id)
			.start();
		let rpc_client = test_validator.get_rpc_client();

		let name: String = Default::default();
		let metadata_url: String = Default::default();
		let funding_request_seed_beneficiary: Pubkey = Pubkey::new_unique();
		let funding_request_seed_index: u16 = Default::default();

		let beneficiary_info = Keypair::new();
		let (funding_request_pubkey, _) = Pubkey::find_program_address(
			&[b"funding_request", funding_request_seed_beneficiary.as_ref(), funding_request_seed_index.to_le_bytes().as_ref()],
			&program_id,
		);
		let accounts = vec![
			AccountMeta::new(fee_payer_info.pubkey(), true),
			AccountMeta::new(beneficiary_info.pubkey(), true),
			AccountMeta::new(funding_request_pubkey, false),
			AccountMeta::new_readonly(system_program::id(), false),
		];
		let data = SafeCrowdfundingInstruction::CreateFundingRequest(crate::generated::instructions::CreateFundingRequestArgs{
			name,
			metadata_url,
			funding_request_seed_beneficiary,
			funding_request_seed_index,
		});
		let signers = vec![
			&fee_payer_info,
			&beneficiary_info,
        ];

		create_account(&rpc_client, &fee_payer_info, &beneficiary_info, &program_id, 0);

        let instruction = Instruction::new_with_borsh(program_id, &data, accounts.to_vec());
		let recent_blockhash = rpc_client.get_latest_blockhash().unwrap();
        let mut transaction = Transaction::new_with_payer(
            &[instruction],
            Some(&fee_payer_info.pubkey()),
        );
        transaction.sign(&signers, recent_blockhash);

	    assert_matches!(rpc_client.send_and_confirm_transaction(&transaction), Ok(_));
    }
}