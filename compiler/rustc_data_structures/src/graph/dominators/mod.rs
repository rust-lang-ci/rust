//! Finding the dominators in a control-flow graph.
//!
//! Algorithm based on Keith D. Cooper, Timothy J. Harvey, and Ken Kennedy,
//! "A Simple, Fast Dominance Algorithm",
//! Rice Computer Science TS-06-33870,
//! <https://www.cs.rice.edu/~keith/EMBED/dom.pdf>.

use super::iterate::reverse_post_order;
use super::ControlFlowGraph;
use rustc_index::vec::{Idx, IndexVec};
use std::cmp::Ordering;

#[cfg(test)]
mod tests;

pub fn dominators<G: ControlFlowGraph>(graph: G) -> Dominators<G::Node> {
    let start_node = graph.start_node();
    let rpo = reverse_post_order(&graph, start_node);
    dominators_given_rpo(graph, &rpo)
}

fn dominators_given_rpo<G: ControlFlowGraph>(graph: G, rpo: &[G::Node]) -> Dominators<G::Node> {
    let start_node = graph.start_node();
    assert_eq!(rpo[0], start_node);

    // compute the post order index (rank) for each node
    let mut post_order_rank = IndexVec::from_elem_n(0, graph.num_nodes());
    for (index, node) in rpo.iter().rev().cloned().enumerate() {
        post_order_rank[node] = index;
    }

    let mut immediate_dominators = IndexVec::from_elem_n(None, graph.num_nodes());
    immediate_dominators[start_node] = Some(start_node);

    let mut changed = true;
    while changed {
        changed = false;

        for &node in &rpo[1..] {
            let mut new_idom = None;
            for pred in graph.predecessors(node) {
                if immediate_dominators[pred].is_some() {
                    // (*) dominators for `pred` have been calculated
                    new_idom = Some(if let Some(new_idom) = new_idom {
                        intersect(&post_order_rank, &immediate_dominators, new_idom, pred)
                    } else {
                        pred
                    });
                }
            }

            if new_idom != immediate_dominators[node] {
                immediate_dominators[node] = new_idom;
                changed = true;
            }
        }
    }

    Dominators { post_order_rank, immediate_dominators }
}

fn intersect<Node: Idx>(
    post_order_rank: &IndexVec<Node, usize>,
    immediate_dominators: &IndexVec<Node, Option<Node>>,
    mut node1: Node,
    mut node2: Node,
) -> Node {
    while node1 != node2 {
        while post_order_rank[node1] < post_order_rank[node2] {
            node1 = immediate_dominators[node1].unwrap();
        }

        while post_order_rank[node2] < post_order_rank[node1] {
            node2 = immediate_dominators[node2].unwrap();
        }
    }

    node1
}

#[derive(Clone, Debug)]
pub struct Dominators<N: Idx> {
    post_order_rank: IndexVec<N, usize>,
    immediate_dominators: IndexVec<N, Option<N>>,
}

impl<Node: Idx> Dominators<Node> {
    pub fn dummy() -> Self {
        Self { post_order_rank: IndexVec::new(), immediate_dominators: IndexVec::new() }
    }

    pub fn is_reachable(&self, node: Node) -> bool {
        self.immediate_dominators[node].is_some()
    }

    pub fn immediate_dominator(&self, node: Node) -> Node {
        assert!(self.is_reachable(node), "node {:?} is not reachable", node);
        self.immediate_dominators[node].unwrap()
    }

    pub fn dominators(&self, node: Node) -> Iter<'_, Node> {
        assert!(self.is_reachable(node), "node {:?} is not reachable", node);
        Iter { dominators: self, node: Some(node) }
    }

    pub fn is_dominated_by(&self, node: Node, dom: Node) -> bool {
        // FIXME -- could be optimized by using post-order-rank
        self.dominators(node).any(|n| n == dom)
    }

    /// Provide deterministic ordering of nodes such that, if any two nodes have a dominator
    /// relationship, the dominator will always precede the dominated. (The relative ordering
    /// of two unrelated nodes will also be consistent, but otherwise the order has no
    /// meaning.) This method cannot be used to determine if either Node dominates the other.
    pub fn rank_partial_cmp(&self, lhs: Node, rhs: Node) -> Option<Ordering> {
        self.post_order_rank[lhs].partial_cmp(&self.post_order_rank[rhs])
    }
}

pub struct Iter<'dom, Node: Idx> {
    dominators: &'dom Dominators<Node>,
    node: Option<Node>,
}

impl<'dom, Node: Idx> Iterator for Iter<'dom, Node> {
    type Item = Node;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(node) = self.node {
            let dom = self.dominators.immediate_dominator(node);
            if dom == node {
                self.node = None; // reached the root
            } else {
                self.node = Some(dom);
            }
            Some(node)
        } else {
            None
        }
    }
}
