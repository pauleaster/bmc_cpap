
use chrono::{Datelike, NaiveDate};
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



fn init_vec_lookup() -> ( Vec<usize>, Vec<usize>, Vec<String>, Vec<usize>) {

    let name_vec:Vec<Field> = vec![    
        Field::Year,
        Field:: Month,
        Field::Day,
        Field::Hour,
        Field::Minute,
        Field::Second,
        Field::Reslex,
        Field::Ipap,
        Field::Epap,
        Field::TidalVol,
        Field::RepRate,
];
    let string_vec:Vec<String> = vec![
        "Year".to_string(),
        "Month".to_string(),
        "Day".to_string(),
        "Hour".to_string(),
        "Minute".to_string(),
        "Second".to_string(),
        "Reslex".to_string(),
        "Ipap".to_string(),
        "Epap".to_string(),
        "TidalVol".to_string(),
        "RepRate".to_string(),];

    let ymdhms_headers= vec!["Year".to_string(),"Month".to_string(),"Day".to_string(),
                                        "Hour".to_string(),"Minute".to_string(),"Second".to_string()];

    let mut ymdhms_locations:Vec<usize> = vec![];
    for ymdhms_val in ymdhms_headers {
        ymdhms_locations.push(string_vec.iter().position(|r| r==&ymdhms_val).unwrap());
    }

    let (hm_locn, hm_size) = initialise_table() ;
    let mut locn:Vec<usize> = vec![];
    let mut size:Vec<usize> = vec![];
    for field in name_vec.iter(){
        locn.push(hm_locn[field]);
        size.push(hm_size[field]);
    }
    (locn, size, string_vec, ymdhms_locations)
}


fn initialise_table() -> (HashMap<Field, usize>, HashMap<Field, usize>) {
    
    let mut map:  HashMap<Field, usize> = HashMap::new();
    map.insert(Field::Reslex, 2);
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

fn wanted_csv_headers(headers:&[String]) -> Vec<u8> {

    let mut csv_str: String = "".to_string();
    for s in headers.iter() {
        csv_str.push_str(s);
        csv_str.push(',');
    }
    csv_str.pop(); // remove the last ','
    csv_str.push('\n'); // add linefeed
    let mut prefix = "ordinal_seconds".to_string();
    prefix.push(',');
    prefix.push_str(&csv_str);
    prefix.as_bytes().to_vec()


}


fn wanted_csv_data(packet: &[u8], locn: &[usize], size:&[usize], ymdhms_locations: &[usize]) -> Vec<u8> {
    
    let mut ymdhms_values: Vec<i32> = vec![0;ymdhms_locations.len()];
    let mut csv_str: String = "".to_string();
    for (idx,(&loc, &s)) in locn.iter().zip(size.iter()).enumerate() {
        let mut val:u16 = 0;
        for i in 0..s {
            let shift = i<<3;
            val |= (packet[loc + i] as u16) << shift;
        }
        csv_str.push_str(&val.to_string());
        csv_str.push(',');
        if let Some(ymdhms_index) = ymdhms_locations.iter().position(|&x| x==idx) {
            ymdhms_values[ymdhms_index] = val as i32;
        }
    }
    csv_str.pop(); // remove the last ','
    csv_str.push('\n'); // add linefeed
    let num_days = NaiveDate::from_ymd(ymdhms_values[0],ymdhms_values[1] as u32,ymdhms_values[2] as u32).num_days_from_ce();
    let ordinal_seconds = num_days as u64 * 86_400 + ymdhms_values[3] as u64 * 3_600 + ymdhms_values[4] as u64*  60 + ymdhms_values[5] as u64;
    let mut prefix = ordinal_seconds.to_string();
    prefix.push(',');
    prefix.push_str(&csv_str);

    prefix.as_bytes().to_vec()

}


pub fn get_data_filenames(data_path: &Path) -> Result<Vec<PathBuf>, GlobError> {

    let mut file_list: Vec<PathBuf> = vec![];
    let pattern : PathBuf = data_path.join( "*.[0-9][0-9][0-9]").iter().collect();
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



pub fn parse_data(file_names: &[PathBuf], output_file_name: &File) {

    let out_file = output_file_name;//File::open(output_file_name);
    
    
    
    let mut writer = BufWriter::new(out_file);
    let mut total_bytes_read = 0;
    // let lookup_table = initialise_array_lookup();
    let mut number_of_lines = 0;
    let mut number_of_written_lines = 0;


    let mut output_csv_bytes : Vec<u8> = vec![];
    let mut csv_line_bytes : Vec<u8>;
    
    let (locn, size, headers, ymdhms_locations) = init_vec_lookup();



    csv_line_bytes = wanted_csv_headers(&headers);
    output_csv_bytes.append(& mut csv_line_bytes);
    number_of_lines += 1;

    for file_name in file_names {
        let file = File::open(file_name);
        let mut reader = BufReader::with_capacity(PACKET_SIZE,file.unwrap());
        colour::green_ln!("Reading {}", file_name.to_str().unwrap());
        let mut length = 1;
        while  length > 0 {
            let buffer: &[u8] = reader.fill_buf().unwrap();
            length = buffer.len();

            if length == PACKET_SIZE {
                csv_line_bytes = wanted_csv_data(buffer, &locn, &size, &ymdhms_locations);
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
                        // colour::green_ln!("Bytes read: {}, Lines written: {}", total_bytes_read, number_of_written_lines);
                    },
                    Err(e) => {
                        panic!("{}",&e);
                    },
                }

            }
        }
        match writer.write_all(&output_csv_bytes) {
            Ok(()) => {
                output_csv_bytes = vec![];
                number_of_written_lines += number_of_lines;
                number_of_lines = 0;
                // colour::green_ln!("Bytes read: {}, Lines written: {}", total_bytes_read, number_of_written_lines);
            },
            Err(e) => {
                panic!("{}",&e);
            },
        }
        println!("Number of unwritten lines = {}",number_of_lines);
    }
    colour::magenta_ln!("Number of bytes read = {}",total_bytes_read);
    colour::yellow_ln!("Number of lines written = {}",number_of_written_lines);

}