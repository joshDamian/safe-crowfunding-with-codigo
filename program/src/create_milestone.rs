use solana_program::account_info::AccountInfo;
use solana_program::entrypoint::ProgramResult;
use solana_program::pubkey::Pubkey;

use crate::generated::{state::{
	AccountPDA,
	FundingRequest,
	Milestone,
}, errors::SafeCrowdfundingError};


/// Create a new milestone.
///
/// Accounts:
/// 0. `[writable, signer]` fee_payer: [AccountInfo] 
/// 1. `[signer]` creator: [AccountInfo] Account that has permission to create milestone
/// 2. `[writable]` funding_request: [FundingRequest] 
/// 3. `[writable]` milestone: [Milestone] 
/// 4. `[]` system_program: [AccountInfo] Auto-generated, for account initialization
///
/// Data:
/// - name: [String] The name for the milestone
/// - target_amount: [u64] The amount required to complete the milestone
/// - expectation: [String] A short explainer on what to expect
/// - funding_request_seed_beneficiary: [Pubkey] Auto-generated, from input funding_request of type [FundingRequest] set the seed named beneficiary, required by the type
/// - funding_request_seed_index: [u16] Auto-generated, from input funding_request of type [FundingRequest] set the seed named index, required by the type
/// - milestone_seed_index: [u16] Auto-generated, from input milestone of type [Milestone] set the seed named index, required by the type
pub fn create_milestone(
	program_id: &Pubkey,
	creator: &AccountInfo,
	funding_request: &mut AccountPDA<FundingRequest>,
	milestone: &mut AccountPDA<Milestone>,
	name: String,
	target_amount: u64,
	expectation: String,
) -> ProgramResult {
     // Check if the creator is the authorized user by the funding_request
    if funding_request.data.beneficiary != *creator.key {
        return Err(SafeCrowdfundingError::InvalidInstruction.into());
    }

    // Initialize state
    milestone.data.name = name;
    milestone.data.expectation = expectation;
    milestone.data.target_amount = target_amount as u128;
    milestone.data.funding_request = *funding_request.info.key;

    funding_request.data.target += target_amount as u128;

    milestone.data.positive_votes = 0;
    milestone.data.negative_votes = 0;
    milestone.data.deliverable_url = "".to_string();

    milestone.data.level = funding_request.data.milestone_count + 1;      
    
    milestone.data.is_unlocked = false;                                     

    funding_request.data.milestone_count += 1;

    Ok(())
}