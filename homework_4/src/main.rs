use std::alloc::{self, Layout};
use std::mem;
use std::ptr::{self, NonNull};


struct RawVec<T> {
    ptr: NonNull<T>,
    cap: usize,
}

impl<T> RawVec<T> {
    fn new() -> Self {
        let cap = if mem::size_of::<T>() == 0 { !0 } else { 0 };

        RawVec {
            ptr: NonNull::dangling(),
            cap: cap,
        }
    }


    fn with_capacity(size: usize) -> Self {
        let mut v: RawVec<T> = RawVec::new();
        for i in 0..size {
            v.grow();
        }
        v
    }

    fn grow(&mut self) {
        assert!(mem::size_of::<T>() != 0, "capacity overflow");

        let (new_cap, new_layout) = if self.cap == 0 {
            (1, Layout::array::<T>(1).unwrap())
        } else {
            let new_cap = 1 + self.cap;

            let new_layout = Layout::array::<T>(new_cap).unwrap();
            (new_cap, new_layout)
        };

        assert!(
            new_layout.size() <= usize::MAX,
            "Allocation too large"
        );

        let new_ptr = if self.cap == 0 {
            unsafe { alloc::alloc(new_layout) }
        } else {
            let old_layout = Layout::array::<T>(self.cap).unwrap();
            let old_ptr = self.ptr.as_ptr() as *mut u8;
            unsafe { alloc::realloc(old_ptr, old_layout, new_layout.size()) }
        };

        self.ptr = match NonNull::new(new_ptr as *mut T) {
            Some(p) => p,
            None => alloc::handle_alloc_error(new_layout),
        };
        self.cap = new_cap;
    }
}


pub struct MyVec<T> {
    buf: RawVec<T>,
    len: usize,
}

impl<T> MyVec<T> {
    fn ptr(&self) -> *mut T {
        self.buf.ptr.as_ptr()
    }

    fn cap(&self) -> usize {
        self.buf.cap
    }

    pub fn new() -> Self {
        MyVec {
            buf: RawVec::new(),
            len: 0,
        }
    }

    pub fn with_capacity(size: usize) -> Self {
        MyVec {
            buf: RawVec::with_capacity(size),
            len: 0,
        }
    }

    pub fn push(&mut self, elem: T) {
        if self.len == self.cap() {
            self.buf.grow();
        }

        unsafe {
            ptr::write(self.ptr().add(self.len), elem);
        }

        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            None
        } else {
            self.len -= 1;
            unsafe { Some(ptr::read(self.ptr().add(self.len))) }
        }
    }


    pub fn remove(&mut self, index: usize) -> T {
        assert!(index < self.len, "index out of bounds");
        unsafe {
            self.len -= 1;
            let result = ptr::read(self.ptr().add(index));
            ptr::copy(
                self.ptr().add(index + 1),
                self.ptr().add(index),
                self.len - index,
            );
            result
        }
    }

    pub fn get(&mut self, index: usize) -> Option<T> {
        unsafe {
            if index >= self.len {
                None
            } else {
                Some(ptr::read(self.ptr().add(index)))
            }
        }
    }

    pub fn resize(&mut self, new_len: usize, value: T) where T: Clone {
        if new_len > self.len {
            for i in self.len..new_len {
                self.push(value.clone());
            }
        } else {
            while new_len < self.len {
                self.pop();
            }
        }
    }
}


fn main() {
    tests::create_push_pop();
    println!("All tests finished OK");
}

mod tests {
    use super::*;

    pub fn create_push_pop() {
        let mut v = MyVec::new();
        v.push(1);
        assert_eq!(1, v.get(0).unwrap());
        v.push(2);
        let x = v.pop();
        assert_eq!(Some(2), x);
        v.push(10);
        let x = v.remove(0);
        assert_eq!(1, v.len);
        v.resize(5, 32);
        assert_eq!(Some(32), v.get(3));
        assert_eq!(5, v.len);
        assert_eq!(5, v.cap());
        let mut v1: MyVec<i32> = MyVec::with_capacity(5);
        assert_eq!(v.cap(), v1.cap());
        assert_eq!(0, v1.len);
        v1.push(4);
        assert_eq!(Some(4), v1.get(0))
    }
}
