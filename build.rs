use std::error::Error;
use vergen_git2::{BuildBuilder, Emitter, Git2Builder};

fn main() -> Result<(), Box<dyn Error>> {
    let build = BuildBuilder::all_build()?;
    let git = Git2Builder::default()
        .branch(true)
        .sha(true)
        .commit_timestamp(true)
        .dirty(true)
        .describe(true, true, None)
        .build()?;
    Emitter::default()
        .add_instructions(&build)?
        .add_instructions(&git)?
        .emit()?;
    Ok(())
}
