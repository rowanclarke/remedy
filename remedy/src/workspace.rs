use std::{
    io::{Read, Write},
    rc::Rc,
};

pub mod fs;

pub enum Access {
    Read,
    Write,
}

pub trait Workspace: Sized {
    type File: Read + Write;

    fn get_file(&self, path: &[Rc<str>], access: Access) -> Self::File;
    fn get_children(&self, path: &[Rc<str>]) -> Vec<(Rc<str>, bool)>;
    fn get_root_path(&self) -> &[Rc<str>];
    fn get_config_path(&self) -> &[Rc<str>];

    fn read_config(&self, path: &[Rc<str>], buf: &mut String) -> usize {
        self.get_file(&self.from_config(path), Access::Read)
            .read_to_string(buf)
            .unwrap()
    }

    fn from_config(&self, path: &[Rc<str>]) -> Vec<Rc<str>> {
        let mut absolute_path = self.get_config_path().to_vec();
        absolute_path.extend_from_slice(path);
        absolute_path
    }

    fn from_root(&self, path: &[Rc<str>]) -> Vec<Rc<str>> {
        let mut absolute_path = self.get_root_path().to_vec();
        absolute_path.extend_from_slice(path);
        absolute_path
    }

    fn get_descendants(&self, path: &[Rc<str>], ignore: &[&[Rc<str>]]) -> Vec<Entry> {
        fn add_descendants<W: Workspace>(
            workspace: &W,
            path: Vec<Rc<str>>,
            from: usize,
            ignore: &[&[Rc<str>]],
            descendants: &mut Vec<Entry>,
        ) {
            let child_ignore: Vec<_> = ignore
                .iter()
                .filter(|p| p.len() > 1)
                .map(|p| &p[1..])
                .collect();
            for (name, is_dir) in workspace.get_children(&path) {
                if ignore.iter().find(|i| name == i[0]).is_some() {
                    continue;
                }
                let mut path = path.clone();
                path.push(name);
                if is_dir {
                    add_descendants(workspace, path, from, &child_ignore, descendants);
                } else {
                    let a = &path[from..];
                    descendants.push(Entry::new(path.into(), from));
                }
            }
        }
        let mut descendants: Vec<Entry> = Vec::new();
        add_descendants(self, path.into(), path.len(), ignore, &mut descendants);
        descendants
    }
}

#[derive(Debug)]
pub struct Entry {
    pub absolute_path: Rc<[Rc<str>]>,
    pub relative_path: Rc<[Rc<str>]>,
}

impl Entry {
    fn new(absolute_path: Rc<[Rc<str>]>, from: usize) -> Self {
        Self {
            relative_path: absolute_path[from..].into(),
            absolute_path,
        }
    }
}
