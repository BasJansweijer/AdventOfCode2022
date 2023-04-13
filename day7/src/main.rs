use std::cell::RefCell;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::rc::Rc;

#[derive(Debug)]
struct FileStruct(String, usize);

impl FileStruct {
    fn new(input: &str) -> FileStruct {
        let (size, name) = input.split_once(' ').unwrap();
        FileStruct(name.to_string(), size.parse::<usize>().unwrap())
    }
}

#[derive(Debug)]
struct Dir {
    directories: Vec<Rc<RefCell<Dir>>>,
    files: Vec<FileStruct>,
    name: String,
    size_cache: usize,
}

impl Dir {
    fn new(name: String) -> Dir {
        Dir {
            directories: Vec::new(),
            files: Vec::new(),
            name: name,
            size_cache: 0,
        }
    }

    fn contains_dir(&self, name: &str) -> bool {
        for dir in self.directories.iter() {
            if dir.borrow().name == name {
                return true;
            }
        }
        return false;
    }

    fn contains_file(&self, name: &str) -> bool {
        for file in self.files.iter() {
            if file.0 == name {
                return true;
            }
        }
        return false;
    }
    /// returns a bool wether the directory was added (true if added)
    fn try_add_subdir(&mut self, new_dir: Dir) -> bool {
        if self.contains_dir(&new_dir.name) {
            return false;
        }
        self.directories.push(Rc::new(RefCell::new(new_dir)));
        true
    }

    fn add_file(&mut self, new_file: FileStruct) -> bool {
        if self.contains_file(&new_file.0) {
            return false;
        }
        self.files.push(new_file);
        true
    }

    fn get_subdir(&self, name: &str) -> Option<Rc<RefCell<Dir>>> {
        match self
            .directories
            .iter()
            .find(|dir| dir.borrow().name == name)
        {
            None => None,
            Some(found_dir) => Some(Rc::clone(found_dir)),
        }
    }

    fn update_cur_size(&mut self) -> usize {
        let mut total_size: usize = 0;
        for file in self.files.iter() {
            total_size += file.1;
        }
        for dir in self.directories.iter() {
            total_size += dir.borrow_mut().update_cur_size();
        }
        self.size_cache = total_size;
        return total_size;
    }
    fn get_size_less_then(&mut self, max_size: usize) -> usize {
        self.update_cur_size();
        let mut total_size = 0;
        if self.size_cache <= max_size {
            total_size = self.size_cache;
        }

        for dir in self.directories.iter() {
            total_size += dir.borrow_mut().get_size_less_then(max_size);
        }
        self.size_cache = total_size;
        return total_size;
    }

    fn get_dir_to_free(&mut self, needed_space: usize) -> Option<usize> {
        if self.size_cache < needed_space {
            return None;
        }

        let mut best_size = self.size_cache;
        for dir in self.directories.iter() {
            if let Some(local_best_size) = dir.borrow_mut().get_dir_to_free(needed_space) {
                if best_size >= local_best_size {
                    best_size = local_best_size;
                }
            }
        }
        return Some(best_size);
    }
}

fn build_directory_tree(input_file: &str) -> Dir {
    let data = fs::read_to_string(input_file).unwrap();
    let root = Rc::new(RefCell::new(Dir::new("/".to_string())));
    let mut dir_stack: Vec<Rc<RefCell<Dir>>> = vec![];
    let mut cur_dir = root;

    for line in data.lines() {
        // check if command
        if line.chars().nth(0) == Some('$') {
            // filter out ls
            if line.get(line.len() - 2..) == Some("ls") {
                continue;
            }
            let args = line.split_whitespace().collect::<Vec<&str>>();
            let dir_name = args.last().unwrap();

            if dir_name == &".." {
                cur_dir = dir_stack.pop().unwrap_or_else(|| cur_dir);
            } else if dir_name == &"/" {
                cur_dir = dir_stack.drain(..).next().unwrap_or_else(|| cur_dir);
            } else {
                dir_stack.push(Rc::clone(&cur_dir));
                cur_dir
                    .borrow_mut()
                    .try_add_subdir(Dir::new(dir_name.to_string()));
                let next_cur = cur_dir.borrow().get_subdir(dir_name).unwrap();
                cur_dir = next_cur;
            }
        } else if Some("dir") == line.get(..3) {
            // it's a dir in our cur_dir!
            let (_, name) = line.split_once(' ').unwrap();
            cur_dir
                .borrow_mut()
                .try_add_subdir(Dir::new(name.to_string()));
        } else {
            // it's a file in our cur_dir!
            cur_dir.borrow_mut().add_file(FileStruct::new(line));
        }
    }

    let root = match dir_stack.into_iter().next() {
        Some(fst_dir) => fst_dir,
        None => cur_dir,
    };
    Rc::try_unwrap(root).unwrap().into_inner()
}

fn main() {
    let mut root = build_directory_tree("day7.input");

    //let mut w = File::create("./test.txt").unwrap();
    //writeln!(&mut w, "{:#?}", root).unwrap();

    let result = root.get_size_less_then(100000);
    println!("{}", result);
    let root_s = root.update_cur_size();
    let space_left = 70000000 - root_s;
    let extra_space_needed = 30000000 - space_left;
    let dir_to_free = root.get_dir_to_free(extra_space_needed);
    println!("{:?}", dir_to_free);
}
