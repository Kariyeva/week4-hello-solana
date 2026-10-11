use solana_program::{
    account_info::AccountInfo,
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let (&tag, data) = instruction_data
        .split_first()
        .ok_or(ProgramError::InvalidInstructionData)?;

    match tag {
        0 => {
            msg!("Hello, Solana! Program ID: {}", program_id);
            msg!("Accounts passed: {}", accounts.len());
        }
        1 => {
            if data.len() > 128 {
                return Err(ProgramError::InvalidInstructionData);
            }

            let text = core::str::from_utf8(data)
                .map_err(|_| ProgramError::InvalidInstructionData)?;

            msg!("Echo: {}", text);
        }
        _ => {
            return Err(ProgramError::InvalidInstructionData);
        }
    }

    Ok(())
}
