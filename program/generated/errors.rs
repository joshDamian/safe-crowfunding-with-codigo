// This file is auto-generated from the CIDL source.
// Editing this file directly is not recommended as it may be overwritten.

use num_derive::FromPrimitive;
use solana_program::decode_error::DecodeError;
use solana_program::msg;
use solana_program::program_error::{PrintProgramError, ProgramError};
use thiserror::Error;

#[derive(Error, FromPrimitive, Debug, Clone)]
pub enum SafeCrowdfundingError {
    #[error("Invalid Instruction")]
    InvalidInstruction,

    #[error("Invalid Signer Permission")]
    InvalidSignerPermission,

    #[error("Not The Expected Account Address")]
    NotExpectedAddress,

    #[error("Wrong Account Owner")]
    WrongAccountOwner,

    #[error("Invalid Account Len")]
    InvalidAccountLen,

    #[error("Executable Account Expected")]
    ExecutableAccountExpected,

 
}

impl From<SafeCrowdfundingError> for ProgramError {
    fn from(e: SafeCrowdfundingError) -> Self {
        ProgramError::Custom(e as u32)
    }
}

impl<T> DecodeError<T> for SafeCrowdfundingError {
    fn type_of() -> &'static str {
        "SafeCrowdfundingError"
    }
}

impl PrintProgramError for SafeCrowdfundingError {
    fn print<E>(&self)
    where
        E: 'static
            + std::error::Error
            + DecodeError<E>
            + PrintProgramError
            + num_traits::FromPrimitive,
    {
        match self {
            SafeCrowdfundingError::InvalidInstruction => msg!("Error: Invalid instruction"),
            SafeCrowdfundingError::InvalidSignerPermission => msg!("Error: The account is_signer value is not the expected one"),
            SafeCrowdfundingError::NotExpectedAddress => {
                msg!("Error: Not the expected account address")
            }
            SafeCrowdfundingError::WrongAccountOwner => msg!("Error: Wrong account owner"),
            SafeCrowdfundingError::InvalidAccountLen => msg!("Error: Invalid account length"),
            SafeCrowdfundingError::ExecutableAccountExpected => msg!("Error: Executable account expected"),
 
        }
    }
}