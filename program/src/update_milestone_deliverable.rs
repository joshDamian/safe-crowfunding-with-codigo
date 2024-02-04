use solana_program::account_info::AccountInfo;
use solana_program::entrypoint::ProgramResult;
use solana_program::pubkey::Pubkey;

use crate::generated::{state::{
	AccountPDA,
	FundingRequest,
	Milestone,
}, errors::SafeCrowdfundingError};


/// Update an existing milestone with deliverable URL.
///
/// Accounts:
/// 0. `[writable, signer]` fee_payer: [AccountInfo] 
/// 1. `[signer]` updater: [AccountInfo] 
/// 2. `[writable]` milestone: [Milestone] 
/// 3. `[]` funding_request: [FundingRequest] 
///
/// Data:
/// - deliverable_url: [String] URL pointing to what was delivered
/// - milestone_seed_index: [u16] Auto-generated, from input milestone of type [Milestone] set the seed named index, required by the type
/// - funding_request_seed_beneficiary: [Pubkey] Auto-generated, from input funding_request of type [FundingRequest] set the seed named beneficiary, required by the type
/// - funding_request_seed_index: [u16] Auto-generated, from input funding_request of type [FundingRequest] set the seed named index, required by the type
pub fn update_milestone_deliverable(
	program_id: &Pubkey,
	updater: &AccountInfo,
	milestone: &mut AccountPDA<Milestone>,
	funding_request: &AccountPDA<FundingRequest>,
	deliverable_url: String,
) -> ProgramResult {
    // Implement your business logic here...
    if funding_request.data.beneficiary != *updater.key {
        return Err(SafeCrowdfundingError::InvalidInstruction.into());
    }

    if milestone.data.funding_request != *funding_request.info.key {
        return Err(SafeCrowdfundingError::InvalidInstruction.into());
    }

    milestone.data.deliverable_url = deliverable_url;

    Ok(())
}