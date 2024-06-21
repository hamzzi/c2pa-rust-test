use anyhow::Result;

pub fn show() -> Result<()> {
    use std::io::Read;

    use c2pa::{format_from_path, Error, Reader};

    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        let path = std::path::PathBuf::from(&args[1]);
        let format = format_from_path(&path).ok_or(Error::UnsupportedType)?;
        let mut file = std::fs::File::open(&path)?;

        let reader = match Reader::from_stream(&format, &mut file) {
            Ok(reader) => Ok(reader),
            Err(Error::RemoteManifestUrl(url)) => {
                println!("Fetching remote manifest from {}", url);
                let mut c2pa_data = Vec::new();
                let resp = ureq::get(&url).call()?;
                resp.into_reader().read_to_end(&mut c2pa_data)?;
                Reader::from_manifest_data_and_stream(&c2pa_data, &format, &mut file)
            }
            Err(Error::JumbfNotFound) => {
                // if not embedded or cloud, check for sidecar first and load if it exists
                let potential_sidecar_path = path.with_extension("c2pa");
                if potential_sidecar_path.exists() {
                    let manifest_data = std::fs::read(potential_sidecar_path)?;
                    Ok(Reader::from_manifest_data_and_stream(
                        &manifest_data,
                        &format,
                        &mut file,
                    )?)
                } else {
                    Err(Error::JumbfNotFound)
                }
            }
            Err(e) => Err(e),
        }?;
        println!("{reader}");
    } else {
        println!("Prints a manifest report (requires a file path argument)")
    }
    Ok(())
}
