The sky is the source of light and rain. It looks similar to the air block's properties but mechanically they are very different.

## Properties

### Light Level
#LightLevel 

A 4-bit number. 

The light source changes intensity as the day passes simulating a day/night cycle. The rate of change of the light level along the day will be a bell curve. The weather caps the maximum light level possible. The curve will change in shape according to the seasons of the year.

### Humidity
#Humidity 

This is a 4-bit number that represents all possible states of weather.

If the value is 0, then it is a sunny day with a clear sky. Higher humidity creates bigger clouds that cap the maximum light level of the sky. Until a certain point where humidity will instead create precipitation. By that point, the clouds won't get bigger but raindrops will become more frequent and larger until it reaches a humidity hard maximum.

### Temperature
#Temperature

This is a 4-bit number that represents all temperatures. Having the lowest value would represent -10°C and the highest 45°C.

All sky blocks are meant to have the temperature set by weather conditions and season. Air blocks under the sky blocks are meant to absorb energy and increase their temperature. Then share the obtained energy with the blocks below and so on. This way the whole world gradually changes temperature locally with time