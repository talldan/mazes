# Mazes

This is a learning project for the [Rust language](https://www.rust-lang.org/) and the [Bevy game engine](https://bevyengine.org/).

It implements four of the maze algorithms from the ['Mazes for Programmers' book by James Buck](http://www.mazesforprogrammers.com/), which are all very simple. Those are:
- [Binary Tree](https://weblog.jamisbuck.org/2011/2/1/maze-generation-binary-tree-algorithm)
- [Sidewinder](https://weblog.jamisbuck.org/2011/2/3/maze-generation-sidewinder-algorithm)
- [Aldous Broder](https://weblog.jamisbuck.org/2011/1/17/maze-generation-aldous-broder-algorithm)
- [Wilson](https://weblog.jamisbuck.org/2011/1/20/maze-generation-wilson-s-algorithm)

That book is originally written in Python, so this is a 'port' of sorts, but the code is different to what the book presents. I've avoided some patterns from the book, like maintaining a list of cells that store references to neighbour cells, as my understanding is that would require reference counting in Rust (`Rc`, or possibly even `Arc` given Bevy systems run in parallel, see https://doc.rust-lang.org/book/ch15-04-rc.html for more info).

## Try the demo

The demo is viewable here - https://talldan.github.io/mazes/.

By default this shows a maze generated using the 'Aldous Broder' algorithm on a 15x15 grid, and the random number generator is seeded using the integer `0`, so when the first maze will always be the same.

The 'Go' and 'End' are the start and end points for the maze, and these will always be as far apart as possible.

There are three controls you can use to the change the maze:
- 'Rando-maze' - this randomizes the random number generator seed, and a random maze will be generated.
- 'Show solution' - this toggles on an overlay that shows the solution for the maze:
    - The numbers in each cell represent the distance to the cell from the start.
    - The red numbers indicate the path from the start to the finish.
- 'Maze type' - this dropdown allows changing maze algorithm.

## Future improvements

I'm not sure whether I'll devote more time to this, but here's some possible future improvements:

- Reduce the build size.
- Add more algorithms from the book.
- Add some unit tests.
- Make the 'Maze type' dropdown close on a click outside.
- Make the maze playable.
- Use HTML for the controls - Bevy's UI primitives are quite basic.
- Add controls for:
    - Changing the maze dimensions.
    - Typing in a specific seed for the random number generator.
