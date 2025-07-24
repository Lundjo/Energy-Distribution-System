pub struct Devices {
    pub d1: i32,
    pub d2: i32,
    pub d3: i32,
    pub d4: i32,
    pub d5: i32,
}

impl Devices {
    pub fn new() -> Self {
        Devices {
            d1: 0,
            d2: 0,
            d3: 0,
            d4: 0,
            d5: 0,
        }
    }

    pub fn list_devices(&self) {
        println!("Device 1: {} active", self.d1);
        println!("Device 2: {} active", self.d2);
        println!("Device 3: {} active", self.d3);
        println!("Device 4: {} active", self.d4);
        println!("Device 5: {} active", self.d5);
    }
}