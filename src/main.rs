use std::env;
use dialoguer::{console::Term, theme::ColorfulTheme, Select};

struct Conf {
    number_1: f64,
    number_2: f64
}

impl Conf {
    fn new(args: &[String]) -> Result<Conf, &str> {
        if args.len() < 3 {
            return Err("Not enough arguments!")
        }
        
        let number_1 = args[1].parse().expect("Write a number at first argument!");
        let number_2 = args[2].parse().expect("Write a number at second argument!");

        Ok(Conf{
            number_1,
            number_2
        })
    }
}

fn main() -> std::io::Result<()> {    
    let args: Vec<String> = env::args().collect();

    let config = Conf::new(&args).expect("Something went wrong");
    
    let variants = vec![
        format!("What is {}% of {}", config.number_1, config.number_2), 
        format!("{} is what percent of {}", config.number_1, config.number_2),
        format!("What is the percentage increase/decrease from {} to {}", config.number_1, config.number_2)
        ];
    let selection = Select::with_theme(&ColorfulTheme::default())
        .items(&variants)
        .default(0)
        .interact_on_opt(&Term::stderr())?;

    match selection {
        Some(0) => what_is(&config),
        Some(1) => is_what(&config),
        Some(2) => of_what(&config),
        None => println!("User did not select anything"),
        _ => println!("User did not select anything")
    }

    Ok(())
}

fn what_is (config: &Conf) {
    let result = (config.number_1 / 100.0) * config.number_2;
    println!("Answer: {}", result)
}

fn is_what (config: &Conf) {
    let result = (config.number_1 / config.number_2) * 100.0;
    println!("Answer: {}", result)
}

fn of_what (config: &Conf) {
    let result = (config.number_2 / config.number_1) * 100.0;
    println!("Answer: {}", result)
}