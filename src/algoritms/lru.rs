use super::*;

pub fn lru(list: Vec<Process>){   
    let mut mem: Vec<Process> = Vec::with_capacity(LENGHT);

    for i in 0..mem.capacity(){
        print!("| {i:.3}");
    }
    print!("\n");

    let mut pos: usize = 0;
    let mut exists: bool = false;
    let mut rep: usize = 0;

    for i in list{
        if mem.len() < 3{
            mem.push(i);
        }else{

            if rep > 1 {
                pos = (pos  + rep) % mem.len();
                rep = 1;
            }
    
            for mem_proc in &mut mem{
                if mem_proc.get_id() == i.get_id(){
                    exists = true;
                    rep += 1;
                    break;
                }
    
            }
            
            if !exists{
                mem[pos] = i;
                pos = (pos + 1 + rep) % mem.len();
            }
            exists = false;
        }
        
        for proc in &mem{
            print!("{:3}", proc.get_id());
        }
        print!("\n");
    }
}