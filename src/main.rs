extern crate rand;

use rand::Rng;

#[derive(Debug)]
struct TreapNode<'a> {
  priority: u32,
  key: i32,
  size: u32,
  l: Option<&'a mut TreapNode<'a>>,
  r: Option<&'a mut TreapNode<'a>>,
}

//static mut EMPTY: Option<&'static mut TreapNode<'static>> = None;

//static EMPTYREF: &'static mut Option<&'static mut TreapNode<'static>> = &mut EMPTY;

impl<'a> TreapNode<'a> {
  fn new(key: i32) -> TreapNode<'a> {
    let n = rand::thread_rng().gen();
    TreapNode {
      key: key,
      size: 1,
      priority: n,
      l: None,
      r: None,
    }
  }
}

fn merge<'a>(a: Option<&'a mut TreapNode<'a>>,
             b: Option<&'a mut TreapNode<'a>>)
             -> Option<&'a mut TreapNode<'a>> {
  match (a, b) {
    (&mut None, b) => *b,
    (a, &mut None) => *a,
    (&mut Some(a), &mut Some(b)) => Some(a),
    //(Some(a), Some(b)) => {
        //if a.priority > b.priority {
            //a.r = merge(a.r, Some(b));
            //Some(a)
        //} else {
            //b.l = merge(Some(a), b.l);
            //Some(b)
        //}
    //},
    //(a, _) => a,
  }
}

fn main() {
    let t = TreapNode::new(3);
}
