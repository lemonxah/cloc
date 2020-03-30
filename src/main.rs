#[macro_use]
extern crate clap;

use walkdir::WalkDir;
use clap::App;
use funlib::Foldable::*;

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> std::io::Result<()> {
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from_yaml(yaml).get_matches();
    let path = matches.value_of("path").unwrap();
    let count: usize = matches.value_of("count").unwrap_or("0").parse::<usize>().unwrap();
    let filetype = matches.value_of("filetype").unwrap();
    let ignores = matches.values_of("ignore").map(|vs| vs.collect()).unwrap_or(vec![]);
    let verbose = matches.occurrences_of("verbose");
    let paths: Vec<_> = WalkDir::new(path).into_iter()
        .map(|r| r.map(|f| String::from(f.path().to_str().unwrap_or(""))).unwrap())
        .filter(|p| {
            if p.ends_with(&format!(".{}", filetype)) && !ignores.any(|i| p.contains(&format!("/{}/", i))) {
                true
            } else {
                false
            }
        }).collect();
    println!("Counting lines of code for filetype: *.{} in path: {}", &filetype, &path);
    println!("*SLOC = Substantial Lines of Code, *LOC = Lines of Code, *Empty = Empty Lines");
    let mut numlines = 0;
    let mut snumlines = 0;
    let mut empties = 0;
    for path in paths {
        let lines: Vec<_> = read_lines(&path)?.map(|l| l.unwrap()).collect();
        let es = &lines.filter(|l| remove_whitespace(l) == "").len();
        let ne = &lines.filter(|l| remove_whitespace(l) != "");
        let nec = &ne.len();
        numlines += nec;
        empties += es;
        let mut cc: usize = 0; 
        if count > 0 {
            cc = ne.filter(|l| l.len() >= count).len();
            snumlines += &cc;
        }
        if verbose > 1 {
            if count > 0 {
                print!("SLOC: {}, ", &cc);
            }
            print!("LOC: {}, Empty: {}, ", &nec, &es);
        }
        if verbose > 0 {
            println!("File: {}", &path);
        }
    }
    if count > 0 {
        print!("SLOC (at least {} charaters): {}, ", count, snumlines);
    }
    println!("LOC: {}, Empty: {}, Total: {}", numlines, empties, numlines + empties);
    Ok(())
}

fn remove_whitespace(s: &str) -> String {
    s.split_whitespace().collect()
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
