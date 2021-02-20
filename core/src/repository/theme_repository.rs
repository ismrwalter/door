use std::fs::{self, DirBuilder, File};
use std::io;
use std::io::{Read, Write};
use std::path::Path;

use colored::*;
use regex::Regex;

use crate::entity::{SiteEntity, ThemeEntity};
use crate::infrastructure::Resource;
use crate::infrastructure::*;

type Result<T> = std::result::Result<T, Error>;

pub trait ThemeRepository {
    fn templates(&self) -> Vec<String>;
    fn template(&self, name: &str) -> Option<String>;
    fn save_assets(&self, path: &str) -> Result<()>;
    fn load(&self, filename: &str) -> Result<ThemeEntity>;
}

pub struct LocalThemeRepository<'a> {
    pub path: &'a str,
}

impl<'a> LocalThemeRepository<'a> {
    pub fn new(path: &'a str) -> LocalThemeRepository<'a> {
        LocalThemeRepository { path }
    }
}

impl<'a> ThemeRepository for LocalThemeRepository<'a> {
    fn templates(&self) -> Vec<String> {
        return vec![];
    }
    fn template(&self, name: &str) -> Option<String> {
        todo!()
    }
    fn save_assets(&self, path: &str) -> Result<()> {
        todo!()
    }
    fn load(&self, filename: &str) -> Result<ThemeEntity> {
        return Ok(ThemeEntity {
            name: "".to_string(),
            path: "".to_string(),
        });
    }
}

pub struct DefaultThemeRepository<'a> {
    pub path: &'a str,
}

impl<'a> DefaultThemeRepository<'a> {
    pub fn new(path: &'a str) -> DefaultThemeRepository<'a> {
        DefaultThemeRepository { path }
    }
}

impl<'a> ThemeRepository for DefaultThemeRepository<'a> {
    fn templates(&self) -> Vec<String> {
        let layout_names: Vec<String> = Resource::list("theme/layouts")
            .into_iter()
            .map(|path| {
                String::from(
                    Path::new(&path)
                        .strip_prefix("theme/layouts")
                        .unwrap()
                        .to_str()
                        .unwrap(),
                )
            })
            .collect();
        return layout_names;
    }
    fn template(&self, name: &str) -> Option<String> {
        let path = Path::new("theme/layouts").join(name);
        return Some(Resource::get_text_content(path).to_string());
    }

    fn save_assets(&self, path: &str) -> Result<()> {
        let asset_paths: Vec<String> = Resource::list("theme/assets");
        for asset_path in asset_paths {
            let asset_content = Resource::get_content(&asset_path);
            let target_asset_path = Path::new(path).join(asset_path);
            let mut file = File::create(target_asset_path).unwrap();
            file.write_all(asset_content).unwrap();
        }
        return Ok(());
    }

    fn load(&self, filename: &str) -> Result<ThemeEntity> {
        let config = Resource::get_text_content("theme/theme.json");
        let theme = serde_json::from_str::<ThemeEntity>(config).unwrap();
        return Ok(theme);
    }
}

pub enum ThemeRepositorys<'a> {
    DefaultThemeRepository(DefaultThemeRepository<'a>),
    LocalThemeRepository(LocalThemeRepository<'a>),
}

#[cfg(test)]
mod tests {
    use crate::repository::{LocalSiteRepository, SiteRepository};
    use filesystem::{FakeFileSystem, TempDir, TempFileSystem};

    use crate::infrastructure::Environment;

    use super::*;

    lazy_static! {
        static ref MOCK_FILESYSTEM: FakeFileSystem = FakeFileSystem::new();
    }

    #[test]
    fn layouts() {
        let temp_fs = MOCK_FILESYSTEM.temp_dir("mysite").unwrap();
        let workspace = temp_fs.path().to_str().unwrap();
        let environment = Environment::new(".", workspace);
        let site_repository = LocalSiteRepository::new(&environment);
        let site = site_repository.create().unwrap();
        let list = DefaultThemeRepository::new("").templates();
        println!("{:?}", list);
    }
}
