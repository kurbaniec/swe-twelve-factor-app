use crate::errors::std_error::StdError;
use std::fs;
use std::fs::File;
use std::path::Path;
use zip::ZipArchive;

pub fn unzip(archive: &mut ZipArchive<File>, path: &Path) -> Result<(), StdError> {
    // Source: https://github.com/zip-rs/zip/blob/master/examples/extract.rs
    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let outpath = match file.enclosed_name() {
            Some(path) => path.to_owned(),
            None => continue,
        };
        let outpath = path.join(outpath);

        if (*file.name()).ends_with('/') {
            fs::create_dir_all(&outpath)?;
        } else {
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(&p)?;
                }
            }
            let mut outfile = fs::File::create(&outpath)?;
            std::io::copy(&mut file, &mut outfile)?;
        }
    }
    Ok(())
}
