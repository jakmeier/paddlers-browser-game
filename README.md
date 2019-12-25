![Image: Paddlers logo](./paddlers-frontend/art/logo_text.png)

## Game description

In Paddlers, a player is in control of ducks living in Paddland.
The goal is to make all ducks happy and live in harmony.
Starting with a single follower, that task is not very difficult.
But soon, there are visitors coming to the player's village and they are in a particularly bad mood and need someone to cheer them up.

In order to keep all visitors satisfied, more and more resources are required, hence the player will need to build up the infrastructure in the village. But as the quality of life inside of the village grows, the deeper is the depression of visitors. The only way to make everyone happy seems to be to unite all of Paddland in a single community. But will players cooperate with each other? Or will they fight for the leadership in this community?

## Game characteristics

Paddlers can be categorized as an MMORTS browser game (massively multiplayer online real-time strategy) with a peaceful-looking theme around ducks and religions.

Real-time in this context means that the clock is always ticking and that it matters *when* players do something. (As opposed to round-based games.)
However, Paddlers is nothing like Starcraft or similar RTS games which require good players to perform several actions per second.

## Status

The demo currently shows an alpha version which can be played by everyone by signing up. (No email verification required)
Many things are lacking, including:
 * Tutorials / explanations
 * User-friendly mobile control
 * Game balance
 * Many, many features

## Demo

A [live-demo](http://demo.paddlers.ch) of the current state is online now! However, it's only semi-automated deployment at this point, therefore the deployed version may diverge from the master branch by a couple of days.



##  Roadmap for 2020

At the core, Paddlers is a developer's playground.
It's existence is already justified by the fact that I had no clue about WebAssembly just 6 months ago and now I am understanding at least how it is used from Rust and JavaScript. But I still want to dig much deeper and also get my hands dirty writing some WebAssembly myself, without the high-level abstraction of Rust.

Probably, this will stay a long-term hobby project which I keep on working from time to time.
I am especially interested in optimizing and refactoring the frontend code of Paddlers.
This should give me a better understanding of the architectural trade-offs available in the browser when combining Rust, WASM and JS.
Maybe this would then lead to a couple of crates and/or contributions to existing projects, adding a bit to the quickly growing ecosystem.

There is no plan to make Paddlers commercial and therefore it is unlikely it will ever be a quality game or even have an active community of players.

I will try to make it complete to the extent that it is playable, however.
For me, this includes a well-rounded set of game features as well as technical stability.
But it does not include better graphics than the amateur drafts you can see right now.
And in terms of features, one should expect many pragmatic solutions, rather than perfect ones.

## Blog

Some of the game development process is documented in this blog series:

- [Gamedev #0: Creating a new universe](https://www.jakobmeier.ch/blogging/Paddlers_0.html) (Story introduction)
- [Gamedev #1: Game mechanics](https://www.jakobmeier.ch/blogging/Paddlers_1.html)
- [Gamedev #2: Multiplayer and procedural map generation](https://www.jakobmeier.ch/blogging/Paddlers_2.html)
- [Gamedev #3: Fun with Rust and distributed systems](https://www.jakobmeier.ch/blogging/Paddlers_3.html) (Technical overview)


## Compiling and running the project 

To build and run the complete project locally with all servers, you can use docker. 
If everything is set up properly (by me), a simple `make` in the root directory of the repository should give you a locally running game server hosted at `localhost:80`.
It will take some time for the first compilation (~30 minutes) but later builds should be much faster.

Local builds without docker are also possible using a recent nightly Rust and [cargo web](https://github.com/koute/cargo-web) for the frontend.

## Contributing
If you are interested in joining the project (for fun or learning purposes, I'd guess) then feel free to contact me at jakob@paddlers.ch! :)

Even if you are not a developer, there are also plenty of other interesting tasks to do (game-design, story telling, art, balancing, translations, ... you name it!) and more contributors = more content.

If you have programming experience, just not with Rust, you are very welcome as well. Hobby projects are the way to learn new skills!

## License
[MIT / Apache-2.0](https://github.com/jakmeier/paddlers-browser-game/blob/master/LICENSE.md)