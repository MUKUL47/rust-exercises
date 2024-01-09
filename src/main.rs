use std::{*, process::exit, io::Write, borrow::Borrow, ops::Deref};
mod account_usage;
mod stack;

fn main(){
    let mut ll = LinkedList::new(3);
    ll.add_at_start(22).add_at_start(2).add_at_start(1);
    println!("{:?}",ll);
    ll.delete_node(3);
    println!("{:?}",ll);
}
#[derive(Debug,Clone)]
struct LinkedList{
    head_ref: Option<Node>,
}
#[derive(Debug,Clone)]
struct Node{
    data: i32,
    next: Option<Box<Node>>,
}

impl LinkedList{
    fn new(data: i32) -> Self{
        return LinkedList{
            head_ref: Some(Node{ data, next: None })
        }
    }

    fn delete_node(&mut self, data: i32) -> & Self{
        match self.head_ref.as_mut(){
            Some (head_node) => {
                let mut prev_target_node = head_node;
                if(prev_target_node.data != data){
                    loop {
                        match &mut prev_target_node.next{
                            Some(current_node) => {
                                let mut is_next_node = false;
                                match prev_target_node.next.as_mut().unwrap().next.as_mut(){
                                    Some(next_node) => {
                                        is_next_node = next_node.data == data;
                                    },
                                    None =>{
                                        break;
                                    }
                                }
                                if is_next_node {
                                    prev_target_node.next = Some(*prev_target_node.next.as_mut().unwrap().next.as_mut().unwrap());  
                                    break;
                                }
                            },
                            None => {
                                break;
                            }
                        }
                    }
                }else{
                    self.head_ref = Some(*self.head_ref.as_mut().unwrap().next.as_mut().unwrap().clone());
                }
            },
            Node =>{}
        }
        return self;
    }

    fn add_at_start(&mut self, data:i32) -> &mut Self{
        let mut node  = Node{
            data,
            next: None,
        };
        node.next = Some(Box::new(self.head_ref.clone().unwrap()));
        self.head_ref = Some(node);
        return self;
    }

    fn append(&mut self, data: i32) -> &mut Self{
        if let Some(mut node_head) = self.head_ref.as_mut() {
            while let Some(_) = node_head.next.as_mut() {
                let kk = node_head.next.take();
                node_head= node_head.next.as_mut().unwrap();
            }
            node_head.next = Some(Box::new(Node{ data, next: None }));
        }else{
            self.head_ref = Some(Node { data, next: None });
        }
        return self;
    }
}