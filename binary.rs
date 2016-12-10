
pub struct TreeNode<T> {
  key: T,
  left : Option<Box<TreeNode<T>>>,
  right: Option<Box<TreeNode<T>>>,
}

pub struct Tree<T>{
  root: Option<Box<TreeNode<T>>>,
}

impl<T: Ord> TreeNode<T> {
    /// Creates an empty tree
    pub fn new(key: T) -> Self {
        TreeNode{
        key :key,
        right: None,
        left : None,
        }

        }

    /// Returns `false` if `key` already exists in the tree, and `true` otherwise.
    pub fn insert(&mut self, key: T) -> bool {

        if key > self.key{
                 match self.right{
                         Some(ref mut n) => n.as_mut().insert(key),
                         None        => {self.right = Some(Box::new(TreeNode::new(key)));return true;},
                                }
        }
                        else if key < self.key{
                match self.left{
                         Some(ref mut n) => n.as_mut().insert(key),
                         None        => {self.left = Some(Box::new(TreeNode::new(key)));return true;},
                }
           }
        else {
             return false;
         }
        }
    /// Returns `true` if `key` exists in the tree, and `false` otherwise.
    pub fn find(&self, key: &T) -> bool {
        if self.key == *key{
        true
        }
        else if *key > self.key{
        match self.right{
                Some(ref n) => n.find(key),
                None => false,

        }
        }
        else{
        match self.left {
                Some(ref n) => n.find(key),
                None        => false,
                }
             }
        }
        
       
        fn preorder<'a>(&'a self, vector: &mut Vec<&'a T>){
  
                            vector.push(&self.key);
                           
                           match self.left {
                                Some(ref n) => {n.preorder(vector);},
                                None => {},
                           }
                           
                           match self.right {
                                Some(ref n) => {n.preorder(vector);}, 
                                None => {},
                            }
                        
        }
    
        fn inorder<'a>(&'a self, vector: &mut Vec<&'a T>) {
                           match self.left {
                                Some(ref n) => {n.inorder(vector);},
                                None => {},
                            }
                            
                            vector.push(&self.key);

                            match self.right {
                                Some(ref n) => {n.inorder(vector);}, 
                                None => {},
                            }
                    
    }
        fn postorder<'a>(&'a self, vector: &mut Vec<&'a T>){
                           match self.left {
                                Some(ref n) => {n.postorder(vector);},
                                None => {},
                                        }
                            match self.right {
                                Some(ref n) => {n.postorder(vector);}, 
                                None => {},
                            }
                            
                            vector.push(&self.key);
    }
    
}
                        

impl<T: Ord> Tree<T> {
    /// Creates an empty tree
    pub fn new() -> Self {
        Tree{
        root: None,     }
        }

    /// Returns `false` if `key` already exists in the tree, and `true` otherwise.
    pub fn insert(&mut self, key: T) -> bool {

        match self.root{
                Some(ref mut n) => n.insert(key),
                None  => {self.root = Some(Box::new(TreeNode::new(key))); return true;},
                }
 }

    /// Returns `true` if `key` exists in the tree, and `false` otherwise.
    pub fn find(&self, key: &T) -> bool {
        match self.root{
                Some(ref n) =>n.find(key),
                None => {return false;},
                }
        }
    /// Returns the preorder traversal of the tree.
    pub fn preorder(&self) -> Vec<&T> {
         let mut vector = Vec::<&T>::new();
             match self.root {
                              Some(ref n) => {n.preorder(&mut vector);},
                                None => {},
                           }
        return vector;
    }

/// Returns the inorder traversal of the tree.
    pub fn inorder(&self) -> Vec<&T> {
        let mut vector = Vec::new();
            match self.root{
                Some(ref n) => { 
                        
                        n.inorder(&mut vector);},
                None => { println!("the set in empty"); },
               }
           
        return vector;
        }
    /// Returns the postorder traversal of the tree.
    pub fn postorder(&self) -> Vec<&T> {
        let mut vector = Vec::new();
            match self.root{
                Some(ref n) => { n.postorder(&mut vector);},
                None => { println!("the set in empty"); },
               }
        return vector;
    }
}

n main() {
    let mut x:Tree<i32> = Tree::new();
    x.insert(10);
    x.insert(5);
    x.insert(1);
    x.insert(40);
    x.insert(30);
    x.insert(50);
    x.insert(7);
  
   



    let pre = x.preorder();
    // should be [4,2,1,3,5,6]
    
    println!("pre {:?}", pre);

    //let post = x.postorder();
    // should be [1,3,2,6,5,4]
    //println!("post {:?}", post);

    //let inor = x.inorder(); // should be [1,2,3,4,5,6]
    //println!("in {:?}", inor);
    
}
