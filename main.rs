use std::{collections::HashMap};

fn main() {
    let notes = [ "DO", "DO#/REb", "RE", "RE#/MIb", "MI", "FA", "FA#/SOLb", "SOL", "SOL#/LAb", "LA", "LA#/SIb", "SI"];
    // Storing the semitones distance between each note
    let mut scales: HashMap<&str, Vec<i32>> = HashMap::new();
    scales.insert("MAJOR", vec![2, 2, 1, 2, 2, 2, 1]);

    print_scale(&notes, scales.get("MAJOR").unwrap(), "MAJOR");
}

fn print_scale(notes: &[&str], scale: &[i32], scale_name: &str) {

    println!("{} scale:", scale_name);
    println!("{}", notes[0]);

    let mut current_note = 0;

    for interval in scale {
        current_note += *interval;
        println!("{}", notes[current_note as usize]);
    }
}