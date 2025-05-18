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
    } = Self {
        frame: 0,
        time_elapsed: 0.0,
        started: false,
        event_frequency: 50,

        eat_event_active: false,
        eat_event_start_time:0.0,
        eat_event_timer: 3.0,
        eat_event_time_left: 0.0,
    }
}

turbo::go!{
    let mut state = GameState::load();
    crate::println!("{:?} {}", {}, state.eat_event_time_left/state.eat_event_timer);
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
        if spawn_event_val == 2 && state.eat_event_active == false {
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
            }
            //When the event is active, make it show the icon
            circ!(
                d = 28.0 + (state.eat_event_time_left/state.eat_event_timer)*40.0,
                x = 25.0 + (state.eat_event_timer - state.eat_event_time_left) * 7.0,
                y = -6.0 + (state.eat_event_timer - state.eat_event_time_left) * 7.0,
                
                color = 0xadd8e6,
            );
            sprite!("button_eat_hotdog",
                x = 40,
                y = 10,
                scale = 0.3,
            );
            text!("EAT",
                x = 52,
                y = 3,
            );
            text!("W",
                x = 56,
                y = 45,
                font = "large",
            );
        }
        if state.eat_event_active == true && gamepad(0).up.pressed() {
            state.eat_event_active = false;
        }


    }
    state.frame += 1;
    state.time_elapsed = state.frame as f64 / 60.0;
    state.save();
}

