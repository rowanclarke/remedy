use std::io::Read;
use trql::query::execute;

use crate::{
    parser::{get_root_vm, get_vm},
    pest::Pairs,
    workspace::{fs::LocalWorkspace, Access, Workspace},
};

#[test]
fn read_config() {
    let workspace = LocalWorkspace::new("/home/rowan/wkspc".into());
    let root_vm = get_root_vm(&workspace).unwrap();
    let vm = get_vm(&workspace).unwrap();
    workspace
        .get_descendants(workspace.get_root_path(), &[&[".git".into()]])
        .into_iter()
        .map(|path| {
            (
                root_vm
                    .parse("root", &path.relative_path.join("/"))
                    .unwrap()
                    .next()
                    .unwrap()
                    .as_rule()
                    .to_owned(),
                path.absolute_path,
            )
        })
        .for_each(|(rule, path)| {
            let mut file = workspace.get_file(&path, Access::Read);
            let mut buf = String::new();
            file.read_to_string(&mut buf).unwrap();
            let pairs: Pairs = vm.parse(&rule, &buf).unwrap().into();
            println!("{:?}", execute::<_, String>("book.author", pairs));
        });
}
