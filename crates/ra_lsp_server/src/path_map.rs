use std::{
    fmt,
    path::{Component, Path, PathBuf},
};

use im;
use ra_analysis::{FileId};
use relative_path::RelativePath;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Root {
    Workspace,
    Lib,
}

#[derive(Default, Clone)]
pub struct PathMap {
    next_id: u32,
    path2id: im::HashMap<PathBuf, FileId>,
    id2path: im::HashMap<FileId, PathBuf>,
    id2root: im::HashMap<FileId, Root>,
}

impl fmt::Debug for PathMap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("PathMap { ... }")
    }
}

impl PathMap {
    pub fn get_or_insert(&mut self, path: PathBuf, root: Root) -> (bool, FileId) {
        let mut inserted = false;
        let file_id = self
            .path2id
            .get(path.as_path())
            .map(|&id| id)
            .unwrap_or_else(|| {
                inserted = true;
                let id = self.new_file_id();
                self.insert(path, id, root);
                id
            });
        (inserted, file_id)
    }
    pub fn get_id(&self, path: &Path) -> Option<FileId> {
        self.path2id.get(path).cloned()
    }
    pub fn get_path(&self, file_id: FileId) -> &Path {
        self.id2path.get(&file_id).unwrap().as_path()
    }
    pub fn get_root(&self, file_id: FileId) -> Root {
        self.id2root[&file_id]
    }
    fn insert(&mut self, path: PathBuf, file_id: FileId, root: Root) {
        self.path2id.insert(path.clone(), file_id);
        self.id2path.insert(file_id, path.clone());
        self.id2root.insert(file_id, root);
    }

    fn new_file_id(&mut self) -> FileId {
        let id = FileId(self.next_id);
        self.next_id += 1;
        id
    }
}

fn normalize(path: &Path) -> PathBuf {
    let mut components = path.components().peekable();
    let mut ret = if let Some(c @ Component::Prefix(..)) = components.peek().cloned() {
        components.next();
        PathBuf::from(c.as_os_str())
    } else {
        PathBuf::new()
    };

    for component in components {
        match component {
            Component::Prefix(..) => unreachable!(),
            Component::RootDir => {
                ret.push(component.as_os_str());
            }
            Component::CurDir => {}
            Component::ParentDir => {
                ret.pop();
            }
            Component::Normal(c) => {
                ret.push(c);
            }
        }
    }
    ret
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_resolve() {
        let mut m = PathMap::default();
        let (_, id1) = m.get_or_insert(PathBuf::from("/foo"), Root::Workspace);
        let (_, id2) = m.get_or_insert(PathBuf::from("/foo/bar.rs"), Root::Workspace);
        assert_eq!(m.resolve(id1, &RelativePath::new("bar.rs")), Some(id2),)
    }
}
