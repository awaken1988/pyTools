use std::io;
use std::fs::{self, DirEntry};
use std::path::Path;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

#[derive(Debug)]
struct PathTree {
    children: HashMap<String, Rc<PathTree>>,
    parent: Option<Rc<PathTree>>,
    name : String,
    is_dir : bool,
}

impl PathTree {
    fn walkprint(&self, depth: u32, f: &mut fmt::Formatter)
    {
        for i in 0..depth {
            write!(f, "    ");
        }

        write!(f, "{}\n", self.name);

        if self.is_dir {
            for child in self.children.values() {
                child.walkprint( depth+1, f );
            }
        }
    }
}

impl fmt::Display for PathTree {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.walkprint(0, f);

        return Ok(());
    }
}



fn walkdir(dir: &Path) -> Rc<PathTree>
{
    let mut ret = PathTree{ name: dir.file_name().unwrap().to_str().unwrap().to_string(),
                            parent: None,
                            is_dir: false,
                            children: HashMap::new()};
    if dir.is_file()  {
        //println!("{}", dir.to_str().unwrap());
    } else {
        let dir_entries = fs::read_dir(dir.to_str().unwrap()).unwrap();
        ret.is_dir = true;

        for entry in dir_entries {
            let child = walkdir(& entry.unwrap().path());

            ret.children.insert(child.name.clone(), child);
            //child.parent = Some(Rc::new(ret))
        }
    }
    return Rc::new(ret);
}

fn compare_dir(left: &PathTree, right: &PathTree)
{
    for (iPath, iChild) in left.children.iter() {
        if( !right.children.contains_key(iPath) ) {
            println!("-{}", iPath)
        } else {
            compare_dir(&*left.children[iPath],&*right.children[iPath]);
        }
    }
}

fn main()
{
    let left = walkdir(Path::new("./left"));
    let right = walkdir(Path::new("./right"));
    //walkprint(&path_tree, 0);
    //println!("{}", path_tree);

    compare_dir(&*left, &*right);
}
