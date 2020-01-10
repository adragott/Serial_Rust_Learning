extern crate serial;

use std::env;
use std::io;
use std::time::Duration;
use std::str;
use std::io::prelude::*;
use serial::prelude::*;

fn main() {
    for arg in env::args_os().skip(1) {
        let mut port = serial::open(&arg).unwrap();
        interact(&mut port).unwrap();
    }
}

fn interact<T: SerialPort>(port: &mut T) -> io::Result<()> {
    port.reconfigure(&|settings| {
        settings.set_baud_rate(serial::Baud9600)?;
        settings.set_char_size(serial::Bits8);
        settings.set_parity(serial::ParityNone);
        settings.set_stop_bits(serial::Stop1);
        settings.set_flow_control(serial::FlowNone);
        Ok(())
    })?;

    port.set_timeout(Duration::from_millis(10))?;
    loop
    {
        let mut buf = [0_u8; 256];
        let count = match port.read(&mut buf)
        {
            Ok(v) => v,
            Err(e) => 
            {
                continue;
            },
        };
        let s = String::from_utf8_lossy(&buf[0..count]);
        print!("{}", s);
        io::stdout().flush();
   }
    Ok(())
}
