#[derive(Clone, Debug)]
struct List<T: Clone> {
    head: Option<Box<Node<T>>>,
}

#[derive(Clone, Debug)]
struct Node<T: Clone> {
    elem: T,
    next: Option<Box<Node<T>>>,
}

impl<T: Clone> List<T> {
    fn new() -> Self {
        List { head: None }
    }

    fn push(&self, elem: T) -> Self {
        let old_list_clone: List<T> = (*self).clone();
        let new_next2 = old_list_clone.head;
        let new_head_node = Node {
            elem,
            next: new_next2,
        };
        Self {
            head: Some(Box::new(new_head_node)),
        }
    }

    fn head_tail(&self) -> (Option<T>, Self) {
        let head = self.head.as_ref().map(|x| x.elem.clone());
        let tail_head = match self.head.as_ref().map(|x| x.next.clone()) {
            Some(v) => v,
            None => None,
        };
        let tail = List { head: tail_head };
        (head, tail)
    }

    fn to_vec(&self) -> Vec<T> {
        fn to_vec_rec<T: Clone>(list: &List<T>, acc: Vec<T>) -> Vec<T> {
            let (head, tail) = list.head_tail();
            if head.is_none() {
                return acc;
            }
            let new_acc = functional_push_right(acc, head.unwrap());
            to_vec_rec(&tail, new_acc)
        }
        to_vec_rec(self, vec![])
    }

    fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    fn len(&self) -> usize {
        fn len_rec<T: Clone>(list: Option<&Box<Node<T>>>, acc: usize) -> usize {
            match list {
                Some(head) => {
                    let tail = head.next.as_ref();
                    len_rec(tail, acc + 1)
                    // 0
                }
                None => acc,
            }
        }

        len_rec(self.head.as_ref(), 0)
    }

    // fn from_vec(vec: &[T]) -> Self {
    //     fn from_vec_rec<T: Clone>(vec: &mut std::slice::Iter<T>, acc: List<T>) -> List<T> {
    //         // let (head, tail) = head_tail(vec);
    //         let (head, tail) = head_tail_consuming(vec);
    //         if head.is_none() {
    //             return acc;
    //         }
    //         let new_acc = acc.push(head.unwrap().clone());
    //         from_vec_rec(&mut tail.into_iter(), new_acc)
    //     }
    //     from_vec_rec(&mut vec.clone().into_iter(), List::new())
    // }

    fn from_iter(iter: std::slice::Iter<T>) -> Self {
        fn from_vec_rec<T: Clone>(vec: &mut std::slice::Iter<T>, acc: List<T>) -> List<T> {
            // let (head, tail) = head_tail(vec);
            let (head, tail) = head_tail_consuming(vec);
            if head.is_none() {
                return acc;
            }
            let new_acc = acc.push(head.unwrap().clone());
            from_vec_rec(&mut tail.into_iter(), new_acc)
        }
        from_vec_rec(&mut iter.clone().into_iter(), List::new())
    }
}

fn head_tail<T>(vec: std::slice::Iter<T>) -> (Option<&T>, std::slice::Iter<T>) {
    let mut iter = vec.clone();
    let head = iter.next();
    let tail = iter;
    (head, tail)
}

fn head_tail_consuming<'a, T>(iter: &'a mut std::slice::Iter<T>) -> (Option<&'a T>, &'a [T]) {
    let head = iter.next();
    // let tail = iter;
    (head, iter.as_slice())
}

// fn head_tail<T>(vec: &[T]) -> (Option<&T>, std::slice::Iter<T>) {
//     let mut iter = vec.clone().into_iter();
//     let head = iter.next();
//     let tail = iter;
//     (head, tail)
// }

// impl<T: Clone + Copy> FromIterator<T> for List<T> {
//     // type Item = T;
//     // type IntoIter: std::vec::IntoIter<Self::Item>;
//     // type IntoIter = IntoIterator<Item = T>;
//     // type IntoIter = std::vec::IntoIter<T>;

//     // type FromIterator =
//     fn from_iter<I: IntoIterator<Item = T>>(iter: T) -> List<T> {
//         fn from_iter_rec<T: Clone, I>(iter: I, acc: List<T>) -> List<T> {
//             List::new()
//         }
//         from_iter_rec(iter, List::new())
//     }
// }

impl<T: Clone + Copy> Iterator for List<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let to_return = self.head.as_ref().map(|x| x.elem);
        let new_head = match self.head.take().map(|x| x.next) {
            Some(v) => v,
            None => None,
        };
        self.head = new_head;
        to_return
    }
}

fn functional_push_left<T>(acc: Vec<T>, elem: T) -> Vec<T> {
    [elem].into_iter().chain(acc.into_iter()).collect()
}

fn functional_push_right<T>(acc: Vec<T>, elem: T) -> Vec<T> {
    acc.into_iter().chain([elem].into_iter()).collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_is_empty() {
        let list: List<i32> = List::new();
        let res = list.is_empty();
        let expected = true;
        assert_eq!(res, expected);
    }

    #[test]
    fn test_is_empty_returns_false() {
        let list: List<i32> = List::new().push(123);
        let res = list.is_empty();
        let expected = false;
        assert_eq!(res, expected);
    }

    #[test]
    fn test_len_empty() {
        let list: List<i32> = List::new();
        let res = list.len();
        let expected = 0;
        assert_eq!(res, expected);
    }

    #[test]
    fn test_len_3() {
        let list: List<i32> = List::new().push(1).push(2).push(3);
        let res = list.len();
        let expected = 3;
        assert_eq!(res, expected);
    }

    #[test]
    fn test_into_iter() {
        let list: List<i32> = List::new().push(1).push(2).push(3);
        let res = list.into_iter().collect::<Vec<i32>>();
        let expected = [3, 2, 1];
        assert_eq!(res, expected);
    }

    #[test]
    fn test_from_vec() {
        let input = [3, 2, 1];
        let list: List<i32> = List::from_iter(input.iter());
        let res: Vec<i32> = list.into_iter().collect();
        let expected = vec![1, 2, 3];
        assert_eq!(res, expected);
    }
}

fn main() {
    let list = List::new();
    let list2 = list.push(123);
    println!("list: {:?}", list);
    println!("list2: {:?}", list2);

    let list3 = list2.push(33).push(44);
    println!("list3: {:?}", list3);

    let (expect44, tail) = list3.head_tail();
    assert_eq!(expect44, Some(44));
    println!("expect44: {:?}", expect44);
    println!("tail: {:?}", tail);
    let list3_as_vector = list3.to_vec();
    println!("list3_as_vector: {:?}", list3_as_vector);
    let list3_as_iterator = list3.clone().into_iter();
    let list3_as_vector_from_iterator = list3_as_iterator.collect::<Vec<i32>>();
    println!(
        "list3_as_vector_from_iterator: {:?}",
        list3_as_vector_from_iterator
    );
    let list3_as_vector_from_iterator2 = list3.clone().into_iter().collect::<Vec<i32>>();

    println!(
        "list3_as_vector_from_iterator2: {:?}",
        list3_as_vector_from_iterator2
    );
    println!("list3 len: {:?}", list3.len());
    // assert_eq!(list2.push(33), tail);

    let string_list = List::new().push("hi").push("hello");
    println!("string_list: {:?}", string_list.to_vec());
}
