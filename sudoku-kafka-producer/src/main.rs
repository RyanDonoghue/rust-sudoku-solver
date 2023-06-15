mod sudoku;
use sudoku::{SudokuBoard, SudokuRow};
use quick_protobuf::{Writer};
use kafka::producer::{Producer, Record, RequiredAcks, AsBytes};
use std::time::Duration;

fn main() {

    let mut producer =
    Producer::from_hosts(vec!("localhost:9092".to_owned()))
        .with_ack_timeout(Duration::from_secs(1))
        .with_required_acks(RequiredAcks::One)
        .create()
        .unwrap();


    let smessage = SudokuBoard {
        rows: vec!(
            SudokuRow{row: vec!(3,0,5,4,0,2,0,6,0)},
            SudokuRow{row: vec!(4,9,0,7,6,0,1,0,8)},
            SudokuRow{row: vec!(6,0,0,1,0,3,2,4,5)},
            SudokuRow{row: vec!(0,0,3,9,0,0,5,8,0)},
            SudokuRow{row: vec!(9,6,0,0,5,8,7,0,3)},
            SudokuRow{row: vec!(0,8,1,3,0,4,0,9,2)},
            SudokuRow{row: vec!(0,5,0,6,0,1,4,0,0)},
            SudokuRow{row: vec!(2,0,0,5,4,9,0,0,0)},
            SudokuRow{row: vec!(1,4,9,0,0,7,3,0,6)}
        )

    };

    let mut out = Vec::new();
    {
        let mut writer = Writer::new(&mut out);
        writer
            .write_message(&smessage)
            .expect("Cannot write message!");
    }

    println!("Message written successfully!");
    producer.send(&Record::from_value("sudoku-boards", out.as_bytes())).unwrap();

}