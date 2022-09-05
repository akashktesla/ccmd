use std::time::Duration;
use std::thread::sleep;
use chrono::prelude::*;
use std::io::{self,Write};
//
// use std::fmt;

fn main() {
     loop{
        let now = Local::now();
        print!("\r{}:{}:{}",now.hour12().1,now.minute(),now.second());
        io::stdout().flush().unwrap();
        sleep(Duration::new(1,0));
    }

}
