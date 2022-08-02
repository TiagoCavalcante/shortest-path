mod graph;
mod path;
mod rand;

fn main() {
  let graph = graph::Graph::new(1000, 0.01);
  println!(
    "{:?}",
    path::shortest_path(&graph, 0, 299).unwrap_or(vec![])
  )
}
