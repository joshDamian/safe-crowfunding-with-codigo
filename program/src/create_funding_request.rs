use solana_program::account_info::AccountInfo;
use solana_program::entrypoint::ProgramResult;
use solana_program::pubkey::Pubkey;

use crate::generated::state::{
	AccountPDA,
	FundingRequest,
};


/// Create a new funding request.
///
/// Accounts:
/// 0. `[writable, signer]` fee_payer: [AccountInfo] 
/// 1. `[writable, signer]` beneficiary: [AccountInfo] Account that has permission to update the funding request
/// 2. `[writable]` funding_request: [FundingRequest] 
/// 3. `[]` system_program: [AccountInfo] Auto-generated, for account initialization
///
/// Data:
/// - name: [String] The name for the funding request
/// - description: [String] type
/// - funding_request_seed_beneficiary: [Pubkey] Auto-generated, from input funding_request of type [FundingRequest] set the seed named beneficiary, required by the type
/// - funding_request_seed_index: [u16] Auto-generated, from input funding_request of type [FundingRequest] set the seed named index, required by the type
pub fn create_funding_request(
	program_id: &Pubkey,
	beneficiary: &AccountInfo,
	funding_request: &mut AccountPDA<FundingRequest>,
	name: String,
	description: String,
) -> ProgramResult {
    // Set initial funding request state.
    funding_request.data.beneficiary = *beneficiary.key;
    funding_request.data.name = name;
    funding_request.data.description = description;

    funding_request.data.target = 0;
    funding_request.data.total_raised = 0;
    funding_request.data.donor_count = 0;
    funding_request.data.is_open = true;
    funding_request.data.milestone_count = 0;
    funding_request.data.available_capital = 0;

    Ok(())
}