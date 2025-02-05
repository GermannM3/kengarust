pub struct KnowledgeGraph {
    nodes: Vec<Node>,
    edges: Vec<Edge>,
}

struct Node {
    id: String,
    content: String,
}

struct Edge {
    source: String,
    target: String,
    weight: f32,
}

impl KnowledgeGraph {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }

    pub fn add_node(&mut self, id: String, content: String) {
        self.nodes.push(Node { id, content });
    }

    pub fn connect(&mut self, source: &str, target: &str, weight: f32) {
        self.edges.push(Edge {
            source: source.to_string(),
            target: target.to_string(),
            weight,
        });
    }
}
