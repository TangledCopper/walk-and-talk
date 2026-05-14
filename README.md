# Walk-And-Talk
## Purpose 
The purpose of this repository is to not only document my first impressions of Bevy as a software engineer, but also to address some observations and the ways I skirted around issues while using the engine. I have implemented a character controller with gamepad support as well as a level and basic menu system with some interactions (a basic button, two npcs you can speak to). 
I am open sourcing it under the MIT license in the name of having more than one set of eyes on the pattern as well as help/assist others as the other tutorials I have studied have done. 

https://github.com/user-attachments/assets/3fe2cfb0-036e-44c7-82cd-6dd97bb9e028


While studying these examples and attempting to make my own personal project I found that Bevy (at least in its non-appended to/non-forked form) had several configurations/preferences I wished to address:

## Inspiration (and brief overview of systems)

This is my first larger Rust project so any and all feedback on how to properly document this is welcome in PRs (or comments on Reddit). More on this in the roadmap section!

### 1. Inputs, gamepad systems, and other opinionated plugins
One of the first minor DRY issues I ran into while creating inputs and interactions for my character was the sheer amount of code that you would have to write to support a ton of gamepad inputs as well as keyboard and so on. Although you could add more systems, I found quite quickly you would write yourself into a hole. I'm sure if I looked around for crates, there was a well put together input system similar to this one, but I wanted to stay as close to the Bevy engine as possible while creating this template. While not ideal for all games (multiplayer, touch controls, and heavy menu manipulation would need a different system), I feel like I wrote a half decent input parser which utilizes messages to emit and capture the key bindings of the gamepad or keyboard. I have also split the menu and player actions so that you may filter for specific inputs and contexts. "PlayerActions" and "Menu Actions" are sorted by the `InputContext` enum which has the menu navigation and player/other actions sorted. 

See here that when you speak to the npc, you cannot move because your input context has changed from `Game` to `Menu`:

https://github.com/user-attachments/assets/ae05d161-5c91-4f54-8e88-6e4ccfead4a7


The other capability I added was input types. Input types in this context is referring to either the methods of taking in an input and outputting a specific datatype. Within [bidings.ron](./walk-and-talk/assets/bindings.ron) file, there is "JestPressed", "HeldTimed", as well as "Held" for directional movement, jumping, and the generic selection of objects.

See below as I jump at different heights by holding down the button for longer:

https://github.com/user-attachments/assets/05cb4061-8f61-40d7-a17e-72faa79b4f21



### 2. Scenes and Menus
Another thing I noticed was that Bevy gives you lots of freedom of exactly what you can spawn in as well as tagging specific entities. This is great as there are few rules as it is programmable, quite flexible, and fairly agnostic to the context in which you need to create objects in the scene. However, I found many times that I would want to create something but only within a specific level or scene and delete it later. This is resolved in Godot by the tree system and in Unity it's done with Scene files. Being opinionated made me feel I needed to also address the issue. My implementation keeps the ability to spawn however you like, but also adds a registry in which you can spawn and despawn entire levels with a state machine. See [id.rs](./walk-and-talk/src/scenes/id.rs) for more details. I also added the ability to run startup functions and switch the gamestate upon entering a level or menu.

See below me going from the main menu to the first level with the "new game" button:


https://github.com/user-attachments/assets/605ac975-06fc-49d1-9573-a43c3b61a112




### 3. Prefabs
When initally learning Unity and Godot, I found that one of the most powerful features were prefabs. While Bevy does have bundles,I found that I was creating lots of the same boilerplate in addition to wanting to keep all the menu components/plugins/messages within the same folder strucutre, I liked the idea of keeping the scenes and menu items in their own respective sections of the code (I have other components of my game that are not here). While doing this did create some boilerplate for me, it also enabled me to significantly reduce the amount of code per level and also create a much more ergonomic plugin experience than what Bevy by default offers. Overall, this is the least polished of my features but I feel as though someone may want to use it in the future beyond just myself (more likely in a more polished format). 


## Acknowledgements
While I did develop this solo (for now), I would like to acknolwedge some of the inspirations I used while working on this: 

1. the Impatient Programmer's Guide to Bevy and Rust which introduced me to how to use the engine in general
[Link here](https://aibodh.com/posts/Bevy-rust-game-development-chapter-1/)
2. Avian engine's examples (specifically the dynamic 3d character controller) - which helped largely with how to create the character controller
[Link here](https://docs.rs/avian3d/latest/avian3d/)


## AI Transparency Clause
I will admit, I did use *some* ai during the creation of this project. To be more specific, I used ai to often find a better way of doing something which I already had working, or simply an space to discuss how to do something with a specific pattern. For example, I didn't initally know how to use macros (learning rust) or know if using a macro would be the best choice for dynamically prefabbing. Both [scenes](./walk-and-talk/src/scenes/id.rs) and [menus](./walk-and-talk/src/menu/id.rs) were discussed with AI. Another spot where I used ai was learning about the prelude pattern in general but I implemented it myself (which is why some components are missing from preludes). I don't mind if your contributions contain LLM generated code, however, I would like to keep the codebase as free from low-effort/non-helpful comments as much as possible. I would also advise that you keep to the general design pattern or improve upon it. I found that often I would attempt to use AI and it would attach or essentially duct tape the solution directly onto my project as opposed to being much more elegant about the solution as a whole. 

I feel as though I made an honest effort to keep the code as tidy and expandable within reason while also having some of my own opinionated additions. The idea behind this repository as a whole was to first learn Bevy post-tutorial but also to significantly reduce the development time moving forward of my own project. Additionally, I chose to not keep almost any of the comments which the AI produced as they weren't really pointing out anything useful. I assure you that most of the code the ai wrote did not make the final cut or was significantly refactored.


## Contribution Guidelines (Tenants)
Here's some of the core tenants of what I believe makes this a strong contender for getting merged into the codebase (or what I would do ideally if I were to continue on any feature):

### 1. Strong Recognition of Design Pattern and Expandability
One of the strongest features that Bevy has above any other game engine is its modularity. This needs to stay as a default of this repository. Although you *may* use the buttons within the menu system I have included, I have also included the ability to use whatever GUI components you would like via the MenuLayout and keep the menu_nav code. That is why they are seperate. Additionally, the menu module and scene module are seperate, but also having users be able to pick out whatever they would like out of this system. So in soft terms, try to keep your contributions opinionated, but opt-in.

### 2. No Assets or Hard Crate Requirements Outside of Bevy Itself (for future compatibility)
One of the largest struggles I find with Bevy is that often I would find a nice crate which would solve my problem, but often would be outdated and lock in my implementation or technology into a specific version. If there is more than just me utilizing and contributing to this repository in the future, I may decide to continue updating it with the newer versions of Bevy. However, I may also fork or create subtrees to continue compatibility for not only 0.18, but 0.19 and so on.

I have a similar attitude towards tying my rendering and business logic together in examples. As much as it would help out the project to look better, I often find that adding extra code to call in assets often creates a wall between the code I would like to use and the MVP I am trying to create. I'm trying to keep the developer experience as easy as possible.

### 3. Implement Common Game Tropes
Although your game may not require a third person character controller, I believe I have made it fairly easy to swap out for a first person or sidescroller controller if push came to shove. However, I didn't include an inventory system or rollback netplay because I believe that these can change wildly based on the type of game you are making as well as could be difficult to decouple from the project if another user wanted to use just the input system. I'm not opposed to more prefabs, but ideally it would have to be something which is contained within lots of other games have. Such as hovering the mouse over the buttons for menuing, but also the ability to navigate via the controller and keyboard. This to me is a good compromise and ideally where I would want the project to move in terms of things that are implemented. 

### 4. Free for Everyone
This repository as well as any version I will continue to work on will never be closed source. You are more than welcome to steal this code and sell your own game, but ideally I hope this repository itself lives as an educational tool and a "what's next" after bevy's official examples.

## Roadmap

I more than likely will continue to work on this project into the future based strongly on if others would be interested. I have a personal game I am working on and part of the exercise of creating this... extended example was seeing how to not write myself into a hole while building it.

If others were to contribute, here's my wishlist:

### 1. Save System
I would love to see a save system in the game which would enable developers to easily and procedurally add specific components and read directly from the file. This one I feel like is next up on the list and could easily be done with a bit of serde. I just wanted to get this out to the public before I continued adding more features. Not only to gauge interest, but just see if what I had created so far was helpful or was done much better other places. In addition to this, the largest reason I didn't add even a very basic one was due to the fact that I wanted to add a save editor to it which would enable developers to hook in system settings to the options menu (which still is not implemented either). Not an entire gui based editor for their save file, but something which would hook in more nicely with scenes and menus to create well-packaged but expandabe debugger for larger games with lots of levels. 

### 2. More Menu Options
At the moment, the only prefab for menus is buttons. These are great however, I would like to expand into sliders, toggles, and other components. I'm aware these exist within Bevy and Egui but don't have good input bindings with menu_nav. In addition to this, possibly adding layers so that you can have multiplle or a breadcrumb like system for easier navigation/less hardcoded systems in place. 

### 3. Better Documentation
I would love for the code to not be the sole living handbook of how to document this project but as stated before. Ideally the documentation would cover things such as minimal examples like in the avian engine where you can snip out the parts you would like and include them in your own game. 

### 4. Creating a Framework that's Decoupled from the Example
The last and more than likely most important piece to this would be to create a usable crate which lives on a layer above Bevy. For the longevity of this project, it would need to be done in a way which would enable users to easily add their own pieces to each system and I simply don't have the endpoints set up to do so (mainly because I was building my game first and a framework second but boiled down what I found were the notes for the sake of example). This also ties into the tenant of being able to steal the code quickly and get moving with your project. Although, this does open pandora's box on what comes packaged with the code and what code can the user easily transfer over. I'll leave this piece open to interpretation for now and see what others think.


## Final Thoughts 
If you do decide to use any of this code, shoot me an email or DM and I would love to see how you use it in your game :). It keeps the conversation going into not only how to improve this example, but Bevy and Rust as a whole. 

