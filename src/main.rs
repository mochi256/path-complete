extern crate path_complete;

use path_complete::{parse_args, completion};

fn main(){
    let _args = parse_args::get_args();
    match completion::get_complete(_args.get_path().to_string()) {
        Some(_comp) => {
            for c in _comp{
                println!("{}", c);
            }
        },
        None => {}
    };
}