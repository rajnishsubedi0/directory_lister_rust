use std::env;
use std::fs;
use std::path::PathBuf;



fn main() {
    //let mut my_args = arguments_entered::new();
    // println!("{:?}", my_args);

    let a: Vec<String> = env::args().collect();

    let b = env::current_dir().unwrap();

    let mut my_files_in_directory = fs::read_dir(b).unwrap();
    let mut my_vec: Vec<String> = Vec::new();
    let mut numm = 0;
    for i in my_files_in_directory {
        

        my_vec.push(i.unwrap().path().display().to_string());
        println!("{}",my_vec[numm]);
        numm += 1;    
    
};
let src_dir=format!("{}",my_vec.pop().unwrap());
my_files_in_directory=fs::read_dir(src_dir).unwrap();

for i in my_files_in_directory{
    println!("{}",i.unwrap().path().display().to_string())
}

}