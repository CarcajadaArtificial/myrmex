Air blocks are meant to be a simple unit in the environment. Any other thing may occupy the space of an air block. Also, air blocks disperse sunlight among themselves. 

## Properties

### Light Level
#LightLevel 

A 4-bit number value. 

The way I imagine it is as follows: The world's ceiling will be the `sky`. Any air blocks that are vertically in direct connection with the sky will have the light level topped. Any other air block will look for its neighboring air blocks and have a step lower from the one with the highest light level. Finally, any air block without any direct access enclosed without any access to sunlight will have the lowest light level value.

### Temperature
#Temperature

This is a 4-bit number that represents all temperatures. Having the lowest value would represent -10°C and the highest 45°C.

The design of the temperature mechanic es meant to emulate convection. In the real world, particles transfer heat upwards and create motion in loops. I'm going to take inspiration from the upwards bias in nature but ignore the created motion. If a block of air is hotter than the one on top of it, it can transfer up to 3 units of temperature upwards. If it is hotter than a block on the side, it can transfer up to 2 units. And if it can't transfer temperature to the top or the sides, it can transfer it to the block below up to 1 unit at a time. 

