use std::{env, fs::File, path::Path, process::exit, time::Instant};

use bmc_cpap::{get_data_filenames, parse_data};


fn main() {


    let now = Instant::now();
    let args: Vec<String> = env::args().collect();
    let data_directory: String;
    let mut output_file_name: String= "./".to_string();
    let mut opt_data_path: Option<&Path> = None;
    let expanded_path: String;
 

    if args.len() < 4 {
        println!("A min of two arguments are needed, -p '/path/to/data/' and the 'output_file.csv'.");
        exit(1);
    }

    if args[1] == "-p" {
        data_directory = args[2].to_string();
        expanded_path = shellexpand::tilde(&data_directory).to_string();
        opt_data_path = Some(Path::new(&expanded_path));
        
        if opt_data_path.unwrap().exists() {
            colour::magenta_ln!("'{}' exists, continuing...",data_directory);
        }
        else {
            panic!("Path '{}' does not exist!!!",data_directory);
        }
    }
    output_file_name.to_string(); 
    output_file_name.push_str(&args[3].to_string());
    let expanded_output_string = &shellexpand::tilde(&output_file_name).to_string();
    let expanded_output_path = Path::new(expanded_output_string);
    if Path::new(&expanded_output_path).exists() {
        colour::red_ln!("'{}' exists, overwritting...",output_file_name);
        }
        else {
            colour::green_ln!("'{}' does not exist, creating...",output_file_name);
        };
    let opt_out_file = Some(File::create(expanded_output_path).unwrap());
        
    // colour::green_ln!("Output file = '{}'",output_file_name);

    let out_file = opt_out_file.unwrap();
    
    match opt_data_path {
        Some(data_path) => {
            let file_list = get_data_filenames(data_path).unwrap();
            // let output_file = Path::new(&output_file_name);
            parse_data(&file_list, &out_file);
        },
        None => {exit(-1);}
    }
    

    

    let elapsed = now.elapsed().as_secs_f64();
    // colour::blue_ln!("Total elapsed time is {:0.3} seconds",elapsed);
    colour::dark_blue!("Total elapsed time is {:0.3} seconds",elapsed);
}
