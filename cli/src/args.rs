use std::path::PathBuf;

use clap::Parser;

use crate::metadata::FILE_NAME;

#[derive(Parser, Debug)]
#[allow(clippy::struct_excessive_bools)]
pub struct Args {
    /// Defines where your TS bindings will be saved by setting TS_RS_EXPORT_DIR
    #[arg(long, short)]
    pub output_directory: PathBuf,

    /// Disables warnings caused by using serde attributes that ts-rs cannot process
    #[arg(long)]
    pub no_warnings: bool,

    /// Adds the ".js" extension to import paths
    #[arg(long)]
    pub esm_imports: bool,

    /// Formats the generated TypeScript files
    #[arg(long)]
    pub format: bool,

    /// Generates an index.ts file in your --output-directory that re-exports all
    /// types generated by ts-rs
    #[arg(long = "index")]
    pub generate_index_ts: bool,

    /// Generates only a single index.ts file in your --output-directory that
    /// contains all exported types
    #[arg(long = "merge")]
    pub merge_files: bool,

    /// Do not capture `cargo test`'s output, and pass --nocapture to the test binary
    #[arg(long = "nocapture")]
    pub no_capture: bool,
}

// Args is in scope for the entirety of the main function, so this will only
// be executed when the program is finished running. This helps prevent us
// from forgetting to do cleanup if some code branch early returns from main
impl Drop for Args {
    fn drop(&mut self) {
        _ = std::fs::remove_file(self.output_directory.join(FILE_NAME));
    }
}
