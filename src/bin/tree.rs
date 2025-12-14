use std::collections::VecDeque;

#[derive(Debug)]
struct Node {
    val: i32,
    children: Vec<Node>,
}

// DFS preorder
fn dfs_preorder(node: &Node, out: &mut Vec<i32>) {
    out.push(node.val);
    for ch in &node.children {
        dfs_preorder(ch, out);
    }
}

// BFS
fn bfs(root: &Node) -> Vec<Vec<i32>> {
    let mut res = Vec::new();
    let mut q: VecDeque<&Node> = VecDeque::new();
    q.push_back(root);

    while !q.is_empty() {
        let len = q.len();
        let mut level = Vec::with_capacity(len);
        for _ in 0..len {
            let node = q.pop_front().unwrap();
            level.push(node.val);
            for ch in &node.children {
                q.push_back(ch);
            }
        }
        res.push(level);
    }

    res
}

fn main() {
    let root = Node {
        val: 1,
        children: vec![
            Node { val: 2, children: vec![] },
            Node {
                val: 3,
                children: vec![Node { val: 4, children: vec![] }],
            },
        ],
    };

    let mut out = Vec::new();
    dfs_preorder(&root, &mut out);
    println!("dfs_preorder: {:?}", out);
    println!("bfs: {:?}", bfs(&root));
}
