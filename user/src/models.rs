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
        println!("1. Small: {} active", self.d1);
        println!("2. Medium: {} active", self.d2);
        println!("3. Large: {} active", self.d3);
        println!("4. Strong: {} active", self.d4);
        println!("5. Industrial: {} active", self.d5);
    }

    pub fn change_active_device_number(&mut self, dev: i32, num: i32) {
        match dev {
            2 => {
                self.d2 += num;
                if self.d2 < 0 {
                    self.d2 = 0;
                }
            },
            3 => {
                self.d3 += num;
                if self.d3 < 0 {
                    self.d3 = 0;
                }
            },
            3 => {
                self.d3 += num;
                if self.d3 < 0 {
                    self.d3 = 0;
                }
            },
            4 => {
                self.d4 += num;
                if self.d4 < 0 {
                    self.d4 = 0;
                }
            },
            5 => {
                self.d5 += num;
                if self.d5 < 0 {
                    self.d5 = 0;
                }
            },
            _ => (),
        }

        let _ = insert_into_db(self);
    }
}