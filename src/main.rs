mod support;
mod dbpf;

use dbpf::slicer;
use std::borrow::Borrow;
use std::error::Error;
use std::{env, io, fs};
use std::io::{BufReader, Read, Seek, Write};

use imgui::{Ui, TableColumnSetup};
use imgui::TableFlags;


fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if let 0 | 1 = args.len() {
        eprintln!("No Filename Specified!");
        std::process::exit(1);
    }

    let target_path = args.get(1);
    let file = fs::File::options()
        .write(true)
        .read(true)
        .open(target_path.unwrap())?;

    let mut header_reader = BufReader::new(file.borrow());
    let mut index_reader = BufReader::new(file.borrow());
    // let mut file_reader = BufReader::new(file.borrow());

    // let mut index_writer = io::BufWriter::new(file.borrow());
    // let mut header_writer = io::BufWriter::new(file.borrow());

    let header_data = dbpf::read_header_processed(&mut header_reader).unwrap();
    
    // get the version. Versions 2.x and 1.x behave differently
    let version_major = &header_data[1];
    let version_minor = &header_data[2];
    let index_count = &header_data[9];
    let _index_size = &header_data[11];

    println!("Detected DBPF Version {}.{}", version_major, version_minor);


    let mut index_list: Vec<[u32; 9]> = vec![];


    match version_major {
        1 => {
            // the date info is only found in version 1.X
            //let _create_date = &header_data[6];
            //let _modify_date = &header_data[7];
            // versions 1.0 and 1.1 store the index position differently than 2.X
            // 1.x gives you the relative position 
            
            let offset = &header_data[10];
        
            // the hole Index seems to be only used in 1.x
            //let hole_count  =   &header_data[12];
            //let hole_offset = &header_data[13];
            //let hole_size = &header_data[14];

            dbg!(offset);
            //dbg!(hole_count);
            //dbg!(hole_offset);
            //dbg!(hole_size);
            // there are multiple index entry versions that behave slightly different
            // we need to find out which one we are using
            let index_version_minor = &header_data[15] - 1;
            dbg!(index_version_minor);
        }

        2 | 3 => {
            // in 2.x it gives you the the absolute position of the index table
            let index_offset = &header_data[16];
            for index_number in 0..*index_count {
                // create and fill the index data array with the index table data
                let index_content = dbpf::read_index_v2_processed(&mut index_reader, index_offset, index_number).expect("Error reading the index entry");
                index_list.push(index_content);
            }
        }
        _ => println!("Unknown Index Version Detected")
    }
    let system = support::init(file!());
    println!("{}", &system.imgui.frame_count());
    let table_flags = TableFlags::BORDERS_V | TableFlags::REORDERABLE | TableFlags::SORTABLE | TableFlags::SCROLL_Y | TableFlags::SIZING_STRETCH_PROP;

    system.main_loop(move |run, ui| {
        std::thread::sleep(std::time::Duration::from_millis(50));
        let window_token = ui.window("Table")
            .size(ui.io().display_size, imgui::Condition::Always)
            .collapsible(false)
            .movable(false)
            .position([0.0, 0.0], imgui::Condition::Always)
            .always_auto_resize(true)
            .scrollable(false)
            .scroll_bar(false)
            .begin();
            
        if let Some(_t) = window_token {
            if let Some(_tok) = ui.begin_table_with_sizing("Tabelle", 8, table_flags, ui.window_size().map(|a| a - 10.0), 10.0) {

                // set up the header 
                ui.table_setup_column("Group ID");                  // 0
                ui.table_setup_column("Type ID");                   // 1 
                ui.table_setup_column("Instance ID");               // 2
                ui.table_setup_column("File Location");             // 3
                ui.table_setup_column("File Size (Uncompressed)");  // 4
                ui.table_setup_column("File Size (Compressed)");    // 5
                ui.table_setup_column("Compression Flag");          // 6
                ui.table_setup_column("Unknown Flag");              // 7

                ui.table_setup_scroll_freeze(8, 1);
                ui.table_headers_row();


                let clip = imgui::ListClipper::new(index_list.len() as i32).begin(ui);

                for row_num in clip.iter()  {
                    ui.table_next_row();
                    let index = &index_list[row_num as usize];
                    
                    // Group ID
                    ui.table_set_column_index(0);
                    ui.text(format!("0x{:08X}", &index[0]));

                    // Type ID
                    ui.table_set_column_index(1);
                    ui.text(format!("0x{:08X}", &index[1]));

                    // Instance ID
                    ui.table_set_column_index(2);
                    ui.text(format!("0x{:08X}{:08X}", &index[2], &index[3]));

                    // File Location
                    ui.table_set_column_index(3);
                    ui.text(format!("0x{:08X}", &index[4]));

                    // File Size Uncompressed
                    ui.table_set_column_index(4);
                    ui.text(format!("{} B", &index[5]));

                    // File Size Compressed
                    ui.table_set_column_index(5);
                    ui.text(format!("{} B", &index[6]));

                    // Compression Flag
                    ui.table_set_column_index(6);
                    ui.text(format!("0x{:04X}", &index[7]));
                    
                    // Unknown Flag
                    ui.table_set_column_index(7);
                    ui.text(format!("0x{:04X}", &index[8]));
                }
            }
        }


    });
    Ok(())
}
