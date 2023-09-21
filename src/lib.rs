use rand::Rng;
use std::cell::Cell;
use std::collections::HashMap;
use std::fmt::Display;
use std::rc::Rc;
use thiserror::Error;

pub trait Device: Display {
    fn name(&self) -> String;
    fn poll(&self);
}

pub struct Socket {
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
    pub fn new(name: &str) -> Self {
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

pub struct SocketStorage {
    devices: Vec<Rc<Socket>>,
}

impl Default for SocketStorage {
    fn default() -> Self {
        Self::new()
    }
}

impl SocketStorage {
    pub fn new() -> Self {
        SocketStorage { devices: vec![] }
    }
}

impl DeviceStorage<Socket> for SocketStorage {
    fn add(&mut self, device: Rc<Socket>) {
        self.devices.push(device)
    }
}

#[derive(Error, Debug)]
#[error("~=<{0}>=~")]
pub struct WhereAmI(String);

#[derive(Error, Debug)]
#[error("The room {0} does not exist")]
pub struct NoSuchRoom(#[from] WhereAmI);

#[derive(Error, Debug)]
#[error("The room already contains this device: {0}")]
pub struct AlreadyContainsDevice(String);

pub struct House {
    pub name: String,
    pub device_by_room: HashMap<String, HashMap<String, Rc<dyn Device>>>,
    pub sockets: SocketStorage,
}

impl House {
    pub fn new(name: &str) -> Self {
        House {
            name: name.to_owned(),
            device_by_room: HashMap::new(),
            sockets: SocketStorage::new(),
        }
    }

    pub fn rooms(&self) -> Vec<String> {
        self.device_by_room.keys().cloned().collect()
    }

    pub fn devices(&self, room: &str) -> Result<Vec<String>, NoSuchRoom> {
        let devices = self
            .device_by_room
            .get(room)
            .ok_or(WhereAmI(room.to_owned()))?;
        Ok(devices.keys().cloned().collect())
    }

    pub fn add_socket_to_room(
        &mut self,
        socket: Rc<Socket>,
        room: &str,
    ) -> Result<(), AlreadyContainsDevice> {
        // позволяет добавлять помещения
        // позволяет добавлять устройства
        if let Ok(devices) = self.devices(room) {
            if devices.contains(&socket.name()) {
                return Err(AlreadyContainsDevice(socket.name.to_owned()));
            }
        }

        self.device_by_room
            .entry(room.to_owned())
            .or_insert(HashMap::new());
        self.device_by_room
            .get_mut(room)
            .unwrap()
            .insert(socket.name.clone(), socket.clone());
        self.sockets.add(socket.clone());
        Ok(())
    }

    pub fn remove_room(&mut self, room: &str) {
        // позволяет удалять помещения
        self.device_by_room.remove(room);
        // Also, remove all devides in that room from the device storage
    }

    pub fn remove_socket_from_room(&mut self, room: &str, socket: Rc<Socket>) {
        if let Some(devices_in_room) = self.device_by_room.get_mut(room) {
            devices_in_room.remove(&socket.name);
        }
        self.sockets
            .devices
            .retain(|sock| !Rc::ptr_eq(sock, &socket));
    }

    pub fn poll(&self) {
        for devices in self.device_by_room.values() {
            for device in devices.values() {
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
            for device in devices.values() {
                device.fmt(f)?;
            }
        }
        Ok(())
    }
}
