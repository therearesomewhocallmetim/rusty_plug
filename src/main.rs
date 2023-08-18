use rand::Rng;
use std::cell::Cell;
use std::collections::HashMap;
use std::error::Error;
use std::fmt::Display;
use std::rc::Rc;

trait Device: Display {
    fn name(&self) -> String;
    fn poll(&self);
}

struct Socket {
    name: String,
    voltage: Cell<f64>,
}

impl Display for Socket {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "SOCKET:\n    name: {}\n    voltage: {:.2}\n",
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
        self.voltage.set(Self::rand_voltage());
    }
}

impl Socket {
    fn new(name: &str) -> Self {
        let voltage = Self::rand_voltage();

        Socket {
            name: name.to_owned(),
            voltage: voltage.into(),
        }
    }

    fn rand_voltage() -> f64 {
        let mut r = rand::thread_rng();
        r.gen_range(0.0..380.0)
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

#[derive(Debug)]
struct NoSuchRoom(String);
impl Display for NoSuchRoom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.0))
    }
}
impl Error for NoSuchRoom {}

#[derive(Debug)]
struct AlreadyContainsDevice(String);
impl Display for AlreadyContainsDevice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.0))
    }
}

struct House {
    name: String,
    device_by_room: HashMap<String, Vec<Rc<dyn Device>>>,
    sockets: SocketStorage,
}

impl House {
    fn new(name: &str) -> Self {
        House {
            name: name.to_owned(),
            device_by_room: HashMap::new(),
            sockets: SocketStorage::new(),
        }
    }

    fn rooms(&self) -> Vec<String> {
        self.device_by_room.keys().cloned().collect()
    }

    fn devices(&self, room: &str) -> Result<Vec<String>, NoSuchRoom> {
        let devices = self
            .device_by_room
            .get(room)
            .ok_or(NoSuchRoom(room.to_owned()))?;
        Ok(devices.iter().map(|device| device.name()).collect())
    }

    fn add_socket_to_room(
        &mut self,
        socket: Rc<Socket>,
        room: &str,
    ) -> Result<(), AlreadyContainsDevice> {
        // let existing_devices = self.devices(room);
        if let Ok(devices) = self.devices(room) {
            if devices.contains(&socket.name()) {
                return Err(AlreadyContainsDevice(socket.name.to_owned()));
            }
        }

        self.device_by_room.entry(room.to_owned()).or_insert(vec![]);
        self.device_by_room
            .get_mut(room)
            .unwrap()
            .push(socket.clone());
        self.sockets.add(socket.clone());
        Ok(())
    }

    fn poll(&self) {
        for devices in self.device_by_room.values() {
            for device in devices {
                device.poll();
            }
        }
    }
}

impl Display for House {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("House «{}»:\n", self.name))?;
        for (room, devices) in self.device_by_room.iter() {
            room.fmt(f)?;
            "\n".fmt(f)?;
            for device in devices {
                device.fmt(f)?;
            }
        }
        Ok(())
    }
}

fn main() {
    let socket = Socket::new("Hello");
    println!("{}", socket);

    let socket1 = Rc::new(socket);
    let mut house = House::new("The Rising Sun");
    house
        .add_socket_to_room(socket1.clone(), "bedroom")
        .expect("");

    println!("The house is: \n{}", house);
    let socket2 = Rc::new(Socket::new("My other socket"));
    house
        .add_socket_to_room(socket2, "bedroom")
        .expect("Should add");
    let socket3 = Rc::new(Socket::new("Hello"));
    house
        .add_socket_to_room(socket3, "bedroom")
        .expect_err("Should get error");
    println!("The house AFTER ADDING ANOTHER SOCKET is: \n{}", house);
    house.poll();
    println!("The house AFTER POLLING is: \n{}", house);
    println!("Rooms in the house are: {:?}", house.rooms());

    let devices_in_bedroom = house.devices("bedroom");
    match devices_in_bedroom {
        Ok(devices) => println!("Devices in bedroom are {:?}", devices),
        Err(_) => println!("There's been an error"),
    }
}
