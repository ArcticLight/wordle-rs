# Wordlebot: Wordle in Rust

This project is basically a working learning example for myself. I'm implementing it as a project to learn the Rust programming language, and it is published here (and publicly) in the hope that you may find it useful to look at.

At some point in the future, I may attempt a writeup of this learning experience in a set of blog posts explaining how I arrived at certain solutions, and the learning process I went through, in the hope that it helps out new learners of Rust.

> **Warning**
> THIS PROJECT IS NOT:
> - **Good Rust**: While I'm trying to make this project cleanly, I'm still learning, probably even right now. The code may still be weird.
> - **Intended for you to play**: This is a project that's supposed to be a toy problem to learn a programming language. It doesn't offer a great experience. Although you *can* play it with the CLI tool, you should probably play Wordle at [its official site at the NYTimes](https://www.nytimes.com/games/wordle/index.html) (you don't even need to pay for anything).

## The Checklist:

This is the basic outline of what I'm working through. I've ordered these ideas and tasks from easiest-to-hardest and I'll be working on them roughly in-order, to expose myself to more and more of the Rust language. In the beginning, these should be "very easy", as I'll be working through learning the basic language. As I get to some of the harder challenges, I'll be having to deal with bigger and bigger concepts, and maybe solving some interesting problems along the way.

### Stage 1: Basic Wordle game, playable in console

- [x] Be able to print to the console. _(Hello, World!)_
- [x] Represent the Wordle grid.
  - _as characters_?
  - _as strings_?
  - _as u8's_?
- [x] Print a representation of the Wordle grid. Use the word "Hello" as the answer.
- [x] Implement guess coloring (using the real Wordle rules).
- [x] Implement a basic game loop w/ input from the console.
- [x] Load the answer list from a file, and pick an answer word from it.
- [x] Embed the answer list in the binary, instead of loading it at runtime.
  - _as a string, then transformed into data at runtime_?
  - _statically as word rows?_
  - _how to determine the size of `[Row; ??]` at compile time_?
  - _embedded data as `[u8; xx]` using a build script._

### Stage 2: Graphs and fun data structures

- [ ] Build a simple "bot" that will try random Wordle guesses by itself.
- [ ] Using the everything-dictionary (not the answers list) make random guesses but filter on the information already gained from previous turn(s).
- [ ] [Information theory][3b1b]: Build the probability table for E[_I_] on all first-answer guesses.
  - _Can this be done in a reasonable time as a table for **all** guesses and guess patterns..?_
  - _How much memory does the mega-table take up?_
- [ ] [Information theory][3b1b]: Use E[_I_] to make guesses instead of random guesses.
- [ ] Probability: _Sample_ guesses instead, using the space of E[_I_] as the word distribution, so guesses can still be random-ish
    > **Note**
    > The strategy of _sampling_ from E[_I_] is a strictly-worse Wordle player than the strategy of taking some top element from E[_I_].
    > However, I also think it's an interesting exercise (how to convert E[_I_] to some distribution suitable for sampling _from_?) and also a more "interesting" player than always-choosing-the-best-answer.
  - [ ] Animate the sampling-random player in the manner from [the 3b1b video][3b1b]
- [ ] ...

[3b1b]: https://www.3blue1brown.com/lessons/wordle
