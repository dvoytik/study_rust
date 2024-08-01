trait AddressSpace {
    fn get_name(&self) -> &'static str;
}

// A collection of trait objects
struct Bus {
    address_space: Vec<Box<dyn AddressSpace>>,
}

struct Device {
    name: &'static str,
}

impl AddressSpace for Device {
    fn get_name(&self) -> &'static str {
        self.name
    }
}

struct Memory;

impl AddressSpace for Memory {
    fn get_name(&self) -> &'static str {
        "memory"
    }
}

fn main() {
    let mut bus = Bus {
        address_space: Vec::new(),
    };

    bus.address_space
        .push(Box::new(Device { name: "device 0" }));
    bus.address_space
        .push(Box::new(Device { name: "device 1" }));
    bus.address_space.push(Box::new(Memory));

    for a in bus.address_space {
        println!("Name: {}", a.get_name())
    }
}
