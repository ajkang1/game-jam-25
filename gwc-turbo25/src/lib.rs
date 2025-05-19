// This is where your main game loop code goes
// The stuff in this block will run ~60x per sec
use turbo::prelude::*;
turbo::config!{
    display.resolution = [512,256]
}

turbo::init! {
    struct GameState {
        frame: u32,
        time_elapsed: f64,
        started: bool,
        event_frequency: u32,

        eat_event_active: bool,
        eat_event_start_time: f64,
        eat_event_timer: f64,
        eat_event_time_left: f64,
        eat_fail_anim_playing: bool,
        eat_fail_start_time: f64,
        eat_fail_anim_timer: f64,
        eat_fail_anim_time_left: f64,
        eat_success_anim_playing: bool,
        eat_success_start_time: f64,
        eat_success_anim_timer: f64,
        eat_success_anim_time_left: f64,
    } = Self {
        frame: 0,
        time_elapsed: 0.0,
        started: false,
        event_frequency: 150,

        eat_event_active: false,
        eat_event_start_time:0.0,
        eat_event_timer: 0.5, // time player is given to press the button
        eat_event_time_left: 0.0,
        eat_fail_anim_playing: false,
        eat_fail_start_time: 0.0,
        eat_fail_anim_timer: 1.0, // time it takes to play the eat failure animation
        eat_fail_anim_time_left: 0.0,
        eat_success_anim_playing: false,
        eat_success_start_time: 0.0,
        eat_success_anim_timer: 1.0, // time it takes to play the eat success animation
        eat_success_anim_time_left: 0.0,

    }
}

turbo::go!{
    let mut state = GameState::load();
    //crate::println!("{:?} {}", {}, state.eat_event_time_left/state.eat_event_timer);
    if !state.started {
        
        let pointer = pointer();
        let clicked = pointer.just_pressed();
        if clicked {
            state.started = true;
        }
        text_box!(
            "Start Game",
            font = "medium",
            scale = 1.4,
            width = 70,
            height = 20,
            align = "center",
            x = 95,
            y = 64,
        );

    }
    else {
        // game runstate
        sprite!("student_hit_sprite",
            x = 60,
            y = 0
        );
        let event_frequency = state.event_frequency;
        let spawn_event_val = rand() % event_frequency;

        //EAT EVENT
        if spawn_event_val == 1 && state.eat_event_active == false && state.eat_fail_anim_playing == false && state.eat_success_anim_playing == false {
            log!("event active");
            state.eat_event_active = true;
            state.eat_event_start_time = state.time_elapsed;
        }
        if state.eat_event_active == true {
            
            state.eat_event_time_left = state.eat_event_timer + state.eat_event_start_time - state.time_elapsed;
            //crate::println!("{:?} {}", {}, state.time_elapsed);
            //if the timer is up, make you lose health and make the icon disappear
            if state.eat_event_time_left <= 0.0 || state.eat_event_time_left > state.eat_event_start_time{
                state.eat_event_active = false;
                //TAKE DAMAGE
                state.eat_fail_anim_playing = true;
                state.eat_fail_start_time = state.time_elapsed;
            }
            //When the event is active, make it show the icon
            let transparency = (4.0 * (state.time_elapsed - state.eat_event_start_time)/state.eat_event_time_left).clamp(0.0,1.0);
            let alpha = (transparency * 255.0).round() as u32;

            circ!(
                d = 28.0 + (state.eat_event_time_left/state.eat_event_timer) * 40.0,
                x = 60.0 - (28.0 + (state.eat_event_time_left/state.eat_event_timer) * 40.0) / 2.0,
                y = 29.0 - (28.0 + (state.eat_event_time_left/state.eat_event_timer) * 40.0) / 2.0,
                
                color = (0xadd8e6 & 0xffffff00) | alpha,
            );
            sprite!("button_eat_hotdog",
                x = 40,
                y = 10,
                scale = 0.3,
                color = 0xffffff00 | (alpha * 2).clamp(0, 255),
            );
            text!("EAT",
                x = 52,
                y = 3,
                color = 0xffffff00 | (alpha * 2).clamp(0, 255),
            );
            text!("W",
                x = 56,
                y = 45,
                font = "large",
                color = 0xffffff00 | (alpha * 2).clamp(0, 255),
            );
        }
        if state.eat_event_active == true && gamepad(0).up.pressed() {
            state.eat_event_active = false;
            state.eat_success_anim_playing = true;
            state.eat_success_start_time = state.time_elapsed;
        }
        // spawn the green circle and fade-out when you get a success
        if state.eat_success_anim_playing == true {
            state.eat_success_anim_time_left = (state.eat_success_start_time + state.eat_success_anim_timer) - state.time_elapsed;

            // fade-out icons
            if state.eat_success_anim_time_left > 0.0 && state.eat_success_anim_time_left <= state.eat_success_anim_timer {
                let transparency = 1.0 - (2.0 * (state.time_elapsed - state.eat_success_start_time)/state.eat_success_anim_time_left).clamp(0.0,1.0);
                let alpha = (transparency * 255.0).round() as u32;
                let circtransparency = 1.0 - (0.15 * ((state.time_elapsed - state.eat_success_start_time)/state.eat_success_anim_time_left)).clamp(0.0,1.0);
                let circalpha = (circtransparency * 255.0).round() as u32;
                sprite!("button_eat_hotdog",
                x = 40,
                y = 10,
                scale = 0.3,
                color = 0xffffff00 | (alpha).clamp(0, 255),
            );
            text!("EAT",
                x = 52,
                y = 3,
                color = 0xffffff00 | (alpha).clamp(0, 255),
            );
            text!("W",
                x = 56,
                y = 45,
                font = "large",
                color = 0xffffff00 | (alpha).clamp(0, 255),
            );

            // grow the green circle
            if ((state.eat_success_anim_timer - state.eat_success_anim_time_left)/state.eat_success_anim_timer) * 180.0 < 60.0 {
                circ!(
                    d = (((state.eat_success_anim_timer - state.eat_success_anim_time_left)/state.eat_success_anim_timer) * 180.0).clamp(0.0, 60.0),
                    x = 60.0 - ((((state.eat_success_anim_timer - state.eat_success_anim_time_left)/state.eat_success_anim_timer) * 180.0).clamp(0.0, 60.0)) / 2.0,
                    y = 29.0 - ((((state.eat_success_anim_timer - state.eat_success_anim_time_left)/state.eat_success_anim_timer) * 180.0).clamp(0.0, 60.0)) / 2.0,
                    
                    color = (0x1bff0000 & 0xffffff00) | circalpha,
                );
            }
            else {
                circ!(
                    d = 60.0,
                    x = 30.0,
                    y = -1.0,
                    
                    color = (0x1bff0000 & 0xffffff00) | circalpha,
                );
            }
            
            }
            else {
                state.eat_success_anim_playing = false;
            }

        }

        // spawn the red circle and fade-out when you get a failure
        if state.eat_fail_anim_playing == true {
            state.eat_fail_anim_time_left = (state.eat_fail_start_time + state.eat_fail_anim_timer) - state.time_elapsed;
            
            // fade-out icons
            if state.eat_fail_anim_time_left > 0.0 && state.eat_fail_anim_time_left <= state.eat_fail_anim_timer {
                let transparency = 1.0 - (2.0 * (state.time_elapsed - state.eat_fail_start_time)/state.eat_fail_anim_time_left).clamp(0.0,1.0);
                let alpha = (transparency * 255.0).round() as u32;
                let circtransparency = 1.0 - (0.15 * ((state.time_elapsed - state.eat_fail_start_time)/state.eat_fail_anim_time_left)).clamp(0.0,1.0);
                let circalpha = (circtransparency * 255.0).round() as u32;
                sprite!("button_eat_hotdog",
                x = 40,
                y = 10,
                scale = 0.3,
                color = 0xffffff00 | (alpha).clamp(0, 255),
            );
            text!("EAT",
                x = 52,
                y = 3,
                color = 0xffffff00 | (alpha).clamp(0, 255),
            );
            text!("W",
                x = 56,
                y = 45,
                font = "large",
                color = 0xffffff00 | (alpha).clamp(0, 255),
            );

            // grow the red circle
            if ((state.eat_fail_anim_timer - state.eat_fail_anim_time_left)/state.eat_fail_anim_timer) * 180.0 < 60.0 {
                circ!(
                    d = (((state.eat_fail_anim_timer - state.eat_fail_anim_time_left)/state.eat_fail_anim_timer) * 180.0).clamp(0.0, 60.0),
                    x = 60.0 - ((((state.eat_fail_anim_timer - state.eat_fail_anim_time_left)/state.eat_fail_anim_timer) * 180.0).clamp(0.0, 60.0)) / 2.0,
                    y = 29.0 - ((((state.eat_fail_anim_timer - state.eat_fail_anim_time_left)/state.eat_fail_anim_timer) * 180.0).clamp(0.0, 60.0)) / 2.0,
                    
                    color = (0xff000000 & 0xffffff00) | circalpha,
                );
            }
            else {
                circ!(
                    d = 60.0,
                    x = 30.0,
                    y = -1.0,
                    
                    color = (0xff000000 & 0xffffff00) | circalpha,
                );
            }
            
            }
            else {
                state.eat_fail_anim_playing = false;
            }

        }


    }
    state.frame += 1;
    state.time_elapsed = state.frame as f64 / 60.0;
    state.save();
}

