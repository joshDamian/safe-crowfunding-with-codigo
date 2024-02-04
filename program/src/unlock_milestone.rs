use solana_program::account_info::AccountInfo;
use solana_program::entrypoint::ProgramResult;
use solana_program::pubkey::Pubkey;

use crate::generated::{state::{
	AccountPDA,
	FundingRequest,
	Milestone,
}, errors::SafeCrowdfundingError};


/// Unlock allocated funds for milestone.
///
/// Accounts:
/// 0. `[writable, signer]` fee_payer: [AccountInfo] 
/// 1. `[writable, signer]` unlocker: [AccountInfo] 
/// 2. `[]` previous_milestone: [Milestone] 
/// 3. `[writable]` funding_request: [FundingRequest] 
/// 4. `[writable]` milestone: [Milestone] 
///
/// Data:
/// - previous_milestone_seed_funding_request: [Pubkey] Auto-generated, from input previous_milestone of type [Milestone] set the seed named funding_request, required by the type
/// - previous_milestone_seed_index: [u16] Auto-generated, from input previous_milestone of type [Milestone] set the seed named index, required by the type
/// - funding_request_seed_beneficiary: [Pubkey] Auto-generated, from input funding_request of type [FundingRequest] set the seed named beneficiary, required by the type
/// - funding_request_seed_index: [u16] Auto-generated, from input funding_request of type [FundingRequest] set the seed named index, required by the type
/// - milestone_seed_funding_request: [Pubkey] Auto-generated, from input milestone of type [Milestone] set the seed named funding_request, required by the type
/// - milestone_seed_index: [u16] Auto-generated, from input milestone of type [Milestone] set the seed named index, required by the type
pub fn unlock_milestone(
	program_id: &Pubkey,
	unlocker: &AccountInfo,
	previous_milestone: &AccountPDA<Milestone>,
	funding_request: &mut AccountPDA<FundingRequest>,
	milestone: &mut AccountPDA<Milestone>,
) -> ProgramResult {
    // Checks 
    if funding_request.data.beneficiary != *unlocker.key {
        return Err(SafeCrowdfundingError::InvalidInstruction.into());
    }

    if milestone.data.funding_request != *funding_request.info.key {
        return Err(SafeCrowdfundingError::InvalidInstruction.into());
    }

    if previous_milestone.data.funding_request != *funding_request.info.key {
        return Err(SafeCrowdfundingError::InvalidInstruction.into());
    }

    let is_first_milestone = milestone.data.level == 1;

    if !is_first_milestone {
        let donor_count = funding_request.data.donor_count;
    
        let prev_milestone_number_of_votes = previous_milestone.data.positive_votes + previous_milestone.data.negative_votes;
    
        let has_min_required_votes = prev_milestone_number_of_votes >= (0.50 * donor_count as f64) as u128;
    
        if !has_min_required_votes {
            return Err(SafeCrowdfundingError::InvalidInstruction.into());
        }
    
        let has_required_positive_votes = (previous_milestone.data.positive_votes as f64 / prev_milestone_number_of_votes as f64) >= 0.50;
    
        if !has_required_positive_votes {
            return Err(SafeCrowdfundingError::InvalidInstruction.into());
        }
    }

    // Can unlock next milestone 
    
    // Check if funding request has enough funds 
    if funding_request.data.available_capital.saturating_sub(milestone.data.target_amount) <= 0 {
        return Err(solana_program::program_error::ProgramError::InsufficientFunds);
    }

    funding_request.data.available_capital -= milestone.data.target_amount;

    // Move funds to milestone account 
    **funding_request.info.try_borrow_mut_lamports()? -= milestone.data.target_amount as u64;
    **milestone.info.try_borrow_mut_lamports()? += milestone.data.target_amount as u64;
    
    milestone.data.is_unlocked = true;

    Ok(())
}