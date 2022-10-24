# ATC

Status: is playable, has minor gameplay inconsistencies with original, needs more levels.

Reimplementation of the [version by msharov](https://github.com/msharov/bsd-games), which is itself a reimplementation of the traditional game from BSD.

Written in rust, aims to have identical gameplay to the original, although the following changes have been made:
- Minor modificiations of UI.
- Lacks an advanced command preview and pre-submission validation; users see only what they type. This should not be an issue for serious players who I doubt are looking at the input box. 
- Fix how the wrong planes are bolded in the original
- Make it visible on certain dark terminals color schemes such as the one I use. (in the original the planes are exactly the same color as the background for me!)
- Visibility commands do not take effect until the next tick.
- Most of the levels have not been copied over from the original (more new ones are needed!)
- Airports can now be facing northeast/northwest etc.


I have not looked at the source code of any other versions except for getting the layout for `Default` level and getting specific values like low fuel threshold.