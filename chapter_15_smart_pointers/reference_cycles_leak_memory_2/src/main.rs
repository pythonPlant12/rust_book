fn main() {
    // ========================================
    // Creating a Tree Data Structure: A Node with Child Nodes
    // ========================================
    //
    //
    // To start, we'll build a tree with nodes that know about their child nodes. We'll create a
    // struct named `Node` that holds its own i32 value as well as references to its children
    // `Node` values:
    //

    // let leaf = Rc::new(Node {
    //     value: 3,
    //     children: RefCell::new(vec![]),
    // });
    //
    // let branch = Rc::new(Node {
    //     value: 5,
    //     children: RefCell::new(vec![Rc::clone(&leaf)]),
    // });

    // We clone the `Rc<Node>` in `leaf` and store that in `branch`, meaning the `Node` in `leaf`
    // now has two owners: `leaf` and `branch`. We can get from `branch` to `leaf` through
    // `branch.children`, but there's no way to get from `leaf` to `branch`. The reason is that
    // `leaf` has no reference to `branch` and doesn't know they're related. We want `leaf` to know
    // that branch is its parent. We'll do that next.
    //
    //
    // ====================================
    // Adding a Reference from a Child to Its Parent
    // ====================================
    //
    // To make the child node aware of its parent, we need to add a `parent` field to our `Node`
    // struct definition. The trouble is in deciding what the type of `parent` should be. We know
    // it can't contain an Rc<T> because that would create a reference cycle with `leaf.parent`
    // pointing to `branch` and `branch.children` pointing to `leaf`, which would cause their
    // `strong_count` values to never be 0.
    //
    // Thinking about the relationships another way, a parent node should own its children: if a
    // parent node is dropped, its child nodes should be dropped as well. However, a child should
    // not own its parent: if we drop a child node, the parent should still exist. This is a case
    // for WEAK REFERNCES.
    //
    // So, instead of `Rc<T>`, we'll make the type of `parent` use `Weak<T>`, specifically a
    // `RefCell<Weak<Node>>`. Now our `Node` struct definition looks like this: (Refer to old Node
    // implementation)
    //
    //
    // A node will be able to refer to its parent node but doesn't own its parent. In a next
    // listing, we update `main` to use this new definition so the `leaf` node will have a way to
    // refer to its parent, `branch`
    //
    //
    let leaf1 = Rc::new( Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![])
    });

    println!("leaf1 parent = {:?}", leaf1.parent.borrow().upgrade());

    let branch1 = Rc::new( Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf1)])
    });

    *leaf1.parent.borrow_mut() = Rc::downgrade(&branch1);

    println!("leaf1 parent = {:?}", leaf1.parent.borrow().upgrade());

    println!("====================================================");
    println!("Get examples of strong_count and weak_count references");
    println!("====================================================");

    let leaf3 = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!(
        "leaf3 strong = {}, weak = {}",
        Rc::strong_count(&leaf3),
        Rc::weak_count(&leaf3),
    );

    {
        let branch3 = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf3)]),
        });

        *leaf3.parent.borrow_mut() = Rc::downgrade(&branch3);

        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch3),
            Rc::weak_count(&branch3),
        );

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf3),
            Rc::weak_count(&leaf3),
        );
    }

    println!("leaf parent = {:?}", leaf3.parent.borrow().upgrade());
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf3),
        Rc::weak_count(&leaf3),
    );

}
use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
// struct Node {
//     value: i32,
//     children: RefCell<Vec<Rc<Node>>>,
// }

struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

// We want a `Node` to own its children, and we want to share that ownership with variables so we
// can access each `Node` in the tree directly. To do this, we define the `Vec<T>` items to be
// values of type Rc<Node>.
//
// Next, we'll use our struct definition and create one `Node` instance named `leaf` with the value
// `3` and no children, and another instance named `branch` with the value `5` and `leaf` as one of
// its children.
//
