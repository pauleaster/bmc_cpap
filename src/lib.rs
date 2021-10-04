
use glob::glob;
use std::{collections::HashMap, io::{BufWriter, Write}, path::Path};
use glob::GlobError;
use std::{fs::File, io::{BufRead, BufReader}, path::PathBuf};


const PACKET_SIZE:usize = 256;
const NUMBER_OF_CSV_LINES_TO_WRITE: usize = 1024;

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
enum Field {
    Reslex,
    Ipap,
    Epap,
    TidalVol,
    RepRate,
    Year,
    Month,
    Day,
    Hour,
    Minute,
    Second,
}



fn init_vec_lookup() -> (Vec<Field>, Vec<usize>, Vec<usize>, Vec<String>) {

    let name_vec:Vec<Field> = vec![Field::Reslex,
    Field::Ipap,
    Field::Epap,
    Field::TidalVol,
    Field::RepRate,
    Field::Year,
    Field:: Month,
    Field::Day,
    Field::Hour,
    Field::Minute,
    Field::Second];
    let string_vec:Vec<String> = vec!["Reslex".to_string(),
    "Ipap".to_string(),
    "Epap".to_string(),
    "TidalVol".to_string(),
    "RepRate".to_string(),
    "Year".to_string(),
    "Month".to_string(),
    "Day".to_string(),
    "Hour".to_string(),
    "Minute".to_string(),
    "Second".to_string()];

    let (hm_locn, hm_size) = initialise_table() ;
    let num_bytes = 0;
    let mut locn:Vec<usize> = vec![];
    let mut size:Vec<usize> = vec![];
    for field in name_vec.iter(){
        locn.push(hm_locn[field]);
        locn.push(hm_size[field]);
    }
    (name_vec, locn, size, string_vec)
}

struct Packet {
    data: Vec<u8>
}

// impl Packet {
//     fn new() {
//         to_do();
//     }
// }


fn initialise_table() -> (HashMap<Field, usize>, HashMap<Field, usize>) {
    
    let mut map:  HashMap<Field, usize> = HashMap::new();
    map.insert(Field::Reslex, 1*2);
    map.insert(Field::Ipap, 2*2);
    map.insert(Field::Epap, 3*2);
    map.insert(Field::TidalVol, 99*2);
    map.insert(Field::RepRate, 104*2);
    map.insert(Field::Year, 256-8);
    map.insert(Field::Month, 256-8+2);
    map.insert(Field::Day, 256-8+3);
    map.insert(Field::Hour, 256-8+4);
    map.insert(Field::Minute, 256-8+5);
    map.insert(Field::Second, 256-8+6);
    
    let mut size:  HashMap<Field, usize> = HashMap::new();
    size.insert(Field::Reslex, 2);
    size.insert(Field::Ipap, 2);
    size.insert(Field::Epap, 2);
    size.insert(Field::TidalVol, 2);
    size.insert(Field::RepRate, 2);
    size.insert(Field::Year, 2);
    size.insert(Field::Month, 1);
    size.insert(Field::Day, 1);
    size.insert(Field::Hour, 1);
    size.insert(Field::Minute, 1);
    size.insert(Field::Second, 1);

    (map,size)

}

fn wanted_csv_headers(headers:&Vec<String>) -> Vec<u8> {

    let mut csv_str: String = "".to_string();
    for s in headers.iter() {
        csv_str.push_str(s);
        csv_str.push(',');
    }
    csv_str.pop(); // remove the last ','
    csv_str.push('\n'); // add linefeed
    csv_str.as_bytes().to_vec()

}


fn wanted_csv_data(packet: &[u8], locn: &Vec<usize>, size:&Vec<usize>) -> Vec<u8> {
    
    let mut csv_str: String = "".to_string();
    for (&loc, &s) in locn.iter().zip(size.iter()) {
        let mut val:u16 = 0;
        for i in 0..s {
            val = (val << 8) | (packet[loc + i] as u16);
        }
        csv_str.push_str(&val.to_string());
        csv_str.push(',')
    }
    csv_str.pop(); // remove the last ','
    csv_str.push('\n'); // add linefeed
    csv_str.as_bytes().to_vec()

}


pub fn get_data_filenames(data_directory: &str) -> Result<Vec<PathBuf>, GlobError> {

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



pub fn parse_data(file_name: &PathBuf, output_file_name: &File) {

    let file = File::open(file_name);
    let out_file = output_file_name;//File::open(output_file_name);
    
    let mut length = 1;
    let mut reader = BufReader::with_capacity(PACKET_SIZE,file.unwrap());
    let mut writer = BufWriter::new(out_file);
    let mut total_bytes_read = 0;
    // let lookup_table = initialise_array_lookup();
    let mut number_of_lines = 0;
    let mut number_of_written_lines = 0;

    let mut day: Option<usize> = None;

    let mut output_csv_bytes : Vec<u8> = vec![];
    let mut csv_line_bytes : Vec<u8>;
    
    let (params, locn, size, headers) = init_vec_lookup();



    csv_line_bytes = wanted_csv_headers(&headers);
    output_csv_bytes.append(& mut csv_line_bytes);
    number_of_lines += 1;
    while  length > 0 {
        let buffer: &[u8] = reader.fill_buf().unwrap();
        length = buffer.len();

        if length == PACKET_SIZE {
            // wanted_csv_data(packet: &Vec<u8>, locn: &Vec<usize>, size:&Vec<usize>, total_bytes: usize) -> Vec<u8> 
            csv_line_bytes = wanted_csv_data(buffer, &locn, &size);
            output_csv_bytes.append(& mut csv_line_bytes);
            number_of_lines += 1;
        }
        reader.consume(length);
        total_bytes_read += length;
        if number_of_lines >= NUMBER_OF_CSV_LINES_TO_WRITE {
            match writer.write_all(&output_csv_bytes) {
                Ok(()) => {
                    output_csv_bytes = vec![];
                    number_of_written_lines += number_of_lines;
                    number_of_lines = 0;
                    colour::green_ln!("Bytes read: {}, Lines written: {}", total_bytes_read, number_of_written_lines);
                },
                Err(e) => {
                    panic!("{}",&e);
                },
            }

        }
    }
    colour::magenta_ln!("Number of bytes read = {}",total_bytes_read);

}