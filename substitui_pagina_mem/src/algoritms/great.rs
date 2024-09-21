use super::*;

pub fn great(list: Vec<Process>){   
    let mut mem: Vec<Process> = Vec::with_capacity(LENGHT);

    for i in 0..mem.capacity(){
        print!("| {i:.3}");
    }
    print!("\n");

    let mut pos: usize = 0;
    let mut exists: bool = false;
    let mut is_used: bool = false;
    for i in 0..list.len(){
        if mem.len() < 3{
            mem.push(list[i].clone());
        }else{
            for j in 0..mem.len(){
                for k in i..list.len(){
                    mem[j].set_time(k);

                    if mem[j].get_id() == list[i].get_id(){
                        exists = true;
                        break;
                    }else if mem[j].get_id() == list[k].get_id(){
                        break;
                    }
                }
            }
            for j in 0..mem.len(){
                if mem[j].get_time() > mem[pos].get_time(){
                    pos = j;
                }
            }
            if !exists{
                mem[pos] = list[i].clone();
                pos = 0;
            }

            exists = false;
        }
        
        for proc in &mem{
            print!("{:3}^{}", proc.get_id(), proc.get_time());
        }
        print!("\n");
    }
    println!("{pos}");
}