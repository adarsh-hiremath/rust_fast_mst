use std::cell::Cell;

use crate::graph::Graph;

pub fn mst(graph: &impl Graph) -> f64 {
    // Sort edges by weight
    let mut sorted_edges = graph.edges().clone();
    sorted_edges.sort_by(|a, b| a.weight.total_cmp(&b.weight));

    // MAKESET(u) for all u in V
    let mut set = UnionFind::new(graph.vertices() as u32);
    let mut total_count = graph.vertices() - 1;
    let mut total_weight = 0.0;

    // For each (u,v) edge
    for edge in sorted_edges.iter() {
        // If FIND(u) != FIND(v)
        if set.find(edge.u) != set.find(edge.v) {
            // Add edge to total graph
            total_weight += edge.weight;
            total_count -= 1;

            // UNION(u,v)
            set.union(edge.u, edge.v);
        }

        // If we have a tree, we're done
        if total_count == 0 {
            break;
        }
    }

    total_weight
}

pub struct UnionFindTree {
    pub parent: Cell<u32>,
    pub link: Cell<u32>,
    pub rank: Cell<u32>,
}

// Union find data structure on a set of usize
pub struct UnionFind {
    trees: Vec<UnionFindTree>,
}

impl UnionFind {
    pub fn new(size: u32) -> Self {
        Self {
            trees: (0..size)
                .into_iter()
                .map(|index| UnionFindTree {
                    parent: Cell::new(index),
                    link: Cell::new(index),
                    rank: Cell::new(0),
                })
                .collect(),
        }
    }

    pub fn union(&mut self, first: u32, second: u32) {
        let i = self.find(first);
        let j = self.find(second);

        if i == j {
            return;
        }

        self.trees[i as usize]
            .link
            .swap(&self.trees[j as usize].link);

        let i_rank = self.trees[i as usize].rank.get();
        let j_rank = self.trees[j as usize].rank.get();

        use std::cmp::Ordering::*;
        match i_rank.cmp(&j_rank) {
            Less => self.trees[i as usize].parent.set(j),
            Equal => {
                self.trees[i as usize].parent.set(j);
                *self.trees[j as usize].rank.get_mut() += 1;
            }
            Greater => self.trees[j as usize].parent.set(i),
        }
    }

    pub fn find(&self, index: u32) -> u32 {
        // Means that the node is a root
        if self.trees[index as usize].parent.get() == index {
            index
        } else {
            let root = self.find(self.trees[index as usize].parent.get());
            self.trees[index as usize].parent.set(root);
            root
        }
    }
}
