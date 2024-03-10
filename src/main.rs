use std::io::Read;
use termios::{Termios, TCSANOW, ECHO, ICANON,ISIG, tcsetattr};



fn enter_raw_mode() -> Termios {
    let std_fd = 0;
    let mut termios = Termios::from_fd(std_fd)
        .expect("failed to get terminal attributes");
    let orig_termios = termios.clone();

    termios.c_lflag &= !(ICANON | ECHO);
    termios.c_lflag &= !ISIG;

    tcsetattr(std_fd, TCSANOW, &termios)
        .expect("cannot set terminal attributes");



    orig_termios
}

fn exit_raw_mode(_orig_termios: Termios) {
    let stdin_fd = 0; // stdin
    tcsetattr(stdin_fd, TCSANOW, &_orig_termios)
        .expect("could not restore terminal attributes");
}

fn use_term_input() {
    loop {
        let input = std::io::stdin()
            .bytes()
            .next()
            .and_then(|result| result.ok());

        if let Some(byte) = input {
            if byte == 17 {
                break;
            }

            let ch = byte as char;
            println!("'{}', {}", ch, byte);
        }
    }
}

fn main() {
    let _orig_termios =  enter_raw_mode();
    use_term_input();
    exit_raw_mode(_orig_termios);
}
