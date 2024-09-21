use super::*;

pub fn fifo(list: Vec<Process>){   
    let mut mem: Vec<Process> = Vec::with_capacity(LENGHT);

    for i in 0..mem.capacity(){
        print!("| {i:.3}");
    }
    print!("\n");

    let mut pos: usize = 0;
    let mut exists: bool = false;
    for i in list{
        if mem.len() < 3{
            mem.push(i);
        }else{
            for mem_proc in &mut mem{
                if mem_proc.get_id() == i.get_id(){
                    exists = true;
                    break;
                }
    
            }
            
            if !exists{
                mem[pos] = i;
                pos = (pos + 1) % mem.len();
            }
            exists = false;
        }
        
        for proc in &mem{
            print!("{:3}", proc.get_id());
        }
        print!("\n");
    }
}