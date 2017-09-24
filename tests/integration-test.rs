extern crate rustfat;

use std::fs::File;
use std::io::{BufReader, SeekFrom};
use std::io::prelude::*;
use std::str;

use rustfat::FatFileSystem;

const TEST_TEXT: &'static str = "Rust is cool!\n";

fn test_img(name: &str) {
    let file = File::open(name).unwrap();
    let buf_rdr = BufReader::new(file);
    let mut fs = FatFileSystem::new(Box::new(buf_rdr)).unwrap();
    let mut root_dir = fs.root_dir();
    let entries = root_dir.list().unwrap();
    let names = entries.iter().map(|e| e.get_name()).collect::<Vec<String>>();
    assert_eq!(names, ["LONG.TXT", "SHORT.TXT", "VERY"]);
    // Try read again
    let entries = root_dir.list().unwrap();
    let names2 = entries.iter().map(|e| e.get_name()).collect::<Vec<String>>();
    assert_eq!(names2, names);
    
    {
        let mut short_file = entries[1].get_file();
        let mut buf = Vec::new();
        short_file.read_to_end(&mut buf).unwrap();
        assert_eq!(str::from_utf8(&buf).unwrap(), TEST_TEXT);
        
        short_file.seek(SeekFrom::Start(5)).unwrap();
        buf.clear();
        short_file.read_to_end(&mut buf).unwrap();
        assert_eq!(str::from_utf8(&buf).unwrap(), &TEST_TEXT[5..]);
    }
    
    {
        let mut long_file = entries[0].get_file();
        let mut buf = Vec::new();
        long_file.read_to_end(&mut buf).unwrap();
        assert_eq!(str::from_utf8(&buf).unwrap(), TEST_TEXT.repeat(1000));
    }
    
    {
        let mut root_dir = fs.root_dir();
        let mut dir = root_dir.get_dir("very/long/path/").unwrap();
        let entries = dir.list().unwrap();
        let names = entries.iter().map(|e| e.get_name()).collect::<Vec<String>>();
        assert_eq!(names, [".", "..", "TEST.TXT"]);
    }
    
    {
        let mut root_dir = fs.root_dir();
        let mut file = root_dir.get_file("very/long/path/test.txt").unwrap();
        let mut buf = Vec::new();
        file.read_to_end(&mut buf).unwrap();
        assert_eq!(str::from_utf8(&buf).unwrap(), TEST_TEXT);
    }
}

#[test]
fn fat12() {
    test_img("resources/fat12.img");
}

#[test]
fn fat16() {
    test_img("resources/fat16.img");
}

#[test]
fn fat32() {
    test_img("resources/fat32.img");
}