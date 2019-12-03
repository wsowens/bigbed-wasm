// features for the code
#[feature(proc_macro, wasm_custom_section, wasm_import_module)]

extern crate wasm_bindgen;
extern crate console_error_panic_hook;
extern crate web_sys;

extern crate bigbed;

use bigbed::BigBed;
use std::io::{self, Read, Seek, SeekFrom};
use std::convert::TryInto;
use web_sys::{HtmlTextAreaElement};
use wasm_bindgen::prelude::*;

struct FileString {
    data: Vec<u8>,
    pointer: usize,
}

impl FileString {
    fn new(data: Vec<u8>) -> FileString {
        FileString{data, pointer: 0}
    }

    fn from_slice(data: &[u8]) -> FileString {
        FileString{data: data.to_vec(), pointer: 0}
    }

    fn from_string(data: String) -> FileString {
        FileString{data: data.into_bytes(), pointer: 0}
    }

    fn from_str(data: &str) -> FileString {
        FileString{data: data.to_owned().into_bytes(), pointer: 0}
    }
}

impl Read for FileString {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        // if the pointer is out of range, don't read anything
        if self.pointer >= self.data.len() {
            return Ok(0);
        }
        let end = self.pointer + buf.len();
        //check that the end is not out of bounds
        let end = if end > self.data.len() { self.data.len() } else { end };
        //copy each byte
        for (index, byte) in self.data[self.pointer..end].iter().enumerate() {
            buf[index] = *byte;
        }
        let total = end - self.pointer;
        self.pointer = end;
        Ok(total)
    }
}

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

impl Seek for FileString {
    fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
        match pos {
            SeekFrom::Start(x) => {
                match x.try_into() {
                    Ok(as_usize) => {
                        self.pointer = as_usize;
                        Ok(x)
                    } _ => {
                        Err(io::Error::new(io::ErrorKind::InvalidInput, "Destination out of range"))
                    }
                }
            }
            SeekFrom::Current(x) => {
                let val: i64 = self.pointer.try_into().unwrap();
                let destination: i64 = val + x;
                if destination < 0 {
                    Err(io::Error::new(io::ErrorKind::InvalidInput, "Destination cannot be negative"))
                } else {
                    self.pointer = destination as usize;
                    Ok(destination as u64)
                }
            }
            SeekFrom::End(x) => {
                let val: i64 = self.data.len().try_into().unwrap();
                let destination: i64 = val + x;
                if destination < 0 {
                    Err(io::Error::new(io::ErrorKind::InvalidInput, "Destination cannot be negative"))
                } else {
                    self.pointer = destination as usize;
                    Ok(destination as u64)
                }
            }
        }
    }
}

#[cfg(test)]
mod test_file_string {
    use super::*;

    #[test]
    fn test_read() {
        let mut fs = FileString::from_str("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
        let mut buff = [0u8; 5];
        assert_eq!(fs.read(&mut buff).unwrap(), 5);
        assert_eq!(buff, [65, 66, 67, 68, 69]);
        assert_eq!(fs.read(&mut buff).unwrap(), 5);
        assert_eq!(buff, [70, 71, 72, 73, 74]);
        assert_eq!(fs.read(&mut buff).unwrap(), 5);
        assert_eq!(buff, [75, 76, 77, 78, 79]);
        assert_eq!(fs.read(&mut buff).unwrap(), 5);
        assert_eq!(buff, [80, 81, 82, 83, 84]);
        assert_eq!(fs.read(&mut buff).unwrap(), 5);
        assert_eq!(buff, [85, 86, 87, 88, 89]);
        assert_eq!(fs.read(&mut buff).unwrap(), 1);
        assert_eq!(buff, [90, 86, 87, 88, 89]);
        assert_eq!(fs.read(&mut buff).unwrap(), 0);
        assert_eq!(buff, [90, 86, 87, 88, 89]);
        assert_eq!(fs.read(&mut buff).unwrap(), 0);
        assert_eq!(buff, [90, 86, 87, 88, 89]);
    }

    #[test]
    fn test_read_bigbuff() {
        let mut fs = FileString::from_str("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
        let mut buff = [0u8; 30];
        assert_eq!(fs.read(&mut buff).unwrap(), 26);
        assert_eq!(buff, [
            65, 66, 67, 68, 69, 70, 
            71, 72, 73, 74, 75, 76,
            77, 78, 79, 80, 81, 82,
            83, 84, 85, 86, 87, 88,
            89, 90, 0, 0, 0, 0]);
        assert_eq!(fs.read(&mut buff).unwrap(), 0);
        assert_eq!(buff, [
            65, 66, 67, 68, 69, 70, 
            71, 72, 73, 74, 75, 76,
            77, 78, 79, 80, 81, 82,
            83, 84, 85, 86, 87, 88,
            89, 90, 0, 0, 0, 0]);
    }

    #[test]
    fn test_seek() {
        let mut fs = FileString::from_str("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
        let mut buff = [0u8; 5];
        assert_eq!(fs.seek(SeekFrom::Start(3)).unwrap(), 3);
        assert_eq!(fs.read(&mut buff).unwrap(), 5);
        assert_eq!(buff, [68, 69, 70, 71, 72]);
        assert_eq!(fs.seek(SeekFrom::Start(3)).unwrap(), 3);
        assert_eq!(fs.read(&mut buff).unwrap(), 5);
        assert_eq!(buff, [68, 69, 70, 71, 72]);
        assert_eq!(fs.seek(SeekFrom::Current(10)).unwrap(), 18);
        assert_eq!(fs.read(&mut buff).unwrap(), 5);
        assert_eq!(buff, [83, 84, 85, 86, 87]);
        // this should take us over the edge
        assert_eq!(fs.seek(SeekFrom::Current(10)).unwrap(), 33);
        assert_eq!(fs.read(&mut buff).unwrap(), 0);
        assert_eq!(buff, [83, 84, 85, 86, 87]);

        // take us back to the beginning
        assert_eq!(fs.seek(SeekFrom::Start(0)).unwrap(), 0);
        assert_eq!(fs.read(&mut buff).unwrap(), 5);
        assert_eq!(buff, [65, 66, 67, 68, 69]);

        // take us to the end
        assert_eq!(fs.seek(SeekFrom::End(-1)).unwrap(), 25);
        assert_eq!(fs.read(&mut buff).unwrap(), 1);
        assert_eq!(buff, [90, 66, 67, 68, 69]);

        // check that seeking to a negative value results in an error
        assert!(fs.seek(SeekFrom::Current(-100)).is_err());
        assert!(fs.seek(SeekFrom::End(-100)).is_err());

    }
}

#[wasm_bindgen]
pub struct BigBedWrapper(BigBed<FileString>);

#[wasm_bindgen]
pub fn init_panics() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub fn open_bigbed(data: &[u8]) -> Option<BigBedWrapper> {
    //TODO: move this
    web_sys::console::log_1(&format!("Processing {} bytes...", data.len()).into());
    let data = FileString::from_slice(data);
    // todo: refactor this into honest exports
    match BigBed::from_file(data) {
        Err(x) => {
            alert(&format!("{}", x));
            None
        }
        Ok(bb) => {
            Some(BigBedWrapper(bb))
        }
    }
}

// hard limit on the number of lines that can be displayed in the preview window
// edit this value to allow users to preview more lines
static MAX_QUERY_SIZE: usize = 1000;

#[wasm_bindgen]
pub fn query_bigbed(preview: HtmlTextAreaElement, bigbed: &mut BigBedWrapper, chrom: &str, inp_start: &str, inp_end: &str, inp_count: &str) -> String {
    let chrom: Option<&str> = if chrom == "" { None } else { Some(chrom) };
    let mut start: Option<u32> = None;
    if inp_start != "" {
        match inp_start.parse::<u32>() {
            Ok(num) => start = Some(num),
            Err(msg) => return format!("Bad start value... expected value between 0 and {}, received '{}'", 
                                       u32::max_value(), inp_start)
        }
    }
    let mut end: Option<u32> = None;
    if inp_end != "" {
        match inp_end.parse::<u32>() {
            Ok(num) => end = Some(num),
            Err(msg) => return format!("Bad end value... expected value between 0 and {}, received '{}'", 
                                       u32::max_value(), inp_end)
        }
    }
    let mut count: Option<u32> = None;
    if inp_count != "" {
        match inp_count.parse::<u32>() {
            Ok(num) =>  {
                let max: u32 = MAX_QUERY_SIZE.try_into().unwrap();
                if (max < num) {
                    count = Some(max)
                } else {
                    count = Some(num)
                }
            }
            Err(msg) => return format!("Bad count value... expected value between 0 and {}, received '{}'", 
                                       u32::max_value(), inp_count)
        }
    }
    let output = bigbed.0.to_string(chrom, start, end, count).unwrap();
    if output.len() > MAX_QUERY_SIZE {
        preview.set_inner_html(&output[..MAX_QUERY_SIZE].join(""))
    } else {
        preview.set_inner_html(&output.join(""))
    }
    return String::new();
}

#[wasm_bindgen]
pub fn to_bed(bigbed: &mut BigBedWrapper) -> String {
    bigbed.0.to_string(None, None, None, None).unwrap().join("")
}