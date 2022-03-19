use no_std_io::{StreamContainer, StreamReader};

type HashTable = [u8; 0x80];

#[derive(Debug)]
pub struct Header {
    pub(super) hash_table: HashTable,
    pub(super) magic: [u8; 4], // CRO0
    pub(super) name_offset: u32,
    pub(super) next_cro: u32,
    pub(super) previous_cro: u32,
    pub(super) file_size: u32,
    pub(super) bss_size: u32,
    pub(super) fixed_size: u32,
    pub(super) unknown_zero: u32,
    pub(super) unk_segment_tag: u32,
    pub(super) on_load_segment_tag: u32,
    pub(super) on_exit_segment_tag: u32,
    pub(super) on_unresolved_segment_tag: u32,
    pub(super) code_offset: u32,
    pub(super) code_size: u32,
    pub(super) data_offset: u32,
    pub(super) data_size: u32,
    pub(super) module_name_offset: u32,
    pub(super) module_name_size: u32,
    pub(super) segment_table_offset: u32,
    pub(super) segment_num: u32,
    pub(super) export_named_symbol_table_offset: u32,
    pub(super) export_named_symbol_num: u32,
    pub(super) export_indexed_symbol_table_offset: u32,
    pub(super) export_indexed_symbol_num: u32,
    pub(super) export_strings_offset: u32,
    pub(super) export_strings_size: u32,
    pub(super) export_tree_table_offset: u32,
    pub(super) export_tree_num: u32,
    pub(super) import_module_table_offset: u32,
    pub(super) import_module_num: u32,
    pub(super) external_relocation_table_offset: u32,
    pub(super) external_relocation_num: u32,
    pub(super) import_named_symbol_table_offset: u32,
    pub(super) import_named_symbol_num: u32,
    pub(super) import_indexed_symbol_table_offset: u32,
    pub(super) import_indexed_symbol_num: u32,
    pub(super) import_anonymous_symbol_table_offset: u32,
    pub(super) import_anonymous_symbol_num: u32,
    pub(super) import_strings_offset: u32,
    pub(super) import_strings_size: u32,
    pub(super) static_anonymous_symbol_table_offset: u32,
    pub(super) static_anonymous_symbol_num: u32,
    pub(super) internal_relocation_table_offset: u32,
    pub(super) internal_relocation_num: u32,
    pub(super) static_relocation_table_offset: u32,
    pub(super) static_relocation_num: u32,
}

impl Header {
    pub const SIZE: usize = 0x134;
}

pub(super) type RawHeader = [u8; Header::SIZE];

impl From<RawHeader> for Header {
    fn from(bytes: RawHeader) -> Self {
        let mut raw = StreamContainer::new(bytes);

        let hash_table = raw
            .default_read_byte_stream(0x80)
            .try_into()
            .expect("Cro hash table size is invalid!");

        let magic = raw.default_read_stream();

        if magic != [0x43, 0x52, 0x4f, 0x30] {
            panic!("Invalid cro header!")
        }

        Self {
            hash_table,
            magic,
            name_offset: raw.default_read_stream_le(),
            next_cro: raw.default_read_stream_le(),
            previous_cro: raw.default_read_stream_le(),
            file_size: raw.default_read_stream_le(),
            bss_size: raw.default_read_stream_le(),
            fixed_size: raw.default_read_stream_le(),
            unknown_zero: raw.default_read_stream_le(),
            unk_segment_tag: raw.default_read_stream_le(),
            on_load_segment_tag: raw.default_read_stream_le(),
            on_exit_segment_tag: raw.default_read_stream_le(),
            on_unresolved_segment_tag: raw.default_read_stream_le(),
            code_offset: raw.default_read_stream_le(),
            code_size: raw.default_read_stream_le(),
            data_offset: raw.default_read_stream_le(),
            data_size: raw.default_read_stream_le(),
            module_name_offset: raw.default_read_stream_le(),
            module_name_size: raw.default_read_stream_le(),
            segment_table_offset: raw.default_read_stream_le(),
            segment_num: raw.default_read_stream_le(),
            export_named_symbol_table_offset: raw.default_read_stream_le(),
            export_named_symbol_num: raw.default_read_stream_le(),
            export_indexed_symbol_table_offset: raw.default_read_stream_le(),
            export_indexed_symbol_num: raw.default_read_stream_le(),
            export_strings_offset: raw.default_read_stream_le(),
            export_strings_size: raw.default_read_stream_le(),
            export_tree_table_offset: raw.default_read_stream_le(),
            export_tree_num: raw.default_read_stream_le(),
            import_module_table_offset: raw.default_read_stream_le(),
            import_module_num: raw.default_read_stream_le(),
            external_relocation_table_offset: raw.default_read_stream_le(),
            external_relocation_num: raw.default_read_stream_le(),
            import_named_symbol_table_offset: raw.default_read_stream_le(),
            import_named_symbol_num: raw.default_read_stream_le(),
            import_indexed_symbol_table_offset: raw.default_read_stream_le(),
            import_indexed_symbol_num: raw.default_read_stream_le(),
            import_anonymous_symbol_table_offset: raw.default_read_stream_le(),
            import_anonymous_symbol_num: raw.default_read_stream_le(),
            import_strings_offset: raw.default_read_stream_le(),
            import_strings_size: raw.default_read_stream_le(),
            static_anonymous_symbol_table_offset: raw.default_read_stream_le(),
            static_anonymous_symbol_num: raw.default_read_stream_le(),
            internal_relocation_table_offset: raw.default_read_stream_le(),
            internal_relocation_num: raw.default_read_stream_le(),
            static_relocation_table_offset: raw.default_read_stream_le(),
            static_relocation_num: raw.default_read_stream_le(),
        }
    }
}
