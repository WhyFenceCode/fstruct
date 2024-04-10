use std::collections::HashMap;
use std::any::Any;
use std::fmt;

// Define the data structure for a file
#[derive(Debug)]
struct File {
    name: String,
    path: String,
    extension: String,
    args: Vec<String>,
}

// Define the data structure for a folder
#[derive(Debug)]
struct Folder {
    name: String,
    path: String,
    open: bool,
    args: Vec<String>,
    contents: HashMap<String, Box<dyn Any>>,
}

// Define a trait for both File and Folder to allow polymorphic behavior
trait FolderOrFile: Any {
    fn get_name(&self) -> &str;
    fn get_path(&self) -> &str;
    fn get_args(&self) -> &Vec<String>;
}

impl FolderOrFile for File {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_path(&self) -> &str {
        &self.path
    }

    fn get_args(&self) -> &Vec<String> {
        &self.args
    }
}

impl FolderOrFile for Folder {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_path(&self) -> &str {
        &self.path
    }

    fn get_args(&self) -> &Vec<String> {
        &self.args
    }
}

// Manually implement Debug for FolderOrFile
impl fmt::Debug for dyn FolderOrFile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "FolderOrFile")
    }
}

// Helper function to downcast a reference to Any to FolderOrFile
fn downcast_folder_or_file<T: 'static>(any: Box<dyn Any>) -> Option<T> {
    any.downcast().ok()
}

// Define the main structure
#[derive(Debug)]
struct FStruct {
    root: Folder,
}

impl FStruct {
    fn new(path: &str, args: Vec<String>) -> Self {
        let name = path.split('/').last().unwrap().to_string();
        let root = Folder {
            name,
            path: path.to_string(),
            open: false,
            args,
            contents: HashMap::new(),
        };
        FStruct { root }
    }

    fn new_folder(&mut self, path: &str, args: Vec<String>) {
        let name = path.split('/').last().unwrap().to_string();
        let folder = Folder {
            name,
            path: path.to_string(),
            open: false,
            args,
            contents: HashMap::new(),
        };
        self.root.contents.insert(name, Box::new(folder));
    }

    fn new_file(&mut self, path: &str, args: Vec<String>) {
        let parts: Vec<&str> = path.split('/').collect();
        let name = parts.last().unwrap().to_string();
        let extension = parts.last().unwrap().split('.').last().unwrap().to_string();
        let file = File {
            name,
            path: path.to_string(),
            extension,
            args,
        };
        self.root.contents.insert(name, Box::new(file));
    }

    fn set_open_state(&mut self, path: &str, open: bool) {
        let parts: Vec<&str> = path.split('/').collect();
        let name = parts.last().unwrap().to_string();
        if let Some(entry) = self.root.contents.get_mut(&name) {
            if let Some(folder) = downcast_folder_or_file::<Folder>(entry.clone()) {
                folder.open = open;
                *entry = Box::new(folder);
            }
        }
    }

    fn get_open_state(&self, path: &str) -> Option<bool> {
        let parts: Vec<&str> = path.split('/').collect();
        let name = parts.last().unwrap().to_string();
        if let Some(entry) = self.root.contents.get(&name) {
            if let Some(folder) = downcast_folder_or_file::<Folder>(entry.clone()) {
                return Some(folder.open);
            }
        }
        None
    }

    fn get_data(&self, path: &str) -> Option<&Box<dyn FolderOrFile>> {
        let parts: Vec<&str> = path.split('/').collect();
        let name = parts.last().unwrap().to_string();
        self.root.contents.get(&name)
    }

    fn remove(&mut self, path: &str) {
        let parts: Vec<&str> = path.split('/').collect();
        let name = parts.last().unwrap().to_string();
        self.root.contents.remove(&name);
    }
}