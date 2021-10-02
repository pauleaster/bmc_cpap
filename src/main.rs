use std::{collections::HashMap, env, path::Path, process::exit};
// use colour;

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


fn initialise_table() -> HashMap<PacketFieldNames, &'static usize> {
    
    let mut map:  HashMap<PacketFieldNames, &'static usize> = HashMap::new();
    map.insert(PacketFieldNames::Reslex, &1);
    map.insert(PacketFieldNames::Ipap, &2);
    map.insert(PacketFieldNames::Epap, &3);
    map.insert(PacketFieldNames::TidalVol, &99);
    map.insert(PacketFieldNames::RepRate, &104);
    map

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


 
}
