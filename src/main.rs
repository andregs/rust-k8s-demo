use ferris_says::say;
use std::{io::{stdout, BufWriter, Write}, time::Duration};

fn main() {
    let stdout = stdout();
    let mut writer = BufWriter::new(stdout.lock());
    let mut count = 0;
    let messages = vec![
        "Hello, fellow Rustaceans!",
        "I am Ferris.",
        "Nice to meet you!",
        "Ok, I'll say it again.",
    ];

    loop {        
        let message = messages.get(count % messages.len()).unwrap();
        let message = format!("{:04} - {}", count, message);
        let width = message.chars().count();
        
        say(message.as_bytes(), width, &mut writer).unwrap();
        println!("\n");

        writer.flush().unwrap();
        std::thread::sleep(Duration::from_secs(5));
        count += 1;
    }
}
