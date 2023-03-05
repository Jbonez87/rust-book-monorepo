use std::io;

fn main() {
    println!("Hello, world!");
    get_temperature();
}

fn get_temperature() -> String {
    println!("Convert to Fahrenheit or Celsius? Feel free to use lowercase, uppercase or just lowercase/uppercase c or f.");

    let mut temp_metric: String = String::new();

    let mut temperature: String = String::new();

    io::stdin()
        .read_line(&mut temp_metric)
        .expect("Failed to read line");

    println!("Please enter degrees");
    io::stdin()
        .read_line(&mut temperature)
        .expect("Failed to read line");

    let temperature: i32 = match temperature.trim().parse() {
            Ok(num) => num,
            Err(_) => panic!("Error"),
        };
    
    println!("{}", temp_metric.as_str());
    
    let temp_metric: String = match temp_metric.as_str().trim() {
        "Fahrenheit" => String::from("f"),
        "fahrenheit" => String::from("f"),
        "f" => String::from("f"),
        "F" => String::from("f"),
        "Celius" => String::from("c"),
        "celsius" => String::from("c"),
        "c" => String::from("c"),
        "C" => String::from("c"),
        _ => String::from("Error"),
    };

    let converted_temperature = convert_temperature(temperature, temp_metric);

    println!("{converted_temperature}");

    return converted_temperature;
}

fn convert_temperature(degrees: i32, metric: String) -> String {
    if metric == "f" {
        let temperature: i32 = degrees - 32;
        return String::from(temperature.to_string() + "c");
    } else if metric == "c" {
        let temperature: i32 = degrees + 32;
        return String::from(temperature.to_string() + "f");
    } else {
        panic!("Please enter degrees and temperature");
    }
}

