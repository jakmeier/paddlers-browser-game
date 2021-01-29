use std::ops::{Index, IndexMut};

/// Implementation for a compile-time list with variable length. (But fixed max lengths)
///
/// The implementation uses a constant-sized array underneath.
/// Surely it would be nice to not have a maximal size.
///
/// Something like this could  be implemented but is really hard to then use because each size of the list is represented as a different type. (E.g. what it the return type of a function that return 0 or 1 element in the list?)
/// ```
/// pub struct ConstList<ELEMENT, TAIL> {
///     head: ELEMENT,
///     next: TAIL,
/// }
/// ```
///
/// Any implementation without size limit will result in a type with a non-constant size, or with multiple types for multiple sizes.
/// This implementation right here is the only way I can think of right now that makes it possible to return a list whose *length depends on the arguments* (and not only the types) in a const fn.
///
/// Further, one could correctly point out that this means we always return an array of the maximal length.
/// But note that this has no run-time cost, since the compiler knows all the values and will (hopefully always) derive that the empty values are not used.
/// The only cost really is the compile time.
#[derive(Copy, Clone)]
pub struct ConstList<T> {
    data: [Option<T>; CONST_LIST_MAX_LEN],
}

pub const CONST_LIST_MAX_LEN: usize = 64;
impl<T> ConstList<T> {
    pub const MAX_LEN: usize = CONST_LIST_MAX_LEN;
}

impl<T> ConstList<T> {
    pub const fn new() -> ConstList<T> {
        Self {
            data: [None; CONST_LIST_MAX_LEN],
        }
    }
    pub const fn len(&self) -> usize {
        let mut i = 0;
        let mut n = 0;
        loop {
            if i >= CONST_LIST_MAX_LEN {
                return n;
            }
            if self.data[i].is_some() {
                n += 1;
            }
            i += 1;
        }
    }
}
impl<T: Copy> ConstList<T> {
    pub const fn singleton(data: T) -> ConstList<T> {
        let mut array = Self::new();
        array.data[0] = Some(data);
        array
    }

    #[must_use]
    #[rustfmt::skip]
    /// To avoid any mutable references, this returns a copy of the original list with an additional element at the end
    pub const fn push(&self, element: T) -> Self {
        let mut i = 0usize;
        let len = self.len();
        Self {
            data: [
                if  i < len { i+=1; self.data[i-1] }  else if  i == len { i+=1; Some(element) }  else  { None },
                if  i < len { i+=1; self.data[i-1] }  else if  i == len { i+=1; Some(element) }  else  { None },
                if  i < len { i+=1; self.data[i-1] }  else if  i == len { i+=1; Some(element) }  else  { None },
                if  i < len { i+=1; self.data[i-1] }  else if  i == len { i+=1; Some(element) }  else  { None },
                if  i < len { i+=1; self.data[i-1] }  else if  i == len { i+=1; Some(element) }  else  { None },
                if  i < len { i+=1; self.data[i-1] }  else if  i == len { i+=1; Some(element) }  else  { None },
                if  i < len { i+=1; self.data[i-1] }  else if  i == len { i+=1; Some(element) }  else  { None },
                if  i < len { i+=1; self.data[i-1] }  else if  i == len { i+=1; Some(element) }  else  { None },
                if  i < len { i+=1; self.data[i-1] }  else if  i == len { i+=1; Some(element) }  else  { None },
                if  i < len { i+=1; self.data[i-1] }  else if  i == len { i+=1; Some(element) }  else  { None },
                if  i < len { i+=1; self.data[i-1] }  else if  i == len { i+=1; Some(element) }  else  { None },
                if  i < len { i+=1; self.data[i-1] }  else if  i == len { i+=1; Some(element) }  else  { None },
                if  i < len { i+=1; self.data[i-1] }  else if  i == len { i+=1; Some(element) }  else  { None },
                if  i < len { i+=1; self.data[i-1] }  else if  i == len { i+=1; Some(element) }  else  { None },
                if  i < len { i+=1; self.data[i-1] }  else if  i == len { i+=1; Some(element) }  else  { None },
                if  i < len { i+=1; self.data[i-1] }  else if  i == len { i+=1; Some(element) }  else  { None },
                if  i < len { i+=1; self.data[i-1] }  else if  i == len { i+=1; Some(element) }  else  { None },
                if  i < len { i+=1; self.data[i-1] }  else if  i == len { i+=1; Some(element) }  else  { None },
                if  i < len { i+=1; self.data[i-1] }  else if  i == len { i+=1; Some(element) }  else  { None },
                if  i < len { i+=1; self.data[i-1] }  else if  i == len { i+=1; Some(element) }  else  { None },
                if  i < len { i+=1; self.data[i-1] }  else if  i == len { i+=1; Some(element) }  else  { None },
                if  i < len { i+=1; self.data[i-1] }  else if  i == len { i+=1; Some(element) }  else  { None },
                if  i < len { i+=1; self.data[i-1] }  else if  i == len { i+=1; Some(element) }  else  { None },
                if  i < len { i+=1; self.data[i-1] }  else if  i == len { i+=1; Some(element) }  else  { None },
                if  i < len { i+=1; self.data[i-1] }  else if  i == len { i+=1; Some(element) }  else  { None },
                if  i < len { i+=1; self.data[i-1] }  else if  i == len { i+=1; Some(element) }  else  { None },
                if  i < len { i+=1; self.data[i-1] }  else if  i == len { i+=1; Some(element) }  else  { None },
                if  i < len { i+=1; self.data[i-1] }  else if  i == len { i+=1; Some(element) }  else  { None },
                if  i < len { i+=1; self.data[i-1] }  else if  i == len { i+=1; Some(element) }  else  { None },
                if  i < len { i+=1; self.data[i-1] }  else if  i == len { i+=1; Some(element) }  else  { None },
                if  i < len { i+=1; self.data[i-1] }  else if  i == len { i+=1; Some(element) }  else  { None },
                if  i < len { i+=1; self.data[i-1] }  else if  i == len { i+=1; Some(element) }  else  { None },
                if  i < len { i+=1; self.data[i-1] }  else if  i == len { i+=1; Some(element) }  else  { None },
                if  i < len { i+=1; self.data[i-1] }  else if  i == len { i+=1; Some(element) }  else  { None },
                if  i < len { i+=1; self.data[i-1] }  else if  i == len { i+=1; Some(element) }  else  { None },
                if  i < len { i+=1; self.data[i-1] }  else if  i == len { i+=1; Some(element) }  else  { None },
                if  i < len { i+=1; self.data[i-1] }  else if  i == len { i+=1; Some(element) }  else  { None },
                if  i < len { i+=1; self.data[i-1] }  else if  i == len { i+=1; Some(element) }  else  { None },
                if  i < len { i+=1; self.data[i-1] }  else if  i == len { i+=1; Some(element) }  else  { None },
                if  i < len { i+=1; self.data[i-1] }  else if  i == len { i+=1; Some(element) }  else  { None },
                if  i < len { i+=1; self.data[i-1] }  else if  i == len { i+=1; Some(element) }  else  { None },
                if  i < len { i+=1; self.data[i-1] }  else if  i == len { i+=1; Some(element) }  else  { None },
                if  i < len { i+=1; self.data[i-1] }  else if  i == len { i+=1; Some(element) }  else  { None },
                if  i < len { i+=1; self.data[i-1] }  else if  i == len { i+=1; Some(element) }  else  { None },
                if  i < len { i+=1; self.data[i-1] }  else if  i == len { i+=1; Some(element) }  else  { None },
                if  i < len { i+=1; self.data[i-1] }  else if  i == len { i+=1; Some(element) }  else  { None },
                if  i < len { i+=1; self.data[i-1] }  else if  i == len { i+=1; Some(element) }  else  { None },
                if  i < len { i+=1; self.data[i-1] }  else if  i == len { i+=1; Some(element) }  else  { None },
                if  i < len { i+=1; self.data[i-1] }  else if  i == len { i+=1; Some(element) }  else  { None },
                if  i < len { i+=1; self.data[i-1] }  else if  i == len { i+=1; Some(element) }  else  { None },
                if  i < len { i+=1; self.data[i-1] }  else if  i == len { i+=1; Some(element) }  else  { None },
                if  i < len { i+=1; self.data[i-1] }  else if  i == len { i+=1; Some(element) }  else  { None },
                if  i < len { i+=1; self.data[i-1] }  else if  i == len { i+=1; Some(element) }  else  { None },
                if  i < len { i+=1; self.data[i-1] }  else if  i == len { i+=1; Some(element) }  else  { None },
                if  i < len { i+=1; self.data[i-1] }  else if  i == len { i+=1; Some(element) }  else  { None },
                if  i < len { i+=1; self.data[i-1] }  else if  i == len { i+=1; Some(element) }  else  { None },
                if  i < len { i+=1; self.data[i-1] }  else if  i == len { i+=1; Some(element) }  else  { None },
                if  i < len { i+=1; self.data[i-1] }  else if  i == len { i+=1; Some(element) }  else  { None },
                if  i < len { i+=1; self.data[i-1] }  else if  i == len { i+=1; Some(element) }  else  { None },
                if  i < len { i+=1; self.data[i-1] }  else if  i == len { i+=1; Some(element) }  else  { None },
                if  i < len { i+=1; self.data[i-1] }  else if  i == len { i+=1; Some(element) }  else  { None },
                if  i < len { i+=1; self.data[i-1] }  else if  i == len { i+=1; Some(element) }  else  { None },
                if  i < len { i+=1; self.data[i-1] }  else if  i == len { i+=1; Some(element) }  else  { None },
                if  i < len { i+=1; self.data[i-1] }  else if  i == len { i+=1; Some(element) }  else  { None },

            ],
        }
    }
}
impl<T> Index<usize> for ConstList<T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        self.data[index].as_ref().unwrap()
    }
}
impl<T> IndexMut<usize> for ConstList<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.data[index].as_mut().unwrap()
    }
}

#[test]
fn push() {
    let mut list = ConstList::new();
    assert_eq!(list.len(), 0);

    list = list.push(77);
    assert_eq!(list.len(), 1);
    assert_eq!(list[0], 77);
    assert_eq!(list.len(), 1);

    list = list.push(78);
    assert_eq!(list.len(), 2);
    assert_eq!(list[0], 77);
    assert_eq!(list[1], 78);
    assert_eq!(list.len(), 2);
}
#[test]
fn push_to_full_length() {
    let mut list = ConstList::new();
    assert_eq!(list.len(), 0);
    for i in 0..ConstList::<usize>::MAX_LEN {
        list = list.push(i);
    }
    assert_eq!(list.len(), ConstList::<usize>::MAX_LEN);
    for i in 0..ConstList::<usize>::MAX_LEN {
        assert_eq!(list[i], i);
    }
    assert_eq!(list.len(), ConstList::<usize>::MAX_LEN);
}
