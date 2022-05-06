use solana_program::{program_error::ProgramError};
use std::convert::TryInto;

#[derive(Debug)]
pub enum HelloInstruction {
    SayHello,
    SayBye,
    SetValue(u32),
}
impl HelloInstruction {
    //处理instruction, self means HelloInstruction
    //split_first返回切片的第一个元素和剩下的，这里每一个元素是u8
    pub fn unpack(input: &[u8]) -> Result<Self,ProgramError> {
        let (&tag, rest) = input.split_first().ok_or(ProgramError::InvalidInstructionData)?;
        // Ok(match tag {
        //     0 => HelloInstruction::SayHello,
        //     1 => HelloInstruction::SayBye,
        //     _ => return Err(ProgramError::InvalidInstructionData),
        // })
        match tag {
            0 => return Ok(HelloInstruction::SayHello),
            1 => return Ok(HelloInstruction::SayBye),
            2 => {   
                if rest.len() != 4 {
                    return Err(ProgramError::InvalidInstructionData);
                }    
                let val: Result<[u8;4],_> = rest[..4].try_into();  //32位，前8个字节
                match val {
                    Ok(i) => {
                        return Ok(HelloInstruction::SetValue(u32::from_le_bytes(i)))//小端字节序（右->左）
                    },
                    _ => return Err(ProgramError::InvalidInstructionData)
                }
            },
            _ => Err(ProgramError::InvalidInstructionData)
        }
    } 
}