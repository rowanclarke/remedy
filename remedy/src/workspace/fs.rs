use std::{
    fs::{read_dir, File},
    path::{Path, PathBuf},
    rc::Rc,
};

use super::{Access, Workspace};

pub struct LocalWorkspace {
    root: Vec<Rc<str>>,
    config: Vec<Rc<str>>,
}

impl LocalWorkspace {
    pub fn new(root: PathBuf) -> Self {
        let mut config = dirs::config_dir().unwrap();
        config.push("remedy");
        Self {
            root: Self::into_components(root),
            config: Self::into_components(config),
        }
    }

    fn into_components<P: AsRef<Path>>(path: P) -> Vec<Rc<str>> {
        path.as_ref()
            .components()
            .map(|c| c.as_os_str().to_str().unwrap().into())
            .collect()
    }
}

impl Workspace for LocalWorkspace {
    type File = File;

    fn get_file(&self, path: &[Rc<str>], access: super::Access) -> Self::File {
        let path = PathBuf::from_iter(path.iter().map(|s| s.as_ref()));
        match access {
            Access::Read => File::open(path).unwrap(),
            Access::Write => File::create(path).unwrap(),
        }
    }

    fn get_children(&self, path: &[Rc<str>]) -> Vec<(Rc<str>, bool)> {
        let path = PathBuf::from_iter(path.iter().map(|s| s.as_ref()));
        read_dir(&path)
            .unwrap()
            .filter_map(Result::ok)
            .map(move |entry| {
                (
                    entry.file_name().into_string().unwrap().into(),
                    entry.metadata().unwrap().is_dir(),
                )
            })
            .collect()
    }

    fn get_root_path(&self) -> &[Rc<str>] {
        &self.root
    }

    fn get_config_path(&self) -> &[Rc<str>] {
        &self.config
    }
}
