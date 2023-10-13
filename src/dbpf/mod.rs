pub mod slicer;
use slicer::{slicer_32, slicer_raw, slicer_16};

use std::{fs, io};
use std::io::{BufReader, Seek, Read};

// HEADER STUFF
/*
Offset      Start   Description             Version
-----------------------------------------------------
Offset 0    (0)     Signature (DBPF)        *
Offset 1    (4)     Major Version           *
Offset 2    (8)     Minor Version           *
Offset 3    (12)    User Version Major?     2.x
Offset 4    (16)    User Version Minor?     2.x
Offset 5    (20)    Flags?                  2.x
Offset 6    (24)    Creation Date           1.0
Offset 7    (28)    Modification Date       1.0
Offset 8    (32)    Index Version Major     1.x
Offset 9    (36)    Index Entry Count       *
Offset 10   (40)    Index Offset            1.x
Offset 11   (44)    Index Size              *
Offset 12   (48)    Hole Entry Count        1.x
Offset 13   (52)    Hole Offset             1.x
Offset 14   (56)    Hole Size               1.x
Offset 15   (60)    Index Version Minor     1.x
Offset 16   (64)    Index Offset            2.x
Offset 17   (68)    ???                     2.x
Offset 28   (72)    Padding                 *
-
Offset 24   (92)    Padding                 *
*/

fn check_signature(signature_bytes: &[u8; 4]) -> Result<String, String> {
    // returns true if the signature is valid

    let signature_string = String::from_utf8(signature_bytes.to_vec()).expect("Invalid Signature");
    if signature_string.contains("DBPF") {Ok(signature_string)}
    else {Err(signature_string)}
}


pub fn read_header_raw(reader: &mut BufReader<&fs::File>) -> Result<[u8; 96], String> {
    let mut header_raw = [0u8; 96];

    // seek the reader to position 0
    if let Err(e) = reader.rewind() {return Err(e.to_string());}
    // fill the buffer
    if let Err(e) = reader.read_exact(&mut header_raw) {return Err(e.to_string());}
    if let Err(signature) = check_signature(&slicer_raw(&header_raw, 0)) {
        panic!("This does not appear to be a valid DBPF file! Expected signature 'DBPF', found '{}'", signature);
    }
    Ok(header_raw)
}

pub fn process_header(header_data_raw: &[u8; 96]) -> Result<[u32; 24], String> {
    let mut header_processed = [0u32; 24];
    for i in 0..24 {
        header_processed[i] = slicer_32(header_data_raw, i);
    }
    Ok(header_processed)
}

// function that reads a fully processed version of the header
pub fn read_header_processed(reader: &mut BufReader<&fs::File>) ->  Result<[u32; 24], String>{
    let header_raw = read_header_raw(reader)?;
    process_header(&header_raw)     

}

// HEADER STUFF END



// INDEX STUFF
pub fn read_index_v2_raw(reader: &mut BufReader<&fs::File>, addr_of_index: &u32, index_number: u32) -> Result<[u8; 32], ()> {
    let mut raw_index_data = [0u8; 32];
    if let Err(_e) =  reader.seek(io::SeekFrom::Start(((addr_of_index + 4) + (index_number * 32))  as u64)) {return Err(());};
    if let Err(_e) = reader.read_exact(&mut raw_index_data) {return Err(())};
    Ok(raw_index_data)
}

pub fn process_index_v2(unprocessed: &[u8; 32]) -> [u32; 9] {
    let mut index_processed = [0u32; 9];  
    index_processed[0] = slicer_32(unprocessed, 0);                     // Type ID
    index_processed[1] = slicer_32(unprocessed, 1);                     // Group ID
    index_processed[2] = slicer_32(unprocessed, 2);                     // Instance ID HIGH
    index_processed[3] = slicer_32(unprocessed, 3);                     // Instance ID LOW
    index_processed[4] = slicer_32(unprocessed, 4);                     // File Location
    index_processed[5] = slicer_32(unprocessed, 5) & 0x0FFFFFFF;        // File Size Uncompressed
    index_processed[6] = slicer_32(unprocessed, 6);                     // File Size Compressed
    index_processed[7] = slicer_16(unprocessed, 7, 0);      // Compression Flag 
    index_processed[8] = slicer_16(unprocessed, 7, 2);      // Unknown Flag                               
    index_processed
}

pub fn read_index_v2_processed(reader: &mut BufReader<&fs::File>, addr_of_index: &u32, index_number: u32) -> Result<[u32; 9], ()> {
    let raw_index_data = read_index_v2_raw(reader, addr_of_index, index_number)?;
    Ok(process_index_v2(&raw_index_data))
}


