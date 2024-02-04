// This file is auto-generated from the CIDL source.
// Editing this file directly is not recommended as it may be overwritten.

import * as pda from "./pda";
import * as T from "./types";
import {
    Commitment,
    Connection,
    GetAccountInfoConfig,
    Keypair,
    PublicKey,
    sendAndConfirmTransaction,
    SystemProgram,
    Transaction,
    TransactionInstruction,
    TransactionSignature,
} from "@solana/web3.js";
import {deserialize, serialize} from "borsh";


let _programId: PublicKey;
let _connection: Connection;

export const initializeClient = (
    programId: PublicKey,
    connection: Connection
) => {
    _programId = programId;
    _connection = connection;
};

export enum SafeCrowdfundingInstruction {
/**
 * Create a new funding request.
 *
 * Accounts:
 * 0. `[writable, signer]` fee_payer: {@link PublicKey} 
 * 1. `[writable, signer]` beneficiary: {@link PublicKey} Account that has permission to update the funding request
 * 2. `[writable]` funding_request: {@link FundingRequest} 
 * 3. `[]` system_program: {@link PublicKey} Auto-generated, for account initialization
 *
 * Data:
 * - name: {@link string} The name for the funding request
 * - metadata_url: {@link string} URL pointing to data containing description, media, etc.
 * - funding_request_seed_beneficiary: {@link PublicKey} Auto-generated, from input funding_request of type [FundingRequest] set the seed named beneficiary, required by the type
 * - funding_request_seed_index: {@link number} Auto-generated, from input funding_request of type [FundingRequest] set the seed named index, required by the type
 */
    CreateFundingRequest = 0,

/**
 * Create a new milestone.
 *
 * Accounts:
 * 0. `[writable, signer]` fee_payer: {@link PublicKey} 
 * 1. `[signer]` creator: {@link PublicKey} Account that has permission to create milestone
 * 2. `[writable]` funding_request: {@link FundingRequest} 
 * 3. `[writable]` milestone: {@link Milestone} 
 * 4. `[]` system_program: {@link PublicKey} Auto-generated, for account initialization
 *
 * Data:
 * - name: {@link string} The name for the milestone
 * - target_amount: {@link BigInt} The amount required to complete the milestone
 * - expectation: {@link string} A short explainer on what to expect
 * - funding_request_seed_beneficiary: {@link PublicKey} Auto-generated, from input funding_request of type [FundingRequest] set the seed named beneficiary, required by the type
 * - funding_request_seed_index: {@link number} Auto-generated, from input funding_request of type [FundingRequest] set the seed named index, required by the type
 * - milestone_seed_index: {@link number} Auto-generated, from input milestone of type [Milestone] set the seed named index, required by the type
 */
    CreateMilestone = 1,

/**
 * Update an existing milestone with deliverable URL.
 *
 * Accounts:
 * 0. `[writable, signer]` fee_payer: {@link PublicKey} 
 * 1. `[signer]` updater: {@link PublicKey} 
 * 2. `[writable]` milestone: {@link Milestone} 
 * 3. `[]` funding_request: {@link FundingRequest} 
 *
 * Data:
 * - deliverable_url: {@link string} URL pointing to what was delivered
 * - milestone_seed_funding_request: {@link PublicKey} Auto-generated, from input milestone of type [Milestone] set the seed named funding_request, required by the type
 * - milestone_seed_index: {@link number} Auto-generated, from input milestone of type [Milestone] set the seed named index, required by the type
 * - funding_request_seed_beneficiary: {@link PublicKey} Auto-generated, from input funding_request of type [FundingRequest] set the seed named beneficiary, required by the type
 * - funding_request_seed_index: {@link number} Auto-generated, from input funding_request of type [FundingRequest] set the seed named index, required by the type
 */
    UpdateMilestoneDeliverable = 2,

/**
 * Unlock allocated funds for milestone.
 *
 * Accounts:
 * 0. `[writable, signer]` fee_payer: {@link PublicKey} 
 * 1. `[writable, signer]` unlocker: {@link PublicKey} 
 * 2. `[]` previous_milestone: {@link Milestone} 
 * 3. `[writable]` funding_request: {@link FundingRequest} 
 * 4. `[writable]` milestone: {@link Milestone} 
 *
 * Data:
 * - previous_milestone_seed_funding_request: {@link PublicKey} Auto-generated, from input previous_milestone of type [Milestone] set the seed named funding_request, required by the type
 * - previous_milestone_seed_index: {@link number} Auto-generated, from input previous_milestone of type [Milestone] set the seed named index, required by the type
 * - funding_request_seed_beneficiary: {@link PublicKey} Auto-generated, from input funding_request of type [FundingRequest] set the seed named beneficiary, required by the type
 * - funding_request_seed_index: {@link number} Auto-generated, from input funding_request of type [FundingRequest] set the seed named index, required by the type
 * - milestone_seed_funding_request: {@link PublicKey} Auto-generated, from input milestone of type [Milestone] set the seed named funding_request, required by the type
 * - milestone_seed_index: {@link number} Auto-generated, from input milestone of type [Milestone] set the seed named index, required by the type
 */
    UnlockMilestone = 3,

/**
 * Withdraw funds for a particular milestone
 *
 * Accounts:
 * 0. `[writable, signer]` fee_payer: {@link PublicKey} 
 * 1. `[writable, signer]` withdrawer: {@link PublicKey} 
 * 2. `[writable]` milestone: {@link Milestone} 
 * 3. `[]` funding_request: {@link FundingRequest} 
 *
 * Data:
 * - amount: {@link BigInt} Amount to withdraw
 * - milestone_seed_funding_request: {@link PublicKey} Auto-generated, from input milestone of type [Milestone] set the seed named funding_request, required by the type
 * - milestone_seed_index: {@link number} Auto-generated, from input milestone of type [Milestone] set the seed named index, required by the type
 * - funding_request_seed_beneficiary: {@link PublicKey} Auto-generated, from input funding_request of type [FundingRequest] set the seed named beneficiary, required by the type
 * - funding_request_seed_index: {@link number} Auto-generated, from input funding_request of type [FundingRequest] set the seed named index, required by the type
 */
    WithdrawFromMilestone = 4,

/**
 * Vote on a milestone.
 *
 * Accounts:
 * 0. `[writable, signer]` fee_payer: {@link PublicKey} 
 * 1. `[writable, signer]` voter: {@link PublicKey} 
 * 2. `[writable]` donor_account: {@link FundingRequestDonor} 
 * 3. `[writable]` milestone: {@link Milestone} 
 * 4. `[writable]` milestone_vote: {@link MilestoneVote} 
 * 5. `[]` system_program: {@link PublicKey} Auto-generated, for account initialization
 *
 * Data:
 * - confidence: {@link boolean} A bool representing the confidence of the vote
 * - donor_account_seed_funding_request: {@link PublicKey} Auto-generated, from input donor_account of type [FundingRequestDonor] set the seed named funding_request, required by the type
 * - donor_account_seed_owner: {@link PublicKey} Auto-generated, from input donor_account of type [FundingRequestDonor] set the seed named owner, required by the type
 * - donor_account_seed_index: {@link number} Auto-generated, from input donor_account of type [FundingRequestDonor] set the seed named index, required by the type
 * - milestone_seed_funding_request: {@link PublicKey} Auto-generated, from input milestone of type [Milestone] set the seed named funding_request, required by the type
 * - milestone_seed_index: {@link number} Auto-generated, from input milestone of type [Milestone] set the seed named index, required by the type
 * - milestone_vote_seed_voter: {@link PublicKey} Auto-generated, from input milestone_vote of type [MilestoneVote] set the seed named voter, required by the type
 * - milestone_vote_seed_index: {@link number} Auto-generated, from input milestone_vote of type [MilestoneVote] set the seed named index, required by the type
 */
    VoteOnMilestone = 5,

/**
 * create a donor account for donating to a funding request.
 *
 * Accounts:
 * 0. `[writable, signer]` fee_payer: {@link PublicKey} 
 * 1. `[writable, signer]` owner: {@link PublicKey} 
 * 2. `[]` funding_request: {@link FundingRequest} 
 * 3. `[writable]` donor: {@link FundingRequestDonor} 
 * 4. `[]` system_program: {@link PublicKey} Auto-generated, for account initialization
 *
 * Data:
 * - funding_request_seed_beneficiary: {@link PublicKey} Auto-generated, from input funding_request of type [FundingRequest] set the seed named beneficiary, required by the type
 * - funding_request_seed_index: {@link number} Auto-generated, from input funding_request of type [FundingRequest] set the seed named index, required by the type
 * - donor_seed_owner: {@link PublicKey} Auto-generated, from input donor of type [FundingRequestDonor] set the seed named owner, required by the type
 * - donor_seed_index: {@link number} Auto-generated, from input donor of type [FundingRequestDonor] set the seed named index, required by the type
 */
    CreateDonorAccount = 6,

/**
 * Donate to a funding request.
 *
 * Accounts:
 * 0. `[writable, signer]` fee_payer: {@link PublicKey} 
 * 1. `[writable, signer]` owner: {@link PublicKey} 
 * 2. `[writable]` donor: {@link FundingRequestDonor} 
 * 3. `[writable]` funding_request: {@link FundingRequest} 
 *
 * Data:
 * - amount: {@link BigInt} The amount to donate
 * - donor_seed_funding_request: {@link PublicKey} Auto-generated, from input donor of type [FundingRequestDonor] set the seed named funding_request, required by the type
 * - donor_seed_owner: {@link PublicKey} Auto-generated, from input donor of type [FundingRequestDonor] set the seed named owner, required by the type
 * - donor_seed_index: {@link number} Auto-generated, from input donor of type [FundingRequestDonor] set the seed named index, required by the type
 * - funding_request_seed_beneficiary: {@link PublicKey} Auto-generated, from input funding_request of type [FundingRequest] set the seed named beneficiary, required by the type
 * - funding_request_seed_index: {@link number} Auto-generated, from input funding_request of type [FundingRequest] set the seed named index, required by the type
 */
    DonateToFundingRequest = 7,
}

export type CreateFundingRequestArgs = {
    feePayer: PublicKey;
    beneficiary: PublicKey;
    name: string;
    metadataUrl: string;
    fundingRequestSeedBeneficiary: PublicKey;
    fundingRequestSeedIndex: number;
};


/**
 * ### Returns a {@link TransactionInstruction}
 * Create a new funding request.
 *
 * Accounts:
 * 0. `[writable, signer]` fee_payer: {@link PublicKey} 
 * 1. `[writable, signer]` beneficiary: {@link PublicKey} Account that has permission to update the funding request
 * 2. `[writable]` funding_request: {@link FundingRequest} 
 * 3. `[]` system_program: {@link PublicKey} Auto-generated, for account initialization
 *
 * Data:
 * - name: {@link string} The name for the funding request
 * - metadata_url: {@link string} URL pointing to data containing description, media, etc.
 * - funding_request_seed_beneficiary: {@link PublicKey} Auto-generated, from input funding_request of type [FundingRequest] set the seed named beneficiary, required by the type
 * - funding_request_seed_index: {@link number} Auto-generated, from input funding_request of type [FundingRequest] set the seed named index, required by the type
 */
export const createFundingRequest = (args: CreateFundingRequestArgs): TransactionInstruction => {
    const data = serialize(
        {
            struct: {
                id: "u8",
                name: "string",
                metadata_url: "string",
                funding_request_seed_beneficiary: { array: { type: "u8", len: 32 } },
                funding_request_seed_index: "u16",
            },
        },
        {
            id: SafeCrowdfundingInstruction.CreateFundingRequest,
            name: args.name,
            metadata_url: args.metadataUrl,
            funding_request_seed_beneficiary: args.fundingRequestSeedBeneficiary.toBytes(),
            funding_request_seed_index: args.fundingRequestSeedIndex,
        }
    );

    const [fundingRequestPubkey] = pda.deriveFundingRequestPDA({
        beneficiary: args.fundingRequestSeedBeneficiary,
        index: args.fundingRequestSeedIndex,
    }, _programId);

    return new TransactionInstruction({
        data: Buffer.from(data),
        keys: [
            {pubkey: args.feePayer, isSigner: true, isWritable: true},
            {pubkey: args.beneficiary, isSigner: true, isWritable: true},
            {pubkey: fundingRequestPubkey, isSigner: false, isWritable: true},
            {pubkey: new PublicKey("11111111111111111111111111111111"), isSigner: false, isWritable: false},
        ],
        programId: _programId,
    });
};

/**
 * ### Returns a {@link TransactionSignature}
 * Create a new funding request.
 *
 * Accounts:
 * 0. `[writable, signer]` fee_payer: {@link PublicKey} 
 * 1. `[writable, signer]` beneficiary: {@link PublicKey} Account that has permission to update the funding request
 * 2. `[writable]` funding_request: {@link FundingRequest} 
 * 3. `[]` system_program: {@link PublicKey} Auto-generated, for account initialization
 *
 * Data:
 * - name: {@link string} The name for the funding request
 * - metadata_url: {@link string} URL pointing to data containing description, media, etc.
 * - funding_request_seed_beneficiary: {@link PublicKey} Auto-generated, from input funding_request of type [FundingRequest] set the seed named beneficiary, required by the type
 * - funding_request_seed_index: {@link number} Auto-generated, from input funding_request of type [FundingRequest] set the seed named index, required by the type
 */
export const createFundingRequestSendAndConfirm = async (
    args: Omit<CreateFundingRequestArgs, "feePayer" |"beneficiary"> & { 
        signers: { feePayer: Keypair,  beneficiary: Keypair, }
 }
): Promise<TransactionSignature> => {
    const trx = new Transaction();


    trx.add(createFundingRequest({
        ...args,
        feePayer: args.signers.feePayer.publicKey,
        beneficiary: args.signers.beneficiary.publicKey,
    }));

    return await sendAndConfirmTransaction(
        _connection,
        trx,
        [args.signers.feePayer, args.signers.beneficiary, ]
    );
};

export type CreateMilestoneArgs = {
    feePayer: PublicKey;
    creator: PublicKey;
    name: string;
    targetAmount: bigint;
    expectation: string;
    fundingRequestSeedBeneficiary: PublicKey;
    fundingRequestSeedIndex: number;
    milestoneSeedIndex: number;
};


/**
 * ### Returns a {@link TransactionInstruction}
 * Create a new milestone.
 *
 * Accounts:
 * 0. `[writable, signer]` fee_payer: {@link PublicKey} 
 * 1. `[signer]` creator: {@link PublicKey} Account that has permission to create milestone
 * 2. `[writable]` funding_request: {@link FundingRequest} 
 * 3. `[writable]` milestone: {@link Milestone} 
 * 4. `[]` system_program: {@link PublicKey} Auto-generated, for account initialization
 *
 * Data:
 * - name: {@link string} The name for the milestone
 * - target_amount: {@link BigInt} The amount required to complete the milestone
 * - expectation: {@link string} A short explainer on what to expect
 * - funding_request_seed_beneficiary: {@link PublicKey} Auto-generated, from input funding_request of type [FundingRequest] set the seed named beneficiary, required by the type
 * - funding_request_seed_index: {@link number} Auto-generated, from input funding_request of type [FundingRequest] set the seed named index, required by the type
 * - milestone_seed_index: {@link number} Auto-generated, from input milestone of type [Milestone] set the seed named index, required by the type
 */
export const createMilestone = (args: CreateMilestoneArgs): TransactionInstruction => {
    const data = serialize(
        {
            struct: {
                id: "u8",
                name: "string",
                target_amount: "u64",
                expectation: "string",
                funding_request_seed_beneficiary: { array: { type: "u8", len: 32 } },
                funding_request_seed_index: "u16",
                milestone_seed_index: "u16",
            },
        },
        {
            id: SafeCrowdfundingInstruction.CreateMilestone,
            name: args.name,
            target_amount: args.targetAmount,
            expectation: args.expectation,
            funding_request_seed_beneficiary: args.fundingRequestSeedBeneficiary.toBytes(),
            funding_request_seed_index: args.fundingRequestSeedIndex,
            milestone_seed_index: args.milestoneSeedIndex,
        }
    );

    const [fundingRequestPubkey] = pda.deriveFundingRequestPDA({
        beneficiary: args.fundingRequestSeedBeneficiary,
        index: args.fundingRequestSeedIndex,
    }, _programId);
    const [milestonePubkey] = pda.deriveMilestonePDA({
        fundingRequest: args.fundingRequest,
        index: args.milestoneSeedIndex,
    }, _programId);

    return new TransactionInstruction({
        data: Buffer.from(data),
        keys: [
            {pubkey: args.feePayer, isSigner: true, isWritable: true},
            {pubkey: args.creator, isSigner: true, isWritable: false},
            {pubkey: fundingRequestPubkey, isSigner: false, isWritable: true},
            {pubkey: milestonePubkey, isSigner: false, isWritable: true},
            {pubkey: new PublicKey("11111111111111111111111111111111"), isSigner: false, isWritable: false},
        ],
        programId: _programId,
    });
};

/**
 * ### Returns a {@link TransactionSignature}
 * Create a new milestone.
 *
 * Accounts:
 * 0. `[writable, signer]` fee_payer: {@link PublicKey} 
 * 1. `[signer]` creator: {@link PublicKey} Account that has permission to create milestone
 * 2. `[writable]` funding_request: {@link FundingRequest} 
 * 3. `[writable]` milestone: {@link Milestone} 
 * 4. `[]` system_program: {@link PublicKey} Auto-generated, for account initialization
 *
 * Data:
 * - name: {@link string} The name for the milestone
 * - target_amount: {@link BigInt} The amount required to complete the milestone
 * - expectation: {@link string} A short explainer on what to expect
 * - funding_request_seed_beneficiary: {@link PublicKey} Auto-generated, from input funding_request of type [FundingRequest] set the seed named beneficiary, required by the type
 * - funding_request_seed_index: {@link number} Auto-generated, from input funding_request of type [FundingRequest] set the seed named index, required by the type
 * - milestone_seed_index: {@link number} Auto-generated, from input milestone of type [Milestone] set the seed named index, required by the type
 */
export const createMilestoneSendAndConfirm = async (
    args: Omit<CreateMilestoneArgs, "feePayer" |"creator"> & { 
        signers: { feePayer: Keypair,  creator: Keypair, }
 }
): Promise<TransactionSignature> => {
    const trx = new Transaction();


    trx.add(createMilestone({
        ...args,
        feePayer: args.signers.feePayer.publicKey,
        creator: args.signers.creator.publicKey,
    }));

    return await sendAndConfirmTransaction(
        _connection,
        trx,
        [args.signers.feePayer, args.signers.creator, ]
    );
};

export type UpdateMilestoneDeliverableArgs = {
    feePayer: PublicKey;
    updater: PublicKey;
    deliverableUrl: string;
    milestoneSeedFundingRequest: PublicKey;
    milestoneSeedIndex: number;
    fundingRequestSeedBeneficiary: PublicKey;
    fundingRequestSeedIndex: number;
};


/**
 * ### Returns a {@link TransactionInstruction}
 * Update an existing milestone with deliverable URL.
 *
 * Accounts:
 * 0. `[writable, signer]` fee_payer: {@link PublicKey} 
 * 1. `[signer]` updater: {@link PublicKey} 
 * 2. `[writable]` milestone: {@link Milestone} 
 * 3. `[]` funding_request: {@link FundingRequest} 
 *
 * Data:
 * - deliverable_url: {@link string} URL pointing to what was delivered
 * - milestone_seed_funding_request: {@link PublicKey} Auto-generated, from input milestone of type [Milestone] set the seed named funding_request, required by the type
 * - milestone_seed_index: {@link number} Auto-generated, from input milestone of type [Milestone] set the seed named index, required by the type
 * - funding_request_seed_beneficiary: {@link PublicKey} Auto-generated, from input funding_request of type [FundingRequest] set the seed named beneficiary, required by the type
 * - funding_request_seed_index: {@link number} Auto-generated, from input funding_request of type [FundingRequest] set the seed named index, required by the type
 */
export const updateMilestoneDeliverable = (args: UpdateMilestoneDeliverableArgs): TransactionInstruction => {
    const data = serialize(
        {
            struct: {
                id: "u8",
                deliverable_url: "string",
                milestone_seed_funding_request: { array: { type: "u8", len: 32 } },
                milestone_seed_index: "u16",
                funding_request_seed_beneficiary: { array: { type: "u8", len: 32 } },
                funding_request_seed_index: "u16",
            },
        },
        {
            id: SafeCrowdfundingInstruction.UpdateMilestoneDeliverable,
            deliverable_url: args.deliverableUrl,
            milestone_seed_funding_request: args.milestoneSeedFundingRequest.toBytes(),
            milestone_seed_index: args.milestoneSeedIndex,
            funding_request_seed_beneficiary: args.fundingRequestSeedBeneficiary.toBytes(),
            funding_request_seed_index: args.fundingRequestSeedIndex,
        }
    );

    const [milestonePubkey] = pda.deriveMilestonePDA({
        fundingRequest: args.milestoneSeedFundingRequest,
        index: args.milestoneSeedIndex,
    }, _programId);
    const [fundingRequestPubkey] = pda.deriveFundingRequestPDA({
        beneficiary: args.fundingRequestSeedBeneficiary,
        index: args.fundingRequestSeedIndex,
    }, _programId);

    return new TransactionInstruction({
        data: Buffer.from(data),
        keys: [
            {pubkey: args.feePayer, isSigner: true, isWritable: true},
            {pubkey: args.updater, isSigner: true, isWritable: false},
            {pubkey: milestonePubkey, isSigner: false, isWritable: true},
            {pubkey: fundingRequestPubkey, isSigner: false, isWritable: false},
        ],
        programId: _programId,
    });
};

/**
 * ### Returns a {@link TransactionSignature}
 * Update an existing milestone with deliverable URL.
 *
 * Accounts:
 * 0. `[writable, signer]` fee_payer: {@link PublicKey} 
 * 1. `[signer]` updater: {@link PublicKey} 
 * 2. `[writable]` milestone: {@link Milestone} 
 * 3. `[]` funding_request: {@link FundingRequest} 
 *
 * Data:
 * - deliverable_url: {@link string} URL pointing to what was delivered
 * - milestone_seed_funding_request: {@link PublicKey} Auto-generated, from input milestone of type [Milestone] set the seed named funding_request, required by the type
 * - milestone_seed_index: {@link number} Auto-generated, from input milestone of type [Milestone] set the seed named index, required by the type
 * - funding_request_seed_beneficiary: {@link PublicKey} Auto-generated, from input funding_request of type [FundingRequest] set the seed named beneficiary, required by the type
 * - funding_request_seed_index: {@link number} Auto-generated, from input funding_request of type [FundingRequest] set the seed named index, required by the type
 */
export const updateMilestoneDeliverableSendAndConfirm = async (
    args: Omit<UpdateMilestoneDeliverableArgs, "feePayer" |"updater"> & { 
        signers: { feePayer: Keypair,  updater: Keypair, }
 }
): Promise<TransactionSignature> => {
    const trx = new Transaction();


    trx.add(updateMilestoneDeliverable({
        ...args,
        feePayer: args.signers.feePayer.publicKey,
        updater: args.signers.updater.publicKey,
    }));

    return await sendAndConfirmTransaction(
        _connection,
        trx,
        [args.signers.feePayer, args.signers.updater, ]
    );
};

export type UnlockMilestoneArgs = {
    feePayer: PublicKey;
    unlocker: PublicKey;
    previousMilestoneSeedFundingRequest: PublicKey;
    previousMilestoneSeedIndex: number;
    fundingRequestSeedBeneficiary: PublicKey;
    fundingRequestSeedIndex: number;
    milestoneSeedFundingRequest: PublicKey;
    milestoneSeedIndex: number;
};


/**
 * ### Returns a {@link TransactionInstruction}
 * Unlock allocated funds for milestone.
 *
 * Accounts:
 * 0. `[writable, signer]` fee_payer: {@link PublicKey} 
 * 1. `[writable, signer]` unlocker: {@link PublicKey} 
 * 2. `[]` previous_milestone: {@link Milestone} 
 * 3. `[writable]` funding_request: {@link FundingRequest} 
 * 4. `[writable]` milestone: {@link Milestone} 
 *
 * Data:
 * - previous_milestone_seed_funding_request: {@link PublicKey} Auto-generated, from input previous_milestone of type [Milestone] set the seed named funding_request, required by the type
 * - previous_milestone_seed_index: {@link number} Auto-generated, from input previous_milestone of type [Milestone] set the seed named index, required by the type
 * - funding_request_seed_beneficiary: {@link PublicKey} Auto-generated, from input funding_request of type [FundingRequest] set the seed named beneficiary, required by the type
 * - funding_request_seed_index: {@link number} Auto-generated, from input funding_request of type [FundingRequest] set the seed named index, required by the type
 * - milestone_seed_funding_request: {@link PublicKey} Auto-generated, from input milestone of type [Milestone] set the seed named funding_request, required by the type
 * - milestone_seed_index: {@link number} Auto-generated, from input milestone of type [Milestone] set the seed named index, required by the type
 */
export const unlockMilestone = (args: UnlockMilestoneArgs): TransactionInstruction => {
    const data = serialize(
        {
            struct: {
                id: "u8",
                previous_milestone_seed_funding_request: { array: { type: "u8", len: 32 } },
                previous_milestone_seed_index: "u16",
                funding_request_seed_beneficiary: { array: { type: "u8", len: 32 } },
                funding_request_seed_index: "u16",
                milestone_seed_funding_request: { array: { type: "u8", len: 32 } },
                milestone_seed_index: "u16",
            },
        },
        {
            id: SafeCrowdfundingInstruction.UnlockMilestone,
            previous_milestone_seed_funding_request: args.previousMilestoneSeedFundingRequest.toBytes(),
            previous_milestone_seed_index: args.previousMilestoneSeedIndex,
            funding_request_seed_beneficiary: args.fundingRequestSeedBeneficiary.toBytes(),
            funding_request_seed_index: args.fundingRequestSeedIndex,
            milestone_seed_funding_request: args.milestoneSeedFundingRequest.toBytes(),
            milestone_seed_index: args.milestoneSeedIndex,
        }
    );

    const [previousMilestonePubkey] = pda.deriveMilestonePDA({
        fundingRequest: args.previousMilestoneSeedFundingRequest,
        index: args.previousMilestoneSeedIndex,
    }, _programId);
    const [fundingRequestPubkey] = pda.deriveFundingRequestPDA({
        beneficiary: args.fundingRequestSeedBeneficiary,
        index: args.fundingRequestSeedIndex,
    }, _programId);
    const [milestonePubkey] = pda.deriveMilestonePDA({
        fundingRequest: args.milestoneSeedFundingRequest,
        index: args.milestoneSeedIndex,
    }, _programId);

    return new TransactionInstruction({
        data: Buffer.from(data),
        keys: [
            {pubkey: args.feePayer, isSigner: true, isWritable: true},
            {pubkey: args.unlocker, isSigner: true, isWritable: true},
            {pubkey: previousMilestonePubkey, isSigner: false, isWritable: false},
            {pubkey: fundingRequestPubkey, isSigner: false, isWritable: true},
            {pubkey: milestonePubkey, isSigner: false, isWritable: true},
        ],
        programId: _programId,
    });
};

/**
 * ### Returns a {@link TransactionSignature}
 * Unlock allocated funds for milestone.
 *
 * Accounts:
 * 0. `[writable, signer]` fee_payer: {@link PublicKey} 
 * 1. `[writable, signer]` unlocker: {@link PublicKey} 
 * 2. `[]` previous_milestone: {@link Milestone} 
 * 3. `[writable]` funding_request: {@link FundingRequest} 
 * 4. `[writable]` milestone: {@link Milestone} 
 *
 * Data:
 * - previous_milestone_seed_funding_request: {@link PublicKey} Auto-generated, from input previous_milestone of type [Milestone] set the seed named funding_request, required by the type
 * - previous_milestone_seed_index: {@link number} Auto-generated, from input previous_milestone of type [Milestone] set the seed named index, required by the type
 * - funding_request_seed_beneficiary: {@link PublicKey} Auto-generated, from input funding_request of type [FundingRequest] set the seed named beneficiary, required by the type
 * - funding_request_seed_index: {@link number} Auto-generated, from input funding_request of type [FundingRequest] set the seed named index, required by the type
 * - milestone_seed_funding_request: {@link PublicKey} Auto-generated, from input milestone of type [Milestone] set the seed named funding_request, required by the type
 * - milestone_seed_index: {@link number} Auto-generated, from input milestone of type [Milestone] set the seed named index, required by the type
 */
export const unlockMilestoneSendAndConfirm = async (
    args: Omit<UnlockMilestoneArgs, "feePayer" |"unlocker"> & { 
        signers: { feePayer: Keypair,  unlocker: Keypair, }
 }
): Promise<TransactionSignature> => {
    const trx = new Transaction();


    trx.add(unlockMilestone({
        ...args,
        feePayer: args.signers.feePayer.publicKey,
        unlocker: args.signers.unlocker.publicKey,
    }));

    return await sendAndConfirmTransaction(
        _connection,
        trx,
        [args.signers.feePayer, args.signers.unlocker, ]
    );
};

export type WithdrawFromMilestoneArgs = {
    feePayer: PublicKey;
    withdrawer: PublicKey;
    amount: bigint;
    milestoneSeedFundingRequest: PublicKey;
    milestoneSeedIndex: number;
    fundingRequestSeedBeneficiary: PublicKey;
    fundingRequestSeedIndex: number;
};


/**
 * ### Returns a {@link TransactionInstruction}
 * Withdraw funds for a particular milestone
 *
 * Accounts:
 * 0. `[writable, signer]` fee_payer: {@link PublicKey} 
 * 1. `[writable, signer]` withdrawer: {@link PublicKey} 
 * 2. `[writable]` milestone: {@link Milestone} 
 * 3. `[]` funding_request: {@link FundingRequest} 
 *
 * Data:
 * - amount: {@link BigInt} Amount to withdraw
 * - milestone_seed_funding_request: {@link PublicKey} Auto-generated, from input milestone of type [Milestone] set the seed named funding_request, required by the type
 * - milestone_seed_index: {@link number} Auto-generated, from input milestone of type [Milestone] set the seed named index, required by the type
 * - funding_request_seed_beneficiary: {@link PublicKey} Auto-generated, from input funding_request of type [FundingRequest] set the seed named beneficiary, required by the type
 * - funding_request_seed_index: {@link number} Auto-generated, from input funding_request of type [FundingRequest] set the seed named index, required by the type
 */
export const withdrawFromMilestone = (args: WithdrawFromMilestoneArgs): TransactionInstruction => {
    const data = serialize(
        {
            struct: {
                id: "u8",
                amount: "u64",
                milestone_seed_funding_request: { array: { type: "u8", len: 32 } },
                milestone_seed_index: "u16",
                funding_request_seed_beneficiary: { array: { type: "u8", len: 32 } },
                funding_request_seed_index: "u16",
            },
        },
        {
            id: SafeCrowdfundingInstruction.WithdrawFromMilestone,
            amount: args.amount,
            milestone_seed_funding_request: args.milestoneSeedFundingRequest.toBytes(),
            milestone_seed_index: args.milestoneSeedIndex,
            funding_request_seed_beneficiary: args.fundingRequestSeedBeneficiary.toBytes(),
            funding_request_seed_index: args.fundingRequestSeedIndex,
        }
    );

    const [milestonePubkey] = pda.deriveMilestonePDA({
        fundingRequest: args.milestoneSeedFundingRequest,
        index: args.milestoneSeedIndex,
    }, _programId);
    const [fundingRequestPubkey] = pda.deriveFundingRequestPDA({
        beneficiary: args.fundingRequestSeedBeneficiary,
        index: args.fundingRequestSeedIndex,
    }, _programId);

    return new TransactionInstruction({
        data: Buffer.from(data),
        keys: [
            {pubkey: args.feePayer, isSigner: true, isWritable: true},
            {pubkey: args.withdrawer, isSigner: true, isWritable: true},
            {pubkey: milestonePubkey, isSigner: false, isWritable: true},
            {pubkey: fundingRequestPubkey, isSigner: false, isWritable: false},
        ],
        programId: _programId,
    });
};

/**
 * ### Returns a {@link TransactionSignature}
 * Withdraw funds for a particular milestone
 *
 * Accounts:
 * 0. `[writable, signer]` fee_payer: {@link PublicKey} 
 * 1. `[writable, signer]` withdrawer: {@link PublicKey} 
 * 2. `[writable]` milestone: {@link Milestone} 
 * 3. `[]` funding_request: {@link FundingRequest} 
 *
 * Data:
 * - amount: {@link BigInt} Amount to withdraw
 * - milestone_seed_funding_request: {@link PublicKey} Auto-generated, from input milestone of type [Milestone] set the seed named funding_request, required by the type
 * - milestone_seed_index: {@link number} Auto-generated, from input milestone of type [Milestone] set the seed named index, required by the type
 * - funding_request_seed_beneficiary: {@link PublicKey} Auto-generated, from input funding_request of type [FundingRequest] set the seed named beneficiary, required by the type
 * - funding_request_seed_index: {@link number} Auto-generated, from input funding_request of type [FundingRequest] set the seed named index, required by the type
 */
export const withdrawFromMilestoneSendAndConfirm = async (
    args: Omit<WithdrawFromMilestoneArgs, "feePayer" |"withdrawer"> & { 
        signers: { feePayer: Keypair,  withdrawer: Keypair, }
 }
): Promise<TransactionSignature> => {
    const trx = new Transaction();


    trx.add(withdrawFromMilestone({
        ...args,
        feePayer: args.signers.feePayer.publicKey,
        withdrawer: args.signers.withdrawer.publicKey,
    }));

    return await sendAndConfirmTransaction(
        _connection,
        trx,
        [args.signers.feePayer, args.signers.withdrawer, ]
    );
};

export type VoteOnMilestoneArgs = {
    feePayer: PublicKey;
    voter: PublicKey;
    confidence: boolean;
    donorAccountSeedFundingRequest: PublicKey;
    donorAccountSeedOwner: PublicKey;
    donorAccountSeedIndex: number;
    milestoneSeedFundingRequest: PublicKey;
    milestoneSeedIndex: number;
    milestoneVoteSeedVoter: PublicKey;
    milestoneVoteSeedIndex: number;
};


/**
 * ### Returns a {@link TransactionInstruction}
 * Vote on a milestone.
 *
 * Accounts:
 * 0. `[writable, signer]` fee_payer: {@link PublicKey} 
 * 1. `[writable, signer]` voter: {@link PublicKey} 
 * 2. `[writable]` donor_account: {@link FundingRequestDonor} 
 * 3. `[writable]` milestone: {@link Milestone} 
 * 4. `[writable]` milestone_vote: {@link MilestoneVote} 
 * 5. `[]` system_program: {@link PublicKey} Auto-generated, for account initialization
 *
 * Data:
 * - confidence: {@link boolean} A bool representing the confidence of the vote
 * - donor_account_seed_funding_request: {@link PublicKey} Auto-generated, from input donor_account of type [FundingRequestDonor] set the seed named funding_request, required by the type
 * - donor_account_seed_owner: {@link PublicKey} Auto-generated, from input donor_account of type [FundingRequestDonor] set the seed named owner, required by the type
 * - donor_account_seed_index: {@link number} Auto-generated, from input donor_account of type [FundingRequestDonor] set the seed named index, required by the type
 * - milestone_seed_funding_request: {@link PublicKey} Auto-generated, from input milestone of type [Milestone] set the seed named funding_request, required by the type
 * - milestone_seed_index: {@link number} Auto-generated, from input milestone of type [Milestone] set the seed named index, required by the type
 * - milestone_vote_seed_voter: {@link PublicKey} Auto-generated, from input milestone_vote of type [MilestoneVote] set the seed named voter, required by the type
 * - milestone_vote_seed_index: {@link number} Auto-generated, from input milestone_vote of type [MilestoneVote] set the seed named index, required by the type
 */
export const voteOnMilestone = (args: VoteOnMilestoneArgs): TransactionInstruction => {
    const data = serialize(
        {
            struct: {
                id: "u8",
                confidence: "bool",
                donor_account_seed_funding_request: { array: { type: "u8", len: 32 } },
                donor_account_seed_owner: { array: { type: "u8", len: 32 } },
                donor_account_seed_index: "u16",
                milestone_seed_funding_request: { array: { type: "u8", len: 32 } },
                milestone_seed_index: "u16",
                milestone_vote_seed_voter: { array: { type: "u8", len: 32 } },
                milestone_vote_seed_index: "u16",
            },
        },
        {
            id: SafeCrowdfundingInstruction.VoteOnMilestone,
            confidence: args.confidence,
            donor_account_seed_funding_request: args.donorAccountSeedFundingRequest.toBytes(),
            donor_account_seed_owner: args.donorAccountSeedOwner.toBytes(),
            donor_account_seed_index: args.donorAccountSeedIndex,
            milestone_seed_funding_request: args.milestoneSeedFundingRequest.toBytes(),
            milestone_seed_index: args.milestoneSeedIndex,
            milestone_vote_seed_voter: args.milestoneVoteSeedVoter.toBytes(),
            milestone_vote_seed_index: args.milestoneVoteSeedIndex,
        }
    );

    const [donorAccountPubkey] = pda.deriveFundingRequestDonorPDA({
        fundingRequest: args.donorAccountSeedFundingRequest,
        owner: args.donorAccountSeedOwner,
        index: args.donorAccountSeedIndex,
    }, _programId);
    const [milestonePubkey] = pda.deriveMilestonePDA({
        fundingRequest: args.milestoneSeedFundingRequest,
        index: args.milestoneSeedIndex,
    }, _programId);
    const [milestoneVotePubkey] = pda.deriveMilestoneVotePDA({
        milestone: args.milestone,
        voter: args.milestoneVoteSeedVoter,
        index: args.milestoneVoteSeedIndex,
    }, _programId);

    return new TransactionInstruction({
        data: Buffer.from(data),
        keys: [
            {pubkey: args.feePayer, isSigner: true, isWritable: true},
            {pubkey: args.voter, isSigner: true, isWritable: true},
            {pubkey: donorAccountPubkey, isSigner: false, isWritable: true},
            {pubkey: milestonePubkey, isSigner: false, isWritable: true},
            {pubkey: milestoneVotePubkey, isSigner: false, isWritable: true},
            {pubkey: new PublicKey("11111111111111111111111111111111"), isSigner: false, isWritable: false},
        ],
        programId: _programId,
    });
};

/**
 * ### Returns a {@link TransactionSignature}
 * Vote on a milestone.
 *
 * Accounts:
 * 0. `[writable, signer]` fee_payer: {@link PublicKey} 
 * 1. `[writable, signer]` voter: {@link PublicKey} 
 * 2. `[writable]` donor_account: {@link FundingRequestDonor} 
 * 3. `[writable]` milestone: {@link Milestone} 
 * 4. `[writable]` milestone_vote: {@link MilestoneVote} 
 * 5. `[]` system_program: {@link PublicKey} Auto-generated, for account initialization
 *
 * Data:
 * - confidence: {@link boolean} A bool representing the confidence of the vote
 * - donor_account_seed_funding_request: {@link PublicKey} Auto-generated, from input donor_account of type [FundingRequestDonor] set the seed named funding_request, required by the type
 * - donor_account_seed_owner: {@link PublicKey} Auto-generated, from input donor_account of type [FundingRequestDonor] set the seed named owner, required by the type
 * - donor_account_seed_index: {@link number} Auto-generated, from input donor_account of type [FundingRequestDonor] set the seed named index, required by the type
 * - milestone_seed_funding_request: {@link PublicKey} Auto-generated, from input milestone of type [Milestone] set the seed named funding_request, required by the type
 * - milestone_seed_index: {@link number} Auto-generated, from input milestone of type [Milestone] set the seed named index, required by the type
 * - milestone_vote_seed_voter: {@link PublicKey} Auto-generated, from input milestone_vote of type [MilestoneVote] set the seed named voter, required by the type
 * - milestone_vote_seed_index: {@link number} Auto-generated, from input milestone_vote of type [MilestoneVote] set the seed named index, required by the type
 */
export const voteOnMilestoneSendAndConfirm = async (
    args: Omit<VoteOnMilestoneArgs, "feePayer" |"voter"> & { 
        signers: { feePayer: Keypair,  voter: Keypair, }
 }
): Promise<TransactionSignature> => {
    const trx = new Transaction();


    trx.add(voteOnMilestone({
        ...args,
        feePayer: args.signers.feePayer.publicKey,
        voter: args.signers.voter.publicKey,
    }));

    return await sendAndConfirmTransaction(
        _connection,
        trx,
        [args.signers.feePayer, args.signers.voter, ]
    );
};

export type CreateDonorAccountArgs = {
    feePayer: PublicKey;
    owner: PublicKey;
    fundingRequestSeedBeneficiary: PublicKey;
    fundingRequestSeedIndex: number;
    donorSeedOwner: PublicKey;
    donorSeedIndex: number;
};


/**
 * ### Returns a {@link TransactionInstruction}
 * create a donor account for donating to a funding request.
 *
 * Accounts:
 * 0. `[writable, signer]` fee_payer: {@link PublicKey} 
 * 1. `[writable, signer]` owner: {@link PublicKey} 
 * 2. `[]` funding_request: {@link FundingRequest} 
 * 3. `[writable]` donor: {@link FundingRequestDonor} 
 * 4. `[]` system_program: {@link PublicKey} Auto-generated, for account initialization
 *
 * Data:
 * - funding_request_seed_beneficiary: {@link PublicKey} Auto-generated, from input funding_request of type [FundingRequest] set the seed named beneficiary, required by the type
 * - funding_request_seed_index: {@link number} Auto-generated, from input funding_request of type [FundingRequest] set the seed named index, required by the type
 * - donor_seed_owner: {@link PublicKey} Auto-generated, from input donor of type [FundingRequestDonor] set the seed named owner, required by the type
 * - donor_seed_index: {@link number} Auto-generated, from input donor of type [FundingRequestDonor] set the seed named index, required by the type
 */
export const createDonorAccount = (args: CreateDonorAccountArgs): TransactionInstruction => {
    const data = serialize(
        {
            struct: {
                id: "u8",
                funding_request_seed_beneficiary: { array: { type: "u8", len: 32 } },
                funding_request_seed_index: "u16",
                donor_seed_owner: { array: { type: "u8", len: 32 } },
                donor_seed_index: "u16",
            },
        },
        {
            id: SafeCrowdfundingInstruction.CreateDonorAccount,
            funding_request_seed_beneficiary: args.fundingRequestSeedBeneficiary.toBytes(),
            funding_request_seed_index: args.fundingRequestSeedIndex,
            donor_seed_owner: args.donorSeedOwner.toBytes(),
            donor_seed_index: args.donorSeedIndex,
        }
    );

    const [fundingRequestPubkey] = pda.deriveFundingRequestPDA({
        beneficiary: args.fundingRequestSeedBeneficiary,
        index: args.fundingRequestSeedIndex,
    }, _programId);
    const [donorPubkey] = pda.deriveFundingRequestDonorPDA({
        fundingRequest: args.fundingRequest,
        owner: args.donorSeedOwner,
        index: args.donorSeedIndex,
    }, _programId);

    return new TransactionInstruction({
        data: Buffer.from(data),
        keys: [
            {pubkey: args.feePayer, isSigner: true, isWritable: true},
            {pubkey: args.owner, isSigner: true, isWritable: true},
            {pubkey: fundingRequestPubkey, isSigner: false, isWritable: false},
            {pubkey: donorPubkey, isSigner: false, isWritable: true},
            {pubkey: new PublicKey("11111111111111111111111111111111"), isSigner: false, isWritable: false},
        ],
        programId: _programId,
    });
};

/**
 * ### Returns a {@link TransactionSignature}
 * create a donor account for donating to a funding request.
 *
 * Accounts:
 * 0. `[writable, signer]` fee_payer: {@link PublicKey} 
 * 1. `[writable, signer]` owner: {@link PublicKey} 
 * 2. `[]` funding_request: {@link FundingRequest} 
 * 3. `[writable]` donor: {@link FundingRequestDonor} 
 * 4. `[]` system_program: {@link PublicKey} Auto-generated, for account initialization
 *
 * Data:
 * - funding_request_seed_beneficiary: {@link PublicKey} Auto-generated, from input funding_request of type [FundingRequest] set the seed named beneficiary, required by the type
 * - funding_request_seed_index: {@link number} Auto-generated, from input funding_request of type [FundingRequest] set the seed named index, required by the type
 * - donor_seed_owner: {@link PublicKey} Auto-generated, from input donor of type [FundingRequestDonor] set the seed named owner, required by the type
 * - donor_seed_index: {@link number} Auto-generated, from input donor of type [FundingRequestDonor] set the seed named index, required by the type
 */
export const createDonorAccountSendAndConfirm = async (
    args: Omit<CreateDonorAccountArgs, "feePayer" |"owner"> & { 
        signers: { feePayer: Keypair,  owner: Keypair, }
 }
): Promise<TransactionSignature> => {
    const trx = new Transaction();


    trx.add(createDonorAccount({
        ...args,
        feePayer: args.signers.feePayer.publicKey,
        owner: args.signers.owner.publicKey,
    }));

    return await sendAndConfirmTransaction(
        _connection,
        trx,
        [args.signers.feePayer, args.signers.owner, ]
    );
};

export type DonateToFundingRequestArgs = {
    feePayer: PublicKey;
    owner: PublicKey;
    amount: bigint;
    donorSeedFundingRequest: PublicKey;
    donorSeedOwner: PublicKey;
    donorSeedIndex: number;
    fundingRequestSeedBeneficiary: PublicKey;
    fundingRequestSeedIndex: number;
};


/**
 * ### Returns a {@link TransactionInstruction}
 * Donate to a funding request.
 *
 * Accounts:
 * 0. `[writable, signer]` fee_payer: {@link PublicKey} 
 * 1. `[writable, signer]` owner: {@link PublicKey} 
 * 2. `[writable]` donor: {@link FundingRequestDonor} 
 * 3. `[writable]` funding_request: {@link FundingRequest} 
 *
 * Data:
 * - amount: {@link BigInt} The amount to donate
 * - donor_seed_funding_request: {@link PublicKey} Auto-generated, from input donor of type [FundingRequestDonor] set the seed named funding_request, required by the type
 * - donor_seed_owner: {@link PublicKey} Auto-generated, from input donor of type [FundingRequestDonor] set the seed named owner, required by the type
 * - donor_seed_index: {@link number} Auto-generated, from input donor of type [FundingRequestDonor] set the seed named index, required by the type
 * - funding_request_seed_beneficiary: {@link PublicKey} Auto-generated, from input funding_request of type [FundingRequest] set the seed named beneficiary, required by the type
 * - funding_request_seed_index: {@link number} Auto-generated, from input funding_request of type [FundingRequest] set the seed named index, required by the type
 */
export const donateToFundingRequest = (args: DonateToFundingRequestArgs): TransactionInstruction => {
    const data = serialize(
        {
            struct: {
                id: "u8",
                amount: "u64",
                donor_seed_funding_request: { array: { type: "u8", len: 32 } },
                donor_seed_owner: { array: { type: "u8", len: 32 } },
                donor_seed_index: "u16",
                funding_request_seed_beneficiary: { array: { type: "u8", len: 32 } },
                funding_request_seed_index: "u16",
            },
        },
        {
            id: SafeCrowdfundingInstruction.DonateToFundingRequest,
            amount: args.amount,
            donor_seed_funding_request: args.donorSeedFundingRequest.toBytes(),
            donor_seed_owner: args.donorSeedOwner.toBytes(),
            donor_seed_index: args.donorSeedIndex,
            funding_request_seed_beneficiary: args.fundingRequestSeedBeneficiary.toBytes(),
            funding_request_seed_index: args.fundingRequestSeedIndex,
        }
    );

    const [donorPubkey] = pda.deriveFundingRequestDonorPDA({
        fundingRequest: args.donorSeedFundingRequest,
        owner: args.donorSeedOwner,
        index: args.donorSeedIndex,
    }, _programId);
    const [fundingRequestPubkey] = pda.deriveFundingRequestPDA({
        beneficiary: args.fundingRequestSeedBeneficiary,
        index: args.fundingRequestSeedIndex,
    }, _programId);

    return new TransactionInstruction({
        data: Buffer.from(data),
        keys: [
            {pubkey: args.feePayer, isSigner: true, isWritable: true},
            {pubkey: args.owner, isSigner: true, isWritable: true},
            {pubkey: donorPubkey, isSigner: false, isWritable: true},
            {pubkey: fundingRequestPubkey, isSigner: false, isWritable: true},
        ],
        programId: _programId,
    });
};

/**
 * ### Returns a {@link TransactionSignature}
 * Donate to a funding request.
 *
 * Accounts:
 * 0. `[writable, signer]` fee_payer: {@link PublicKey} 
 * 1. `[writable, signer]` owner: {@link PublicKey} 
 * 2. `[writable]` donor: {@link FundingRequestDonor} 
 * 3. `[writable]` funding_request: {@link FundingRequest} 
 *
 * Data:
 * - amount: {@link BigInt} The amount to donate
 * - donor_seed_funding_request: {@link PublicKey} Auto-generated, from input donor of type [FundingRequestDonor] set the seed named funding_request, required by the type
 * - donor_seed_owner: {@link PublicKey} Auto-generated, from input donor of type [FundingRequestDonor] set the seed named owner, required by the type
 * - donor_seed_index: {@link number} Auto-generated, from input donor of type [FundingRequestDonor] set the seed named index, required by the type
 * - funding_request_seed_beneficiary: {@link PublicKey} Auto-generated, from input funding_request of type [FundingRequest] set the seed named beneficiary, required by the type
 * - funding_request_seed_index: {@link number} Auto-generated, from input funding_request of type [FundingRequest] set the seed named index, required by the type
 */
export const donateToFundingRequestSendAndConfirm = async (
    args: Omit<DonateToFundingRequestArgs, "feePayer" |"owner"> & { 
        signers: { feePayer: Keypair,  owner: Keypair, }
 }
): Promise<TransactionSignature> => {
    const trx = new Transaction();


    trx.add(donateToFundingRequest({
        ...args,
        feePayer: args.signers.feePayer.publicKey,
        owner: args.signers.owner.publicKey,
    }));

    return await sendAndConfirmTransaction(
        _connection,
        trx,
        [args.signers.feePayer, args.signers.owner, ]
    );
};

// Getters

export const getFundingRequest = async (
    publicKey: PublicKey,
    commitmentOrConfig: Commitment | GetAccountInfoConfig | undefined = "processed"
): Promise<T.FundingRequest | undefined> => {
    const buffer = await _connection.getAccountInfo(publicKey, commitmentOrConfig);

    if (!buffer) {
        return undefined
    }

    if (buffer.data.length <= 0) {
        return undefined
    }

    return T.decodeFundingRequest(deserialize(T.FundingRequestSchema, buffer.data) as Record<string, unknown>);
}

export const getFundingRequestDonor = async (
    publicKey: PublicKey,
    commitmentOrConfig: Commitment | GetAccountInfoConfig | undefined = "processed"
): Promise<T.FundingRequestDonor | undefined> => {
    const buffer = await _connection.getAccountInfo(publicKey, commitmentOrConfig);

    if (!buffer) {
        return undefined
    }

    if (buffer.data.length <= 0) {
        return undefined
    }

    return T.decodeFundingRequestDonor(deserialize(T.FundingRequestDonorSchema, buffer.data) as Record<string, unknown>);
}

export const getMilestone = async (
    publicKey: PublicKey,
    commitmentOrConfig: Commitment | GetAccountInfoConfig | undefined = "processed"
): Promise<T.Milestone | undefined> => {
    const buffer = await _connection.getAccountInfo(publicKey, commitmentOrConfig);

    if (!buffer) {
        return undefined
    }

    if (buffer.data.length <= 0) {
        return undefined
    }

    return T.decodeMilestone(deserialize(T.MilestoneSchema, buffer.data) as Record<string, unknown>);
}

export const getMilestoneVote = async (
    publicKey: PublicKey,
    commitmentOrConfig: Commitment | GetAccountInfoConfig | undefined = "processed"
): Promise<T.MilestoneVote | undefined> => {
    const buffer = await _connection.getAccountInfo(publicKey, commitmentOrConfig);

    if (!buffer) {
        return undefined
    }

    if (buffer.data.length <= 0) {
        return undefined
    }

    return T.decodeMilestoneVote(deserialize(T.MilestoneVoteSchema, buffer.data) as Record<string, unknown>);
}


// Websocket Events

