mod args;
mod chunk;
mod chunk_type;
mod commands;
mod img;

pub type ProgramError = Box<dyn std::error::Error>;
pub type ProgramResult<T> = std::result::Result<T, ProgramError>;

fn main() -> ProgramResult<()> {
    todo!()
}
