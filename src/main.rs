use std::env;

use lucene::store::Directory;
use lucene::core::Segment;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print!("Please use lucene index directory as a parameter");
        return;
    }

    let _dir = Directory::open(&args[1]).unwrap();
}
