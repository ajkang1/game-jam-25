// This is where your main game loop code goes
// The stuff in this block will run ~60x per sec
use turbo::prelude::*;
turbo::config!{
    display.resolution = [512,256]
}
turbo::go!{
    let mut state = GameState::load();
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
        if spawn_event_val == 2 && state.study_event_active == false {
            state.study_event_active = true;
        }
        if state.study_event_active == true {
            //When the event is active, make it show the icon
            sprite!("button_eat_hotdog",
                x = 40,
                y = 10,
                scale = 0.3,
            );
        }
        if state.study_event_active == true && gamepad(0).up.pressed() {
            state.study_event_active = false;
            
        }


    }
    state.frame += 1;
    state.save();
}

turbo::init! {
    struct GameState {
        frame: u32,
        started: bool,
        event_frequency: u32,
        study_event_active: bool,
    } = Self {
        frame: 0,
        started: false,
        event_frequency: 50,
        study_event_active: false,
    }
}

