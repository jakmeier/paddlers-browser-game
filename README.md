# Paddlers

An MMORTS game (massively multiplayer online real-time strategy) with a peaceful-looking theme around ducks and religions.

Real-time in this context means that the clock is always ticking and that it matters *when* players do something. (As opposed to round-based games.)
However, Paddlers is nothing like Starcraft or similar RTS games which require good players to perform several actions per second.

## Demo

A [live-demo](http://demo.paddlers.ch) of the current state is online now! However, it's only semi-automated deployment at this point, therefore the deployed version may diverge from the master branch by a couple of days.


## Technology

The main developing language across the board is Rust and the frontend compiles to [WebAssmebly](https://webassembly.org/) 


##  Roadmap

The project is in an early stage and the team currently consists of a single person spending some hobby-time into this project.

If everything goes to plan, an alpha version will go **live on the 26th of December** with a first test-run to receive feedback from players (alpha-testers).

## Blog

Some of the game development process is documented in this blog series:

- [Gamedev #0: Creating a new universe](https://www.jakobmeier.ch/blogging/Paddlers_0.html)
- [Gamedev #1: Game mechanics](https://www.jakobmeier.ch/blogging/Paddlers_1.html)


## Compiling and running the project 

To build the project, you can use docker. 
If everything is set up properly (by me), a simple `make` in the root directory of the repository should give you a locally running game server hosted at `localhost:80`.
It will take some time for the first compilation (~30 minutes) but later builds should be much faster.

Local builds without docker are also possible using a recent nightly rust and [cargo web](https://github.com/koute/cargo-web) for the frontend.

## Contributing
If you are interested in joining the project (for fun or learning purposes, I'd guess) then feel free to contact me at jakob@paddlers.ch! :)

Even if you are not a developer, there are also plenty of other interesting tasks to do (game-design, story telling, art, balancing, translations, ... you name it!) and more contributors = more content.

If you have programming experience, just not with rust, you are very welcome as well. Hobby projects are the way to learn new skills!

## License
[MIT / Apache-2.0](https://github.com/jakmeier/paddlers-browser-game/blob/master/LICENSE.md)