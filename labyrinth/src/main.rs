use clap::Parser;
use graph::Graph;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Width of the labyrinth
    #[clap(short, long, default_value_t = 10)]
    width: usize,

    /// Height of the labyrinth
    #[clap(short, long, default_value_t = 10)]
    height: usize,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let mut graph: Graph<usize, bool> = Graph::new_undirected();

    for i in 0..(args.width * args.height) {
        graph.add_node(i);
    }

    for y in 0..args.height {
        for x in 1..args.width {
            let index = y * args.width + x;
            graph.set_edge(index, index - 1, false);
        }
    }

    for y in 1..args.height {
        for x in 0..args.width {
            let index = y * args.width + x;
            graph.set_edge(index, index - args.width, false);
        }
    }

    println!("{:?}", graph);

    Ok(())
}
