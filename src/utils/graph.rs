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
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NodePtr {
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
    node_index: NodePtr,
    first_edge: Option<EdgePtr>,
}

/// Represents the index of an edge in the graph.
///
/// This struct is a transparent wrapper around a `usize` and is used to uniquely
/// identify edges within the graph.
#[repr(transparent)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EdgePtr {
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
    to: NodePtr,
    next_edge: Option<EdgePtr>,
}

impl<N, E> Graph<N, E> {
    /// Creates a new, empty graph.
    ///
    /// # Returns
    ///
    /// A new instance of `Graph`.
    #[allow(dead_code)]
    #[inline]
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }

    pub fn nodes(&self) -> Vec<&N> {
        self.nodes.iter().map(|node| &node.data).collect::<Vec<_>>()
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
    pub fn find_node_index<F>(&self, find_fn: F) -> Option<NodePtr>
    where
        N: PartialEq + Eq,
        F: Fn(&N) -> bool,
    {
        self.nodes
            .iter()
            .find(|node| find_fn(&node.data))
            .map(|node| node.node_index.clone())
    }

    /// # Returns
    ///
    /// Gets the number of nodes in the graph.
    pub fn len(&self) -> usize {
        self.nodes.len()
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
    pub fn get(&self, node_index: &NodePtr) -> &N {
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
    #[allow(dead_code)]
    pub fn get_mut(&mut self, node_index: NodePtr) -> &mut N {
        &mut self.nodes[node_index.idx].data
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
    pub fn add_node(&mut self, data: N) -> NodePtr {
        let node_index = NodePtr {
            idx: self.nodes.len(),
        };
        self.nodes.push(Node {
            data,
            node_index: node_index.clone(),
            first_edge: None,
        });

        node_index
    }

    /// Adds a new edge between two nodes in the graph.
    ///
    /// # Arguments
    ///
    /// * `from` - The index of the source node.
    /// * `to` - The index of the destination node.
    /// * `edge_data` - The data to store in the new edge.
    pub fn add_edge(&mut self, from: NodePtr, to: NodePtr, edge_data: E) {
        let new_edge_index = Some(EdgePtr {
            idx: self.edges.len(),
        });
        self.edges.push(Edge {
            data: edge_data,
            to,
            next_edge: self.nodes[from.idx].first_edge.clone(),
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
    pub fn add_edge_by_data(&mut self, node_a: N, node_b: N, relatoinship: Relationship<E>)
    where
        N: PartialEq + Eq,
    {
        let a_index = match self.find_node_index(|node| node == &node_a) {
            None => self.add_node(node_a),
            Some(node_index) => node_index,
        };

        let b_index = match self.find_node_index(|node| node == &node_b) {
            None => self.add_node(node_b),
            Some(node_index) => node_index,
        };

        match relatoinship {
            Relationship::BiDirectional { a_to_b, b_to_a } => {
                self.add_edge(a_index.clone(), b_index.clone(), a_to_b);
                self.add_edge(b_index, a_index, b_to_a);
            }
            Relationship::AToB(edge) => {
                self.add_edge(a_index, b_index, edge);
            }
            Relationship::BToA(edge) => {
                self.add_edge(b_index, a_index, edge);
            }
        }
    }

    fn get_edge(&self, edge_index: EdgePtr) -> &Edge<E> {
        &self.edges[edge_index.idx]
    }

    pub fn neighbours_iter(&self, node_index: &NodePtr) -> Neighbours<N, E> {
        Neighbours {
            graph: self,
            edges: self.nodes[node_index.idx].first_edge.clone(),
        }
    }
}

pub struct Neighbours<'a, N, E> {
    graph: &'a Graph<N, E>,
    edges: Option<EdgePtr>,
}

impl<'a, N, E> Iterator for Neighbours<'a, N, E>
where
    E: 'a,
{
    type Item = (&'a NodePtr, &'a E);

    fn next(&mut self) -> Option<Self::Item> {
        self.edges.clone().map(|edge_index| {
            let edge = self.graph.get_edge(edge_index);
            self.edges = edge.next_edge.clone();
            (&edge.to, &edge.data)
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
                let mut curr_edge = nodes.first_edge.clone();
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
                while let Some(edge_index) = curr_edge.clone() {
                    let edge = &self.edges[edge_index.idx];
                    writeln!(
                        f,
                        "\t\tEdge: '{:?}' ->  To: '{:?}'",
                        edge.data, self.nodes[edge.to.idx].data
                    )?;
                    curr_edge = edge.next_edge.clone();
                }
                writeln!(f, "\t]")?;
                visited.push(nodes.node_index.clone())
            }
        }
        write!(f, "}}")?;
        Ok(())
    }
}

/// Represents the type of relationship between two nodes in the graph.
///
/// # Type Parameters
/// * `E` - The type of data stored in the edges.
#[derive(Debug, Clone)]
pub enum Relationship<E> {
    /// A bidirectional relationship between two nodes.
    /// Contains data for both directions (a->b and b->a).
    BiDirectional { a_to_b: E, b_to_a: E },

    /// A unidirectional relationship from node A to node B.
    AToB(E),

    /// A unidirectional relationship from node B to node A.
    BToA(E),
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
            graph.add_edge_by_data(from, to, Relationship::AToB(E::default()));
        }
        graph
    }
}

impl<N, E> From<Vec<(N, N, Relationship<E>)>> for Graph<N, E>
where
    N: PartialEq + Eq,
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
    fn from(vec_tuple: Vec<(N, N, Relationship<E>)>) -> Self {
        let mut graph = Self {
            edges: Vec::with_capacity(vec_tuple.len()),
            nodes: Vec::with_capacity(vec_tuple.len()),
        };
        for (from, to, relationship) in vec_tuple {
            graph.add_edge_by_data(from, to, relationship);
        }
        graph
    }
}

impl<N, E, const S: usize> From<[(N, N, Relationship<E>); S]> for Graph<N, E>
where
    N: PartialEq + Eq,
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
    fn from(array_tuple: [(N, N, Relationship<E>); S]) -> Self {
        let mut graph = Self {
            edges: Vec::with_capacity(array_tuple.len()),
            nodes: Vec::with_capacity(array_tuple.len()),
        };

        for (from, to, relationship) in array_tuple {
            graph.add_edge_by_data(from, to, relationship);
        }

        graph
    }
}
