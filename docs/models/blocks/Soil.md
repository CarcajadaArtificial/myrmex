Soil is what's going to make up most of the environment. It can be able to be moved around by creatures and usually contains the nutrients that plants need to grow.

## Properties

### Humidity
#Humidity 

A 4-bit number that represents all states of how damp can a soil block be.

If a drop of water lands in a block of soil, it will begin to absorb it. But if another drop lands on it while it's still absorbing the previous one, the drop will create a small water block on top of the soil one. After finishing absorbing it, if the drop of water remains on top it will be absorbed. And that pattern continues until the soil block has its humidity topped. The plan is to have a hole in the soil and slowly fill it with water. The highest layer of soil blocks will eventually be fully humid so they can hold the body of water blocks that would fill the hole. Soil blocks will have a humidity-spreading rate liked to their proximity to the closest body of water.

Humid soil blocks should also dry out their humidity gradually. This is a tricky characteristic as well. Firstly, soil blocks with direct contact with sunlight must dry faster than underground humidity. If a soil block became dries while being close to a water block, then it must give away some of the water to it. Additionally, soil blocks could dry out at a rate linked to the light level of the air block above it.

If a humid block has underneath an air block, it will create a falling drop of water that will fall until the cycle repeats itself. This way bodies of water can develop underground increasing the land's humidity.

### Fertility
#Fertility

Three 4-bit digit number values, one for every element.

In order to keep it simple the fertility will be limited to the common NPK trio. Every nutrient will have a different purpose in the development of a plant. It is the plant's responsibility to evolve to require more or fewer amounts of certain nutrients. Like everything else, plant nutrition and soil fertility are extremely generalized, and the role of each nutrient is as follows:

**Nitrogen** - Is in charge of growing leaves and stems. The larger the plant the higher amounts of nitrogen it requires to be healthy. If there is a deficiency in nitrogen the plant will grow smaller and have increasingly unpigmented leaves until they fall.

**Phosphorous** - Is in charge of growing the plant's "product". It makes thicker roots, flowers might appear, and even fruit might come out. The higher the usage of phosphorous the more likely it is for this to happen. Deficiency will create darker leaves.

**Potassium** - Is in charge of making resistant plants. It improves resistance to temperatures, creates harder and tougher bodies, and increases drought resistance. Deficiency will make brown borders in the leaves.