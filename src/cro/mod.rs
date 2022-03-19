mod header;

use header::{Header, RawHeader};
use no_std_io::Reader;

#[derive(Debug)]
pub struct Cro {
    data: Vec<u8>,
    header: Header,
}

impl Cro {
    pub fn new(data: Vec<u8>) -> Self {
        let raw_header: RawHeader = data[..Header::SIZE].try_into().expect("Cro too small!");
        let header = Header::from(raw_header);
        Self { data, header }
    }

    pub fn get_header(&self) -> &Header {
        &self.header
    }

    pub fn get_name(&self) -> String {
        let module_name_offset: usize = self
            .header
            .module_name_offset
            .try_into()
            .expect("Cro module_name_offset does not fit into usize!");
        let module_name_size: usize = self
            .header
            .module_name_size
            .try_into()
            .expect("Cro module_name_size does not fit into usize!");

        let bytes = self
            .data
            .default_read_byte_vec(module_name_offset, module_name_size);

        String::from_utf8(bytes).expect("Cro has invalid utf8 name!")
    }
}
