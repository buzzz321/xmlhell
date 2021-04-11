use quick_xml::{events::Event, Reader};
use std::fs;

fn main() {
    let xml = fs::read_to_string(r#"test.xml"#).expect("Something went wrong reading the file");
    let mut reader = Reader::from_str(xml.as_str());
    reader.trim_text(true);

    //let mut count: i32 = 0;
    let mut txt = Vec::new();
    let mut buf = Vec::new();
    let mut inc_comp: i64 = 0;
    let mut inc_filename: i64 = 0;
    let mut inc_name: i64 = 0;

    let mut file_name = String::from("");
    let mut name = String::from("");
    // The `Reader` does not implement `Iterator` because it outputs borrowed data (`Cow`s)
    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Start(ref e)) => match e.name() {
                b"compound" => {
                    println!(
                        "attributes values: {:?}",
                        e.attributes().map(|a| a.unwrap().value).collect::<Vec<_>>()
                    );
                    inc_comp += 1;
                    let found = e
                        .attributes()
                        .find(|item| item.as_ref().unwrap().key == b"attrib");
                    match found {
                        Some(e) if e.is_ok() => {
                            let res = e.unwrap();
                            println!("---> {:?}", res);
                        }
                        _ => (),
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
            _ => (), // There are several other `Event`s we do not consider here
        }

        // if we don't keep a borrow elsewhere, we can clear the buffer to keep memory usage low
        buf.clear();
    }
    println!("Hello, world! {:?}", txt);
}
