use std::{env, fs::File, path::Path, process::exit};

use bmc_cpap::{get_data_filenames, parse_data};










fn main() {
    let args: Vec<String> = env::args().collect();
    let mut data_directory: String= "".to_string();
    let mut output_file_name: String= "./".to_string();
 




    if args.len() < 4 {
        println!("A min of two arguments are needed, -p '/path/to/data/' and the 'output_file.csv'.");
        exit(1);
    }

    if args[1] == "-p" {
        data_directory = args[2].to_string();
        if Path::new(&data_directory).exists() {
            colour::magenta_ln!("'{}' exists, continuing...",data_directory);
        }
        else {
            panic!("Path '{}' does not exist!!!",data_directory);
        }
    }
    output_file_name.to_string(); 
    output_file_name.push_str(&args[3].to_string());
    let opt_out_file = if Path::new(&output_file_name).exists() {
        colour::magenta_ln!("'{}' exists, continuing...",output_file_name);
        Some(File::create(output_file_name).unwrap())
    }
    else {
        panic!("File '{}' does not exist!!!",output_file_name);
    };
    // colour::green_ln!("Output file = '{}'",output_file_name);

    let out_file = opt_out_file.unwrap();

    let file_list = get_data_filenames(&data_directory).unwrap();
    // let output_file = Path::new(&output_file_name);
    parse_data(&file_list[0], &out_file);
    
    for file_path in file_list {
        colour::blue_ln!("{:?}",file_path.to_str());
        let f = File::open(file_path);
        let metadata = f.unwrap().metadata();
        colour::yellow_ln!("Size = {}",metadata.unwrap().len());
    }

    

 
}
