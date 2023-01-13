//  __      __       _    _   ___                       _   _
//  \ \    / /__ _ _| |__| | | _ \_ _ ___ _ __  ___ _ _| |_(_)___ ___
//   \ \/\/ / _ \ '_| / _` | |  _/ '_/ _ \ '_ \/ -_) '_|  _| / -_|_-<
//    \_/\_/\___/_| |_\__,_| |_| |_| \___/ .__/\___|_|  \__|_\___/__/
//                                       |_|
//=====================================================================================================//
use super::window;
use chrono::prelude::Utc;

#[derive(Copy, Clone)]
pub struct WorldProperties {
    /// The number of ticks that have happended since the beginning of time.
    ticks_since_start: i128,
    // weather_light_level: i4,
    // weather_temperature: i4,
    // weather_humidty: i4
    /// - [true] The world loops to the next tick,
    /// - [false] The next tick will run as soon as it changes to play.
    play: bool,
    /// The speed time inside the world.
    /// - [x<0] Ignored and counts as 0.
    /// - [0] Ignore tick speed and run processes as soon as the previous one finished.
    /// - [0<x<1] Ignored and counts as 0.
    /// - [1] The default value, means the world runs at a speed of one tick per second.
    /// - [1<x] This values determines the amount of seconds a tick lasts.
    tick_speed: f32,
    /// How long it took for the last tick to run in _ms_.
    usage_last_tick: f32,
    /// Filter for the air block's light level property.
    filter_air_light_level: bool,
}

impl Default for WorldProperties {
    fn default() -> WorldProperties {
        WorldProperties {
            ticks_since_start: 0,
            play: false,
            tick_speed: 1.0,
            usage_last_tick: 0.0,
            filter_air_light_level: false,
        }
    }
}

//== == == == == == == == == == == == == == == == == == == == == == == == == == == == == == ==//
/// @todo - Implement window_is_open in this imp instead of the MyrmexGui imp.
impl WorldProperties {
    ///
    pub fn environment(self, ctx: &egui::Context, is_open: &mut bool) {
        window::render(ctx, "Environment", is_open, |ui| {
            ui.label(format!("{}", Utc::now().format("%a, %b %e - %I:%M:%S %P")));
            ui.label("x days passed");
            ui.separator();
            egui::Grid::new("Weather indicators")
                .num_columns(2)
                .show(ui, |ui| {
                    ui.label("Light");
                    ui.add(egui::ProgressBar::new(0.46).show_percentage());
                    ui.end_row();
                    ui.label("Temperature");
                    ui.add(egui::ProgressBar::new(0.8).text("45 °C"));
                    ui.end_row();
                    ui.label("Humidity");
                    ui.add(egui::ProgressBar::new(0.06).text("Mostly sunny"));
                });
        });
    }

    ///
    pub fn property_filter(self, ctx: &egui::Context, is_open: &mut bool) {
        window::render(ctx, "Property Filter", is_open, |ui| {
            ui.label("This is the property filter widget.");
        });
    }

    ///
    pub fn time_control(self, ctx: &egui::Context, is_open: &mut bool) {
        window::render(ctx, "Time Control", is_open, |ui| {
            ui.label("This is the time control widget.");
        });
    }

    ///
    pub fn usage_indicator(self, ctx: &egui::Context, is_open: &mut bool) {
        window::render(ctx, "Usage Indicator", is_open, |ui| {
            ui.label("This is the usage indicator widget.");
        });
    }
}
