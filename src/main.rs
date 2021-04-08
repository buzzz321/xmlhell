use quick_xml::{events::Event, Reader};
use std::fs;

fn main() {
    let xml = fs::read_to_string(r#"test.xml"#).expect("Something went wrong reading the file");
    let mut reader = Reader::from_str(xml.as_str());
    reader.trim_text(true);

    //let mut count: i32 = 0;
    let mut txt = Vec::new();
    let mut buf = Vec::new();

    // The `Reader` does not implement `Iterator` because it outputs borrowed data (`Cow`s)
    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Start(ref e)) => match e.name() {
                b"compound" => println!(
                    "attributes values: {:?}",
                    e.attributes().map(|a| a.unwrap().value).collect::<Vec<_>>()
                ),
                //b"name" =>  println!("data: {:?}", e.att),
                _ => (),
            },
            Ok(Event::Text(e)) => txt.push(e.unescape_and_decode(&reader).unwrap()),
            Ok(Event::Eof) => break, // exits the loop when reaching end of file
            Ok(Event::End(ref e)) => match e.name(){
                b"compound" => println!("/compound"),
                _ => (),
            },
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ => (), // There are several other `Event`s we do not consider here
        }

        // if we don't keep a borrow elsewhere, we can clear the buffer to keep memory usage low
        buf.clear();
    }
    println!("Hello, world! {:?}",txt);
}
