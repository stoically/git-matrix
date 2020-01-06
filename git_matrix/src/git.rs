use git2::{Buf, Config, Repository, Sort};

use crate::error::Error;

pub struct Git {
    pub repo: Repository,
}

pub struct Pack {
    pub content: Vec<u8>,
}

impl Git {
    pub fn new() -> Result<Git, Error> {
        let repo = Repository::open_from_env()?;
        Ok(Self { repo })
    }

    pub fn pack(&self, src: &str) -> Result<Pack, git2::Error> {
        let mut revwalk = self.repo.revwalk()?;
        revwalk.set_sorting(Sort::TIME);
        revwalk.push_ref(src)?;

        let mut packbuilder = self.repo.packbuilder()?;
        packbuilder.insert_walk(&mut revwalk)?;

        let mut buf = Buf::new();
        packbuilder.write_buf(&mut buf)?;

        Ok(Pack {
            content: buf.to_vec(),
        })
    }

    pub fn ref_id(&self, src: &str) -> Result<String, Error> {
        Ok(self.repo.refname_to_id(src)?.to_string())
    }
}

pub fn get_config() -> Result<Config, Error> {
    Ok(Config::open_default()?)
}
