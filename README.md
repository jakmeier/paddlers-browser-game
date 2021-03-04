![Image: Paddlers logo](./paddlers-frontend/art/logo_text.png)

## Game description

In Paddlers, a player is in control of ducks living in Paddland.
The goal is to make all ducks happy and live in harmony.
Starting with a single follower, that task is not very difficult.
But soon, there are visitors coming to the player's village and they are in a particularly bad mood and need someone to cheer them up.

In order to keep all visitors satisfied, more and more resources are required, hence the player will need to build up the infrastructure in the village. But as the quality of life inside of the village grows, the deeper is the depression of visitors. The only way to make everyone happy seems to be to unite all of Paddland in a single community. But will players cooperate with each other? Or will they fight for the leadership in this community?

## Demo

A [live-demo](https://demo.paddlers.ch) of the current state is online now! Usually, the latest tagged commit is deployed. However, it's only semi-automated deployment at this point, therefore the deployed version may diverge from the master branch.

Currently deployed version: 0.2.1

If you experience problems with the live-demo, I would appreciate a short bug report that explains what you see and what kind of device and browser you are using. Firefox on a laptop with Ubuntu 20.04 is what I test the most but the goal is that it works on all (major) platforms.

## Game characteristics

Paddlers can be categorized as an MMORTS browser game (massively multiplayer online real-time strategy) with a peaceful-looking theme around ducks and religions.

It can be played on any device with a browser, although right now it is optimally played on a laptop or desktop PC.
Players create an account for themselves and then join global server where all players share a universe.
All changes made by the players are permanent and the account keeps going even while the player is not logged in.

Real-time (the RT in MMORTS) for Paddlers means that the clock is always ticking and that it matters *when* players do something. (As opposed to round-based games.)
However, Paddlers is nothing like Starcraft or similar RTS games which require good players to perform several actions per second.

## Status & Goal

The demo currently shows an alpha version which can be played by everyone who signs up. (No email verification required, fake addresses are okay. And please note that the chosen user name will be displayed publicly on a scoreboard.) When a new version is updated, the account usually are still saved but the player state will be reset.

The game itself sadly is not complete enough to provide a good experience.
But I firmly believe the game could be fun if the right kind of people put in enough effort to polish it. (I can try the best on my own but I a painfully aware of my lack of expertise in many topics around creating a game.)

At the core, Paddlers is a developer's playground.
I started this project in 2019 to learn more about WASM in my free time, building on my experience with Rust from many other projects before that.
Working on Paddlers has been good fun, I like the creative process it involves.
And it has proven to be a good platform to try many different things and learn more about numerous topics. 

Moving forward, I want to retain the freedom to try out new and experimental things but also start to iron out a few things.
It would be really awesome to draw some conclusions from the learning process, remove experiments that did not turn out well, and double-down on those which did.
Ideally, this would lead to some contributions also useful to other projects.

##  Roadmap for 2021
- Improve and separate [Paddle](https://github.com/jakmeier/paddle) the framework enough from Paddlers the game such that it could be useful to others.
- Make the game mobile friendly with support for as many platforms as possible
- ...

## Commercial aspiration?

I still see no commercial path for Paddlers.
Although, in the long run, I would love to see the game achieve a certain level of quality and an active player base.
If this is ever achieved, then I would need to find a solution to cover the server costs.

In this hypothetical scenario, I would prefer to avoid ads if at all possible and I definitely would stay away from pay-to-win micro transactions.
Maybe some kind of a premium account that makes certain things more convenient (without giving an actual advantage).
Or some pure cosmetics features that need to be purchased.

To be perfectly clear, I think it is rather unlikely to happen.
I just want to be open and clear about my strategy right from the beginning, so that interested people and potential contributors can understand possible scenarios for the project.

## Blog and Videos

Some of the game development process is documented in this blog series:

- [Gamedev #0: Creating a new universe](https://www.jakobmeier.ch/blogging/Paddlers_0.html) (Story introduction)
- [Gamedev #1: Game mechanics](https://www.jakobmeier.ch/blogging/Paddlers_1.html)
- [Gamedev #2: Multiplayer and procedural map generation](https://www.jakobmeier.ch/blogging/Paddlers_2.html)
- [Gamedev #3: Fun with Rust and distributed systems](https://www.jakobmeier.ch/blogging/Paddlers_3.html) (Technical overview)
- [Gamedev #4: Benefits of full-stack Rust](https://www.jakobmeier.ch/blogging/Paddlers_4.html)
- [Gamedev #5: Version 0.2 Released](https://www.jakobmeier.ch/blogging/Paddlers_5.html)
- [Gamedev #6: New features in 0.2.1](https://www.jakobmeier.ch/blogging/Paddlers_6.html)

There is now also a [Paddlers YouTube channel](https://www.youtube.com/channel/UCoSAx5MYeSP5wXvE4syN-LA) and a first [teaser](https://youtu.be/3Syw7hxQ-z0) that shows the game in action.

## Website

[Paddlers Website](https://paddlers.ch) with [News and full text changelog](https://paddlers.ch/news.html)

## Compiling and running the project 

To build and run the complete project locally with all servers, you can use docker. 
If everything is set up properly (by me), a simple `make` in the root directory of the repository should give you a locally running game server hosted at `localhost:80`.
It will take some time for the first compilation (~30 minutes) but later builds should be much faster.

Local builds without docker are also possible using a recent nightly Rust and [cargo web](https://github.com/koute/cargo-web) for the frontend.

## Contributing
If you are interested in joining the project (for fun or learning purposes, I'd guess) then feel free to contact me at jakob@paddlers.ch or comment on some issue you are interested in! :)

I use the github issues to keep track of long-term goals and also to give outsiders a hint on what I am working on.
However, most of my notes are still offline and I usually do not bother to create an issue before implementing a new feature.
Therefore, potential contributors are encouraged to just tell me what area they might be interested in and then we can have a discussion on possibilities.

Even if you are not a developer, there are also plenty of other interesting tasks to do (game-design, story telling, art, balancing, translations, ... you name it!) and more contributors = more content. Also, I want to add that any contributor can greatly influence the direction the game goes, as it is still only a prototype and it is moving forward with the slow pace of your typical hobby-project.

If you have programming experience, just not with Rust, you are very welcome as well. Hobby projects are the best way to learn new skills!

## License
[MIT / Apache-2.0](https://github.com/jakmeier/paddlers-browser-game/blob/master/LICENSE.md)
