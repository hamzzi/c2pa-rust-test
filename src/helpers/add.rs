use anyhow::Result;
use c2pa::{create_signer, Builder, SigningAlg};
use std::path::PathBuf;
use tempfile::tempdir;
use ureq::serde::{self, Serialize};

#[derive(Serialize)]
struct Test {
    my_tag: usize,
}

pub fn add() -> Result<()> {
    let mut builder = Builder::from_json(r#"{"title": "Test"}"#)?;
    builder.add_assertion("org.contentauth.test", &Test { my_tag: 42 })?;

    // Create a ps256 signer using certs and key files
    let signer = create_signer::from_files(
        "../../tests/fixtures/certs/ps256.pub",
        "../../tests/fixtures/certs/ps256.pem",
        SigningAlg::Ps256,
        None,
    )?;

    // embed a manifest using the signer
    std::fs::remove_file("../../tmp/lib_sign.jpg"); // ensure the file does not exist
    builder.sign_file(
        &*signer,
        "../../tests/fixtures/C.jpg",
        ".../../tmp/lib_sign.jpg",
    )?;
    Ok(())
}
