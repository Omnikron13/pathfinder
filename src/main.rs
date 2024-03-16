use std::{
   env,
};
use itertools::Itertools;

mod fs_node;
use crate::fs_node::FSNode;

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

