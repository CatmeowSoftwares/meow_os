use core::ptr;

use alloc::vec::Vec;

use crate::{
    arch::x86_64::tss::TSS,
    elf::{self, ElfHeader, PhFlags, ProgramHeader, SectionHeader},
    kprintln,
    mem::{
        pmm::{self, PAGE_SIZE},
        vmm::{Flags, Pml4, cr3_map_address, create_cr3, get_hhdm, map_address},
    },
    scheduler::thread::jump_usermode,
};

#[derive(Debug)]
struct Process {
    entry: u64,
    cr3: u64,
    stack: u64,
}
impl Process {
    pub fn new(data: &[u8]) -> Self {
        let elf_header: ElfHeader = bytemuck::pod_read_unaligned(&data[..size_of::<ElfHeader>()]);
        kprintln!("{elf_header:#?}");
        let ph_size = size_of::<ProgramHeader>();
        let ph_off = elf_header.program_header_offset as usize;
        let ph_num = elf_header.number_of_entries_in_program_header as usize;

        let mut program_headers: Vec<ProgramHeader> = Vec::new();

        for i in 0..ph_num {
            let start = ph_off + (i * ph_size);
            program_headers.push(bytemuck::pod_read_unaligned(&data[start..start + ph_size]));
        }

        let sh_size = size_of::<SectionHeader>();
        let sh_off = elf_header.section_header_table_offset as usize;
        let sh_num = elf_header.number_of_entries_in_section_header as usize;

        let section_headers: Vec<SectionHeader> = (0..sh_num)
            .map(|i| {
                let start = sh_off + i * sh_size;
                bytemuck::pod_read_unaligned(&data[start..start + sh_size])
            })
            .collect();
        kprintln!("creating cr3");
        let cr3 = create_cr3() as _;
        kprintln!("{:#x}", cr3);

        for entry in &program_headers {
            unsafe {
                kprintln!("{entry:#x?}");
                if !entry.is_loadable() {
                    continue;
                }

                let vaddr = entry.virt_addr;
                let mem_offset = entry.offset;
                let file_size = entry.filesz;
                let mem_size = entry.memsz;

                let count = mem_size.max(PAGE_SIZE) / PAGE_SIZE;
                for i in 0..count {
                    // Allocate vmem for each segment from `vaddr`, size: `memsz`

                    let phys = pmm::allocate();
                    let flags = elf::PhFlags::from_bits_retain(entry.flags);

                    let mut page_flags: Flags = Flags::PRESENT | Flags::USER;

                    if flags.contains(PhFlags::WRITE) {
                        page_flags |= Flags::WRITE;
                    }
                    if !flags.contains(PhFlags::EXECUTE) {
                        page_flags |= Flags::XD;
                    }

                    cr3_map_address(
                        &mut *((cr3 + get_hhdm()) as *mut Pml4),
                        (vaddr + (PAGE_SIZE * i)) as _,
                        phys as _,
                        page_flags,
                    );
                    map_address(
                        vaddr as _,
                        phys as _,
                        Flags::PRESENT | Flags::WRITE | Flags::USER,
                    );

                    kprintln!("mapped");
                    // TODO: fix this part
                    // Copy segment data from `offset`
                    let src: *const u8 = (data.as_ptr() as u64 + mem_offset) as _;
                    let dest = vaddr as _;
                    let count = file_size as _;
                    ptr::copy_nonoverlapping::<u8>(src, dest, count);

                    // Zero the rest of the stuff
                    let zero_dst = vaddr + mem_size;
                    let zero_count = mem_size - file_size;
                    ptr::write_bytes::<u8>(zero_dst as _, 0, zero_count as _);
                }
            }
        }
        let count = section_headers.len();
        kprintln!("count: {count}");

        let addr: u64 = pmm::allocate() as _;
        map_address(
            elf_header.program_entry_offset as _,
            addr as _,
            Flags::USER | Flags::WRITE | Flags::PRESENT,
        );
        let top = (addr as u64) + 0x1000;
        let stack = top;
        kprintln!("created process (????)");
        Self {
            cr3,
            entry: elf_header.program_entry_offset,
            stack,
        }
    }
}
pub fn init(data: &[u8]) {
    let process = Process::new(data);
    kprintln!("{:?}", process);
    kprintln!(
        "entry: {:#x}, stack: {:#x}, cr3: {:#x}",
        process.entry,
        process.stack,
        process.cr3
    );
    {
        let mut tss = TSS.lock();
        let frame = pmm::allocate() as u64;
        let kstack_top = frame + get_hhdm() + PAGE_SIZE;
        unsafe {
            core::ptr::write_unaligned(&raw mut tss.rsp0, kstack_top);
        }
    }
    jump_usermode(process.entry, process.stack, process.cr3);
}
