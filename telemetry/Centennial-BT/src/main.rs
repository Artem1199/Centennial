use bluetooth_serial_port::{BtProtocol, BtSocket, BtSocketConnect, BtAsync};
use std::io::{Read, Write, BufRead, BufReader};
use std::io;
use std::slice::Split;
use std::cmp::Ordering;
use std::str::FromStr;
use std::num::ParseIntError;
use mio::{Events, Poll, PollOpt, Token, Ready, unix::UnixReady};
use std::time::Duration;
use byteorder::{ByteOrder, LE};
use csv::{WriterBuilder, QuoteStyle};
use cobs;

use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::TryRecvError;
use std::{thread, time};

fn is_closed(state: Ready) -> bool {
    state.contains(UnixReady::hup() | UnixReady::error())
}



fn main() {

    let stdin_channel = spawn_stdin_channel();



        // create and connect the RFCOMM socket
    let mut socket = BtSocket::new(BtProtocol::RFCOMM).unwrap();

        // scan for devices
    //let devices = bluetooth_serial_port::scan_devices(Duration::from_secs(5)).unwrap();
    //if devices.len() == 0 { panic!("No devices found"); };

    

        // Cetennial bot string
    let botname = String::from("Cent");

    
        // Connect to BT module if it exists

    'connectlp: loop {
        let devices = bluetooth_serial_port::scan_devices(Duration::from_secs(5)).unwrap();
        // if devices.len() == 0 { panic!("No devices found"); };
        println!("{:?}", devices);

        for dev in devices.into_iter(){
            match botname.cmp(&dev.name){
                Ordering::Equal => {
                    #[warn(unused_must_use)]
                    socket.connect(dev.addr);
                    println!("Connecting to `{}` ({})", dev.name, dev.addr.to_string());
                    break 'connectlp;
                    },
                _ => println!("Ignoring `{}` ({})", dev.name, dev.addr.to_string())
            }
        }  

    }
        
    
    let mut buffer = [0;1024];

        // create wtr for writing data to file
    let mut wtr = WriterBuilder::new().delimiter(b'\t').quote_style(QuoteStyle::NonNumeric).from_path("/home/artem/Desktop/git/Centennial/telemetry/botdata.csv").unwrap();

        // create storage for events
    let mut events = Events::with_capacity(128);
    //let temp = async_socket.advance().unwrap();

    // // BtSocket also implements `mio::Evented` for async IO
    let poll = Poll::new().unwrap();
    poll.register(&socket, Token(0), Ready::readable() , PollOpt::edge() | PollOpt::oneshot()).unwrap();
    // poll.register(&socket, Token(0), Ready::readable(), PollOpt::edge() | PollOpt::oneshot()).unwrap();

    let mut settings = (0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        // ... poll events and wait for socket to be readable/writable ...
    'outer: loop {
       // println!("Polling for events"); 
        if let Err(ref e) = poll.poll(&mut events, None){
            println!("poll failed: {}", e);
            break;
        }
        if events.is_empty() {
            println!("Read timed out!");
            continue;
        }

        for event in events.iter () {
            match event.token() {
                Token(0) => {
                    let ready = event.readiness();

                    if is_closed(ready){
                        println!("Quitting due to event: {:?}", ready);
                        break 'outer;
                    }
                    let mut received_std = false;
                    

                    match stdin_channel.try_recv() {
                        Ok(key) =>{
                            println!("Received: {:?}", key);
                            received_std = true;
                            settings = key;
                        },
                        Err(TryRecvError::Empty) => {},
                        Err(TryRecvError::Disconnected) => panic!("Channel disconnected"),
                    }

                    if ready.is_writable() {
                        println!("Event is writable: {:?}", event);

                       let mut tx_buf = [0; 24];
                       LE::write_f32(&mut tx_buf[0..4], settings.0);
                       LE::write_f32(&mut tx_buf[4..8], settings.1);
                       LE::write_f32(&mut tx_buf[8..12], settings.2);
                       LE::write_f32(&mut tx_buf[12..16], settings.3);
                       LE::write_f32(&mut tx_buf[16..20], settings.4);
                       LE::write_f32(&mut tx_buf[20..24], settings.5);
                       println!("Writing data: {:?}", tx_buf);
                       let count = socket.write(&tx_buf[0..24]).unwrap();
                       println!("Wrote {} bytes to robot.", count);
                       received_std = false;
    

                        poll.reregister(&socket, Token(0), Ready::readable(), PollOpt::edge() | PollOpt::oneshot()).unwrap();
                    } 
                    else if ready.is_readable() {
                       // println!("Event is readable: {:?}", event); 
                       // loop {
                            buffer = [0;1024];
                            match socket.read(&mut buffer[..]){
                                Ok(count) => {
                                    // println!("Converting and storing bot data to file. Read: {} ", count);
                                    let mut i = 0;
                                    let mut q = [0; 18];
                                    for x in buffer.iter(){
                                       //println!{"index({}), {}", i, *x};
                                        if *x == 0 && i == 17 {

                                                if let Ok(n) = cobs::decode_in_place(&mut q[..]) {
                                                //    println!("q buff after decode: {:?}", q);
                                                    
                                                    let mut start = 0;
                                                        let q1 = LE::read_f32(&mut q[start..start + 4]);
                                                        start += 4;  //4
                                                        let q2 = LE::read_f32(&mut q[start..start + 4]);
                                                        start += 4;  //8
                                                        let q3 = LE::read_f32(&mut q[start..start + 4]);
                                                        start += 4;  //12
                                                        let q4 = LE::read_f32(&mut q[start..start + 4]);
                                                        if let Err(ref e) = wtr.serialize(&[q1, q2, q3, q4]){
                                                            println!("Error storing data to file: {:?}", e);
                                                        }
                                                        if let Err(ref e) = wtr.flush(){
                                                            println!("Error flushing wtr: {:?}", e);
                                                        }
                                                        i = 0;
                                                        q = [0;18];

                                                    }
                                            //}
                                        } else if *x == 0 && i != 17 {
                                            q = [0;18];
                                            i = 0;
                                        } else if *x != 0 && i == 17 {
                                            q = [0;18];
                                            i = 0;
                                        } else {
                                            q[i] = *x;
                                            i += 1;
                                        }

                                    }
                                }
                                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                                    println!("WouldBlock reached");
                                    break;
                                }
                                Err(ref e) => {
                                    println!("Quitting due to read error: {}", e);
                                    break 'outer;
                                }
                            }    
                       // }
                        if received_std{
                            poll.reregister(&socket, Token(0), Ready::writable() | Ready::readable(), PollOpt::edge() | PollOpt::oneshot()).unwrap();
                        } else {
                            poll.reregister(&socket, Token(0), Ready::readable(), PollOpt::edge() | PollOpt::oneshot()).unwrap();
                        }

                    } else {
                        println!("Not readable and no data to send: {:?}", event);
                    }
                

 
                }

                    t => unreachable!("Unexpected token:  {:?}", t),
            }
            
        }
    }
}

fn spawn_stdin_channel() -> Receiver<(f32, f32, f32, f32, f32, f32)> {
    let (tx, rx) = mpsc::channel::<(f32, f32, f32, f32, f32, f32)>();
    thread::spawn(move || loop {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();

        let mut vec: Vec<&str> = buffer.split_whitespace().collect();
        let mut VV = Vec::new();

            for i in vec {
                VV.push(String::from(i).parse().unwrap());
            }

        let dat_truple = (VV[0], VV[1], VV[2], VV[3], VV[4], VV[5]);

        tx.send(dat_truple).unwrap();
    });
    rx
}

fn sleep(millis: u64) {
    let duration = time::Duration::from_millis(millis);
    thread::sleep(duration);
}
