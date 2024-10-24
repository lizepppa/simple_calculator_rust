use std::io;

struct Calculator {
    last_result: f64,
}

impl Calculator {
    fn new() -> Calculator {
        Calculator { last_result: 0.0 }
    }

    fn add(&mut self, a: f64, b: f64) -> f64 {
        self.last_result = a + b;
        self.last_result
    }

    fn subtract(&mut self, a: f64, b: f64) -> f64 {
        self.last_result = a - b;
        self.last_result
    }

    fn multiply(&mut self, a: f64, b: f64) -> f64 {
        self.last_result = a * b;
        self.last_result
    }

    fn divide(&mut self, a: f64, b: f64) -> Result<f64, String> {
        if b == 0.0 {
            Err(String::from("Помилка: ділення на нуль"))
        } else {
            self.last_result = a / b;
            Ok(self.last_result)
        }
    }

    fn get_last_result(&self) -> f64 {
        self.last_result
    }
}

fn main() {
    let mut calculator = Calculator::new();
    loop {
        println!("Введіть перше число (або 'exit' для виходу):");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Не вдалося прочитати рядок");
        let input = input.trim();

        if input == "exit" {
            break;
        }

        let a: f64 = match input.parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Помилка: введіть дійсне число");
                continue;
            }
        };

        println!("Введіть операцію (+, -, *, /):");
        let mut operation = String::new();
        io::stdin().read_line(&mut operation).expect("Не вдалося прочитати рядок");
        let operation = operation.trim();

        println!("Введіть друге число:");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Не вдалося прочитати рядок");
        let input = input.trim();

        let b: f64 = match input.parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Помилка: введіть дійсне число");
                continue;
            }
        };

        match operation {
            "+" => {
                let result = calculator.add(a, b);
                println!("Результат: {}", result);
            }
            "-" => {
                let result = calculator.subtract(a, b);
                println!("Результат: {}", result);
            }
            "*" => {
                let result = calculator.multiply(a, b);
                println!("Результат: {}", result);
            }
            "/" => {
                match calculator.divide(a, b) {
                    Ok(result) => println!("Результат: {}", result),
                    Err(e) => println!("{}", e),
                }
            }
            _ => {
                println!("Помилка: невідома операція");
            }
        }

        println!("Останній результат: {}", calculator.get_last_result());
    }
}
