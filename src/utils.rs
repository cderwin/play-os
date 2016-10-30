use core::ptr;

#[derive(Debug)]
pub struct Volatile<T: Copy>(T);

impl<T: Copy> Volatile<T> {
    pub fn read(&self) -> T {
        unsafe { ptr::read_volatile(&self.0) }
    }

    pub fn write(&mut self, value: T) {
        unsafe { ptr::write(&mut self.0, value) }
    }

    pub fn update<F>(&mut self, f: F)
        where F: FnOnce(&mut T)
    {
        let mut value = self.read();
        f(&mut value);
        self.write(value);
    }
}
