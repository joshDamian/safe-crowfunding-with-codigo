use solana_program::account_info::AccountInfo;
use solana_program::entrypoint::ProgramResult;
use solana_program::pubkey::Pubkey;

use crate::generated::{state::{
	AccountPDA,
	FundingRequest,
	FundingRequestDonor,
}, errors::SafeCrowdfundingError};


/// Donate to a funding request.
///
/// Accounts:
/// 0. `[writable, signer]` fee_payer: [AccountInfo] 
/// 1. `[writable, signer]` owner: [AccountInfo] 
/// 2. `[writable]` donor: [FundingRequestDonor] 
/// 3. `[writable]` funding_request: [FundingRequest] 
///
/// Data:
/// - amount: [u64] The amount to donate
/// - donor_seed_funding_request: [Pubkey] Auto-generated, from input donor of type [FundingRequestDonor] set the seed named funding_request, required by the type
/// - donor_seed_owner: [Pubkey] Auto-generated, from input donor of type [FundingRequestDonor] set the seed named owner, required by the type
/// - donor_seed_index: [u16] Auto-generated, from input donor of type [FundingRequestDonor] set the seed named index, required by the type
/// - funding_request_seed_beneficiary: [Pubkey] Auto-generated, from input funding_request of type [FundingRequest] set the seed named beneficiary, required by the type
/// - funding_request_seed_index: [u16] Auto-generated, from input funding_request of type [FundingRequest] set the seed named index, required by the type
pub fn donate_to_funding_request(
	program_id: &Pubkey,
	owner: &AccountInfo,
	donor: &mut AccountPDA<FundingRequestDonor>,
	funding_request: &mut AccountPDA<FundingRequest>,
	amount: u64,
) -> ProgramResult {
    // Check if the owner is the authorized user by the donor account
    if donor.data.owner != *owner.key {
        return Err(SafeCrowdfundingError::InvalidInstruction.into());
    }

    // Update funding request
    funding_request.data.total_raised += amount as u128;
    funding_request.data.available_capital += amount as u128;

    let is_new_donor = donor.data.amount_donated == 0;

    if is_new_donor {
        // update donor count
        funding_request.data.donor_count += 1;
    }

    // Transfer funds
    **owner.try_borrow_mut_lamports()? -= amount;
    **funding_request.info.try_borrow_mut_lamports()? += amount;

    Ok(())
}