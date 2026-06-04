use std::{
    env, fs,
    path::{Path, PathBuf},
};

use schema_next::{AsschemaArtifact, Name, SchemaEngine, SchemaPackage, SchemaSourceArtifact};
use schema_rust_next::{GeneratedFile, RustEmissionOptions, RustEmitter};

fn main() {
    SchemaBuild::from_environment().run();
}

struct SchemaBuild {
    crate_root: PathBuf,
    output_directory: PathBuf,
}

impl SchemaBuild {
    fn module_name() -> Name {
        Name::new("meta-signal-cloud")
    }

    fn from_environment() -> Self {
        Self {
            crate_root: PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").expect("manifest dir set")),
            output_directory: PathBuf::from(env::var_os("OUT_DIR").expect("out dir set")),
        }
    }

    fn run(&self) {
        println!("cargo:rerun-if-changed=schema/meta-signal-cloud.schema");
        println!("cargo:rerun-if-changed=schema/meta-signal-cloud.asschema");
        println!("cargo:rerun-if-changed=src/schema/meta_signal_cloud.rs");

        let generated = self.generated_schema_file();
        self.assert_generated_schema_path(&generated);
        self.assert_checked_in_schema_is_fresh(&generated);
    }

    fn generated_schema_file(&self) -> GeneratedFile {
        let package = SchemaPackage::new(&self.crate_root, "meta-signal-cloud", "0.1.0");
        let source = package
            .load_module(Self::module_name())
            .expect("read schema/meta-signal-cloud.schema");
        let source_artifact =
            SchemaSourceArtifact::new(source.to_schema_source().expect("decode schema source"));
        let source_file = GeneratedSchemaSourceFile::new(&self.output_directory);
        source_artifact
            .write_schema_file(source_file.path())
            .expect("write generated canonical schema source artifact");
        let recovered_source = SchemaSourceArtifact::read_schema_file(source_file.path())
            .expect("read generated canonical schema source artifact");
        if recovered_source != source_artifact {
            panic!(
                "generated schema source artifact did not round-trip through {}",
                source_file.path().display()
            );
        }

        let asschema = recovered_source
            .source()
            .lower(&SchemaEngine::default(), source.identity().clone())
            .expect("lower meta-signal-cloud schema source");
        let artifact = AsschemaArtifact::new(asschema);
        let artifact_files = GeneratedAsschemaArtifactFiles::new(&self.output_directory);
        artifact
            .write_nota_file(artifact_files.nota_path())
            .expect("write generated asschema NOTA artifact");
        artifact
            .write_binary_file(artifact_files.binary_path())
            .expect("write generated asschema rkyv artifact");

        let checked_in_artifact = CheckedInAsschemaArtifact::new(&self.crate_root);
        checked_in_artifact.assert_matches_generated_artifact(&artifact_files);

        RustEmitter::new(RustEmissionOptions::feature_gated_nota("nota-text"))
            .emit_file_from_nota_path(checked_in_artifact.path())
            .expect("emit Rust from checked-in asschema NOTA artifact")
            .assert_matches_binary_artifact(&artifact_files)
    }

    fn assert_generated_schema_path(&self, generated: &GeneratedFile) {
        if generated.path.as_str() != "src/schema/meta_signal_cloud.rs" {
            panic!(
                "meta-signal-cloud schema must emit src/schema/meta_signal_cloud.rs, found {}",
                generated.path
            );
        }
    }

    fn assert_checked_in_schema_is_fresh(&self, generated: &GeneratedFile) {
        let checked_in = CheckedInSchemaSource::new(&self.crate_root, generated);
        let actual = fs::read_to_string(checked_in.path()).unwrap_or_else(|error| {
            panic!(
                "checked-in generated schema source is missing at {}: {error}",
                checked_in.path().display()
            )
        });
        let expected = checked_in.expected_source();
        if actual != expected {
            if env::var_os("META_SIGNAL_CLOUD_UPDATE_SCHEMA_ARTIFACTS").is_some() {
                fs::write(checked_in.path(), expected).unwrap_or_else(|error| {
                    panic!(
                        "failed to update checked-in generated schema source at {}: {error}",
                        checked_in.path().display()
                    )
                });
                return;
            }
            panic!(
                "checked-in generated schema source is stale at {}; regenerate it from schema/meta-signal-cloud.schema",
                checked_in.path().display()
            );
        }
    }
}

struct CheckedInAsschemaArtifact {
    path: PathBuf,
}

impl CheckedInAsschemaArtifact {
    fn new(crate_root: &Path) -> Self {
        Self {
            path: crate_root.join("schema").join("meta-signal-cloud.asschema"),
        }
    }

    fn path(&self) -> &Path {
        &self.path
    }

    fn assert_matches_generated_artifact(&self, artifact_files: &GeneratedAsschemaArtifactFiles) {
        let generated = fs::read_to_string(artifact_files.nota_path())
            .expect("read generated asschema artifact");
        let update_requested = env::var_os("META_SIGNAL_CLOUD_UPDATE_SCHEMA_ARTIFACTS").is_some();
        let checked_in = match fs::read_to_string(self.path()) {
            Ok(checked_in) => checked_in,
            Err(_) if update_requested => {
                self.write_checked_in(&generated);
                return;
            }
            Err(error) => panic!(
                "checked-in assembled schema artifact is missing at {}: {error}",
                self.path().display()
            ),
        };
        if checked_in != generated {
            if update_requested {
                self.write_checked_in(&generated);
                return;
            }
            panic!(
                "checked-in assembled schema artifact is stale at {}; regenerate it from schema/meta-signal-cloud.schema",
                self.path().display()
            );
        }
    }

    fn write_checked_in(&self, generated: &str) {
        fs::write(self.path(), generated).unwrap_or_else(|error| {
            panic!(
                "failed to update checked-in assembled schema artifact at {}: {error}",
                self.path().display()
            )
        });
    }
}

struct GeneratedSchemaSourceFile(PathBuf);

impl GeneratedSchemaSourceFile {
    fn new(output_directory: &Path) -> Self {
        Self(output_directory.join("meta-signal-cloud.schema"))
    }

    fn path(&self) -> &Path {
        &self.0
    }
}

struct GeneratedAsschemaArtifactFiles {
    nota_path: PathBuf,
    binary_path: PathBuf,
}

impl GeneratedAsschemaArtifactFiles {
    fn new(output_directory: &Path) -> Self {
        Self {
            nota_path: output_directory.join("meta-signal-cloud.asschema"),
            binary_path: output_directory.join("meta-signal-cloud.asschema.rkyv"),
        }
    }

    fn nota_path(&self) -> &Path {
        &self.nota_path
    }

    fn binary_path(&self) -> &Path {
        &self.binary_path
    }
}

trait GeneratedFileArtifactWitness {
    fn assert_matches_binary_artifact(
        self,
        artifact_files: &GeneratedAsschemaArtifactFiles,
    ) -> Self;
}

impl GeneratedFileArtifactWitness for GeneratedFile {
    fn assert_matches_binary_artifact(
        self,
        artifact_files: &GeneratedAsschemaArtifactFiles,
    ) -> Self {
        let from_binary = RustEmitter::new(RustEmissionOptions::feature_gated_nota("nota-text"))
            .emit_file_from_binary_path(artifact_files.binary_path())
            .expect("emit Rust from generated asschema rkyv artifact");
        if self != from_binary {
            panic!(
                "generated Rust differs between asschema NOTA artifact {} and rkyv artifact {}",
                artifact_files.nota_path().display(),
                artifact_files.binary_path().display()
            );
        }
        self
    }
}

struct CheckedInSchemaSource<'schema> {
    crate_root: &'schema PathBuf,
    generated: &'schema GeneratedFile,
}

impl<'schema> CheckedInSchemaSource<'schema> {
    fn new(crate_root: &'schema PathBuf, generated: &'schema GeneratedFile) -> Self {
        Self {
            crate_root,
            generated,
        }
    }

    fn path(&self) -> PathBuf {
        self.crate_root.join(&self.generated.path)
    }

    fn expected_source(&self) -> String {
        self.generated.code.as_str().to_owned()
    }
}
