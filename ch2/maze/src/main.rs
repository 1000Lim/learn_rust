use rand::Rng;

const MAP_N: usize = 25;

fn main() {
    // create a random generator.
    let mut rng = rand::thread_rng();

    // create a map.
    let mut maze = [[0; MAP_N]; MAP_N];

    // fill the wall of the map.
    for i in 0..MAP_N {
        maze[i][0] = 1; // left wall
        maze[i][MAP_N - 1] = 1; // right wall
        maze[0][i] = 1; // top wall
        maze[MAP_N - 1][i] = 1; // bottom wall
    }

    // create a random maze.
    for i in 2..MAP_N - 2 {
        for j in 2..MAP_N - 2 {
            // generates block each 2 steps.
            if i % 2 == 1 || j % 2 == 1 {
                continue;
            }

            // generates a block randomly.
            let r = rng.gen_range(0..=3);

            match r {
                0 => maze[i - 1][j] = 1, // up
                1 => maze[i + 1][j] = 1, // down
                2 => maze[i][j - 1] = 1, // left
                3 => maze[i][j + 1] = 1, // right
                _ => (),
            }
        }
    }

    // print the maze.
    let tiles = [" ", "■"];
    for i in 0..MAP_N {
        for j in 0..MAP_N {
            print!("{}", tiles[maze[i][j]]);
        }
        println!();
    }
}
