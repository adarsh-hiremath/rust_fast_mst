use rand::{thread_rng, Rng};

#[derive(Copy, Clone)]
pub struct Edge {
    pub u: u32,
    pub v: u32,
    pub weight: f64,
}

pub trait Graph {
    fn vertices(&self) -> usize;
    fn edges(&self) -> &Vec<Edge>;
}

pub struct CompleteGraph {
    size: usize,
    edges: Vec<Edge>,
}

impl CompleteGraph {
    pub fn create_with_filter(size: usize, filter: impl Fn(usize) -> f64) -> Self {
        let filter = filter(size);

        let mut rng = thread_rng();
        let mut edges = Vec::new();

        for u in 0..size {
            for v in 0..u {
                let weight = rng.gen_range(0.0..1.0);

                if weight <= filter {
                    edges.push(Edge {
                        u: u as u32,
                        v: v as u32,
                        weight,
                    });
                }
            }
        }

        Self { size, edges }
    }
}

impl Graph for CompleteGraph {
    fn edges(&self) -> &Vec<Edge> {
        &self.edges
    }

    fn vertices(&self) -> usize {
        self.size
    }
}

pub struct EuclideanGraph<const D: usize> {
    vertices: Vec<[f64; D]>,
    edges: Vec<Edge>,
}

fn distance<const D: usize>(a: [f64; D], b: [f64; D]) -> f64 {
    let mut dot = 0_f64;
    for index in 0..D {
        dot += (a[index] - b[index]).powi(2);
    }
    dot.sqrt()
}

impl<const D: usize> EuclideanGraph<D> {
    pub fn create_with_filter(size: usize, filter: impl Fn(usize) -> f64) -> Self {
        let filter = filter(size);

        let mut vertices = Vec::with_capacity(size);
        let mut rng = thread_rng();

        for _ in 0..size {
            let mut vertex = [0.0; D];
            for coord in vertex.iter_mut().take(D) {
                *coord = rng.gen_range(0.0..1.0);
            }
            vertices.push(vertex);
        }

        let mut edges = Vec::new();
        for u in 0..size {
            for v in 0..=u {
                let weight = distance(vertices[u], vertices[v]);

                if weight <= filter {
                    edges.push(Edge {
                        u: u as u32,
                        v: v as u32,
                        weight,
                    });
                }
            }
        }

        Self { vertices, edges }
    }
}

impl<const D: usize> Graph for EuclideanGraph<D> {
    fn edges(&self) -> &Vec<Edge> {
        &self.edges
    }

    fn vertices(&self) -> usize {
        self.vertices.len()
    }
}
