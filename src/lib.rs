use std::collections::HashMap;
use std::path::Path;

#[derive(Debug)]
pub enum FileSystemItem {
    File {
        name: String,
        extension: String,
        path: String,
        args: Vec<String>,
    },
    Folder {
        name: String,
        path: String,
        open: bool,
        args: Vec<String>,
        items: HashMap<String, FileSystemItem>,
    },
}

impl FileSystemItem {
    pub fn new(path: String, args: Vec<String>) -> Self {
        let path = Path::new(&path);
        let name = path.file_name().unwrap().to_str().unwrap().to_string();
        let folder_path = path.parent().unwrap().to_str().unwrap().to_string();
        FileSystemItem::Folder {
            name,
            path: folder_path,
            open: false,
            args,
            items: HashMap::new(),
        }
    }

    pub fn new_file(path: String, args: Vec<String>) -> Self {
        let path = Path::new(&path);
        let name = path.file_name().unwrap().to_str().unwrap().to_string();
        let extension = path.extension().unwrap().to_str().unwrap().to_string();
        FileSystemItem::File {
            name,
            extension,
            path: path.to_str().unwrap().to_string(),
            args,
        }
    }

    pub fn new_folder(path: String, args: Vec<String>) -> Self {
        let path = Path::new(&path);
        let name = path.file_name().unwrap().to_str().unwrap().to_string();
        let folder_path = path.parent().unwrap().to_str().unwrap().to_string();
        FileSystemItem::Folder {
            name: name.clone(),
            path: folder_path + "/" + &name,
            open: false,
            args,
            items: HashMap::new(),
        }
    }

    pub fn add_item(&mut self, item: FileSystemItem) {
        if let FileSystemItem::Folder { items, .. } = self {
            let item_name = match &item {
                FileSystemItem::File { name, .. } => name,
                FileSystemItem::Folder { name, .. } => name,
            };
            items.insert(item_name.clone(), item);
        }
    }

    pub fn set_open_state(&mut self, path: &str, open: bool) {
        if let FileSystemItem::Folder { path: folder_path, items, .. } = self {
            if *folder_path == path {
                if let FileSystemItem::Folder { open: folder_open, .. } = self {
                    *folder_open = open;
                }
            } else {
                for item in items.values_mut() {
                    item.set_open_state(path, open);
                }
            }
        }
    }

    pub fn remove_item(&mut self, path: &str) {
        if let FileSystemItem::Folder { items, .. } = self {
            items.retain(|_, item| {
                if let FileSystemItem::Folder { path: item_path, .. } = item {
                    *item_path != path
                } else {
                    true
                }
            });
        }
    }

    pub fn get_data(&self, path: &str) -> Vec<String> {
        if let FileSystemItem::Folder { items, .. } = self {
            for item in items.values() {
                if let FileSystemItem::File { name, extension, path: item_path, args } = item {
                    if *item_path == path {
                        return vec![
                            "file".to_string(),
                            name.to_string(),
                            path.to_string(),
                            extension.to_string(),
                            args.join(", "),
                        ];
                    }
                } else if let FileSystemItem::Folder { name, path: item_path, open, args, items: sub_items } = item {
                    if *item_path == path {
                        let subfiles = sub_items.keys().cloned().collect::<Vec<String>>().join(", ");
                        return vec![
                            "folder".to_string(),
                            name.to_string(),
                            path.to_string(),
                            open.to_string(),
                            args.join(", "),
                            subfiles,
                        ];
                    }
                }
            }
        }
        vec![]
    }
}
