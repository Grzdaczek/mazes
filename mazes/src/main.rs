use clap::Parser;
use graph::Graph;
use rand::{seq::SliceRandom, thread_rng};
use std::cell::Cell;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Width of the maze
    #[clap(short, long, default_value_t = 20)]
    width: usize,

    /// Height of the maze
    #[clap(short, long, default_value_t = 10)]
    height: usize,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // fetch arguments
    let args = Args::parse();
    let w = args.width;
    let h = args.height;

    // generate grid
    let graph = {
        let mut graph: Graph<Cell<usize>, Cell<bool>> = Graph::new_undirected();

        for i in 0..(w * h) {
            graph.add_node(Cell::new(i));
        }

        for y in 0..h {
            for x in 1..w {
                let index = y * w + x;
                graph.set_edge(index, index - 1, Cell::new(false));
            }
        }

        for y in 1..h {
            for x in 0..w {
                let index = y * w + x;
                graph.set_edge(index, index - w, Cell::new(false));
            }
        }

        graph
    };

    // create queue for shuffled edges
    let edge_refs = {
        let mut rng = thread_rng();
        let mut edges = Vec::from_iter(graph.edges());
        edges.shuffle(&mut rng);

        edges
    };

    // modified Kruskal's algorithm
    edge_refs.iter().for_each(|e| {
        let a = e.a().value().get();
        let b = e.b().value().get();

        // if edge spans nodes of two different groups,
        // merge those groups and mark the edge
        if a != b {
            graph
                .nodes()
                .filter(|n| n.value().get() == b)
                .for_each(|n| n.value().set(a));
            e.value().set(true);
        }
    });

    // display result
    let ww = w * 2 + 1;
    let hh = h * 2 + 1;
    for y in 0..hh {
        for x in 0..ww {
            match (x, y) {
                // border left
                (0, _) => print!("█"),
                // border right
                (x, _) if x == ww - 1 => println!("█"),
                // border top
                (_, 0) => print!("█"),
                // border bottom
                (_, y) if y == hh - 1 => print!("█"),
                // border inner
                (x, y) if x % 2 == 0 && y % 2 == 0 => print!("█"),
                // nodes
                (x, y) if x % 2 == 1 && y % 2 == 1 => print!("░"),
                // vertical edges
                (x, y) if y % 2 == 0 => {
                    let a = (y / 2 - 1) * w + (x / 2);
                    let b = a + w;
                    let e = graph.edge(a, b).map(|x| x.value().get());
                    match e {
                        Some(false) => print!("█"),
                        Some(_) | None => print!("░"),
                    }
                }
                // horizontal edges
                (x, y) if x % 2 == 0 => {
                    let a = (y / 2) * w + (x / 2 - 1);
                    let b = a + 1;
                    let e = graph.edge(a, b).map(|x| x.value().get());
                    match e {
                        Some(false) => print!("█"),
                        Some(_) | None => print!("░"),
                    }
                }
                (_, _) => print!(" "),
            }
        }
    }

    Ok(())
}
