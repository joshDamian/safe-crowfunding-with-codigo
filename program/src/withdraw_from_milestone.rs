use solana_program::account_info::AccountInfo;
use solana_program::entrypoint::ProgramResult;
use solana_program::pubkey::Pubkey;

use crate::generated::{state::{
	AccountPDA,
	FundingRequest,
	Milestone,
}, errors::SafeCrowdfundingError};


/// Withdraw funds for a particular milestone
///
/// Accounts:
/// 0. `[writable, signer]` fee_payer: [AccountInfo] 
/// 1. `[writable, signer]` withdrawer: [AccountInfo] 
/// 2. `[writable]` milestone: [Milestone] 
/// 3. `[]` funding_request: [FundingRequest] 
///
/// Data:
/// - amount: [u64] Amount to withdraw
/// - milestone_seed_index: [u16] Auto-generated, from input milestone of type [Milestone] set the seed named index, required by the type
/// - funding_request_seed_index: [u16] Auto-generated, from input funding_request of type [FundingRequest] set the seed named index, required by the type
pub fn withdraw_from_milestone(
	program_id: &Pubkey,
	withdrawer: &AccountInfo,
	milestone: &mut AccountPDA<Milestone>,
	funding_request: &AccountPDA<FundingRequest>,
	amount: u64,
) -> ProgramResult {
    // Implement your business logic here...
    if funding_request.data.beneficiary != *withdrawer.key {
        return Err(SafeCrowdfundingError::InvalidInstruction.into());
    }

    if milestone.data.funding_request != *funding_request.info.key {
        return Err(SafeCrowdfundingError::InvalidInstruction.into());
    }

    **milestone.info.try_borrow_mut_lamports()? -= amount;
    **withdrawer.try_borrow_mut_lamports()? += amount;

    Ok(())
}