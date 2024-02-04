// This file is auto-generated from the CIDL source.
// Editing this file directly is not recommended as it may be overwritten.

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::account_info::AccountInfo;
use solana_program::pubkey::Pubkey;

#[derive(Clone, Debug)]
pub struct Account<'a, 'b, T> {
    pub data: T,
    pub info: &'a AccountInfo<'b>,
}

#[derive(Clone, Debug)]
pub struct AccountPDA<'a, 'b, T> {
    pub data: T,
    pub info: &'a AccountInfo<'b>,
    pub bump: u8,
}

impl<'a, 'b, T> Account<'a, 'b, T> {
    pub fn new(info: &'a AccountInfo<'b>, data: T) -> Self {
        Self { data, info }
    }
}

impl<'a, 'b, T> AccountPDA<'a, 'b, T> {
    pub fn new(info: &'a AccountInfo<'b>, data: T, bump: u8) -> Self {
        Self { data, info, bump }
    }
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, Default)]
pub struct FundingRequest {
	pub beneficiary: Pubkey,
	pub target: u128,
	pub total_raised: u128,
	pub available_capital: u128,
	pub name: String,
	pub is_open: bool,
	pub milestone_count: u16,
	pub metadata_url: String,
	pub donor_count: u64,
}

impl FundingRequest {
	pub const LEN: usize = 291; 
	}

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, Default)]
pub struct FundingRequestDonor {
	pub owner: Pubkey,
	pub amount_donated: u128,
	pub funding_request: Pubkey,
}

impl FundingRequestDonor {
	pub const LEN: usize = 80; 
	}

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, Default)]
pub struct Milestone {
	pub name: String,
	pub positive_votes: u128,
	pub negative_votes: u128,
	pub target_amount: u128,
	pub expectation: String,
	pub deliverable_url: String,
	pub funding_request: Pubkey,
	pub level: u16,
	pub is_unlocked: bool,
}

impl Milestone {
	pub const LEN: usize = 542; 
	}

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, Default)]
pub struct MilestoneVote {
	pub milestone: Pubkey,
	pub voter: Pubkey,
	pub confidence: bool,
	pub voted: bool,
}

impl MilestoneVote {
	pub const LEN: usize = 66; 
	}

