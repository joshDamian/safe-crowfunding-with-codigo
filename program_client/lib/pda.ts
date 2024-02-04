// This file is auto-generated from the CIDL source.
// Editing this file directly is not recommended as it may be overwritten.

import {PublicKey} from "@solana/web3.js";

export type FundingRequestSeeds = {
    beneficiary: PublicKey, 
    index: number, 
};

export const deriveFundingRequestPDA = (
    seeds: FundingRequestSeeds,
    programId: PublicKey
): [PublicKey, number] => {
    return PublicKey.findProgramAddressSync(
        [
            Buffer.from("funding_request"),
            seeds.beneficiary.toBuffer(),
            Buffer.from(Uint16Array.from([seeds.index]).buffer),
        ],
        programId,
    )
};

export type FundingRequestDonorSeeds = {
    fundingRequest: PublicKey, 
    owner: PublicKey, 
    index: number, 
};

export const deriveFundingRequestDonorPDA = (
    seeds: FundingRequestDonorSeeds,
    programId: PublicKey
): [PublicKey, number] => {
    return PublicKey.findProgramAddressSync(
        [
            Buffer.from("funding_request_donor"),
            seeds.fundingRequest.toBuffer(),
            seeds.owner.toBuffer(),
            Buffer.from(Uint16Array.from([seeds.index]).buffer),
        ],
        programId,
    )
};

export type MilestoneSeeds = {
    fundingRequest: PublicKey, 
    index: number, 
};

export const deriveMilestonePDA = (
    seeds: MilestoneSeeds,
    programId: PublicKey
): [PublicKey, number] => {
    return PublicKey.findProgramAddressSync(
        [
            Buffer.from("milestone"),
            seeds.fundingRequest.toBuffer(),
            Buffer.from(Uint16Array.from([seeds.index]).buffer),
        ],
        programId,
    )
};

export type MilestoneVoteSeeds = {
    milestone: PublicKey, 
    voter: PublicKey, 
    index: number, 
};

export const deriveMilestoneVotePDA = (
    seeds: MilestoneVoteSeeds,
    programId: PublicKey
): [PublicKey, number] => {
    return PublicKey.findProgramAddressSync(
        [
            Buffer.from("milestone_vote"),
            seeds.milestone.toBuffer(),
            seeds.voter.toBuffer(),
            Buffer.from(Uint16Array.from([seeds.index]).buffer),
        ],
        programId,
    )
};

