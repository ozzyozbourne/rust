#[derive(Debug)]
pub struct LinkedList<T> {
    data: T,
    next: Option<Box<LinkedList<T>>>,
}

impl<T: std::ops::AddAssign> LinkedList<T> {
    pub fn add_up(&mut self, n: T) {
        self.data += n;
    }
}

fn main() {
    let mut ll = LinkedList {
        data: 3,
        next: Some(Box::new(LinkedList {
            data: 2,
            next: None,
        })),
    };
    println!("{:?}", ll);

    if let Some(ref mut v) = ll.next {
        v.add_up(10);
    }

    let mut q = "   Hello   ".to_string();

    let _pq = q.trim();
    q.push_str("shflsdjfk");

    println!("{:?}", ll);
}
