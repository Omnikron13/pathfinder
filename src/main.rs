#![feature(ptr_metadata)]

use std::{
   env,
   fmt,
   fs,
   io,
   path::PathBuf,
   //ptr::metadata,
};
use itertools::Itertools;

fn main() {
   if let Ok(fsn) = FSNode::read_path(env::current_dir().unwrap(), 3) {
      //println!("fsn: {fsn:?}");

      for child in fsn.child_dirs().sorted() {
         println!("dir: {child}");
         for child in child.child_dirs().sorted() {
            println!("  dir: {child}");
         }
         for child in child.child_files().sorted() {
            println!("  file: {child}");
         }
      }

      for child in fsn.child_files().sorted() {
         println!("file: {child}");
      }
   }
   else {
      panic!("CANT READ PATH");
   }
}


#[derive(Debug)]
#[derive(PartialOrd, PartialEq, Ord, Eq)]
enum FSNode {
   File {
      name: String,
   },
   Directory {
      name: String,
      children: Vec<FSNode>,
   },
}

impl FSNode {
   fn new_file(name: String) -> Self {
      FSNode::File {
         name,
      }
   }

   fn new_directory(name: String) -> Self {
      FSNode::Directory {
         name,
         children: Vec::new(),
      }
   }

   fn read_path(path: PathBuf, depth: u8) -> io::Result<FSNode> {
      let metadata = fs::metadata(&path)?;
      if let Some(name) = path.file_name() {
         let name = name.to_string_lossy().to_string();

         if metadata.is_file() {
            return Ok(FSNode::new_file(name));
         }
         if metadata.is_dir() {
            if depth == 0 {
               return Ok(FSNode::new_directory(name));
            }
            let contents = fs::read_dir(path)?;
            let mut dir = FSNode::new_directory(name);
            for child in contents {
               if let Ok(child) = child {
                  let child = FSNode::read_path(child.path(), depth - 1)?;
                  dir.add_child(child);
               }
               else {
                  return Err(io::Error::new(io::ErrorKind::Other, "CANT READ CHILD"));
               }
            }
            return Ok(dir);
         }
      }
      return Err(io::Error::new(io::ErrorKind::Other, "CANT GET FILE NAME"));
   }

   fn add_child(&mut self, child: FSNode) {
      if let FSNode::Directory{name: _, children} = self {
         children.push(child);
         return;
      }
      panic!("CANT ADD CHILD TO FILE");
   }

   fn child_dirs(&self) -> impl Iterator<Item = &FSNode> {
      if let FSNode::Directory{name: _, children} = self {
         return children.iter().filter(|c| match c {
            FSNode::Directory{name: _, children: _} => true,
            FSNode::File{name: _} => false,
         });
      }
      panic!("CANT GET CHILD DIRS OF FILE");
   }

   fn child_files(&self) -> impl Iterator<Item = &FSNode> {
      if let FSNode::Directory{name: _, children} = self {
         return children.iter().filter(|c| match c {
            FSNode::Directory{name: _, children: _} => false,
            FSNode::File{name: _} => true,
         });
      }
      panic!("CANT GET CHILD FILES OF FILE");
   }
}

impl fmt::Display for FSNode {
   fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      let name = match self {
         FSNode::File{name} => name,
         FSNode::Directory{name, children: _} => name,
      };
      write!(f, "{}", name)
   }
}

