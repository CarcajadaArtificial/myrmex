Air blocks are meant to be a simple unit in the environment. Any other thing may occupy the space of an air block. Also, air blocks disperse sunlight among themselves. 

## Properties

### Light Level
#LightLevel 

A 4-bit number value. 

The way I imagine it is as follows: The world's ceiling will be the `sky`. Any air blocks that are vertically in direct connection with the sky will have the light level topped. Any other air block will look for its neighboring air blocks and have a step lower from the one with the highest light level. Finally, any air block without any direct access enclosed without any access to sunlight will have the lowest light level value.

### Temperature
#Temperature

