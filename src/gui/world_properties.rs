struct ConfigProperties {
    /// The number of ticks that have happended since the beginning of time.
    ticks_since_start: i128,
    // weather_light_level: i4,
    // weather_temperature: i4,
    // weather_humidty: i4
    /// - [true] The world loops to the next tick,
    /// - [false] The next tick will run as soon as it changes to play.
    play: bool,
    /// The speed time inside the world.
    /// - [0] Ignore tick speed and run processes as soon as the previous one finished.
    /// - [0<x<1] This values are going to be ignored and count as 0.
    /// - [1] The default value, means the world runs at a speed of one tick per second.
    /// - [1<x] This values determines the amount of seconds a tick lasts.
    tick_speed: f32,
    /// How long it took for the last tick to run in _ms_.
    usage_last_tick: f32,
    /// Filter for the air block's light level property.
    filter_air_light_level: bool,
}
