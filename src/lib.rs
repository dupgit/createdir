use bash_builtins::{builtin_metadata, Args, Builtin, BuiltinOptions, Result};
use std::fs;
use std::path::Path;

builtin_metadata!(
    name = "createdir",
    create = CreateDir::default,
    short_doc = "createdir directory",
    long_doc = "
    Creates 'directory' directory and all its parents if needed.
    ",
);

#[derive(BuiltinOptions)]
enum Opt {
    #[allow(dead_code)] // The field here is used inside BuiltinOptions derivation
    #[opt = 'i']
    Identifier(String),
}

#[derive(Default)]
struct CreateDir;

impl Builtin for CreateDir {
    fn call(&mut self, args: &mut Args) -> Result<()> {
        for name in args.string_arguments().flatten() {
            fs::create_dir_all(Path::new(name))?;
        }
        Ok(())
    }
}
