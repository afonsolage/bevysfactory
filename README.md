  # Bevysfactory

An exploration, factorying, strategy and arpg made with [Bevy Engine](https://github.com/bevyengine/bevy).

# Overview

The main focus is to have a resource extraction/factory builder game, but which actually gives meaning to
the sources extracted. The main problem which I see with many factory games, at least to me, is that you
don't have any meaningfull use for what you have built, aside from the satisfaction, lore or some mission.

So I want to have a game loop which involves exploring to find resources, building to extract resources, craft
new weapons/gear/army units to defend, explore new areas, repeat.

Player will have a main character to explore and build in first/third person: There won't be any bird's eye view
or building camera by design, so the building placements and terrain overview matters.

Also Player will have to manage resources for his unit/army/itself, since exploring new areas will start to become 
increasingly difficult. The whole point of the game is to build a factory to feed and supply the army, in order to
face more difficult areas, which gives more and better resources.

## Art style

The game will have a low-poly 3D art style, a bit like Albion but more vibrant colors and a bit of toon-shader

## 

# Exploration

The world will be divided in instanced maps, for performance reasons and also to make it easy for players to reason
about where to go next, since each map will have it's own tier, resources and difficult level.

The resources on each map will be determined by the biome, enemies level and some randomness: Not every map will have
the same amount and type of resources.

## Resources
Players will look for tree kind of resources:
 - __Non-living organic__: Plants, trees, mushrooms, bushes and anything that is a non-living organic being;
 - __Living organic__: mammals, insects, reptiles, fishes...dragons?...maybe;
 - __Inorganic__: rock, iron, sand, mud, water, oil and anything that does't try to run or figth back;

More Resources will be added in the futuro to icnrease gameplay variety.

## Resource quality
Each resource will have a quality indicator, which determines how good that resource is for processing. Resource
quality will be calculated differently for each resource type:
 - __Non-living organic__: Based on map tier + a random factor which will be calculated at extraction, so on the same
map, two threes can yeild slightly different quality.
 - __Living organic__: Will be based on entity level, which is determined using other formula, based on Battle mechanic
 - __Inorganic__: Based on map tier + a random factor for each resource node, so on the same map, two iron nodes may
have slightly different quality, but each node will always yield the same quality for each resource extracted.

## POI
When discoverying a new map, it may contain different POIs, which offers additional features to the map. Here 
are some examples:
 - __Bandit camp__: Usually protecting some resource node or blocking some road.
 - __Den__: Contains some living organic boss which yields high quality resource once killed.
 - __Ruins__: May have some unique/high quality loot or just a trap.
 - __Settlements/Cities__: Some NPC controlled city/settlement which allows player to trade with.

More POIs will be added in the future to increase gameplay variety.

## Maps

Each map will have a fixed size, with procerually generated content, and will be conected to other maps. Not every
map corner will have a connection to a neighbor map, giving players a maze-like challenging if they wanna reach other
maps, but the algorithm must ensure every map is reacheable somehow.

There won't be any form of terraforming, to simply things.

The first initial map will be at minimum tier and the futher player moves from the center, more likely it is to a high
tier map is generated.

# Factorying

>Since I'm not english native speaker, as you can tell, I feel no shame in creating new therminology.
>Factorying refers to the factory building mechanic 

Building a factory needs to be one of the strong points of this game, so some elements needs to be exists in order to
make building a factory fun and enjoyable:
 - __No refound cost__: Destroying a player-owned building must refound 100% of resources, so players won't have any
kind of fear while trying new things;
 - __OCD friendly placement__: Most factory players have some degree of OCD and like things placed at right corner, 
well aligned and with clicking sound when everything fits together, so having tools to help to align things is a must.
Those tools must be optional, since also there are the chaotic-organization players;
 - __Clipping is ok__: The game should allow players to do clipping (non-blocking collisions), but alert about
the clipping when placing objects;
 - __Belts must show items__: Players must be able to see their items flowing from one place to another, since this
gives a great sense of proudness and also helps to mitigate factory problems. This may impose some performance
problems, but nothing that Bevy + Rust shouldn't handle.
 - __3D Factory__: The game should allow player to build their factories in all 3 dimensions, allowing them to
interact with montains, cliffs, hills and many more obstacles.
 - __Optional floor, walls and roofs__: This may seems obvios, but there should be optional floors, walls and roofs. 
The game should give no bonus or penalities in using them, this should be visually appealing only.

### Extractos

### Transporters

### Processors

### Storages

### Misc

# PvEing

### Melee combat

### Ranged combat

### Support

### AI

### Looting

### RTS style or Companion-like?