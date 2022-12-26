
use crate::calculator_logic;
use super::ui_trait::*;

use calculator_logic::calculator::*;

mod components;

#[derive(Debug, Default)]
pub struct GraphicalUI { }

impl GraphicalUI {
    fn start_kas(&self, calculator: Calculator) -> kas::shell::Result<()> {
        env_logger::init();

        let mut window = components::window::Window::default();
        window.attach_calculator(calculator);

        let theme = kas::theme::SimpleTheme::new().with_font_size(16.0);

        kas::shell::Toolkit::new(theme)?
            .with(window)?
            .run()
    }
}

impl CalculatorUI for GraphicalUI {
    fn start(&mut self, calculator: Calculator) -> Result<(), String> {
        self.start_kas(calculator).map_err(|e| e.to_string())
    }
}
