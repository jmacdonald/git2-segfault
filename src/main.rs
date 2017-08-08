extern crate git2;
extern crate onig;

use std::env;
use git2::Repository;

fn main() {
    let current_dir = env::current_dir().unwrap();
    let repo = Repository::discover(&current_dir).unwrap();
    let remote = repo.find_remote("origin").unwrap();
    let url = remote.url().unwrap();

    println!("remote url: {}", url);
}
