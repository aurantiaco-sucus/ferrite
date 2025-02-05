use std::path::Path;
use std::process::{Command, ExitStatus, Stdio};
use std::{fs, io};
use std::fs::File;
use std::io::BufWriter;

pub fn download_file(url: &str, output: impl AsRef<Path>) -> Result<(), DownloadError> {
    let path = output.as_ref();
    
    #[cfg(target_family = "unix")] if fs::exists("/bin/curl").map_err(DownloadError::Io)? {
        let mut cmd = Command::new("curl")
            .args(["-L", "-o"])
            .arg(path)
            .arg(url)
            .stdout(Stdio::inherit())
            .spawn().map_err(DownloadError::Io)?;
        let stat = cmd.wait().map_err(DownloadError::Io)?;
        if !stat.success() {
            return Err(DownloadError::Curl(stat))
        }
        return Ok(())
    }
    
    #[cfg(feature = "reqwest")] {
        let file = File::create(path).map_err(DownloadError::Io)?;
        let mut file = BufWriter::new(file);
        let client = reqwest::blocking::Client::builder()
            .user_agent(format!("ferrite/{} (reqwest)", env!("CARGO_PKG_VERSION")))
            .build().map_err(DownloadError::Reqwest)?;
        let mut resp = client.get(url).send().map_err(DownloadError::Reqwest)?;
        io::copy(&mut resp, &mut file).map_err(DownloadError::Io)?;
        return Ok(())
    }
    
    #[allow(unreachable_code)]
    Err(DownloadError::NoMethod)
}

pub enum DownloadError {
    Io(io::Error),
    Curl(ExitStatus),
    #[cfg(feature = "reqwest")]
    Reqwest(reqwest::Error),
    NoMethod,
}