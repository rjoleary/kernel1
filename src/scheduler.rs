use task::TaskDescriptor;

pub struct Scheduler<'a> {
    next: &'a TaskDescriptor
}

impl<'a> Scheduler<'a> {
    pub fn register(& mut self, td: &'a TaskDescriptor) {
        self.next = td;
    }

    pub fn next(&'a self) -> &'a TaskDescriptor {
        return self.next;
    }

    pub fn new(init: &TaskDescriptor) -> Scheduler {
        return Scheduler{next: init};
    }
}
