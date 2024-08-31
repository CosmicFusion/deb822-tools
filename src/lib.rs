use std::path::PathBuf;
use deb822_lossless::Deb822;
use std::fs::File;

use std::io::Write;

#[derive(Debug, Clone, Default)]
pub struct Deb822Repository {
    pub filepath: String,
    pub enabled: Option<String>,
    pub types: Option<String>,
    pub uris: Option<String>,
    pub suites: Option<String>,
    pub components: Option<String>,
    pub architectures: Option<String>,
    pub languages: Option<String>,
    pub targets: Option<String>,
    pub pdiffs: Option<String>,
    pub by_hash: Option<String>,
    pub allow_insecure: Option<String>,
    pub allow_weak: Option<String>,
    pub allow_downgrade_to_insecure: Option<String>,
    pub trusted: Option<String>,
    pub signed_by: Option<String>,
    pub check_valid_until: Option<String>,
    pub valid_until_min: Option<String>,
    pub check_date: Option<String>,
    pub date_max_future: Option<String>,
    pub inrelease_path: Option<String>,
    pub snapshot: Option<String>,
    pub repolib_name: Option<String>,
    pub repolib_id: Option<String>,
    pub repolib_default_mirror: Option<String>,
}

impl Deb822Repository {
    pub fn fn_new_from_file(file_path: PathBuf) -> Deb822Repository {
        let deb822 = Deb822::from_file(&file_path).unwrap();
        let paragraph = deb822.paragraphs().nth(0).unwrap();
        let final_struct = Deb822Repository {
            filepath: file_path.to_string_lossy().to_string(),
            enabled: paragraph.get("Enabled"),
            types: paragraph.get("Types"),
            uris: paragraph.get("URIs"),
            suites: paragraph.get("Suites"),
            components: paragraph.get("Components"),
            architectures: paragraph.get("Architectures"),
            languages: paragraph.get("Languages"),
            targets: paragraph.get("Targets"),
            pdiffs: paragraph.get("PDiffs"),
            by_hash: paragraph.get("By-Hash"),
            allow_insecure: paragraph.get("Allow-Insecure"),
            allow_weak: paragraph.get("Allow-Weak"),
            allow_downgrade_to_insecure: paragraph.get("Downgrade-To-Insecure"),
            trusted: paragraph.get("Trusted"),
            signed_by: paragraph.get("Signed-By"),
            check_valid_until: paragraph.get("Check-Valid-Until"),
            valid_until_min:  paragraph.get("Valid-Until-Min"),
            check_date: paragraph.get("Check-Date"),
            date_max_future:  paragraph.get("Date-Max-Future"),
            inrelease_path:  paragraph.get("InRelease-Path"),
            snapshot: paragraph.get("Snapshot"),
            repolib_name: paragraph.get("X-Repolib-Name"),
            repolib_id: paragraph.get("X-Repolib-ID"),
            repolib_default_mirror: paragraph.get("X-Repolib-Default-Mirror")
        };
        final_struct
    }
    
    pub fn get_deb822_sources() -> std::io::Result<Vec<Self>> {
        let mut sources_vec = Vec::new();
        let sources_paths = std::fs::read_dir("/etc/apt/sources.list.d")?
            .filter_map(|res| res.ok())
            .map(|dir_entry| dir_entry.path())
            .filter_map(|path| {
                if path.extension().map_or(false, |ext| ext == "sources") {
                    Some(path)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        for source_path in sources_paths {
            sources_vec.push(Self::fn_new_from_file(source_path));
        }
        Ok(sources_vec)
    }

    pub fn write_to_file(self, file_path: PathBuf) -> std::io::Result<()> {
        let mut pharsed_output = String::new();
        match self.enabled {
            Some(t) => pharsed_output.push_str(&format!("Enabled: {}\n", t)),
            None => {}
        }
        match self.types {
            Some(t) => pharsed_output.push_str(&format!("Types: {}\n", t)),
            None => {}
        }
        match self.uris {
            Some(t) => pharsed_output.push_str(&format!("URIs: {}\n", t)),
            None => {}
        }
        match self.suites {
            Some(t) => pharsed_output.push_str(&format!("Suites: {}\n", t)),
            None => {}
        }
        match self.components {
            Some(t) => pharsed_output.push_str(&format!("Components: {}\n", t)),
            None => {}
        }
        match self.architectures {
            Some(t) => pharsed_output.push_str(&format!("Architectures: {}\n", t)),
            None => {}
        }
        match self.languages {
            Some(t) => pharsed_output.push_str(&format!("Languages: {}\n", t)),
            None => {}
        }
        match self.targets {
            Some(t) => pharsed_output.push_str(&format!("Targets: {}\n", t)),
            None => {}
        }
        match self.pdiffs {
            Some(t) => pharsed_output.push_str(&format!("PDiffs: {}\n", t)),
            None => {}
        }
        match self.by_hash {
            Some(t) => pharsed_output.push_str(&format!("By-Hash: {}\n", t)),
            None => {}
        }
        match self.allow_insecure {
            Some(t) => pharsed_output.push_str(&format!("Allow-Insecure: {}\n", t)),
            None => {}
        }
        match self.allow_weak {
            Some(t) => pharsed_output.push_str(&format!("Allow-Weak: {}\n", t)),
            None => {}
        }
        match self.allow_downgrade_to_insecure {
            Some(t) => pharsed_output.push_str(&format!("Downgrade-To-Insecure: {}\n", t)),
            None => {}
        }
        match self.trusted {
            Some(t) => pharsed_output.push_str(&format!("Trusted: {}\n", t)),
            None => {}
        }
        match self.signed_by {
            Some(t) => pharsed_output.push_str(&format!("Signed-By: {}\n", t)),
            None => {}
        }
        match self.check_valid_until {
            Some(t) => pharsed_output.push_str(&format!("Check-Valid-Until: {}\n", t)),
            None => {}
        }
        match self.valid_until_min {
            Some(t) => pharsed_output.push_str(&format!("Valid-Until-Min: {}\n", t)),
            None => {}
        }
        match self.check_date {
            Some(t) => pharsed_output.push_str(&format!("Check-Date: {}\n", t)),
            None => {}
        }
        match self.date_max_future {
            Some(t) => pharsed_output.push_str(&format!("Date-Max-Future: {}\n", t)),
            None => {}
        }
        match self.inrelease_path {
            Some(t) => pharsed_output.push_str(&format!("InRelease-Path: {}\n", t)),
            None => {}
        }
        match self.snapshot {
            Some(t) => pharsed_output.push_str(&format!("Snapshot: {}\n", t)),
            None => {}
        }
        match self.repolib_name {
            Some(t) => pharsed_output.push_str(&format!("X-Repolib-Name: {}\n", t)),
            None => {}
        }
        match self.repolib_id {
            Some(t) => pharsed_output.push_str(&format!("X-Repolib-ID: {}\n", t)),
            None => {}
        }
        match self.repolib_default_mirror {
            Some(t) => pharsed_output.push_str(&format!("X-Repolib-Default-Mirror: {}\n", t)),
            None => {}
        }
        let mut file = File::create(file_path)?;
        file.write_all(pharsed_output.as_bytes())?;
        Ok(())
    }
}