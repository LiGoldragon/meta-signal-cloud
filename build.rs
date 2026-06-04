use std::{env, path::PathBuf};

use schema_rust_next::build::CargoSchemaMetadata;

fn main() {
    SchemaBuild::from_environment().run();
}

struct SchemaBuild {
    crate_root: PathBuf,
}

impl SchemaBuild {
    fn from_environment() -> Self {
        Self {
            crate_root: PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").expect("manifest dir set")),
        }
    }

    fn run(&self) {
        println!("cargo:rerun-if-changed=schema/lib.schema");
        CargoSchemaMetadata::new("meta-signal-cloud").emit_schema_directory(&self.crate_root);
    }
}
