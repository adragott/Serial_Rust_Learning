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

    port.set_timeout(Duration::from_millis(1000))?;
    loop
    {
        let mut buf: Vec<u8> = (0..255).collect();
        let retthing = match port.read(&mut buf[..])
        {
            Ok(v) => v,
            Err(e) => 
            {
                println!("{}", e);
                continue;
            },
        };
        
        let mut out = String::new();
        loop
        {
            match str::from_utf8(&buf)
            {
                Ok(s) => 
                {
                    out.push_str(s);
                    break;
                }
                Err(e) => 
                {
                    let (good, bad) = buf.split_at(e.valid_up_to());
                    if !good.is_empty()
                    {
                        let s = unsafe 
                        {
                            // This is safe because we have already validated this
                            // UTF-8 data via the call to str::from_utf8`; There's
                            // no need to check it a second time
                            str::from_utf8_unchecked(good)
                        };
                        out.push_str(s);
                    }
                    
                    if bad.is_empty()
                    {
                        break;
                    }

                    buf = bad[1..].to_vec();
                
                }
            }
        }
        
        println!("{}", out);
    }
    Ok(())
}
