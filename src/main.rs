use quick_xml::{events::Event, Reader};
use std::fs;
//use std::str;

fn main() {
    let xml = fs::read_to_string(r#"test.xml"#).expect("Something went wrong reading the file");
    let mut reader = Reader::from_str(xml.as_str());
    reader.trim_text(true);

    let mut txt = Vec::new();
    let mut buf = Vec::new();
    let mut inc_comp: i64 = 0;
    let mut inc_filename: i64 = 0;
    let mut inc_name: i64 = 0;

    let mut file_name = String::from("");
    let mut name = String::from("");
    let mut attrib =String::from("");

    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Start(ref e)) => match e.name() {
                b"compound" => {
                   /* println!(
                        "attributes values: {:?}",
                        e.attributes().map(|a| a.unwrap().value).collect::<Vec<_>>()
                    );
                    */
                    inc_comp += 1;
                    let found = e.attributes().find(|item| 
                        //println!("~~>{:?}", item.as_ref().unwrap().key == b"attrib");
                        item.as_ref().unwrap().key == b"attrib"
                    );
                    match found {
                        Some(e) => {
                            if e.is_ok() {
                                let res = e.unwrap();
                                //println!("---> {:?}", res);
                                attrib = String::from_utf8(res.value.into_owned()).unwrap();
                            }
                        }
                        None => (),
                    }
                }
                b"filename" => {
                    if inc_comp > 0 {
                        inc_filename += 1;
                    }
                }
                b"name" => {
                    if inc_comp > 0 {
                        inc_name += 1;
                    }
                }
                //b"name" =>  println!("data: {:?}", e.att),
                _ => (),
            },
            Ok(Event::Text(e)) => {
                if inc_filename > 0 {
                    file_name = e.unescape_and_decode(&reader).unwrap().to_string();
                }
                if inc_name > 0 {
                    name = e.unescape_and_decode(&reader).unwrap().to_string();
                }
            }
            Ok(Event::Eof) => break, // exits the loop when reaching end of file
            Ok(Event::End(ref e)) => match e.name() {
                b"compound" => {
                    println!("/compound");
                    inc_comp -= 1;
                    txt.push(name.to_string() + ":" + &file_name.to_string());
                    name.clear();
                    file_name.clear();
                    attrib.clear();
                }
                b"filename" => {
                    inc_filename -= 1;
                }
                b"name" => {
                    inc_name -= 1;
                }
                _ => (),
            },
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ => (), 
        }

        buf.clear();
    }
    println!("Hello, world! {:?}", txt);
}
