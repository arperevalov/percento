use std::env;

struct Conf {
    number_1: f64,
    number_2: f64,
    prefix: CalcType
}

enum CalcType {
    WhatIs,
    IsWhat,
    OfWhat,
    Error
}

impl Conf {
    fn new(args: &[String]) -> Result<Conf, &str> {
        if args.len() < 4 {
            return Err("Not enough arguments!")
        }

        let prefix = match &args[1][..] {
            "-wi" => CalcType::WhatIs,
            "-iw" => CalcType::IsWhat,
            "-ow" => CalcType::OfWhat,
            _ => CalcType::Error
        };

        if matches!(prefix, CalcType::Error) {
            return Err("Provide correct prefix of operation")
        }
        
        let number_1 = args[2].parse().expect("Write a number at second argument!");
        let number_2 = args[3].parse().expect("Write a number at third argument!");

        Ok(Conf{
            prefix,
            number_1,
            number_2
        })
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Conf::new(&args).expect("Something went wrong");

    if matches!(config.prefix, CalcType::WhatIs) {
        what_is(&config)
    }
    if matches!(config.prefix, CalcType::IsWhat) {
        is_what(&config)
    }
    if matches!(config.prefix, CalcType::OfWhat) {
        of_what(&config)
    }
}

fn what_is (config: &Conf) {
    let result = (config.number_1 / 100.0) * config.number_2;
    println!("what is: {}", result)
}

fn is_what (config: &Conf) {
    let result = (config.number_1 / config.number_2) * 100.0;
    println!("is what: {}", result)
}

fn of_what (config: &Conf) {
    let result = (config.number_1 / config.number_2) * 100.0;
    println!("of what: {}", result)
}