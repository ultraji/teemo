use rand::distributions::Alphanumeric;
use rand::Rng;
use thirtyfour::Key;

pub fn gen_random_filename() -> String {
    return rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(16)
        .map(char::from)
        .collect();
}

pub fn gen_key_content_from_input(keys: &Vec<String>) -> String {
    let mut result = String::new();
    for v in keys {
        match v.as_str() {
            "Key::Null" => result.push(Key::Null.into()),
            "Key::Cancel" => result.push(Key::Cancel.into()),
            "Key::Help" => result.push(Key::Help.into()),
            "Key::Backspace" => result.push(Key::Backspace.into()),
            "Key::Tab" => result.push(Key::Tab.into()),
            "Key::Clear" => result.push(Key::Clear.into()),
            "Key::Return" => result.push(Key::Return.into()),
            "Key::Enter" => result.push(Key::Enter.into()),
            "Key::Shift" => result.push(Key::Shift.into()),
            "Key::Control" => result.push(Key::Control.into()),
            "Key::Alt" => result.push(Key::Alt.into()),
            "Key::Pause" => result.push(Key::Pause.into()),
            "Key::Escape" => result.push(Key::Escape.into()),
            "Key::Space" => result.push(Key::Space.into()),
            "Key::PageUp" => result.push(Key::PageUp.into()),
            "Key::PageDown" => result.push(Key::PageDown.into()),
            "Key::End" => result.push(Key::End.into()),
            "Key::Home" => result.push(Key::Home.into()),
            "Key::Left" => result.push(Key::Left.into()),
            "Key::Up" => result.push(Key::Up.into()),
            "Key::Right" => result.push(Key::Right.into()),
            "Key::Down" => result.push(Key::Down.into()),
            "Key::Insert" => result.push(Key::Insert.into()),
            "Key::Delete" => result.push(Key::Delete.into()),
            "Key::Semicolon" => result.push(Key::Semicolon.into()),
            "Key::Equals" => result.push(Key::Equals.into()),
            "Key::NumPad0" => result.push(Key::NumPad0.into()),
            "Key::NumPad1" => result.push(Key::NumPad1.into()),
            "Key::NumPad2" => result.push(Key::NumPad2.into()),
            "Key::NumPad3" => result.push(Key::NumPad3.into()),
            "Key::NumPad4" => result.push(Key::NumPad4.into()),
            "Key::NumPad5" => result.push(Key::NumPad5.into()),
            "Key::NumPad6" => result.push(Key::NumPad6.into()),
            "Key::NumPad7" => result.push(Key::NumPad7.into()),
            "Key::NumPad8" => result.push(Key::NumPad8.into()),
            "Key::NumPad9" => result.push(Key::NumPad9.into()),
            "Key::Multiply" => result.push(Key::Multiply.into()),
            "Key::Add" => result.push(Key::Add.into()),
            "Key::Separator" => result.push(Key::Separator.into()),
            "Key::Subtract" => result.push(Key::Subtract.into()),
            "Key::Decimal" => result.push(Key::Decimal.into()),
            "Key::Divide" => result.push(Key::Divide.into()),
            "Key::F1" => result.push(Key::F1.into()),
            "Key::F2" => result.push(Key::F2.into()),
            "Key::F3" => result.push(Key::F3.into()),
            "Key::F4" => result.push(Key::F4.into()),
            "Key::F5" => result.push(Key::F5.into()),
            "Key::F6" => result.push(Key::F6.into()),
            "Key::F7" => result.push(Key::F7.into()),
            "Key::F8" => result.push(Key::F8.into()),
            "Key::F9" => result.push(Key::F9.into()),
            "Key::F10" => result.push(Key::F10.into()),
            "Key::F11" => result.push(Key::F11.into()),
            "Key::F12" => result.push(Key::F12.into()),
            "Key::Meta" => result.push(Key::Meta.into()),
            "Key::Command" => result.push(Key::Command.into()),
            others => result.push_str(&others),
        }
    }
    result
}