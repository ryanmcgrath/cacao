use std::sync::{Arc, RwLock};

use cacao::lazy_static::lazy_static;
use cacao::macos::App;

use crate::CalculatorApp;

#[derive(Clone, Debug)]
pub enum Msg {
    Push(i32),
    Add,
    Subtract,
    Multiply,
    Divide,
    Decimal,
    Clear,
    Mod,
    Invert,
    Equals
}

/// Asynchronously calls back through to the top of the application
/// on the main thread.
pub fn dispatch(msg: Msg) {
    println!("Dispatching UI message: {:?}", msg);
    CALCULATOR.run(msg)
}

lazy_static! {
    pub static ref CALCULATOR: Calculator = Calculator::new();
}

#[derive(Debug)]
pub struct Calculator(Arc<RwLock<Vec<String>>>);

impl Calculator {
    pub fn new() -> Self {
        Calculator(Arc::new(RwLock::new(Vec::new())))
    }

    pub fn run(&self, message: Msg) {
        let mut expression = self.0.write().unwrap();
        
        match message {
            Msg::Push(i) => {
                // Realistically you might want to check decimal length here or something.
                // We're not bothering for this example.
                (*expression).push(i.to_string());
                let display = (*expression).join("").split(" ").last().unwrap_or("0").to_string();
                App::<CalculatorApp, String>::dispatch_main(display);
            },
            
            Msg::Decimal => {
                let display = (*expression).join("").split(" ").last().unwrap_or("0").to_string();
                if !display.contains(".") {
                    (*expression).push(".".to_string());
                    App::<CalculatorApp, String>::dispatch_main(display + ".");
                }
            },

            Msg::Add => {
                if let Some(last_entry) = (*expression).last() {
                    if !last_entry.ends_with(" ") {
                        (*expression).push(" + ".to_string());
                    }
                }
            },
            
            Msg::Subtract => {
                (*expression).push(" - ".to_string());
            },
            
            Msg::Multiply => {
                (*expression).push(" * ".to_string());
            },

            Msg::Divide => {
                (*expression).push(" / ".to_string());
            },

            Msg::Clear => {
                (*expression) = Vec::new();
                App::<CalculatorApp, String>::dispatch_main("0".to_string())
            },

            Msg::Equals => {
                let mut expr = (*expression).join("");
                if expr.ends_with(" ") {
                    expr.truncate(expr.len() - 3);
                }

                println!("Expr: {}", expr);
                
                match eval::eval(&expr) {
                    Ok(val) => { App::<CalculatorApp, String>::dispatch_main(val.to_string()); },
                    Err(e) => { eprintln!("Error parsing expression: {:?}", e); }
                }
            }

            _ => {}
        }       
    }
}
