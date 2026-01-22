pub fn rb_tree_insert_case4(mut tree: Ptr<RBTree>, mut node: Ptr<RBTreeNode>) {
    let mut next_node: Ptr<RBTreeNode> = Default::default();
    let mut side: RBTreeNodeSide = Default::default();
    
    side = rb_tree_node_side(node);
    if (side != rb_tree_node_side(node.parent)) {
        next_node = node.parent;
        rb_tree_rotate(tree, node.parent, (1 - side));
    } else {
        next_node = node;
    }
    rb_tree_insert_case5(tree, next_node);
}