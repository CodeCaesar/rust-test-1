struct Car {
    manufacturer: String,
    model: String,
    price: u32,
    year_of_release: u16
}

fn main() {
    let manufacturer: String = String::from("Honda");
    let model: String = String::from("ZR-V");
    let price: u32 = 29_450;
    let year_of_release: u16 = 2022;
    let honda_element: Car = Car { manufacturer, model, price, year_of_release };

    println!("{} {}\ncost: £{}\nreleased in: {}", honda_element.manufacturer, honda_element.model, honda_element.price, honda_element.year_of_release)
}