use super::*;

pub fn second_chance(list: Vec<Process>){   
    let mut mem: Vec<Process> = Vec::with_capacity(LENGHT);

    for i in 0..mem.capacity(){
        print!("| {i:.3}");
    }
    print!("\n");

    let mut pos: usize = 0;
    let mut exists: bool = false;
    for i in list{
        if mem.len() < 3{
            mem.push(i)
        }else{
            let mut cont = 0;
            for mem_proc in &mut mem{
                if mem_proc.get_id() == i.get_id(){
                    mem_proc.set_used(true);
                    exists = true;
                    pos = cont;
                    break;
                }
                cont += 1;
            }
            if !exists{
                loop{
                    if mem[pos].get_used(){
                        mem[pos].switch_used();
                    }else{
                        mem[pos] = i;
                        break;
                    }
                    pos = (pos + 1) % mem.len();
                }
            }
        }
        exists = false;
        for proc in &mem{
            print!("{:3}", proc.get_id());
        }
        print!("\n");
    }
}