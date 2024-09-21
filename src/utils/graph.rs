use std::collections::HashMap;
use std::fmt::Formatter;

/// A graph data structure where nodes and edges are stored in vectors.
///
/// This implementation is inspired by the blog post ["Modeling graphs in Rust using vector indices"
/// by Niko Matsakis](https://smallcultfollowing.com/babysteps/blog/2015/04/06/modeling-graphs-in-rust-using-vector-indices/).
/// The high-level idea is to represent a "pointer" to a node or edge using an index. A graph consists
/// of a vector of nodes and a vector of edges, much like the mathematical description G=(V,E).
///
/// # Advantages
/// - This approach aligns well with Rust's ownership model.
/// - Unlike `Rc` pointers, an index alone is not enough to mutate the graph, which allows tracking
///   the mutability of the graph as a whole.
/// - Graphs implemented this way can easily be sent between threads and used in data-parallel code.
/// - The overall data structure is very compact, with no need for separate allocations for each node.
///
/// # Disadvantages
/// - Removing nodes or edges from the graph can be problematic, as it may lead to "dangling indices"
///   or require a placeholder, similar to issues with `malloc`/`free`. `(For now removal is not implemented.)`
/// - Indices from one graph should not be used with another graph to avoid misuse.
///
/// # Type Parameters
/// * `N` - The type of data stored in the nodes.
/// * `E` - The type of data stored in the edges.
///
/// # Examples
///
/// ```
/// // Create a new graph
/// let mut graph = Graph::new();
///
/// // Add nodes to the graph
/// let node_a = graph.add_node("A");
/// let node_b = graph.add_node("B");
/// let node_c = graph.add_node("C");
///
/// let edge_data = ();
///
/// // Add edges between nodes
/// graph.add_edge(node_a, node_b, edge_data);
/// graph.add_edge(node_b, node_c, edge_data);
/// graph.add_edge(node_c, node_a, edge_data);
///
/// // Find a node by data
/// if let Some(node_index) = graph.find_node_index(|node: &&str| node == &"B") {
///     // Retrieve and print the data of the found node
///     let node_data = graph.get_node_data(node_index);
///     println!("Node data: {}", node_data);
/// }
///
/// // Print the graph
/// println!("{:?}", graph);
/// ```
pub struct Graph<N, E> {
    nodes: Vec<Node<N>>,
    edges: Vec<Edge<E>>,
}

/// Represents the index of a node in the graph.
///
/// This struct is a transparent wrapper around a `usize` and is used to uniquely
/// identify nodes within the graph.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NodeIndex {
    idx: usize,
}

/// A node in the graph.
///
/// # Type Parameters
///
/// * `N` - The type of data stored in the node.
#[derive(Debug)]
struct Node<N> {
    data: N,
    node_index: NodeIndex,
    first_edge: Option<EdgeIndex>,
}

/// Represents the index of an edge in the graph.
///
/// This struct is a transparent wrapper around a `usize` and is used to uniquely
/// identify edges within the graph.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EdgeIndex {
    idx: usize,
}

/// An edge in the graph.
///
/// # Type Parameters
///
/// * `E` - The type of data stored in the edge.
#[derive(Debug)]
struct Edge<E> {
    data: E,
    to: NodeIndex,
    next_edge: Option<EdgeIndex>,
}

impl<N, E> Graph<N, E> {
    /// Creates a new, empty graph.
    ///
    /// # Returns
    ///
    /// A new instance of `Graph`.
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }

    /// Finds the index of a node containing the specified data.
    ///
    /// # Arguments
    ///
    /// * `find_fn` - A closure that takes a reference to the node data and returns a boolean indicating
    ///   whether the node matches the search criteria.
    ///
    /// # Returns
    ///
    /// An `Option` containing the `NodeIndex` if found, or `None` if not found.
    pub fn find_node_index<F>(&self, find_fn: F) -> Option<NodeIndex>
    where
        N: PartialEq + Eq,
        F: Fn(&N) -> bool,
    {
        self.nodes
            .iter()
            .find(|node| find_fn(&node.data))
            .map(|node| node.node_index)
    }

    pub fn num_of_nodes(&self) -> usize {
        self.nodes.len()
    }

    /// Adds a new node with the specified data to the graph.
    ///
    /// # Arguments
    ///
    /// * `data` - The data to store in the new node.
    ///
    /// # Returns
    ///
    /// The `NodeIndex` of the newly added node.
    pub fn add_node(&mut self, data: N) -> NodeIndex {
        let node_index = NodeIndex {
            idx: self.nodes.len(),
        };
        self.nodes.push(Node {
            data,
            node_index,
            first_edge: None,
        });
        node_index
    }

    /// Gets a reference to the data stored in the node at the specified index.
    ///
    /// # Arguments
    ///
    /// * `node_index` - The index of the node.
    ///
    /// # Returns
    ///
    /// A reference to the data stored in the node.
    pub fn get_node_data(&self, node_index: NodeIndex) -> &N {
        &self.nodes[node_index.idx].data
    }

    /// Gets a mutable reference to the data stored in the node at the specified index.
    ///
    /// # Arguments
    ///
    /// * `node_index` - The index of the node.
    ///
    /// # Returns
    ///
    /// A mutable reference to the data stored in the node.
    fn get_node_data_mut(&mut self, node_index: NodeIndex) -> &mut N {
        &mut self.nodes[node_index.idx].data
    }

    /// Adds a new edge between two nodes in the graph.
    ///
    /// # Arguments
    ///
    /// * `from` - The index of the source node.
    /// * `to` - The index of the destination node.
    /// * `edge_data` - The data to store in the new edge.
    pub fn add_edge(&mut self, from: NodeIndex, to: NodeIndex, edge_data: E) {
        let new_edge_index = Some(EdgeIndex {
            idx: self.edges.len(),
        });
        self.edges.push(Edge {
            data: edge_data,
            to,
            next_edge: self.nodes[from.idx].first_edge,
        });
        self.nodes[from.idx].first_edge = new_edge_index;
    }

    /// Adds a new edge between two nodes, identified by their data.
    ///
    /// # Arguments
    ///
    /// * `from` - The data of the source node.
    /// * `to` - The data of the destination node.
    /// * `edge_data` - The data to store in the new edge.
    pub fn add_edge_by_data(&mut self, from: N, to: N, edge_data: E)
    where
        N: PartialEq + Eq,
    {
        let from_index = match self.find_node_index(|node| node == &from) {
            None => self.add_node(from),
            Some(node_index) => node_index,
        };

        let to_index = match self.find_node_index(|node| node == &to) {
            None => self.add_node(to),
            Some(node_index) => node_index,
        };

        self.add_edge(from_index, to_index, edge_data);
    }

    fn get_edge(&self, edge_index: EdgeIndex) -> &Edge<E> {
        &self.edges[edge_index.idx]
    }

    pub fn neighbours_iter(&self, node_index: NodeIndex) -> Neighbours<N, E> {
        Neighbours {
            graph: self,
            edges: self.nodes[node_index.idx].first_edge,
        }
    }
}

pub struct Neighbours<'a, N, E> {
    graph: &'a Graph<N, E>,
    edges: Option<EdgeIndex>,
}

impl<N, E> Iterator for Neighbours<'_, N, E> {
    type Item = NodeIndex;

    fn next(&mut self) -> Option<Self::Item> {
        self.edges.map(|edge_index| {
            let edge = self.graph.get_edge(edge_index);
            self.edges = edge.next_edge;
            edge.to
        })
    }
}

impl<N, E> std::fmt::Debug for Graph<N, E>
where
    N: std::fmt::Debug,
    E: std::fmt::Debug,
{
    /// Formats the graph using the given formatter.
    ///
    /// # Arguments
    ///
    /// * `f` - The formatter to use.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or failure.
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut visited = Vec::with_capacity(self.nodes.len());
        writeln!(f, "Graph: ({} nodes) {{", self.nodes.len())?;
        for nodes in self.nodes.iter() {
            if !visited.contains(&nodes.node_index) {
                let mut curr_edge = nodes.first_edge;
                if curr_edge.is_none() {
                    writeln!(
                        f,
                        "\tNode: ({:?}) (Data: '{:?}') : []",
                        nodes.node_index, nodes.data
                    )?;
                    continue;
                }
                writeln!(
                    f,
                    "\tNode: ({:?}) (Data: '{:?}') : [",
                    nodes.node_index, nodes.data
                )?;
                while let Some(edge_index) = curr_edge {
                    let edge = &self.edges[edge_index.idx];
                    writeln!(
                        f,
                        "\t\tEdge: '{:?}' ->  To: '{:?}'",
                        edge.data, self.nodes[edge.to.idx].data
                    )?;
                    curr_edge = edge.next_edge;
                }
                writeln!(f, "\t]")?;
                visited.push(nodes.node_index)
            }
        }
        write!(f, "}}")?;
        Ok(())
    }
}

impl<N, E> From<HashMap<N, N>> for Graph<N, E>
where
    N: PartialEq + Eq,
    E: Default,
{
    /// Creates a graph from a `HashMap` where keys and values represent nodes.
    ///
    /// # Arguments
    ///
    /// * `hash_map` - The `HashMap` to convert into a graph.
    ///
    /// # Returns
    ///
    /// A new instance of `Graph`.
    fn from(hash_map: HashMap<N, N>) -> Self {
        let mut graph = Self {
            edges: Vec::with_capacity(hash_map.len()),
            nodes: Vec::with_capacity(hash_map.len()),
        };
        for (from, to) in hash_map {
            graph.add_edge_by_data(from, to, E::default());
        }
        graph
    }
}

impl<N, E> From<Vec<(N, N)>> for Graph<N, E>
where
    N: PartialEq + Eq,
    E: Default,
{
    /// Creates a graph from a vector of tuples where each tuple represents an edge.
    ///
    /// # Arguments
    ///
    /// * `vec_tuple` - The vector of tuples to convert into a graph.
    ///
    /// # Returns
    ///
    /// A new instance of `Graph`.
    fn from(vec_tuple: Vec<(N, N)>) -> Self {
        let mut graph = Self {
            edges: Vec::with_capacity(vec_tuple.len()),
            nodes: Vec::with_capacity(vec_tuple.len()),
        };
        for (from, to) in vec_tuple {
            graph.add_edge_by_data(from, to, E::default());
        }
        graph
    }
}

impl<N, E, const S: usize> From<[(N, N); S]> for Graph<N, E>
where
    N: PartialEq + Eq,
    E: Default,
{
    /// Creates a graph from a vector of tuples where each tuple represents an edge.
    ///
    /// # Arguments
    ///
    /// * `vec_tuple` - The vector of tuples to convert into a graph.
    ///
    /// # Returns
    ///
    /// A new instance of `Graph`.
    fn from(array_tuple: [(N, N); S]) -> Self {
        let mut graph = Self {
            edges: Vec::with_capacity(array_tuple.len()),
            nodes: Vec::with_capacity(array_tuple.len()),
        };
        for (from, to) in array_tuple {
            graph.add_edge_by_data(from, to, E::default());
        }
        graph
    }
}
