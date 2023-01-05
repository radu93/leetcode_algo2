// pub struct Post {
//     state: Option<Box<dyn State>>,
//     content: String,
// }

// impl Post {
//     pub fn new() -> Post {
//         Post {
//             state: Some(Box::new(Draft {})),
//             content: String::new(),
//         }
//     }

//     pub fn add_text(&mut self, text: &str) {
//         self.content.push_str(text);
//     }

//     pub fn content(&self) -> &str {
//         ""
//     }

//     pub fn request_review(&mut self) {
//         let mut x = Some(Box::new(2));
//         let mut xp = &mut x;

//         let a = xp.unwrap();
//         if let Some(y) = *xp {

//         }

//         let a = self.state;

//         if let Some(s) = self.state.take() {
//             self.state = Some(s.request_review())
//         }
//     }
// }

// struct Draft {}

// trait State {
//     fn request_review(self: Box<Self>) -> Box<dyn State>;
// }

// impl State for Draft {
//     fn request_review(self: Box<Self>) -> Box<dyn State> {
//         self
//     }
// }

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

struct Solution;

impl Solution {
    pub fn delete_duplicates_my(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut out_head = head;
        let mut node = &mut out_head;

        loop {
            let mut advance = true;
            if let Some(n) = node {
                if let Some(r) = &mut n.next {
                    if n.val == r.val {
                        n.next = r.next.take();
                        advance = false;
                    }
                } else {
                    break;
                }
            }
            if advance {
                if let Some(n) = node {
                    node = &mut n.next;
                }
            }
        }
        out_head
    }

    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut out_head = head;
        let mut hook = &mut out_head;
        let mut removing = false;

        loop {
            let mut advance = true;

            if let Some(ptr) = hook {

                let hook_next = &mut ptr.next;
                if let Some(ptr_next) = hook_next {

                    if ptr.val == ptr_next.val {
                        *hook = ptr.next.take();
                        advance = false;
                        removing = true;
                    } else {
                        if removing {
                            *hook = ptr.next.take();
                            advance = false;
                        }
                        removing = false;
                    }
                } else {
                    if removing {
                        *hook = ptr.next.take();
                        advance = false;
                    }
                    removing = false;
                }
            } else {
                break;
            }

            if advance {
                if let Some(ptr) = hook {
                    hook = &mut ptr.next;
                }
            }
        }

        out_head
    }
}

fn arr_to_list(arr: &[i32]) -> Option<Box<ListNode>> {
    let mut head = None;
    for n in arr {
        let mut node = ListNode::new(*n);
        node.next = head;
        head = Some(Box::new(node));
    }
    head
}

fn main() {
    let head = arr_to_list(&[1, 1, 1]);
    let result = Solution::delete_duplicates(head);

    let mut iter = &result;

    while iter.is_some() {
        let node = iter.as_ref().unwrap();
        println!("{}", node.val);
        iter = &node.next;
    }
}
