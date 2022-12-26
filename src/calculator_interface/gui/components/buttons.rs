use kas::{impl_scope, widgets::TextButton, event::VirtualKeyCode};
use super::window::CalculatorAction as CAction;
use super::window::CursorDirection as CDir;

impl_scope! {
    #[widget{
        layout = grid: {
            0, 0: TextButton::new_msg("&clear", CAction::Clear(true));
            0, 1: TextButton::new_msg("Back", CAction::Backspace(true)).with_keys(&[VirtualKeyCode::Back]);
            0, 2: TextButton::new_msg("Del", CAction::Delete(true)).with_keys(&[VirtualKeyCode::Delete]);
            0, 3: TextButton::new_msg("Sbmt", CAction::Submit).with_keys(&[VirtualKeyCode::Return, VirtualKeyCode::NumpadEnter, VirtualKeyCode::Equals]);
            1, 0: TextButton::new_msg("<", CAction::Cursor(CDir::Left)).with_keys(&[VirtualKeyCode::Left]);
            1, 1: TextButton::new_msg("^", CAction::Cursor(CDir::Up)).with_keys(&[VirtualKeyCode::Up]);
            1, 2: TextButton::new_msg(">", CAction::Cursor(CDir::Right)).with_keys(&[VirtualKeyCode::Right]);
            1, 3: TextButton::new_msg("v", CAction::Cursor(CDir::Down)).with_keys(&[VirtualKeyCode::Down]);
            2, 1: TextButton::new_msg("&+", CAction::Insert("+".to_string(), true));
            2, 2: TextButton::new_msg("&-", CAction::Insert("-".to_string(), true));
            2, 3: TextButton::new_msg("&*", CAction::Insert("*".to_string(), true));
            2, 4: TextButton::new_msg("&/", CAction::Insert("/".to_string(), true));
            3, 0: TextButton::new_msg("&^", CAction::Insert("^".to_string(), true));
            3, 1: TextButton::new_msg("&%", CAction::Insert("%".to_string(), true));
            3, 2: TextButton::new_msg("&!", CAction::Insert("!".to_string(), true));
            3, 3: TextButton::new_msg("&0", CAction::Insert("0".to_string(), true));
            4, 0: TextButton::new_msg("&1", CAction::Insert("1".to_string(), true));
            4, 1: TextButton::new_msg("&2", CAction::Insert("2".to_string(), true));
            4, 2: TextButton::new_msg("&3", CAction::Insert("3".to_string(), true));
            4, 3: TextButton::new_msg("&.", CAction::Insert(".".to_string(), true));
            5, 0: TextButton::new_msg("&4", CAction::Insert("4".to_string(), true));
            5, 1: TextButton::new_msg("&5", CAction::Insert("5".to_string(), true));
            5, 2: TextButton::new_msg("&6", CAction::Insert("6".to_string(), true));
            6, 0: TextButton::new_msg("&7", CAction::Insert("7".to_string(), true));
            6, 1: TextButton::new_msg("&8", CAction::Insert("8".to_string(), true));
            6, 2: TextButton::new_msg("&9", CAction::Insert("9".to_string(), true));
        };
    }]

    #[derive(Debug, Default)]
    pub (in crate::calculator_interface::gui) struct Buttons(widget_core!());
}