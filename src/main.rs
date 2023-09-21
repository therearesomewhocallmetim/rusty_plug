use smart_home_with_rc::{House, Socket};
use std::rc::Rc;

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
        .add_socket_to_room(socket3.clone(), "bedroom")
        .expect_err("Should get error");
    println!("The house AFTER ADDING ANOTHER SOCKET is: \n{}", house);
    house.poll();
    println!("The house AFTER POLLING is: \n{}", house);
    println!("Rooms in the house are: {:?}", house.rooms());

    let socket4 = Rc::new(Socket::new("Hello"));
    let res = house.add_socket_to_room(socket4, "bedroom");
    if let Err(e) = res {
        println!("{}", e);
    }

    let res = house.devices("No such room");
    if let Err(e) = res {
        println!("Composed error: {}", e);
    }

    house.remove_socket_from_room("bedroom", socket3);
    println!("The house AFTER REMOVING socket is: \n{}", house);

    let devices_in_bedroom = house.devices("bedroom");
    match devices_in_bedroom {
        Ok(devices) => println!("Devices in bedroom are {:?}", devices),
        Err(_) => println!("There's been an error"),
    }
    house.remove_room("bedroom");
    println!("The house AFTER REMOVING bedroom is: \n{}", house);
}
