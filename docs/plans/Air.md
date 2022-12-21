Air blocks are meant to be a simple unit in the environment. Any other thing may occupy the space of an air block. Also, air blocks disperse sunlight among themselves. 

## Properties

### Light
#Light

A small number with a 4 or 5-bit number value. 

The way I imagine it is as follows: The world's ceiling will be the `sky`. Any air blocks that are vertically in direct connection with the sky will have the light level topped. Any other air block will look for its neighboring air blocks and have a step lower from the one with the highest light level. Finally, any air block without any direct access enclosed without any access to sunlight will have the lowest light level value.

### Humidity 
#Humidity 

I don't know how large of a value to use for humidity. There is a direct relationship between the size of the value and how often it evaporates. If I use a small value like 2 or 3 bits, it should evaporate less often than using an 8-bit number. The tradeoff is between a more natural but heavy humidity system or a light and janky one.

As a note, having an air block with the highest humidity will not turn it into a water block. It means the air is at its moistest and dampest. It is more analogous to fog than anything else.

### Temperature
#Temperature

