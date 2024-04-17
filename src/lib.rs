use std::mem;
use std::ptr;
use std::sync::atomic::{AtomicPtr, Ordering};

use std::collections::VecDeque;
use std::sync::Mutex;

struct Node<T> {
    data: T,
    next: AtomicPtr<Node<T>>,
}

pub struct ConcurrentQueue<T> {
    head: AtomicPtr<Node<T>>,
    tail: AtomicPtr<Node<T>>,
}

impl<T> ConcurrentQueue<T> {
    pub fn new() -> Self {
        let dummy = Box::into_raw(Box::new(Node {
            data: unsafe { mem::zeroed() },
            next: AtomicPtr::new(ptr::null_mut()),
        }));
        ConcurrentQueue {
            head: AtomicPtr::new(dummy),
            tail: AtomicPtr::new(dummy),
        }
    }

    pub fn enqueue(&self, data: T) {
        let new_node = Box::into_raw(Box::new(Node {
            data,
            next: AtomicPtr::new(ptr::null_mut()),
        }));
        loop {
            let tail = self.tail.load(Ordering::Acquire);
            let next = unsafe { (*tail).next.load(Ordering::Acquire) };
            if next.is_null() {
                if unsafe {
                    (*tail)
                        .next
                        .compare_exchange(next, new_node, Ordering::Release, Ordering::Relaxed)
                        .is_ok()
                } {
                    self.tail
                        .compare_exchange(tail, new_node, Ordering::Release, Ordering::Relaxed)
                        .unwrap();
                    return;
                }
            } else {
                self.tail
                    .compare_exchange(tail, next, Ordering::Release, Ordering::Relaxed)
                    .unwrap();
            }
        }
    }

    pub fn dequeue(&self) -> Option<T> {
        loop {
            let head = self.head.load(Ordering::Acquire);
            let tail = self.tail.load(Ordering::Acquire);
            let next = unsafe { (*head).next.load(Ordering::Acquire) };
            if head == tail {
                if next.is_null() {
                    return None;
                }
                self.tail
                    .compare_exchange(tail, next, Ordering::Release, Ordering::Relaxed)
                    .unwrap();
            } else {
                if self
                    .head
                    .compare_exchange(head, next, Ordering::Release, Ordering::Relaxed)
                    .is_ok()
                {
                    let head = unsafe { Box::from_raw(head) };
                    return Some(unsafe { ptr::read(&head.data) });
                }
            }
        }
    }
}

impl<T> Drop for ConcurrentQueue<T> {
    fn drop(&mut self) {
        while self.dequeue().is_some() {}
        let head = self.head.load(Ordering::Acquire);
        unsafe {
            let _ = Box::from_raw(head);
        }
    }
}

// =============================== Lock-based Quueue Impls ================================== //
pub struct LockQueue<T> {
    queue: Mutex<VecDeque<T>>,
}

impl<T> LockQueue<T> {
    pub fn new() -> Self {
        LockQueue {
            queue: Mutex::new(VecDeque::new()),
        }
    }

    pub fn enqueue(&self, data: T) {
        let mut queue = self.queue.lock().unwrap();
        queue.push_back(data);
    }

    pub fn dequeue(&self) -> Option<T> {
        let mut queue = self.queue.lock().unwrap();
        queue.pop_front()
    }
}
