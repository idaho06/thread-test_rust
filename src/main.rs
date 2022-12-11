use core::time;
use std::sync::mpsc::Sender;
use std::{sync::mpsc, thread, io};
use std::io::Write;

#[derive(Debug, Clone, Copy)]
enum Messages {
    NoMessage,
    Message01,
    Message02,
    Quit,
}

fn event_creator_thread(tx: Sender<Messages>) {
    let mut current_message = Messages::Message01;
    loop {
        thread::sleep(time::Duration::from_secs(1_u64));
        let message_totx = current_message;
        tx.send(message_totx).unwrap();
        match current_message {
            Messages::Message01 => current_message = Messages::Message02,
            Messages::Message02 => current_message = Messages::Quit,
            Messages::Quit => break,
            _ => (),
        }
    }
}

fn main() {
    println!("Hello, threads!");
    let mut spin: Vec<char> = vec!['|', '/', '-', '\\'];

    let (tx, rx) = mpsc::channel::<Messages>();

    let handle = thread::spawn(move || event_creator_thread(tx));

    let mut count = 0_usize;
    loop {

        let message = rx.try_recv().unwrap_or(Messages::NoMessage);
        match message {
            Messages::NoMessage => thread::sleep(time::Duration::from_millis(1)),
            Messages::Message01 => println!("Message 1!!"),
            Messages::Message02 => println!("Message 2!!"),
            Messages::Quit => break,
            //_ => ()
        }
        print!("\u{8}{}",spin[0]);
        io::stdout().flush().expect("Could not flush stdout");
        spin.rotate_left(1);

        count += 1;
    }

    println!("count = {}", count);
    handle.join().unwrap();
}
