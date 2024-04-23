extern crate user32;
use user32::MessageBeep;

fn main() {
  call_message_beep(16);
}

fn call_message_beep(beep: u32){
    unsafe {
        MessageBeep(beep);
    }
}