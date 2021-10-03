use std::{collections::HashMap, env, fmt::Display, fs::File, path::Path, process::exit};


use glob::{GlobError, glob};
use std::path::PathBuf;
const PACKET_SIZE:usize = 256;

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
enum PacketFieldNames {
    Reslex,
    Ipap,
    Epap,
    TidalVol,
    RepRate,
}

struct Packet {
    data: Vec<u8>
}

// impl Packet {
//     fn new() {
//         to_do();
//     }
// }


fn initialise_table() -> HashMap<PacketFieldNames, usize> {
    
    let mut map:  HashMap<PacketFieldNames, usize> = HashMap::new();
    map.insert(PacketFieldNames::Reslex, 1);
    map.insert(PacketFieldNames::Ipap, 2);
    map.insert(PacketFieldNames::Epap, 3);
    map.insert(PacketFieldNames::TidalVol, 99);
    map.insert(PacketFieldNames::RepRate, 104);
    map

}

fn get_data_filenames(data_directory: &str) -> Result<Vec<PathBuf>, GlobError> {

    let mut file_list: Vec<PathBuf> = vec![];
    let expanded_path = shellexpand::tilde(data_directory);
    let pattern : PathBuf = [&expanded_path, "*.[0-9][0-9][0-9]"].iter().collect();
    let mut err: Option<GlobError> = None;

    for entry in glob(pattern.to_str().unwrap()).expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => {file_list.push(path)},
            
            Err(e) => {err = Some(e);
                                break;},
        };
    }
    match err {
        Some(e) => Err(e),
        None => {
            file_list.sort_unstable();
            Ok(file_list)
        }
    }

}


fn main() {
    let args: Vec<String> = env::args().collect();
    let mut data_directory: &str= "";
    let mut output_file: &str= "";
    let mut packet = &[0_u8;PACKET_SIZE];

    if args.len() < 4 {
        println!("A min of two arguments are needed, -p '/path/to/data/' and the 'output_file.csv'.");
        exit(1);
    }

    if args[1] == "-p" {
        data_directory = &args[2];
        if Path::new(data_directory).exists() {
            colour::magenta_ln!("'{}' exists, continuing...",data_directory);
        }
        else {
            panic!("Path '{}' does not exist!!!",data_directory);
        }
    }
    output_file = &args[3];
    colour::green_ln!("Output file = '{}'",output_file);

    let file_list = get_data_filenames(data_directory).unwrap();
    
    for file_path in file_list {
        colour::blue_ln!("{:?}",file_path.to_str());
        let f = File::open(file_path);
        let metadata = f.unwrap().metadata();
        colour::yellow_ln!("Size = {}",metadata.unwrap().len());
    }


 
}
