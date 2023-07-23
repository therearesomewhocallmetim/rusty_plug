use rand::Rng;
use std::cell::Cell;
use std::collections::HashMap;
use std::fmt::Display;
use std::hash::{Hash, Hasher};
use std::rc::Rc;

trait Device: Display {
    fn name(&self) -> String;
    fn poll(&self);
}

struct Socket {
    name: String,
    voltage: Cell<f64>,
}

struct Thermometer {
    name: String,
    temperature: Cell<f64>,
}

impl Display for Socket {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "SOCKET:\n    name: {}\n    voltage: {}\n",
            self.name,
            self.voltage.get()
        ))
    }
}

impl Device for Socket {
    fn name(&self) -> String {
        self.name.clone()
    }
    fn poll(&self) {
        let mut rng = rand::thread_rng();

        self.voltage.set(rng.gen_range(0.0..380.0));
    }
}

impl Socket {
    fn new(name: &str) -> Self {
        let mut r = rand::thread_rng();
        let voltage = r.gen_range(0.0..380.0);

        Socket {
            name: name.to_owned(),
            voltage: voltage.into(),
        }
    }
}

trait DeviceStorage<T: Device> {
    fn add(&mut self, device: Rc<T>);
}

struct SocketStorage {
    devices: Vec<Rc<Socket>>,
}

impl SocketStorage {
    fn new() -> Self {
        SocketStorage { devices: vec![] }
    }
}

impl DeviceStorage<Socket> for SocketStorage {
    fn add(&mut self, device: Rc<Socket>) {
        self.devices.push(device)
    }
}

struct Room {
    devices: Vec<Rc<dyn Device>>,
    name: String,
}
impl PartialEq for Room {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for Room {}

impl Hash for Room {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state)
    }
}

impl Room {
    fn add_device(&mut self, device: Rc<dyn Device>) {
        self.devices.push(device)
    }
}

impl Display for Room {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("ROOM: {}\n", self.name))
    }
}

struct House<'a> {
    device_by_room: HashMap<&'a Room, Vec<Rc<dyn Device>>>,
    sockets: SocketStorage,
}

impl<'a> House<'a> {
    fn new() -> Self {
        House {
            device_by_room: HashMap::new(),
            sockets: SocketStorage::new(),
        }
    }

    fn add_socket_to_room(&mut self, socket: Rc<Socket>, room: &'a Room) {
        self.device_by_room.entry(room).or_insert(vec![]);
        self.device_by_room
            .get_mut(room)
            .unwrap()
            .push(socket.clone());
        self.sockets.add(socket.clone());
    }

    fn poll(&self) {
        for (_, devices) in &self.device_by_room {
            for device in devices {
                device.poll();
            }
        }
    }
}

impl<'a> Display for House<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("House:\n")?;
        for (room, devices) in self.device_by_room.iter() {
            room.fmt(f)?;
            for device in devices {
                device.fmt(f)?;
            }
        }
        Ok(())
    }
}

fn main() {
    println!("Hello, world!");
    let socket = Socket::new("Hello");
    println!("{}", socket);

    let mut sockets = SocketStorage { devices: vec![] };
    let pointer = Rc::new(socket);

    sockets.add(pointer.clone());

    let mut room = Room {
        devices: vec![],
        name: "My Room".to_owned(),
    };
    room.add_device(pointer.clone());

    println!("Room: {}", room);

    let mut house = House::new();
    house.add_socket_to_room(pointer.clone(), &room);

    println!("The house is: \n{}", house);
    let socket2 = Rc::new(Socket::new("My other socket"));
    house.add_socket_to_room(socket2, &room);
    println!("The house NOW is: \n{}", house);
    house.poll();
    println!("The house AFTER POLLING is: \n{}", house);
}
