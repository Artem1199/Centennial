extern crate byteorder;
extern crate cobs;
extern crate kiss3d;
extern crate nalgebra as na;

#[macro_use]
extern crate itertools;

use std::io::{self, BufRead, BufReader};
use std::sync::mpsc::{self, Sender, TryRecvError};
use std::thread;

use byteorder::{ByteOrder, LE};
use kiss3d::text::Font;
use kiss3d::light::Light;
use kiss3d::window::Window;
use na::{Point2, Point3, UnitQuaternion};
use std::iter::Zip;

fn main() {
    let (mut sender, receiver) = mpsc::channel();
    thread::spawn(move || {
        parse(&mut sender);
    });

    let mut window = Window::new("Quaternion visualizer");
    let mut c = window.add_cube(0.3, 0.01, 0.2);
    let font = Font::default();

    c.set_color(0.0, 1.0, 0.0);

    window.set_light(Light::StickToCamera);

    let mut last = None;
    while window.render() {
        // grab the latest parsed quaternion
        loop {
            match receiver.try_recv() {
                Ok(q) => last = Some(q),
                Err(TryRecvError::Empty) => break,
                Err(TryRecvError::Disconnected) => return,
            }
        }

        if let Some(q) = last {
            // NOTE In kiss3d the coordinate axes look like this
            //
            //      ^ Y
            //      |
            // X    |
            // <----X Z
            //
            // whereas the gyroscope axes on the F3 look like this
            //
            //   ^ Z
            //   |
            //   |
            // X o----> Y
            //
            // when the USB connectors are facing in that +Y way
            c.set_local_rotation(UnitQuaternion::from_quaternion(na::Quaternion::new(
                q.0,
                -q.2, // -y
                q.3,  // +z
                -q.1, // -x
            )));


            let strArr = ["Acc X: ", "Acc Y: ", "Acc Z: ", "Gyr X: ", "Gyr Y: ", "Gyr Z: "];
            let valArr = [q.1, q.2, q.3, q.4, q.5, q.6];
            let mut index = 0.;


            for (&e1, &e2) in strArr.iter().zip(valArr.iter()){
                let ss = format!("{}{}", e1, e2);

                window.draw_text(
                &ss,
                &Point2::new(60.0, (120.0 + index * 75.)),
                60.0,
                &font,
                &Point3::new(1.0, 1.0, 1.0),
                );

                index += 1.;
            }
                        
        }
    }
}

// parses quaternions from stdin
fn parse(sender: &mut Sender<(f32, f32, f32, f32, f32, f32, f32)>) {
    let stdin = io::stdin();
   // let mut wtr = csv::Writer::from_writer(io::stdout());

    let mut wtr = csv::WriterBuilder::new()
            .delimiter(b'\t')
            .quote_style(csv::QuoteStyle::NonNumeric)
            .from_writer(io::stdout());

            let mut x_avg = 0.0;
            let mut y_avg = 0.0;
            let mut z_avg = 0.0;
            let mut gx_avg = 0.0;
            let mut gy_avg = 0.0;
            let mut gz_avg = 0.0;
            let mut avg_cnt = 0;   

    for mut frame in BufReader::new(stdin.lock()).split(0) {
        let mut frame = frame.unwrap();
        if let Ok(n) = cobs::decode_in_place(&mut frame) {

            println!("{:?}", frame);
            if n == 16 {
                let mut start = 0;
                println!("2 {:?}", frame);
                let q1 = LE::read_f32(&mut frame[start..start + 4]);
                start += 4;  //4
                let q2 = LE::read_f32(&mut frame[start..start + 4]);
                start += 4;  //8
                let q3 = LE::read_f32(&mut frame[start..start + 4]);
                start += 4;  //12
                let q4 = LE::read_f32(&mut frame[start..start + 4]);
                let _ = wtr.serialize(&[q1, q2, q3, q4]);
                let _ = wtr.flush();

            }

            // if n == 40 {
            //     let mut start = 0;
            //     let q1 = LE::read_f32(&mut frame[start..start + 4]);
            //     start += 4;
            //     let q2 = LE::read_f32(&mut frame[start..start + 4]);
            //     start += 4;
            //     let q3 = LE::read_f32(&mut frame[start..start + 4]);
            //     start += 4;
            //     let q4 = LE::read_f32(&mut frame[start..start + 4]);

            //     start += 4;
            //     let q5 = LE::read_f32(&mut frame[start..start + 4]);
            //     start += 4;
            //     let q6 = LE::read_f32(&mut frame[start..start + 4]);
            //     start += 4;
            //     let q7 = LE::read_f32(&mut frame[start..start + 4]);
            //     start += 4;

            //     let q8 = LE::read_f32(&mut frame[start..start + 4]);
            //     start += 4;
            //     let q9 = LE::read_f32(&mut frame[start..start + 4]);
            //     start += 4;
            //     let qa = LE::read_f32(&mut frame[start..start + 4]);
            //     start += 4;


            //     let _ = wtr.serialize(&[q1, q2, q3, q4, q5, q6, q7, q8, q9, qa]);
            //     let _ = wtr.flush();


           // }
        }
    }
}
