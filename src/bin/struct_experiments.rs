struct Car {
    manufacturer: String,
    model: String,
    price: u32,
    year_of_release: u16
}

fn build_car(manufacturer: String, model: String, price: u32, year_of_release: u16) -> Car {
    Car {
        manufacturer: manufacturer,
        model: model,
        price: price,
        year_of_release: year_of_release
    }
}

fn main() {
    // Multi-line struct definition
    let manufacturer: String = String::from("Honda");
    let model: String = String::from("ZR-V");
    let price: u32 = 29_450;
    let year_of_release: u16 = 2022;
    let honda_zrv: Car = Car { manufacturer, model, price, year_of_release };

    // Single-line struct definition
    let mercedes_glb: Car = Car {manufacturer: String::from("Mercedes-Benz"), model: String::from("GLB"), price: 24_990, year_of_release: 2020};

    // Function-based struct definition
    let ferrari_roma: Car = build_car(String::from("Ferrari"), String::from("Roma"), 147_500, 2021);

    println!("{} {}\ncost: £{}\nreleased in: {}", honda_zrv.manufacturer, honda_zrv.model, honda_zrv.price, honda_zrv.year_of_release);
    println!();
    println!("{} {}\ncost: £{}\nreleased in: {}", mercedes_glb.manufacturer, mercedes_glb.model, mercedes_glb.price, mercedes_glb.year_of_release);
    println!();
    println!("{} {}\ncost: £{}\nreleased in: {}", ferrari_roma.manufacturer, ferrari_roma.model, ferrari_roma.price, ferrari_roma.year_of_release);
}