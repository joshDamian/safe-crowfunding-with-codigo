// This file is auto-generated from the CIDL source.
// Editing this file directly is not recommended as it may be overwritten.

import type {Schema} from 'borsh';
import type {Decoded} from "./utils";
import {PublicKey} from "@solana/web3.js";
import { deserialize } from "borsh";

export interface FundingRequest {
  beneficiary: PublicKey;
  target: bigint;
  totalRaised: bigint;
  availableCapital: bigint;
  name: string;
  isOpen: boolean;
  milestoneCount: number;
  metadataUrl: string;
  donorCount: bigint;
}

export const decodeFundingRequest = (decoded: Decoded): FundingRequest => ({
    beneficiary: new PublicKey(decoded["beneficiary"] as Uint8Array),
    target: decoded["target"] as bigint,
    totalRaised: decoded["total_raised"] as bigint,
    availableCapital: decoded["available_capital"] as bigint,
    name: decoded["name"] as string,
    isOpen: decoded["is_open"] as boolean,
    milestoneCount: decoded["milestone_count"] as number,
    metadataUrl: decoded["metadata_url"] as string,
    donorCount: decoded["donor_count"] as bigint,
});

export const FundingRequestSchema: Schema =  {
    struct: {
        beneficiary: { array: { type: "u8", len: 32 } },
        target: "u128",
        total_raised: "u128",
        available_capital: "u128",
        name: "string",
        is_open: "bool",
        milestone_count: "u16",
        metadata_url: "string",
        donor_count: "u64",
    }
};

export interface FundingRequestDonor {
  owner: PublicKey;
  amountDonated: bigint;
  fundingRequest: PublicKey;
}

export const decodeFundingRequestDonor = (decoded: Decoded): FundingRequestDonor => ({
    owner: new PublicKey(decoded["owner"] as Uint8Array),
    amountDonated: decoded["amount_donated"] as bigint,
    fundingRequest: new PublicKey(decoded["funding_request"] as Uint8Array),
});

export const FundingRequestDonorSchema: Schema =  {
    struct: {
        owner: { array: { type: "u8", len: 32 } },
        amount_donated: "u128",
        funding_request: { array: { type: "u8", len: 32 } },
    }
};

export interface Milestone {
  name: string;
  positiveVotes: bigint;
  negativeVotes: bigint;
  targetAmount: bigint;
  expectation: string;
  deliverableUrl: string;
  fundingRequest: PublicKey;
  level: number;
  isUnlocked: boolean;
}

export const decodeMilestone = (decoded: Decoded): Milestone => ({
    name: decoded["name"] as string,
    positiveVotes: decoded["positive_votes"] as bigint,
    negativeVotes: decoded["negative_votes"] as bigint,
    targetAmount: decoded["target_amount"] as bigint,
    expectation: decoded["expectation"] as string,
    deliverableUrl: decoded["deliverable_url"] as string,
    fundingRequest: new PublicKey(decoded["funding_request"] as Uint8Array),
    level: decoded["level"] as number,
    isUnlocked: decoded["is_unlocked"] as boolean,
});

export const MilestoneSchema: Schema =  {
    struct: {
        name: "string",
        positive_votes: "u128",
        negative_votes: "u128",
        target_amount: "u128",
        expectation: "string",
        deliverable_url: "string",
        funding_request: { array: { type: "u8", len: 32 } },
        level: "u16",
        is_unlocked: "bool",
    }
};

export interface MilestoneVote {
  milestone: PublicKey;
  voter: PublicKey;
  confidence: boolean;
  voted: boolean;
}

export const decodeMilestoneVote = (decoded: Decoded): MilestoneVote => ({
    milestone: new PublicKey(decoded["milestone"] as Uint8Array),
    voter: new PublicKey(decoded["voter"] as Uint8Array),
    confidence: decoded["confidence"] as boolean,
    voted: decoded["voted"] as boolean,
});

export const MilestoneVoteSchema: Schema =  {
    struct: {
        milestone: { array: { type: "u8", len: 32 } },
        voter: { array: { type: "u8", len: 32 } },
        confidence: "bool",
        voted: "bool",
    }
};



