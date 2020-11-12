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


            let strArr = ["X: ", "Y: ", "X: ", "X Avg: ", "Y Avg: ", "Pitch: "];
            let valArr = [-q.1, -q.2, q.3, q.4, q.5, q.6];
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

    for mut frame in BufReader::new(stdin.lock()).split(0) {
        let mut frame = frame.unwrap();
        if let Ok(n) = cobs::decode_in_place(&mut frame) {
            if n == 40 {
                let mut start = 0;
                let w = LE::read_f32(&mut frame[start..start + 4]);
                start += 4;
                let x = LE::read_f32(&mut frame[start..start + 4]);
                start += 4;
                let y = LE::read_f32(&mut frame[start..start + 4]);
                start += 4;
                let z = LE::read_f32(&mut frame[start..start + 4]);

                start += 4;
                let m_x = LE::read_f32(&mut frame[start..start + 4]);
                start += 4;
                let m_y = LE::read_f32(&mut frame[start..start + 4]);
                start += 4;
                let m_z = LE::read_f32(&mut frame[start..start + 4]);
                start += 4;

                let avg_x = LE::read_f32(&mut frame[start..start + 4]);
                start += 4;
                let avg_y = LE::read_f32(&mut frame[start..start + 4]);
                start += 4;
                let pitch = LE::read_f32(&mut frame[start..start + 4]);
                start += 4;


                let _ = wtr.serialize(&[m_x, m_y, m_z]);
                let _ = wtr.flush();

                assert_eq!(start, n);
                sender.send((w, x, y, z, avg_x, avg_y, pitch)).unwrap();
            }
        }
    }
}
