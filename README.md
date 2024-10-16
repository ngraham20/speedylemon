# SpeedyLemon
**speedy** /ˈspēdē/ : moving quickly  
**lemon [informal]** /ˈlemən/ : a person or thing, especially an automobile, regarded as unsatisfactory, dissappointing, or feeble

Designed off of https://github.com/killer415tv/gw2_speedometer as a base

# Why the re-write?
Originally, this wasn't actually intended to be a speedometer! I wanted to write a LiveSplit-like system that could track checkpoints individually and inform me which checkpoints I was having trouble with. I tried adding that functionality to Killer's original speedometer, but I've always enjoyed re-inventing the wheel, so here we are! The original speedometer written in python and is fairly complex. It could use a bit of a tune-up and code re-organization so that new features can more easily be added. Time for Rust!

# Some limitations
- SpeedyLemon cannot render an overlay for ghosts or checkpoints (Linux more or less prohibits this functionality on a core level)
- I haven't implemented a Graphical User Interface (GUI) yet, but it is planned
- I don't intend on implementing multiplayer like the original does

# Features
## Run SpeedyLemon on Linux
Unlike the original, SpeedyLemon does run on Linux! I haven't figured out how to get the original to run on Linux, but if you figure it out, let me know, and I'll happily update that info here!

## Display racer speed
The racer's speed is displayed for you to understand how your racing is actually affecting your speed. Did you hit a bump and it felt like you lost some speed? Maybe you did! Now you'll know for sure.

## Track checkpoint times and splits
![Infernal Leap](assets/infernal-leap.png)

The split times are the original reason I decided to take on this project. It's extremely helpful to know how long each individual checkpoint took you to complete, because maybe that 00:02 seconds that you're trying to make up are all coming from one checkpoint, and you don't have to perfect the whole track yet!

The timer also functions fairly well for Jumping Puzzles!

![Clock Tower](assets/clock-tower.png)

## Select Cups and Tracks from beetlerank
![cup selector](assets/cup-selector.png)

This idea comes straight from [LiveSplit](http://livesplit.org/), a tool that **speedrunners** use to keep track of individual segments of a run and not just the final time.

## Record and upload racer personal best lap times to https://beetlerank.com
What kind of tracker would this be if you couldn't tell if you beat your best time? When you achieve a new best time, that time is uploaded straight to **beetlerank** to immortalize your achievement for the world to see!

Beetlerank keeps track of a **huge** amount of data, thanks to the work of the original speedometer's creator, [killer514tv](https://github.com/killer415tv0). Seriously, go donate a coffee to that guy, his work's incredible.

## Custom Track Creator
![track creator](assets/track-creator.png)

# How does this even work?
## Keeping track of checkpoints
The **checkpoints** that the speedometer keeps track of are not tied to the ones that exist in Guild Wars 2. Instead, the speedometer keeps track of its own checkpoints and simply tests whether the player's position is near the next checkpoint. This means that not only can we keep track of lap times for ArenaNet's in-game races, but it means we can make our own and still record lap times!

## Mumble interface
The Mumble Link API is typically designed to allow proximity-chat for use in [Mumble](https://www.mumble.info/).
This data includes things like the camera and player positions, which we are using to calculate the player's speed instead. 

# Will this get me banned from Guild Wars 2?
Nope! This is **not a mod**! The Mumble API is information freely given by Guild Wars 2, and it is designed to be used in external programs, so no need to fret!
