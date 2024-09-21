#[derive(PartialEq, PartialOrd, Ord, Eq, Clone)]
pub struct Process{
    id: usize,
    used: bool,
    lock: bool,
    trash: bool,
    time: usize
}

impl Process{
    pub fn new(id: usize) -> Process{
        Process{
            id: id,
            used: true,
            lock: false,
            trash: false,
            time: std::usize::MIN
        }
    }

    pub fn get_id(&self) -> usize{
        self.id
    }
    pub fn get_used(&self) -> bool{
        self.used
    }
    pub fn get_lock(&self) -> bool{
        self.lock
    }
    pub fn get_trash(&self) -> bool{
        self.trash
    }
    pub fn get_time(&self) -> usize{
        self.time
    }
    pub fn set_used(&mut self, value: bool){
        self.used = value;
    }
    pub fn set_time(&mut self, value: usize){
        self.time = value;
    }


    pub fn switch_used(&mut self){
        self.used = !self.used;
    }
    
    pub fn switch_lock(&mut self){
        self.lock = !self.lock;
    }
    
    pub fn switch_trash(&mut self){
        self.trash = !self.trash;
    }
}