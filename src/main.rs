use rppal::gpio::Gpio;
use std::thread;
use log::*;
use std::time::{Duration, Instant};

const CHUTE_DEPLOY_ALT: f32 = (1000.0 / 3.28);
const BALLOON_RELEASE_ALT: f32 = (1500.0 / 3.28);
const BALLOON_POP_ALT: f32 = (1800.0 / 3.28);
const ROCKET_ALT: f32 = (2000.0 / 3.28);

fn main() {
    /* State bit one: current state. 
    Bit two: Has rocket fired? 
    Bit three: Has balloon popped? 
    Bit four: Has balloon been released? 
    Five: Has chute deployed?
    */
    let mut state: (u8, bool, bool, bool, bool) = (0, false, false, false, false);
    let mut barometer_check: u8 = 0;
    let mut altitudes_times: (f32, f32, Instant, Instant) = (0.0, 0.0, Instant::now(), Instant::now());
    while true{
        match state{
            (1, true, true, true, false) => deploy_chute(state),
            (2, true, true, false, false) => release_balloon(state),
            (3, true, false, false, false) => pop_balloon(state),
            (4, false, false, false, false) => launch_rocket(state),
            _ => check(state, barometer_check, &mut altitudes_times)
        };
    }
}

fn check(state: (u8, bool, bool, bool, bool), mut barometer_check: u8, altitudes: &mut (f32, f32, Instant, Instant)) -> (u8, bool, bool, bool, bool){

    let mut altitude = altitudes.1;
    if (barometer_check >= 255){
        barometer_check = 0;
    } else {
        barometer_check += 1;
    }
    if (barometer_check == 0) { 
        altitudes.2 = altitudes.3;
        altitudes.3 = Instant::now();

        altitudes.0 = altitudes.1;
        altitude = checkAltitude();
        altitudes.1 = altitude;
        info!("Current read altitude: {altitude} metres.");
    }

    let flags: (bool, bool, bool, bool) = checkFlags();

    if (altitude >= ROCKET_ALT && state.0 == 0 || flags.0) {
        return (4, state.1, state.2, state.3, state.4);
    } else if (altitude <= BALLOON_POP_ALT || flags.1){
        return (3, state.1, state.2, state.3, state.4);
    } else if (altitude <= BALLOON_RELEASE_ALT || flags.2){
        return (2, state.1, state.2, state.3, state.4);
    } else if (altitude <= CHUTE_DEPLOY_ALT || flags.3){
        return (1, state.1, state.2, state.3, state.4);
    }
    
    return state;
}

// What pins go to what things:
// Chute Deploy: GP10
// Balloon Release: GP11
// Balloon Pop: GP12
// Rocket Deploy: GP13

const CHUTE_PIN: u8 = 10;
const RELEASE_PIN: u8 = 11;
const POP_PIN: u8 = 12;
const ROCKET_PIN: u8 = 13;

fn deploy_chute(state: (u8, bool, bool, bool, bool)) -> (u8, bool, bool, bool, bool){
    info!("Deploying chute!");
    thread::spawn(|| {
        let mut pin = Gpio::new()?.get(CHUTE_PIN)?.into_output();
        // This time value may need to change in future.
        pin.set_high();
        thread::sleep(Duration::from_millis(500));
        pin.set_low();
    });
    return (0, state.1, state.2, state.3, true);
}

fn release_balloon(state: (u8, bool, bool, bool, bool)) -> (u8, bool, bool, bool, bool){
    info!("Releasing balloon!");
    thread::spawn(|| {
        let mut pin = Gpio::new()?.get(RELEASE_PIN)?.into_output();
        // This time value may need to change in future.
        pin.set_high();
        thread::sleep(Duration::from_millis(500));
        pin.set_low();
    });
    return (0, state.1, state.2, true, state.4);
}

fn pop_balloon(state: (u8, bool, bool, bool, bool)) -> (u8, bool, bool, bool, bool){
    info!("Popping balloon!");
    thread::spawn(|| {
        let mut pin = Gpio::new()?.get(POP_PIN)?.into_output();
        // This time value may need to change in future.
        pin.set_high();
        thread::sleep(Duration::from_millis(500));
        pin.set_low();
    });
    return (0, state.1, true, state.3, state.4);
}

fn launch_rocket(state: (u8, bool, bool, bool, bool)) -> (u8, bool, bool, bool, bool){
    info!("Launching rocket!");
    thread::spawn(|| {
        let mut pin = Gpio::new()?.get(ROCKET_PIN)?.into_output();
        // This time value may need to change in future.
        pin.set_high();
        thread::sleep(Duration::from_millis(500));
        pin.set_low();
    });
    return (0, true, state.2, state.3, state.4);
}

fn checkAltitude() -> f32 { return 0.0; }
fn checkFlags() -> (bool, bool, bool, bool) { return (false, false, false, false) }

fn getAltitudeDelta(altitudes: (f32, f32, Instant, Instant)) -> f32{
    let time = (altitudes.3.duration_since(altitudes.2).as_millis() as f64) / 1000000.0;
    return (altitudes.1 - altitudes.0) / (time as f32);
}