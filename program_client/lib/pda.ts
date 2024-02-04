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
            seeds.owner.toBuffer(),
            Buffer.from(Uint16Array.from([seeds.index]).buffer),
        ],
        programId,
    )
};

export type MilestoneSeeds = {
    index: number, 
};

export const deriveMilestonePDA = (
    seeds: MilestoneSeeds,
    programId: PublicKey
): [PublicKey, number] => {
    return PublicKey.findProgramAddressSync(
        [
            Buffer.from("milestone"),
            Buffer.from(Uint16Array.from([seeds.index]).buffer),
        ],
        programId,
    )
};

export type MilestoneVoteSeeds = {
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
            seeds.voter.toBuffer(),
            Buffer.from(Uint16Array.from([seeds.index]).buffer),
        ],
        programId,
    )
};

