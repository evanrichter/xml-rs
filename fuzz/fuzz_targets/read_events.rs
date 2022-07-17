#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &str| {
    let mut reader = xml::reader::EventReader::from_str(data);
    while let Ok(_) = reader.next() {}
});
