use crate::database::insert_into_db;

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

    pub fn change_active_device_number(&mut self, dev: i32, num: i32) {
        match dev {
            1 => self.d1 += num,
            2 => self.d2 += num,
            3 => self.d3 += num,
            4 => self.d4 += num,
            5 => self.d5 += num,
            _ => (),
        }

        let _ = insert_into_db(self);
    }
}