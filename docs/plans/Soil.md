Soil is what's going to make up most of the environment. It can be able to be moved around by creatures and usually contains the nutrients that plants need to grow.

## Properties

### Humidity
#Humidity 

This property has similar difficulties to the air block property. I'm uncertain as to how large of a value to use for humidity and the tradeoff is similar. The main difference with air humidity is that soil can hold water if it is completely humid. 

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

---

## Rocks

A rock is the simplest block, it's unbreakable and unaffected by gravity. Its main purpose is to hinder organisms and create more diverse arrangements. When encountering rocks, the ants are unable to lift them and must build around them. The only property of rocks is to which neighboring rock blocks it is connected. The uncertain part about rocks is that I don't know how to add gravity to them. 

What happens if ants dig under a rock and it becomes suspended in the air? Should there be rocks so small that can be lifted by one ant? Should multiple ants be able to lift a single rock like in Pikmin? 