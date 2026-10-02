pub mod apic;
struct Timer {
    elapsed: u64,
    wait_time: u64,
    processing: bool,
}

impl Timer {
    fn set_wait_time(&mut self, time: u64) {}
    fn get_wait_time(&self) -> u64 {
        0
    }
    fn set_one_shot(&mut self, value: bool) {}
    fn is_one_shot(&mut self) -> bool {
        false
    }
    fn start(&mut self) {}
    fn stop(&mut self) {}
    fn set_paused(&mut self, value: bool) {}
    fn is_paused(&self) -> bool {
        false
    }
    fn is_stopped(&self) -> bool {
        false
    }
    fn get_time_left(&self) -> u64 {
        0
    }
}

trait TTimer {}
