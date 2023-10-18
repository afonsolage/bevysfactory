# Bevysfactory

An exploration, factorying, strategy and arpg made with [Bevy Engine](https://github.com/bevyengine/bevy).

# Overview

The main focus is to have a resource extraction/factory builder game, but which actually gives meaning to
the resources extracted. The main problem which I see with many factory games, at least for me, is that you
don't have any meaningful use for what you have built, aside from the satisfaction, lore or some mission.

So I want to have a game loop which involves exploring to find resources, building to extract resources, craft
new weapons/gear/army units to defend, explore new areas, repeat.

Player will have a main character to explore and build in first/third person: There won't be any bird's eye view
or building camera by design, so the building placements and terrain overview matters.

Also Player will have to manage resources for his unit/army/itself, since exploring new areas will start to become 
increasingly difficult. The whole point of the game is to build a factory to feed and supply the army, in order to
face more difficult areas, which gives more and better resources.

## Art style

The game will have a low-poly 3D art style, a bit like Albion but more vibrant colors and a bit of toon-shader

# Exploration

The world will be divided in instanced maps, for performance reasons and also to make it easy for players to reason
about where to go next, since each map will have it's own tier, resources and difficult level.

The resources on each map will be determined by the biome, enemies level and some randomness: Not every map will have
the same amount and type of resources, so finding the best balance between enemies and resources will be one of the 
selling points of exploration.

## Resources
Players will look for tree kind of resources:
 - __Non-living organic__: Plants, trees, mushrooms, bushes and anything that is a non-living organic being;
 - __Living organic__: Mammals, Insects, Reptiles, Fishes...dragons?...maybe;
 - __Inorganic__: rock, iron, sand, mud, water, oil and anything that does't try to run or fight back;

More Resources will be added in the future to increase game play variety.

## Resource quality
Each resource will have a quality indicator, which determines how good that resource is for processing. Resource
quality will be calculated differently for each resource type:
 - __Non-living organic__: Based on map tier + a random factor which will be calculated at extraction, so on the same
map, two threes can yield slightly different quality.
 - __Living organic__: Will be based on entity level, which is determined using other formula, based on Battle mechanic
 - __Inorganic__: Based on map tier + a random factor for each resource node, so on the same map, two iron nodes may
have slightly different quality, but each node will always yield the same quality for each resource extracted.

## POI
When discovering a new map, it may contain different POIs, which offers additional features to the map. Here 
are some examples:
 - __Bandit camp__: Usually protecting some resource node or blocking some road.
 - __Den/Nest__: Contains some living organic boss which yields high quality resource once killed.
 - __Ruins__: May have some unique/high quality loot or just a trap.
 - __Settlements/Cities__: Some NPC controlled city/settlement which allows player to trade with.

More POIs will be added in the future to increase game play variety.

## Maps

Each map will have a fixed size, with procedurally generated content, and will be connected to other maps. Not every
map corner will have a connection to a neighbor map, giving players a maze-like challenging if they wanna reach other
maps, but the algorithm must ensure every map is reachable somehow.

There won't be any form of terraforming, to simply things.

The first initial map will be at minimum tier and the further player moves from the center, more likely it is to a high
tier map is generated.

# Factorying

>Since I'm not native english speaker, as you can tell, I feel no shame in creating new terminology.
>Factorying refers to the factory building mechanic 

Building a factory needs to be one of the strong points of this game, so some elements needs to exists in order to
make building a factory fun and enjoyable:
 - __No refound cost__: Destroying a player-owned building must refound 100% of it's resources, so players won't have
any kind of fear while trying new things;
 - __OCD friendly placement__: Most factory players have some degree of OCD and like things placed at right corner, 
well aligned and with a clicking sound when everything fits together, so having tools to help to align things is a must.
Those tools must be optional, since also there are the chaotic-organization players;
 - __Clipping is ok__: The game should allow players to do clipping (non-blocking collisions), but alert about
the clipping when placing objects;
 - __Visible items flow__: Players must be able to see their items flowing from one place to another, since this
gives a great sense of proudness and also helps to mitigate factory problems. This may impose some performance
problems, but nothing that Bevy + Rust shouldn't handle.
 - __3D Factory__: The game should allow player to build their factories in all 3 dimensions, allowing them to
interact with mountains, cliffs, hills and many more obstacles.
 - __Optional floor, walls and roofs__: This may seems obvious, but there should be optional floors, walls and roofs. 
The game should give no bonus or penalities in using them, this should be visually appealing only.
 - __Instant Building__: There won't be any major cooldown or waiting time to create Factory Buildings, since the major
 limiting point will be the extractors, transporters and belts.

### Quality
Another important aspect of the Factory is how it's tied to resource quality, since each factory building will have also
a quality indicator, which will determine how fast it works or the storage capacity, for instance.

The quality indicator should have a higher threshold, so Players won't have to rebuild their factories whenever a new
resource now, which a slight better quality is discovered. Quality indicator should require significant better resources
but offer also significant better bonuses.

### Extractors
Extractors will generate resource when placed on a resource spot. This resource spot can be anything, depending on
extracted resource and map. 

- __Mining__: Will be able to extract iron, rocks, sulphur and most solid inorganic resources;
- __Water__: Will pump water from lakes and rivers;;
- __Farm Plot__: Will act like a farm plot, but automatically harvest crops and yield it's resources;
- __Farm Livestock__: Will act like a farm of livestock, which will yield it's resource automatically;

### Transporters
Mainly moves resources from one node to another. Transporters can work on different land types:

- __Horizontal Belts__: Will move resource horizontally, having at most 45º degree angle, using a conveyor belt;
- __Vertical Belts__: Will move resource horizontally, having at minimum 45º degree angle, using a conveyor belt;
- __Pipe Tube__: Will move liquid resources in any direction. May require __Pumper__ to enable liquid flow to higher
altitudes;

Other types of transporters may be added in the future.

### Controllers
Controls item flow between nodes (storages, transporters, processors and so on). The main idea is to give Players some
controllers to have a better automation and more intelligent automation. Most controllers are optional, but enables
more advanced control over the built factory.

- __Splitter__: Split incoming resources into many outputs;
- __Merger__: Merge many incoming resources into a single output;
- __Overflow Gate__: Redirect output to another output whenever there is overflow of resources;
- __Limiter__: Limits the amount of resources in a output, redirecting to another output when the limit is reached;

Other types of controllers will be added in the future.

### Processors
Accepts one or more resources as input and produces an output with optional by product. Processors are the main point
of the factory building, since they enable using the gathered resources and creating resources for others processors
or some item, food, equipment or anything to be used by player.

Processors have mainly four types, in order of complexity:

- __Single Input Single Output__: SISO, the most basic one and create basic products, like iron ingots or gravel;
- __Multiple Input Single Output__: MISO, combine multiple inputs to create a single one, like steel ingots or cement;
- __Single Input Multiple Output__: SIMO, breakdown the input into multiple outputs, like rock yielding both sand and 
gravel;
- __Multiple Input Multiple Output__: MIMO, create most advanced products, like weapons, rare resources and so on.

### Storages
Containers which will storage raw or processed resources. Every storage must have one input and one output, at least,
which will allow Players to create buffers around the factory.

### Misc
Additional components like floors, walls, roofs and lights.

# PvEing
The combat will be required in order to explore new maps and advance into the game, acquiring better resources and
building better quality factories, which in turn, will yield better combat items and gear.

Game combat won't be advanced like a Souls Game, but more a mid paced game like Valheim.

### Melee combat
Melee combat will be based on taking damage and hitting back, dodge or parry timing won't be required.

### Ranged combat
Ranged combat won't have auto aim or projectile path line, but will have a fixed cross aim and the drop will be low,
mostly based on weapon range, after the range is reached, the projectile drop will increase greatly.

### Support
There will be some form of support skills and items, like healing potions/bandages, food buffs and so on, so this will
enable players to prepare for boss nor harder battles.

### AI
The enemy and ally AI will be almost the same, having no variation on difficulty, the enemy/ally will have better gear,
stats and so on.

The AI shouldn't be nothing advanced and have some basic behaviors, like passive, defensive, aggressive, but also flee
from combat if it is heavily injured (but not always tho, to avoid Players running across the map to give the final hit)

### Looting
Enemies (and allies) when dead, will yield some loot. Some resources will be yielded only by looting, to give players
a reason to go out and explore, aside from finding better resources spots.

But there won't be any major game mechanic that will be only achievable through looting, only minor/mid things.


### Companions
Players will be able to summon/train companions, which will be AI-powered NPCs that have some basic commands to obey:
S
- __Follow__: Follows the player and will engage battle whenever player also engages.
- __Stay__: Will stay in position and defend itself, won't engage in battle unless attacked.
- __Patrol__: Guard an area path and will attack any hostile target that it founds.
- __Attack__: Attack a target and engage in battle. Will stop only if dead or no more hostile target nearby.


## v0.1.0 - Basic extractors

- [X] Create random resources spots on a flat map [#2](https://github.com/afonsolage/bevysfactory/issues/2);
- [ ] Enable extractor placement on resources spots [#3](https://github.com/afonsolage/bevysfactory/issues/3);
- [ ] Extract resources from resources spots using extractors [#4](https://github.com/afonsolage/bevysfactory/issues/4);
- [ ] Show UI with extracted resources [#5](https://github.com/afonsolage/bevysfactory/issues/5);
- [ ] Place a storage [#6](https://github.com/afonsolage/bevysfactory/issues/6);
- [ ] Place a transporter belt between extractor and storage [#7](https://github.com/afonsolage/bevysfactory/issues/7);
- [ ] Move resources from extractor to storage [#8](https://github.com/afonsolage/bevysfactory/issues/8);
- [ ] Show items moving while on transporters [#9](https://github.com/afonsolage/bevysfactory/issues/9);