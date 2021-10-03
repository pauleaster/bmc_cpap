
use glob::glob;
use std::{collections::HashMap, io::{BufWriter, Write}, path::Path};
use glob::GlobError;
use std::{fs::File, io::{BufRead, BufReader}, path::PathBuf};


const PACKET_SIZE:usize = 256;
const NUMBER_OF_CSV_LINES_TO_WRITE: usize = 1024;

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
enum PacketFieldNames {
    ReslexLow,
    ReslexHigh,
    IpapLow,
    IpapHigh,
    EpapLow,
    EpapHigh,
    TidalVolLow,
    TidalVolHigh,
    RepRateLow,
    RepRateHigh,
    YearLow,
    YearHigh,
    Month,
    Day,
    Hour,
    Minute,
    Second,
}

const PACKET_FIELD_NUMBER: usize = 17;

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
    map.insert(PacketFieldNames::ReslexLow, 2);
    map.insert(PacketFieldNames::ReslexHigh, 3);
    map.insert(PacketFieldNames::IpapLow, 4);
    map.insert(PacketFieldNames::IpapHigh, 5);
    map.insert(PacketFieldNames::EpapLow, 6);
    map.insert(PacketFieldNames::EpapHigh, 7);
    map.insert(PacketFieldNames::TidalVolLow, 99*2);
    map.insert(PacketFieldNames::TidalVolHigh, 99*2+1);
    map.insert(PacketFieldNames::RepRateLow, 104*2);
    map.insert(PacketFieldNames::RepRateHigh, 104*2+1);
    map.insert(PacketFieldNames::YearLow, 256-8+0);
    map.insert(PacketFieldNames::YearHigh, 256-8+1);
    map.insert(PacketFieldNames::Month, 256-8+2);
    map.insert(PacketFieldNames::Day, 256-8+3);
    map.insert(PacketFieldNames::Hour, 256-8+4);
    map.insert(PacketFieldNames::Minute, 256-8+5);
    map.insert(PacketFieldNames::Second, 256-8+6);
    map

}

fn initialise_array_lookup() -> [usize;PACKET_FIELD_NUMBER]{

    let map = initialise_table();

    let mut lookup = [0_usize;PACKET_FIELD_NUMBER];
    lookup[0] = map[&PacketFieldNames::Day];
    lookup[1] = map[&PacketFieldNames::Month];
    lookup[2] = map[&PacketFieldNames::YearLow];
    lookup[3] = map[&PacketFieldNames::YearHigh];
    lookup[4] = map[&PacketFieldNames::Hour];
    lookup[5] = map[&PacketFieldNames::Minute];
    lookup[6] = map[&PacketFieldNames::Second];
    lookup[7] = map[&PacketFieldNames::ReslexLow];
    lookup[8] = map[&PacketFieldNames::ReslexHigh];
    lookup[9] = map[&PacketFieldNames::IpapLow];
    lookup[10] = map[&PacketFieldNames::IpapHigh];
    lookup[11] = map[&PacketFieldNames::EpapLow];
    lookup[12] = map[&PacketFieldNames::EpapHigh];
    lookup[13] = map[&PacketFieldNames::TidalVolLow];
    lookup[14] = map[&PacketFieldNames::TidalVolHigh];
    lookup[15] = map[&PacketFieldNames::RepRateLow];
    lookup[16] = map[&PacketFieldNames::RepRateHigh];
    lookup    
}

fn wanted_data(packet: &[u8], lookup_table: &[usize;12]) -> [u8;12] {
    let mut result = [0_u8;12];
    for (idx, &data_idx) in lookup_table.iter().enumerate() {
        result[idx] = packet[data_idx];
    }
    result
}

fn calc_packet_csv_bytes(packet_data: [u16;12]) -> Vec <u16> {
    
    let mut csv_str: String = "".to_string();
    let mut year_low: u32 =  0;
    let mut year_high: u32 = 0;
    for (idx, value) in packet_data.iter().enumerate() {
        if idx==2 {
            year_low = *value as u32;
            continue;
        } else if idx == 3 {
            year_high = *value as u32;
            let year_value = year_high * 256 + year_low;
            csv_str.push_str(&year_value.to_string());
            csv_str.push(',')
        } else {
        csv_str.push_str(&value.to_string());
        csv_str.push(',');
        }
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
    let lookup_table = initialise_array_lookup();
    let mut number_of_lines = 0;
    let mut number_of_written_lines = 0;

    let mut day: Option<usize> = None;

    let mut output_csv_bytes : Vec<u8> = vec![];
    let mut csv_line_bytes : Vec<u8>;
    let mut this_data = [0u8;12];



    while  length > 0 {
        let buffer: &[u8] = reader.fill_buf().unwrap();
        length = buffer.len();

        if length == PACKET_SIZE {
            this_data = wanted_data(buffer, &lookup_table);
            csv_line_bytes = calc_packet_csv_bytes(this_data);
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