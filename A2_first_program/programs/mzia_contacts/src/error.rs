use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Contact name must be 1-32 bytes")]
    InvalidName,
    #[msg("A contact cannot point to its own owner")]
    SelfContact,
}
