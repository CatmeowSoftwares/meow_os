pub mod allocators;
pub mod heap;
pub mod pmm;
pub mod vmm;
pub fn init() {
    pmm::init();
    vmm::init();
    heap::init();
}
