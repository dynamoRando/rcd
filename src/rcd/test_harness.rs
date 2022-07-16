use lazy_static::lazy_static;
use std::sync::Mutex;
use std::thread;

// http://oostens.me/posts/singletons-in-rust/
// we want to increment for all tests the ports used
// so that way we can run multiple client/servers

lazy_static! {
    pub static ref TEST_SETTINGS:Mutex<TestSettings> = Mutex::new(TestSettings{
        max_port: 6000
    });
}

pub struct TestSettings {
    max_port: u32,
}

impl TestSettings {
    pub fn get_next_avail_port(&mut self) -> u32 {
        self.max_port = self.max_port + 1;
        return self.max_port;
    }

    pub fn get_current_port(&self) -> u32{
        return self.max_port;
    }
}
