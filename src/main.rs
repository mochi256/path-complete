extern crate path_complete;

use path_complete::{parse_args, completion};

fn main(){
    let _args = parse_args::get_args();
    let _comp: Vec<String> = completion::get_complete(
        _args.get_path().to_string()
    );
    for c in _comp{
        println!("{}", c);
    }
}