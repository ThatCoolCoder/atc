# ATC

Status: is playable, has minor gameplay inconsistencies with original, needs more levels.

Reimplementation of the [version by msharov](https://github.com/msharov/bsd-games), which is itself a reimplementation of the traditional game from BSD.

Written in rust, aims to have identical gameplay to the original, although the following changes have been made:
- Minor modificiations of UI.
- Lacks an advanced command preview and pre-submission validation; users see only what they type. This should not be an issue for serious players who I doubt are looking at the input box. 
- Fix how the wrong planes are bolded in the original
- Make it visible on certain dark terminal color schemes such as the one I use. (in the original the planes are exactly the same color as the background for me!)
- Visibility commands do not take effect until the next tick.
- Most of the scenarios have not been copied over from the original (more new ones are needed!)
- Airports can now be facing in diagonal directions, not only orthogonally.

I have not looked at the source code of any other versions except for getting the layout for `Default` level and getting specific values like low fuel threshold.

#### Adding new scenarios (levels)
To add a new scenario, you need to create a new rust file in `levels/`, reference it in `levels/mod.rs` and reference it in `main.rs`. You can figure out how to do this by looking at existing levels. Levels are defined in code, but it does not require much knowledge of rust. Convention is for exits to be numbered in clockwise order starting from top left. Airports and beacons are numbered from top to bottom. Numbering starts from 0 (not 1)