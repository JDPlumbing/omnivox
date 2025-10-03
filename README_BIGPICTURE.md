This doc is for chatgpt context-building moving forward.

This repo "omnivox" is the rust backend simulation framework we have been building that incorporates all of the modules we hav built to handle each major aspect of the simulation logic.

BIG PICTURE GOAL: think, GitHub for physical reality via a digital twin. User enters known values about whatever they are simulating, we handle creating a simulated "digital twin" where, realistic systems are run over time and within their spatial context to determine failure rates, estimated lifespans, store locations and diffs (think remodel, property mapping, bom generation etc..).

the fine details.

this is an ecs concept. the only difference between this and any other simulated world is that we are trying as hard as we can to mimic reality close enough to be useful. 

Majorly important points.

This is not a cartesian space simulation. we spent the better part of a month developing UvoxID - which is 4 sets of 64 bit integers