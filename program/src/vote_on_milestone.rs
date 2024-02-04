use solana_program::account_info::AccountInfo;
use solana_program::entrypoint::ProgramResult;
use solana_program::pubkey::Pubkey;

use crate::generated::{state::{
	AccountPDA,
	FundingRequestDonor,
	Milestone,
	MilestoneVote,
}, errors::SafeCrowdfundingError};


/// Vote on a milestone.
///
/// Accounts:
/// 0. `[writable, signer]` fee_payer: [AccountInfo] 
/// 1. `[writable, signer]` voter: [AccountInfo] 
/// 2. `[writable]` donor_account: [FundingRequestDonor] 
/// 3. `[writable]` milestone: [Milestone] 
/// 4. `[writable]` milestone_vote: [MilestoneVote] 
/// 5. `[]` system_program: [AccountInfo] Auto-generated, for account initialization
///
/// Data:
/// - confidence: [bool] A bool representing the confidence of the vote
/// - donor_account_seed_funding_request: [Pubkey] Auto-generated, from input donor_account of type [FundingRequestDonor] set the seed named funding_request, required by the type
/// - donor_account_seed_owner: [Pubkey] Auto-generated, from input donor_account of type [FundingRequestDonor] set the seed named owner, required by the type
/// - donor_account_seed_index: [u16] Auto-generated, from input donor_account of type [FundingRequestDonor] set the seed named index, required by the type
/// - milestone_seed_funding_request: [Pubkey] Auto-generated, from input milestone of type [Milestone] set the seed named funding_request, required by the type
/// - milestone_seed_index: [u16] Auto-generated, from input milestone of type [Milestone] set the seed named index, required by the type
/// - milestone_vote_seed_voter: [Pubkey] Auto-generated, from input milestone_vote of type [MilestoneVote] set the seed named voter, required by the type
/// - milestone_vote_seed_index: [u16] Auto-generated, from input milestone_vote of type [MilestoneVote] set the seed named index, required by the type
pub fn vote_on_milestone(
	program_id: &Pubkey,
	voter: &AccountInfo,
	donor_account: &mut AccountPDA<FundingRequestDonor>,
	milestone: &mut AccountPDA<Milestone>,
	milestone_vote: &mut AccountPDA<MilestoneVote>,
	confidence: bool,
) -> ProgramResult {
    if donor_account.data.owner != *voter.key {
        return Err(SafeCrowdfundingError::InvalidInstruction.into());
    }

    if milestone.data.funding_request != donor_account.data.funding_request {
        return Err(SafeCrowdfundingError::InvalidInstruction.into());
    }

    milestone_vote.data.voter = *voter.key;
    milestone_vote.data.confidence = confidence;

    if !milestone_vote.data.voted {
        if confidence {
            milestone.data.positive_votes += 1;
        } else {
            milestone.data.negative_votes += 1;
        }

        milestone_vote.data.voted = true;
    }

    Ok(())
}