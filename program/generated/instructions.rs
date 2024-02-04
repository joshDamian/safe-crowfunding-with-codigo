// This file is auto-generated from the CIDL source.
// Editing this file directly is not recommended as it may be overwritten.

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;
use crate::generated::errors::SafeCrowdfundingError;

#[derive(BorshSerialize, Debug)]
pub enum SafeCrowdfundingInstruction {
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
	CreateFundingRequest(CreateFundingRequestArgs),

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
	CreateMilestone(CreateMilestoneArgs),

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
	UpdateMilestoneDeliverable(UpdateMilestoneDeliverableArgs),

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
/// - previous_milestone_seed_index: [u16] Auto-generated, from input previous_milestone of type [Milestone] set the seed named index, required by the type
/// - funding_request_seed_beneficiary: [Pubkey] Auto-generated, from input funding_request of type [FundingRequest] set the seed named beneficiary, required by the type
/// - funding_request_seed_index: [u16] Auto-generated, from input funding_request of type [FundingRequest] set the seed named index, required by the type
/// - milestone_seed_index: [u16] Auto-generated, from input milestone of type [Milestone] set the seed named index, required by the type
	UnlockMilestone(UnlockMilestoneArgs),

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
	WithdrawFromMilestone(WithdrawFromMilestoneArgs),

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
/// - donor_account_seed_owner: [Pubkey] Auto-generated, from input donor_account of type [FundingRequestDonor] set the seed named owner, required by the type
/// - donor_account_seed_index: [u16] Auto-generated, from input donor_account of type [FundingRequestDonor] set the seed named index, required by the type
/// - milestone_seed_index: [u16] Auto-generated, from input milestone of type [Milestone] set the seed named index, required by the type
/// - milestone_vote_seed_voter: [Pubkey] Auto-generated, from input milestone_vote of type [MilestoneVote] set the seed named voter, required by the type
/// - milestone_vote_seed_index: [u16] Auto-generated, from input milestone_vote of type [MilestoneVote] set the seed named index, required by the type
	VoteOnMilestone(VoteOnMilestoneArgs),

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
	CreateDonorAccount(CreateDonorAccountArgs),

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
/// - donor_seed_owner: [Pubkey] Auto-generated, from input donor of type [FundingRequestDonor] set the seed named owner, required by the type
/// - donor_seed_index: [u16] Auto-generated, from input donor of type [FundingRequestDonor] set the seed named index, required by the type
/// - funding_request_seed_beneficiary: [Pubkey] Auto-generated, from input funding_request of type [FundingRequest] set the seed named beneficiary, required by the type
/// - funding_request_seed_index: [u16] Auto-generated, from input funding_request of type [FundingRequest] set the seed named index, required by the type
	DonateToFundingRequest(DonateToFundingRequestArgs),

}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct CreateFundingRequestArgs {
	pub name: String,
	pub description: String,
	pub funding_request_seed_beneficiary: Pubkey,
	pub funding_request_seed_index: u16,
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct CreateMilestoneArgs {
	pub name: String,
	pub target_amount: u64,
	pub expectation: String,
	pub funding_request_seed_beneficiary: Pubkey,
	pub funding_request_seed_index: u16,
	pub milestone_seed_index: u16,
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct UpdateMilestoneDeliverableArgs {
	pub deliverable_url: String,
	pub milestone_seed_index: u16,
	pub funding_request_seed_beneficiary: Pubkey,
	pub funding_request_seed_index: u16,
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct UnlockMilestoneArgs {
	pub previous_milestone_seed_index: u16,
	pub funding_request_seed_beneficiary: Pubkey,
	pub funding_request_seed_index: u16,
	pub milestone_seed_index: u16,
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct WithdrawFromMilestoneArgs {
	pub amount: u64,
	pub milestone_seed_index: u16,
	pub funding_request_seed_index: u16,
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct VoteOnMilestoneArgs {
	pub confidence: bool,
	pub donor_account_seed_owner: Pubkey,
	pub donor_account_seed_index: u16,
	pub milestone_seed_index: u16,
	pub milestone_vote_seed_voter: Pubkey,
	pub milestone_vote_seed_index: u16,
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct CreateDonorAccountArgs {
	pub funding_request_seed_beneficiary: Pubkey,
	pub funding_request_seed_index: u16,
	pub donor_seed_owner: Pubkey,
	pub donor_seed_index: u16,
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct DonateToFundingRequestArgs {
	pub amount: u64,
	pub donor_seed_owner: Pubkey,
	pub donor_seed_index: u16,
	pub funding_request_seed_beneficiary: Pubkey,
	pub funding_request_seed_index: u16,
}

impl SafeCrowdfundingInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (&variant, rest) = input.split_first().ok_or(SafeCrowdfundingError::InvalidInstruction)?;

        Ok(match variant {
			0 => Self::CreateFundingRequest(CreateFundingRequestArgs::try_from_slice(rest).unwrap()),
			1 => Self::CreateMilestone(CreateMilestoneArgs::try_from_slice(rest).unwrap()),
			2 => Self::UpdateMilestoneDeliverable(UpdateMilestoneDeliverableArgs::try_from_slice(rest).unwrap()),
			3 => Self::UnlockMilestone(UnlockMilestoneArgs::try_from_slice(rest).unwrap()),
			4 => Self::WithdrawFromMilestone(WithdrawFromMilestoneArgs::try_from_slice(rest).unwrap()),
			5 => Self::VoteOnMilestone(VoteOnMilestoneArgs::try_from_slice(rest).unwrap()),
			6 => Self::CreateDonorAccount(CreateDonorAccountArgs::try_from_slice(rest).unwrap()),
			7 => Self::DonateToFundingRequest(DonateToFundingRequestArgs::try_from_slice(rest).unwrap()),
			_ => return Err(SafeCrowdfundingError::InvalidInstruction.into())
        })
    }
}