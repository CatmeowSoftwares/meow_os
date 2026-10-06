use bitflags::bitflags;
use num_enum::{IntoPrimitive, TryFromPrimitive};

#[derive(bytemuck::Zeroable, bytemuck::Pod, Clone, Copy, Debug)]
#[repr(C)]
pub(crate) struct ElfHeader {
    pub(crate) magic: u32,
    pub(crate) class: u8,
    pub(crate) endian_type: u8,
    pub(crate) elf_header_version: u8,
    pub(crate) os_abi: u8,
    pub(crate) padding: u64,
    pub(crate) elf_type: u16,
    pub(crate) isa: u16,
    pub(crate) elf_version: u32,
    pub(crate) program_entry_offset: u64,
    pub(crate) program_header_offset: u64,
    pub(crate) section_header_table_offset: u64,
    pub(crate) flags: u32,
    pub(crate) elf_header_size: u16,
    pub(crate) size_of_entry_in_program_header: u16,
    pub(crate) number_of_entries_in_program_header: u16,

    pub(crate) size_of_entry_in_section_header: u16,
    pub(crate) number_of_entries_in_section_header: u16,
    pub(crate) section_string_idx: u16,
}

#[derive(bytemuck::Zeroable, bytemuck::Pod, Clone, Copy, Debug)]
#[repr(C)]
pub(crate) struct ProgramHeader {
    pub(crate) segment_type: u32,
    pub(crate) flags: u32,
    pub(crate) offset: u64,
    pub(crate) virt_addr: u64,
    pub(crate) phys_addr: u64,
    pub(crate) filesz: u64,
    pub(crate) memsz: u64,
    pub(crate) alignment: u64,
}

impl ProgramHeader {
    pub(crate) fn get_type(&self) -> SegmentType {
        SegmentType::try_from(self.segment_type).unwrap()
    }
    pub(crate) fn is_loadable(&self) -> bool {
        matches!(self.get_type(), SegmentType::Load)
    }
}
#[derive(bytemuck::Zeroable, bytemuck::Pod, Clone, Copy, Debug)]
#[repr(C)]
pub(crate) struct SectionHeader {
    pub(crate) section_name: u32,
    pub(crate) section_type: u32,
    pub(crate) flags: u64,
    pub(crate) addr: u64,
    pub(crate) offset: u64,
    pub(crate) size: u64,
    pub(crate) link: u32,
    pub(crate) info: u32,
    pub(crate) align: u64,
    pub(crate) entry_size: u64,
}
bitflags! {
    pub(crate) struct PhFlags: u32 {
        const EXECUTE = 0x1;
        const WRITE = 0x2;
        const READ = 0x4;
        const MASKOS = 0x00ff0000;
        const MASKPROC = 0xff000000;
    }
}
bitflags! {
    pub(crate) struct ShFlags: u32 {
        const WRITE = 0x1;
        const ALLOC = 0x2;
        const EXECINSTR = 0x4;
        const MASKOS = 0x0f00_0000;
        const MASKPROC = 0xf000_0000;
    }
}
pub(crate) enum FileType {
    NoType,
    RelocatableObject,
    Executable,
    SharedObject,
    Core,
}
#[derive(TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub(crate) enum SegmentType {
    Null = 0,
    Load,
    Dynamic,
    Interp,
    Note,
    ShLib,
    PHdr,
    Loos = 0x6000_0000,
    Hios = 0x6fff_ffff,
    LoProc = 0x7000_0000,
    HiProc = 0x7fff_ffff,
}
