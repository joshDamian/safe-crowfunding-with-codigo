// This file is auto-generated from the CIDL source.
// Editing this file directly is not recommended as it may be overwritten.

use std::str::FromStr;
use borsh::BorshSerialize;
use solana_program::account_info::{AccountInfo, next_account_info, next_account_infos};
use solana_program::borsh0_10::try_from_slice_unchecked;
use solana_program::entrypoint::ProgramResult;
use solana_program::program::{invoke, invoke_signed};
use solana_program::pubkey::Pubkey;
use solana_program::rent::Rent;
use solana_program::system_instruction::create_account;
use solana_program::{msg, system_program};
use solana_program::sysvar::Sysvar;
use solana_program::program_pack::Pack;
use crate::generated::errors::SafeCrowdfundingError;
use crate::generated::instructions::SafeCrowdfundingInstruction;

use crate::generated::state::{
	Account,
	AccountPDA,
	FundingRequest,
	FundingRequestDonor,
	Milestone,
	MilestoneVote,
};
use crate::src::*;

pub struct Processor;

impl Processor {
    pub fn process(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        data: &[u8],
    ) -> ProgramResult {
        let instruction = SafeCrowdfundingInstruction::unpack(data)?;

        match instruction {
			SafeCrowdfundingInstruction::CreateFundingRequest(args) => {
				msg!("Instruction: CreateFundingRequest");
				Self::process_create_funding_request(
					program_id,
					accounts, 
					args.name,
					args.description,
					args.funding_request_seed_beneficiary,
					args.funding_request_seed_index,
				)
			}
			SafeCrowdfundingInstruction::CreateMilestone(args) => {
				msg!("Instruction: CreateMilestone");
				Self::process_create_milestone(
					program_id,
					accounts, 
					args.name,
					args.target_amount,
					args.expectation,
					args.funding_request_seed_beneficiary,
					args.funding_request_seed_index,
					args.milestone_seed_index,
				)
			}
			SafeCrowdfundingInstruction::UpdateMilestoneDeliverable(args) => {
				msg!("Instruction: UpdateMilestoneDeliverable");
				Self::process_update_milestone_deliverable(
					program_id,
					accounts, 
					args.deliverable_url,
					args.milestone_seed_index,
					args.funding_request_seed_beneficiary,
					args.funding_request_seed_index,
				)
			}
			SafeCrowdfundingInstruction::UnlockMilestone(args) => {
				msg!("Instruction: UnlockMilestone");
				Self::process_unlock_milestone(
					program_id,
					accounts, 
					args.previous_milestone_seed_index,
					args.funding_request_seed_beneficiary,
					args.funding_request_seed_index,
					args.milestone_seed_index,
				)
			}
			SafeCrowdfundingInstruction::WithdrawFromMilestone(args) => {
				msg!("Instruction: WithdrawFromMilestone");
				Self::process_withdraw_from_milestone(
					program_id,
					accounts, 
					args.amount,
					args.milestone_seed_index,
					args.funding_request_seed_index,
				)
			}
			SafeCrowdfundingInstruction::VoteOnMilestone(args) => {
				msg!("Instruction: VoteOnMilestone");
				Self::process_vote_on_milestone(
					program_id,
					accounts, 
					args.confidence,
					args.donor_account_seed_owner,
					args.donor_account_seed_index,
					args.milestone_seed_index,
					args.milestone_vote_seed_voter,
					args.milestone_vote_seed_index,
				)
			}
			SafeCrowdfundingInstruction::CreateDonorAccount(args) => {
				msg!("Instruction: CreateDonorAccount");
				Self::process_create_donor_account(
					program_id,
					accounts, 
					args.funding_request_seed_beneficiary,
					args.funding_request_seed_index,
					args.donor_seed_owner,
					args.donor_seed_index,
				)
			}
			SafeCrowdfundingInstruction::DonateToFundingRequest(args) => {
				msg!("Instruction: DonateToFundingRequest");
				Self::process_donate_to_funding_request(
					program_id,
					accounts, 
					args.amount,
					args.donor_seed_owner,
					args.donor_seed_index,
					args.funding_request_seed_beneficiary,
					args.funding_request_seed_index,
				)
			}
        }
    }

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
	pub fn process_create_funding_request(
		program_id: &Pubkey,
		accounts: &[AccountInfo],
		name: String,
		description: String,
		funding_request_seed_beneficiary: Pubkey,
		funding_request_seed_index: u16,
	) -> ProgramResult {
		let account_info_iter = &mut accounts.iter();
		let fee_payer_info = next_account_info(account_info_iter)?;
		let beneficiary_info = next_account_info(account_info_iter)?;
		let funding_request_info = next_account_info(account_info_iter)?;
		let system_program_info = next_account_info(account_info_iter)?;

		// Derive PDAs
		let (funding_request_pubkey, funding_request_bump) = Pubkey::find_program_address(
			&[b"funding_request", funding_request_seed_beneficiary.as_ref(), funding_request_seed_index.to_le_bytes().as_ref()],
			program_id,
		);

		// Security Checks
		if fee_payer_info.is_signer != true {
			return Err(SafeCrowdfundingError::InvalidSignerPermission.into());
		}

		if beneficiary_info.is_signer != true {
			return Err(SafeCrowdfundingError::InvalidSignerPermission.into());
		}

		if *funding_request_info.key != funding_request_pubkey {
			return Err(SafeCrowdfundingError::NotExpectedAddress.into());
		}

		if *system_program_info.key != Pubkey::from_str("11111111111111111111111111111111").unwrap() {
			return Err(SafeCrowdfundingError::NotExpectedAddress.into());
		}


		// Accounts Initializations
		let space = FundingRequest::LEN;
		let rent = Rent::get()?;
		let rent_minimum_balance = rent.minimum_balance(space);

		invoke_signed(
			&create_account(
				&fee_payer_info.key,
				&funding_request_info.key,
				rent_minimum_balance,
				space as u64,
				program_id,
			),
			&[fee_payer_info.clone(), funding_request_info.clone()],
			&[&[b"funding_request", funding_request_seed_beneficiary.as_ref(), funding_request_seed_index.to_le_bytes().as_ref(), &[funding_request_bump]]],
		)?;


		// Security Checks
		if *fee_payer_info.owner != Pubkey::from_str("11111111111111111111111111111111").unwrap() {
			return Err(SafeCrowdfundingError::WrongAccountOwner.into());
		}

		if funding_request_info.data_len() != FundingRequest::LEN {
			return Err(SafeCrowdfundingError::InvalidAccountLen.into());
		}


		// Accounts Deserialization
		let funding_request = &mut AccountPDA::new(
			&funding_request_info,
			try_from_slice_unchecked::<FundingRequest>(&funding_request_info.data.borrow()).unwrap(),
			funding_request_bump,
		);

		// Calling STUB
		create_funding_request::create_funding_request(
			program_id,
			beneficiary_info,
			funding_request,
			name,
			description,
		)?;

		// Accounts Serialization
		funding_request.data.serialize(&mut &mut funding_request_info.data.borrow_mut()[..])?;
		
		Ok(())
	}

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
	pub fn process_create_milestone(
		program_id: &Pubkey,
		accounts: &[AccountInfo],
		name: String,
		target_amount: u64,
		expectation: String,
		funding_request_seed_beneficiary: Pubkey,
		funding_request_seed_index: u16,
		milestone_seed_index: u16,
	) -> ProgramResult {
		let account_info_iter = &mut accounts.iter();
		let fee_payer_info = next_account_info(account_info_iter)?;
		let creator_info = next_account_info(account_info_iter)?;
		let funding_request_info = next_account_info(account_info_iter)?;
		let milestone_info = next_account_info(account_info_iter)?;
		let system_program_info = next_account_info(account_info_iter)?;

		// Derive PDAs
		let (funding_request_pubkey, funding_request_bump) = Pubkey::find_program_address(
			&[b"funding_request", funding_request_seed_beneficiary.as_ref(), funding_request_seed_index.to_le_bytes().as_ref()],
			program_id,
		);
		let (milestone_pubkey, milestone_bump) = Pubkey::find_program_address(
			&[b"milestone", milestone_seed_index.to_le_bytes().as_ref()],
			program_id,
		);

		// Security Checks
		if fee_payer_info.is_signer != true {
			return Err(SafeCrowdfundingError::InvalidSignerPermission.into());
		}

		if creator_info.is_signer != true {
			return Err(SafeCrowdfundingError::InvalidSignerPermission.into());
		}

		if *funding_request_info.key != funding_request_pubkey {
			return Err(SafeCrowdfundingError::NotExpectedAddress.into());
		}

		if *milestone_info.key != milestone_pubkey {
			return Err(SafeCrowdfundingError::NotExpectedAddress.into());
		}

		if *system_program_info.key != Pubkey::from_str("11111111111111111111111111111111").unwrap() {
			return Err(SafeCrowdfundingError::NotExpectedAddress.into());
		}


		// Accounts Initializations
		let space = Milestone::LEN;
		let rent = Rent::get()?;
		let rent_minimum_balance = rent.minimum_balance(space);

		invoke_signed(
			&create_account(
				&fee_payer_info.key,
				&milestone_info.key,
				rent_minimum_balance,
				space as u64,
				program_id,
			),
			&[fee_payer_info.clone(), milestone_info.clone()],
			&[&[b"milestone", milestone_seed_index.to_le_bytes().as_ref(), &[milestone_bump]]],
		)?;


		// Security Checks
		if *fee_payer_info.owner != Pubkey::from_str("11111111111111111111111111111111").unwrap() {
			return Err(SafeCrowdfundingError::WrongAccountOwner.into());
		}

		if funding_request_info.data_len() != FundingRequest::LEN {
			return Err(SafeCrowdfundingError::InvalidAccountLen.into());
		}

		if milestone_info.data_len() != Milestone::LEN {
			return Err(SafeCrowdfundingError::InvalidAccountLen.into());
		}


		// Accounts Deserialization
		let funding_request = &mut AccountPDA::new(
			&funding_request_info,
			try_from_slice_unchecked::<FundingRequest>(&funding_request_info.data.borrow()).unwrap(),
			funding_request_bump,
		);
		let milestone = &mut AccountPDA::new(
			&milestone_info,
			try_from_slice_unchecked::<Milestone>(&milestone_info.data.borrow()).unwrap(),
			milestone_bump,
		);

		// Calling STUB
		create_milestone::create_milestone(
			program_id,
			creator_info,
			funding_request,
			milestone,
			name,
			target_amount,
			expectation,
		)?;

		// Accounts Serialization
		funding_request.data.serialize(&mut &mut funding_request_info.data.borrow_mut()[..])?;		milestone.data.serialize(&mut &mut milestone_info.data.borrow_mut()[..])?;
		
		Ok(())
	}

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
	pub fn process_update_milestone_deliverable(
		program_id: &Pubkey,
		accounts: &[AccountInfo],
		deliverable_url: String,
		milestone_seed_index: u16,
		funding_request_seed_beneficiary: Pubkey,
		funding_request_seed_index: u16,
	) -> ProgramResult {
		let account_info_iter = &mut accounts.iter();
		let fee_payer_info = next_account_info(account_info_iter)?;
		let updater_info = next_account_info(account_info_iter)?;
		let milestone_info = next_account_info(account_info_iter)?;
		let funding_request_info = next_account_info(account_info_iter)?;

		// Derive PDAs
		let (milestone_pubkey, milestone_bump) = Pubkey::find_program_address(
			&[b"milestone", milestone_seed_index.to_le_bytes().as_ref()],
			program_id,
		);
		let (funding_request_pubkey, funding_request_bump) = Pubkey::find_program_address(
			&[b"funding_request", funding_request_seed_beneficiary.as_ref(), funding_request_seed_index.to_le_bytes().as_ref()],
			program_id,
		);

		// Security Checks
		if fee_payer_info.is_signer != true {
			return Err(SafeCrowdfundingError::InvalidSignerPermission.into());
		}

		if updater_info.is_signer != true {
			return Err(SafeCrowdfundingError::InvalidSignerPermission.into());
		}

		if *milestone_info.key != milestone_pubkey {
			return Err(SafeCrowdfundingError::NotExpectedAddress.into());
		}

		if *funding_request_info.key != funding_request_pubkey {
			return Err(SafeCrowdfundingError::NotExpectedAddress.into());
		}



		// Security Checks
		if *fee_payer_info.owner != Pubkey::from_str("11111111111111111111111111111111").unwrap() {
			return Err(SafeCrowdfundingError::WrongAccountOwner.into());
		}

		if milestone_info.data_len() != Milestone::LEN {
			return Err(SafeCrowdfundingError::InvalidAccountLen.into());
		}

		if funding_request_info.data_len() != FundingRequest::LEN {
			return Err(SafeCrowdfundingError::InvalidAccountLen.into());
		}


		// Accounts Deserialization
		let milestone = &mut AccountPDA::new(
			&milestone_info,
			try_from_slice_unchecked::<Milestone>(&milestone_info.data.borrow()).unwrap(),
			milestone_bump,
		);
		let funding_request = AccountPDA::new(
			&funding_request_info,
			try_from_slice_unchecked::<FundingRequest>(&funding_request_info.data.borrow()).unwrap(),
			funding_request_bump,
		);

		// Calling STUB
		update_milestone_deliverable::update_milestone_deliverable(
			program_id,
			updater_info,
			milestone,
			&funding_request,
			deliverable_url,
		)?;

		// Accounts Serialization
		milestone.data.serialize(&mut &mut milestone_info.data.borrow_mut()[..])?;
		
		Ok(())
	}

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
	pub fn process_unlock_milestone(
		program_id: &Pubkey,
		accounts: &[AccountInfo],
		previous_milestone_seed_index: u16,
		funding_request_seed_beneficiary: Pubkey,
		funding_request_seed_index: u16,
		milestone_seed_index: u16,
	) -> ProgramResult {
		let account_info_iter = &mut accounts.iter();
		let fee_payer_info = next_account_info(account_info_iter)?;
		let unlocker_info = next_account_info(account_info_iter)?;
		let previous_milestone_info = next_account_info(account_info_iter)?;
		let funding_request_info = next_account_info(account_info_iter)?;
		let milestone_info = next_account_info(account_info_iter)?;

		// Derive PDAs
		let (previous_milestone_pubkey, previous_milestone_bump) = Pubkey::find_program_address(
			&[b"milestone", previous_milestone_seed_index.to_le_bytes().as_ref()],
			program_id,
		);
		let (funding_request_pubkey, funding_request_bump) = Pubkey::find_program_address(
			&[b"funding_request", funding_request_seed_beneficiary.as_ref(), funding_request_seed_index.to_le_bytes().as_ref()],
			program_id,
		);
		let (milestone_pubkey, milestone_bump) = Pubkey::find_program_address(
			&[b"milestone", milestone_seed_index.to_le_bytes().as_ref()],
			program_id,
		);

		// Security Checks
		if fee_payer_info.is_signer != true {
			return Err(SafeCrowdfundingError::InvalidSignerPermission.into());
		}

		if unlocker_info.is_signer != true {
			return Err(SafeCrowdfundingError::InvalidSignerPermission.into());
		}

		if *previous_milestone_info.key != previous_milestone_pubkey {
			return Err(SafeCrowdfundingError::NotExpectedAddress.into());
		}

		if *funding_request_info.key != funding_request_pubkey {
			return Err(SafeCrowdfundingError::NotExpectedAddress.into());
		}

		if *milestone_info.key != milestone_pubkey {
			return Err(SafeCrowdfundingError::NotExpectedAddress.into());
		}



		// Security Checks
		if *fee_payer_info.owner != Pubkey::from_str("11111111111111111111111111111111").unwrap() {
			return Err(SafeCrowdfundingError::WrongAccountOwner.into());
		}

		if previous_milestone_info.data_len() != Milestone::LEN {
			return Err(SafeCrowdfundingError::InvalidAccountLen.into());
		}

		if funding_request_info.data_len() != FundingRequest::LEN {
			return Err(SafeCrowdfundingError::InvalidAccountLen.into());
		}

		if milestone_info.data_len() != Milestone::LEN {
			return Err(SafeCrowdfundingError::InvalidAccountLen.into());
		}


		// Accounts Deserialization
		let previous_milestone = AccountPDA::new(
			&previous_milestone_info,
			try_from_slice_unchecked::<Milestone>(&previous_milestone_info.data.borrow()).unwrap(),
			previous_milestone_bump,
		);
		let funding_request = &mut AccountPDA::new(
			&funding_request_info,
			try_from_slice_unchecked::<FundingRequest>(&funding_request_info.data.borrow()).unwrap(),
			funding_request_bump,
		);
		let milestone = &mut AccountPDA::new(
			&milestone_info,
			try_from_slice_unchecked::<Milestone>(&milestone_info.data.borrow()).unwrap(),
			milestone_bump,
		);

		// Calling STUB
		unlock_milestone::unlock_milestone(
			program_id,
			unlocker_info,
			&previous_milestone,
			funding_request,
			milestone,
		)?;

		// Accounts Serialization
		funding_request.data.serialize(&mut &mut funding_request_info.data.borrow_mut()[..])?;		milestone.data.serialize(&mut &mut milestone_info.data.borrow_mut()[..])?;
		
		Ok(())
	}

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
	pub fn process_withdraw_from_milestone(
		program_id: &Pubkey,
		accounts: &[AccountInfo],
		amount: u64,
		milestone_seed_index: u16,
		funding_request_seed_index: u16,
	) -> ProgramResult {
		let account_info_iter = &mut accounts.iter();
		let fee_payer_info = next_account_info(account_info_iter)?;
		let withdrawer_info = next_account_info(account_info_iter)?;
		let milestone_info = next_account_info(account_info_iter)?;
		let funding_request_info = next_account_info(account_info_iter)?;

		// Derive PDAs
		let (milestone_pubkey, milestone_bump) = Pubkey::find_program_address(
			&[b"milestone", milestone_seed_index.to_le_bytes().as_ref()],
			program_id,
		);
		let (funding_request_pubkey, funding_request_bump) = Pubkey::find_program_address(
			&[b"funding_request", withdrawer_info.key.as_ref(), funding_request_seed_index.to_le_bytes().as_ref()],
			program_id,
		);

		// Security Checks
		if fee_payer_info.is_signer != true {
			return Err(SafeCrowdfundingError::InvalidSignerPermission.into());
		}

		if withdrawer_info.is_signer != true {
			return Err(SafeCrowdfundingError::InvalidSignerPermission.into());
		}

		if *milestone_info.key != milestone_pubkey {
			return Err(SafeCrowdfundingError::NotExpectedAddress.into());
		}

		if *funding_request_info.key != funding_request_pubkey {
			return Err(SafeCrowdfundingError::NotExpectedAddress.into());
		}



		// Security Checks
		if *fee_payer_info.owner != Pubkey::from_str("11111111111111111111111111111111").unwrap() {
			return Err(SafeCrowdfundingError::WrongAccountOwner.into());
		}

		if milestone_info.data_len() != Milestone::LEN {
			return Err(SafeCrowdfundingError::InvalidAccountLen.into());
		}

		if funding_request_info.data_len() != FundingRequest::LEN {
			return Err(SafeCrowdfundingError::InvalidAccountLen.into());
		}


		// Accounts Deserialization
		let milestone = &mut AccountPDA::new(
			&milestone_info,
			try_from_slice_unchecked::<Milestone>(&milestone_info.data.borrow()).unwrap(),
			milestone_bump,
		);
		let funding_request = AccountPDA::new(
			&funding_request_info,
			try_from_slice_unchecked::<FundingRequest>(&funding_request_info.data.borrow()).unwrap(),
			funding_request_bump,
		);

		// Calling STUB
		withdraw_from_milestone::withdraw_from_milestone(
			program_id,
			withdrawer_info,
			milestone,
			&funding_request,
			amount,
		)?;

		// Accounts Serialization
		milestone.data.serialize(&mut &mut milestone_info.data.borrow_mut()[..])?;
		
		Ok(())
	}

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
	pub fn process_vote_on_milestone(
		program_id: &Pubkey,
		accounts: &[AccountInfo],
		confidence: bool,
		donor_account_seed_owner: Pubkey,
		donor_account_seed_index: u16,
		milestone_seed_index: u16,
		milestone_vote_seed_voter: Pubkey,
		milestone_vote_seed_index: u16,
	) -> ProgramResult {
		let account_info_iter = &mut accounts.iter();
		let fee_payer_info = next_account_info(account_info_iter)?;
		let voter_info = next_account_info(account_info_iter)?;
		let donor_account_info = next_account_info(account_info_iter)?;
		let milestone_info = next_account_info(account_info_iter)?;
		let milestone_vote_info = next_account_info(account_info_iter)?;
		let system_program_info = next_account_info(account_info_iter)?;

		// Derive PDAs
		let (donor_account_pubkey, donor_account_bump) = Pubkey::find_program_address(
			&[b"funding_request_donor", donor_account_seed_owner.as_ref(), donor_account_seed_index.to_le_bytes().as_ref()],
			program_id,
		);
		let (milestone_pubkey, milestone_bump) = Pubkey::find_program_address(
			&[b"milestone", milestone_seed_index.to_le_bytes().as_ref()],
			program_id,
		);
		let (milestone_vote_pubkey, milestone_vote_bump) = Pubkey::find_program_address(
			&[b"milestone_vote", milestone_vote_seed_voter.as_ref(), milestone_vote_seed_index.to_le_bytes().as_ref()],
			program_id,
		);

		// Security Checks
		if fee_payer_info.is_signer != true {
			return Err(SafeCrowdfundingError::InvalidSignerPermission.into());
		}

		if voter_info.is_signer != true {
			return Err(SafeCrowdfundingError::InvalidSignerPermission.into());
		}

		if *donor_account_info.key != donor_account_pubkey {
			return Err(SafeCrowdfundingError::NotExpectedAddress.into());
		}

		if *milestone_info.key != milestone_pubkey {
			return Err(SafeCrowdfundingError::NotExpectedAddress.into());
		}

		if *milestone_vote_info.key != milestone_vote_pubkey {
			return Err(SafeCrowdfundingError::NotExpectedAddress.into());
		}

		if *system_program_info.key != Pubkey::from_str("11111111111111111111111111111111").unwrap() {
			return Err(SafeCrowdfundingError::NotExpectedAddress.into());
		}


		// Accounts Initializations
		let space = MilestoneVote::LEN;
		let rent = Rent::get()?;
		let rent_minimum_balance = rent.minimum_balance(space);

		invoke_signed(
			&create_account(
				&fee_payer_info.key,
				&milestone_vote_info.key,
				rent_minimum_balance,
				space as u64,
				program_id,
			),
			&[fee_payer_info.clone(), milestone_vote_info.clone()],
			&[&[b"milestone_vote", milestone_vote_seed_voter.as_ref(), milestone_vote_seed_index.to_le_bytes().as_ref(), &[milestone_vote_bump]]],
		)?;


		// Security Checks
		if *fee_payer_info.owner != Pubkey::from_str("11111111111111111111111111111111").unwrap() {
			return Err(SafeCrowdfundingError::WrongAccountOwner.into());
		}

		if donor_account_info.data_len() != FundingRequestDonor::LEN {
			return Err(SafeCrowdfundingError::InvalidAccountLen.into());
		}

		if milestone_info.data_len() != Milestone::LEN {
			return Err(SafeCrowdfundingError::InvalidAccountLen.into());
		}

		if milestone_vote_info.data_len() != MilestoneVote::LEN {
			return Err(SafeCrowdfundingError::InvalidAccountLen.into());
		}


		// Accounts Deserialization
		let donor_account = &mut AccountPDA::new(
			&donor_account_info,
			try_from_slice_unchecked::<FundingRequestDonor>(&donor_account_info.data.borrow()).unwrap(),
			donor_account_bump,
		);
		let milestone = &mut AccountPDA::new(
			&milestone_info,
			try_from_slice_unchecked::<Milestone>(&milestone_info.data.borrow()).unwrap(),
			milestone_bump,
		);
		let milestone_vote = &mut AccountPDA::new(
			&milestone_vote_info,
			try_from_slice_unchecked::<MilestoneVote>(&milestone_vote_info.data.borrow()).unwrap(),
			milestone_vote_bump,
		);

		// Calling STUB
		vote_on_milestone::vote_on_milestone(
			program_id,
			voter_info,
			donor_account,
			milestone,
			milestone_vote,
			confidence,
		)?;

		// Accounts Serialization
		donor_account.data.serialize(&mut &mut donor_account_info.data.borrow_mut()[..])?;		milestone.data.serialize(&mut &mut milestone_info.data.borrow_mut()[..])?;		milestone_vote.data.serialize(&mut &mut milestone_vote_info.data.borrow_mut()[..])?;
		
		Ok(())
	}

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
	pub fn process_create_donor_account(
		program_id: &Pubkey,
		accounts: &[AccountInfo],
		funding_request_seed_beneficiary: Pubkey,
		funding_request_seed_index: u16,
		donor_seed_owner: Pubkey,
		donor_seed_index: u16,
	) -> ProgramResult {
		let account_info_iter = &mut accounts.iter();
		let fee_payer_info = next_account_info(account_info_iter)?;
		let owner_info = next_account_info(account_info_iter)?;
		let funding_request_info = next_account_info(account_info_iter)?;
		let donor_info = next_account_info(account_info_iter)?;
		let system_program_info = next_account_info(account_info_iter)?;

		// Derive PDAs
		let (funding_request_pubkey, funding_request_bump) = Pubkey::find_program_address(
			&[b"funding_request", funding_request_seed_beneficiary.as_ref(), funding_request_seed_index.to_le_bytes().as_ref()],
			program_id,
		);
		let (donor_pubkey, donor_bump) = Pubkey::find_program_address(
			&[b"funding_request_donor", donor_seed_owner.as_ref(), donor_seed_index.to_le_bytes().as_ref()],
			program_id,
		);

		// Security Checks
		if fee_payer_info.is_signer != true {
			return Err(SafeCrowdfundingError::InvalidSignerPermission.into());
		}

		if owner_info.is_signer != true {
			return Err(SafeCrowdfundingError::InvalidSignerPermission.into());
		}

		if *funding_request_info.key != funding_request_pubkey {
			return Err(SafeCrowdfundingError::NotExpectedAddress.into());
		}

		if *donor_info.key != donor_pubkey {
			return Err(SafeCrowdfundingError::NotExpectedAddress.into());
		}

		if *system_program_info.key != Pubkey::from_str("11111111111111111111111111111111").unwrap() {
			return Err(SafeCrowdfundingError::NotExpectedAddress.into());
		}


		// Accounts Initializations
		let space = FundingRequestDonor::LEN;
		let rent = Rent::get()?;
		let rent_minimum_balance = rent.minimum_balance(space);

		invoke_signed(
			&create_account(
				&fee_payer_info.key,
				&donor_info.key,
				rent_minimum_balance,
				space as u64,
				program_id,
			),
			&[fee_payer_info.clone(), donor_info.clone()],
			&[&[b"funding_request_donor", donor_seed_owner.as_ref(), donor_seed_index.to_le_bytes().as_ref(), &[donor_bump]]],
		)?;


		// Security Checks
		if *fee_payer_info.owner != Pubkey::from_str("11111111111111111111111111111111").unwrap() {
			return Err(SafeCrowdfundingError::WrongAccountOwner.into());
		}

		if funding_request_info.data_len() != FundingRequest::LEN {
			return Err(SafeCrowdfundingError::InvalidAccountLen.into());
		}

		if donor_info.data_len() != FundingRequestDonor::LEN {
			return Err(SafeCrowdfundingError::InvalidAccountLen.into());
		}


		// Accounts Deserialization
		let funding_request = AccountPDA::new(
			&funding_request_info,
			try_from_slice_unchecked::<FundingRequest>(&funding_request_info.data.borrow()).unwrap(),
			funding_request_bump,
		);
		let donor = &mut AccountPDA::new(
			&donor_info,
			try_from_slice_unchecked::<FundingRequestDonor>(&donor_info.data.borrow()).unwrap(),
			donor_bump,
		);

		// Calling STUB
		create_donor_account::create_donor_account(
			program_id,
			owner_info,
			&funding_request,
			donor,
		)?;

		// Accounts Serialization
		donor.data.serialize(&mut &mut donor_info.data.borrow_mut()[..])?;
		
		Ok(())
	}

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
	pub fn process_donate_to_funding_request(
		program_id: &Pubkey,
		accounts: &[AccountInfo],
		amount: u64,
		donor_seed_owner: Pubkey,
		donor_seed_index: u16,
		funding_request_seed_beneficiary: Pubkey,
		funding_request_seed_index: u16,
	) -> ProgramResult {
		let account_info_iter = &mut accounts.iter();
		let fee_payer_info = next_account_info(account_info_iter)?;
		let owner_info = next_account_info(account_info_iter)?;
		let donor_info = next_account_info(account_info_iter)?;
		let funding_request_info = next_account_info(account_info_iter)?;

		// Derive PDAs
		let (donor_pubkey, donor_bump) = Pubkey::find_program_address(
			&[b"funding_request_donor", donor_seed_owner.as_ref(), donor_seed_index.to_le_bytes().as_ref()],
			program_id,
		);
		let (funding_request_pubkey, funding_request_bump) = Pubkey::find_program_address(
			&[b"funding_request", funding_request_seed_beneficiary.as_ref(), funding_request_seed_index.to_le_bytes().as_ref()],
			program_id,
		);

		// Security Checks
		if fee_payer_info.is_signer != true {
			return Err(SafeCrowdfundingError::InvalidSignerPermission.into());
		}

		if owner_info.is_signer != true {
			return Err(SafeCrowdfundingError::InvalidSignerPermission.into());
		}

		if *donor_info.key != donor_pubkey {
			return Err(SafeCrowdfundingError::NotExpectedAddress.into());
		}

		if *funding_request_info.key != funding_request_pubkey {
			return Err(SafeCrowdfundingError::NotExpectedAddress.into());
		}



		// Security Checks
		if *fee_payer_info.owner != Pubkey::from_str("11111111111111111111111111111111").unwrap() {
			return Err(SafeCrowdfundingError::WrongAccountOwner.into());
		}

		if donor_info.data_len() != FundingRequestDonor::LEN {
			return Err(SafeCrowdfundingError::InvalidAccountLen.into());
		}

		if funding_request_info.data_len() != FundingRequest::LEN {
			return Err(SafeCrowdfundingError::InvalidAccountLen.into());
		}


		// Accounts Deserialization
		let donor = &mut AccountPDA::new(
			&donor_info,
			try_from_slice_unchecked::<FundingRequestDonor>(&donor_info.data.borrow()).unwrap(),
			donor_bump,
		);
		let funding_request = &mut AccountPDA::new(
			&funding_request_info,
			try_from_slice_unchecked::<FundingRequest>(&funding_request_info.data.borrow()).unwrap(),
			funding_request_bump,
		);

		// Calling STUB
		donate_to_funding_request::donate_to_funding_request(
			program_id,
			owner_info,
			donor,
			funding_request,
			amount,
		)?;

		// Accounts Serialization
		donor.data.serialize(&mut &mut donor_info.data.borrow_mut()[..])?;		funding_request.data.serialize(&mut &mut funding_request_info.data.borrow_mut()[..])?;
		
		Ok(())
	}
}