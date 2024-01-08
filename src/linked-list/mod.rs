#[derive(Debug,Clone)]
struct Node{
    data: i32,
    next: Option<Box<Node>>,
}
/**
 * only than 1 mutable ref
 * more than 1 immutable ref 
 */
impl Node{
    fn new(data: i32) -> Self{
        Node { data, next: None }
    }
    fn append_at_last(&mut self, data: i32) -> &mut Self{
        let node = self.get_last_node();
        node.next = Some(Box::new(Node::new(data)));
        return self;
    }
    fn append_at_start(&mut self, data: i32) -> &Self{
        self.next = Some(Box::new(Node::new(data)));
        return self;
    }
    // fn delete_first_node(&mut self) -> &Self{
    //         match &mut self.next {
    //         Some(ref mut first_head) => {
    //             let mut continue_new_head = Node::new(self.data);
    //             self =  continue_new_head;
    //             let mut header = first_head;
    //             loop {
    //                 match header.next{
    //                     Some(ref mut h) => {
    //                         continue_new_head.next = Some(Box::new(Node::new(h.data)));
    //                         continue_new_head = *continue_new_head.next.unwrap();
    //                         header = h;
    //                     },
    //                     None => {
    //                         return self;
    //                     }
    //                 }
    //             }
    //         },
    //         None => {
    //             print!("Only node remaining")
    //         }
    //     }
    //     return self;
    // }
    fn search(&mut self, data: i32) -> Option<&Node>{
        let mut head = self;
        loop {
            match head.next{
                Some(ref mut h) => {
                    if h.data == data {
                        return Some(h);
                    }
                    head = h;
                },
                None => {
                    return None;
                }
            }
        }
    }
    fn get_last_node(&mut self)-> &mut Node  {
        let mut head = self;
        loop {
            match head.next{
                Some(ref mut h) => {
                    head = h;
                },
                None => {
                    return head;
                }
            }
        }
    }

}

// impl Deref for Node {
//     type Target = Node;

//     fn deref(&self) -> &Self::Target {
//         self
//     }
// }
