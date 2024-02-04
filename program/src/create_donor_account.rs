use solana_program::account_info::AccountInfo;
use solana_program::entrypoint::ProgramResult;
use solana_program::pubkey::Pubkey;

use crate::generated::state::{
	AccountPDA,
	FundingRequest,
	FundingRequestDonor,
};


/// create a donor account for donating to a funding request.
///
/// Accounts:
/// 0. `[writable, signer]` fee_payer: [AccountInfo] 
/// 1. `[writable, signer]` owner: [AccountInfo] 
/// 2. `[]` funding_request: [FundingRequest] 
/// 3. `[writable]` donor: [FundingRequestDonor] 
/// 4. `[]` system_program: [AccountInfo] Auto-generated, for account initialization
///
/// Data:
/// - funding_request_seed_beneficiary: [Pubkey] Auto-generated, from input funding_request of type [FundingRequest] set the seed named beneficiary, required by the type
/// - funding_request_seed_index: [u16] Auto-generated, from input funding_request of type [FundingRequest] set the seed named index, required by the type
/// - donor_seed_owner: [Pubkey] Auto-generated, from input donor of type [FundingRequestDonor] set the seed named owner, required by the type
/// - donor_seed_index: [u16] Auto-generated, from input donor of type [FundingRequestDonor] set the seed named index, required by the type
pub fn create_donor_account(
	program_id: &Pubkey,
	owner: &AccountInfo,
	funding_request: &AccountPDA<FundingRequest>,
	donor: &mut AccountPDA<FundingRequestDonor>,
) -> ProgramResult {
    // Create donor account

    donor.data.owner = *owner.key;
    donor.data.funding_request = *funding_request.info.key;


    Ok(())
}