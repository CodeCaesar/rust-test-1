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

fn print_car(car: Car) {
    println!("{} {}\ncost: £{}\nreleased in: {}", car.manufacturer, car.model, car.price, car.year_of_release);
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

    print_car(honda_zrv);
    println!();
    print_car(mercedes_glb);
    println!();
    print_car(ferrari_roma);
}