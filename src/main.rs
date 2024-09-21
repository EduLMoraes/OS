mod structs;
use structs::*;
mod algoritms;
use algoritms::*;

const LENGHT: usize = 3;
const ALGORITM: &str = "great";

fn main() {
    
    let sequence: Vec<usize> = vec![7,0,1,2,3,0,7,5,4,6,2,3,2,4,5,7,4,3,2,1];
    println!("Usando algoritmo: {}\nSequência de entrada: {:?}\n", ALGORITM, sequence);
    let mut list: Vec<Process> = Vec::new();

    for id in sequence {
        list.push(Process::new(id));
    }

    match ALGORITM{
        "great" => great(list),
        "fifo" => fifo(list),
        "sc" => second_chance(list),
        "lru" => lru(list),
        _ => todo!()
    }
}
