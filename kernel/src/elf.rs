struct ElfHeader {
    magic: u32,
    class: u8,
    endian_type: u8,
    elf_header_version: u8,
    os_abi: u8,
    _padding: u64,
    elf_type: u16,
    isa: u16,
    elf_version: u32,
    program_entry_offset: u64,
    program_header_offset: u64,
    section_header_table_offset: u64,
    flags: u32,
    elf_header_size: u16,
    size_of_entry_in_program_header: u16,
    number_of_entries_in_program_header: u16,

    size_of_entry_in_section_header: u16,
    number_of_entries_in_section_header: u16,
}

struct ProgramHeader {
    segment_type: u32,
    flags: u32,
    offset: u64,
    virt_addr: u64,
    phys_addr: u64,
    filesz: u64,
    memsz: u64,
    required_alignment: u64,
}
