use rand::thread_rng;
use rand_distr::{Distribution, Exp, Uniform};

use crate::kruskals::UnionFind;

#[inline(never)]
pub fn fast_zero_dim_mst(size: usize) -> f64 {
    // MAKESET(u) for all u in V
    let mut set = UnionFind::new(size as u32);
    let mut total_count = size - 1;
    let mut total_weight = 0.0;

    let mut rng = thread_rng();
    let exp = Exp::new((size * size / 2) as f64 - 1.0).unwrap();

    let mut weight = 0.0;

    let vertex_distr = Uniform::new(0_u32, size as u32);

    // For each (u,v) edge
    loop {
        let u = vertex_distr.sample(&mut rng);
        let v = vertex_distr.sample(&mut rng);
        weight += exp.sample(&mut rng);

        // If FIND(u) != FIND(v)
        if set.find(u) != set.find(v) {
            // UNION(u,v)
            set.union(u, v);

            // Add edge to total graph
            total_weight += weight;
            total_count -= 1;
        }

        // If we have a tree, we're done
        if total_count == 0 {
            break;
        }
    }

    total_weight
}
