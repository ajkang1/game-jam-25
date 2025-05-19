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

        score: u32,
        health: f32,
        misinput_debounce: bool,
        debounce_start_time: f64,
        is_hit: bool,
        is_game_over: bool,
        is_played: bool, // <-- damage sound check
        is_played_point: bool, // <-- point sound check

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

        sleep_event_active: bool,
        sleep_event_start_time: f64,
        sleep_event_timer: f64,
        sleep_event_time_left: f64,
        sleep_fail_anim_playing: bool,
        sleep_fail_start_time: f64,
        sleep_fail_anim_timer: f64,
        sleep_fail_anim_time_left: f64,
        sleep_success_anim_playing: bool,
        sleep_success_start_time: f64,
        sleep_success_anim_timer: f64,
        sleep_success_anim_time_left: f64,

        water_event_active: bool,
        water_event_start_time: f64,
        water_event_timer: f64,
        water_event_time_left: f64,
        water_fail_anim_playing: bool,
        water_fail_start_time: f64,
        water_fail_anim_timer: f64,
        water_fail_anim_time_left: f64,
        water_success_anim_playing: bool,
        water_success_start_time: f64,
        water_success_anim_timer: f64,
        water_success_anim_time_left: f64,

        grass_event_active: bool,
        grass_event_start_time: f64,
        grass_event_timer: f64,
        grass_event_time_left: f64,
        grass_fail_anim_playing: bool,
        grass_fail_start_time: f64,
        grass_fail_anim_timer: f64,
        grass_fail_anim_time_left: f64,
        grass_success_anim_playing: bool,
        grass_success_start_time: f64,
        grass_success_anim_timer: f64,
        grass_success_anim_time_left: f64,
    } = Self {
        frame: 0,
        time_elapsed: 0.0,
        started: false,
        event_frequency: 150,

        score: 0,
        health: 100.0,
        misinput_debounce: false,
        debounce_start_time: 0.0,
        is_hit: false,
        is_game_over: false,
        is_played: false,
        is_played_point: false,

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

        sleep_event_active: false,
        sleep_event_start_time:0.0,
        sleep_event_timer: 3.0, // time player is given to press the button
        sleep_event_time_left: 0.0,
        sleep_fail_anim_playing: false,
        sleep_fail_start_time: 0.0,
        sleep_fail_anim_timer: 1.0, // time it takes to play the sleep failure animation
        sleep_fail_anim_time_left: 0.0,
        sleep_success_anim_playing: false,
        sleep_success_start_time: 0.0,
        sleep_success_anim_timer: 1.0, // time it takes to play the sleep success animation
        sleep_success_anim_time_left: 0.0,

        water_event_active: false,
        water_event_start_time:0.0,
        water_event_timer: 3.5, // time player is given to press the button
        water_event_time_left: 0.0,
        water_fail_anim_playing: false,
        water_fail_start_time: 0.0,
        water_fail_anim_timer: 1.0, // time it takes to play the water failure animation
        water_fail_anim_time_left: 0.0,
        water_success_anim_playing: false,
        water_success_start_time: 0.0,
        water_success_anim_timer: 1.0, // time it takes to play the water success animation
        water_success_anim_time_left: 0.0,

        grass_event_active: false,
        grass_event_start_time:0.0,
        grass_event_timer: 3.5, // time player is given to press the button
        grass_event_time_left: 0.0,
        grass_fail_anim_playing: false,
        grass_fail_start_time: 0.0,
        grass_fail_anim_timer: 1.0, // time it takes to play the grass failure animation
        grass_fail_anim_time_left: 0.0,
        grass_success_anim_playing: false,
        grass_success_start_time: 0.0,
        grass_success_anim_timer: 1.0, // time it takes to play the grass success animation
        grass_success_anim_time_left: 0.0,
    }
}

turbo::go!{
    let mut state = GameState::load();

    //music
    if !audio::is_playing("lofi") {
        audio::play("lofi");
    }

    sprite!("study-room",x = -25, y = 0, scale = 0.57);
    //score
    let score_string = format!("Score: {}", state.score);
    text!(&score_string,
        x = 1,
        y = 10,
    );
    // create red rectangle for the health bar with location (0,0) and width of the current wealth and height as 10
        text!("Health: ",
            x = 1,
            y = 0,
        );
        rect!(
            x = 40,
            y = 0,
            w = state.health,
            h = 10,
            color = 0xff0000ff,
        );
    //crate::println!("{:?} {}", {}, state.eat_event_time_left/state.eat_event_timer);
    if !state.started {
        
        let pointer = pointer();
        let clicked = pointer.just_pressed();
        if clicked {
            state.started = true;
            state.health = 100.0;
            state.score = 0;
            audio::play("earn_point");
        }
        if !state.is_game_over{
            text_box!(
                "Start Game",
                font = "SuperPixel",
                width = 70,
                height = 50,
                align = "center",
                x = 90,
                y = 54,
            );
            text!(
                "Take care of yourself to avoid crashing out!",
                font = "medium",
                x = 17,
                y = 40,
            )
        }
        else {
            sprite!("student_gameover_sprite",
                x = 60,
                y = 0
            );

            text_box!("Crashed out!",
                font = "SuperPixel",
                width = 100,
                height = 50,
                align = "center",
                x = 80,
                y = 100,
            );
            text!("Click to play again",
                font = "medium",
                x = 80,
                y = 15,
            );
        }

    }
    else {
        //health check
        if state.health <= 0.0 {
            state.started = false;
            state.is_game_over = true;
            audio::play("game_over");
        }

        // game runstate
        if !(state.eat_fail_anim_playing || state.water_fail_anim_playing || state.sleep_fail_anim_playing || state.grass_fail_anim_playing || state.misinput_debounce) {
            sprite!("student_study_sprite",
                x = 60,
                y = 0,
            );
        }
        else {
            sprite!("student_hit_sprite",
                x = 60,
                y = 0,
            );
        }

        let event_frequency = state.event_frequency;
        
        let spawn_event_val = rand() % event_frequency;

        // EAT EVENT
        if spawn_event_val == 1 && state.eat_event_active == false && state.eat_fail_anim_playing == false && state.eat_success_anim_playing == false {
            log!("eat event active");
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
                // if the event passes, health decreases by 10
                state.health -= 10.0;
                // if the current health is less than 0, keep the health at zero so it doesn't go negative
                if state.health < 0.0 { state.health = 0.0;}
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
            state.score += 1;
            state.eat_event_active = false;
            state.eat_success_anim_playing = true;
            state.eat_success_start_time = state.time_elapsed;
        }
        // spawn the green circle and fade-out when you get a success
        if state.eat_success_anim_playing == true {
            if !state.is_played_point {
                audio::play("earn_point");
                state.is_played_point = true;
            }
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
                state.is_played_point = false;
            }

        }

        // spawn the red circle and fade-out when you get a failure
        if state.eat_fail_anim_playing == true {
            if !state.is_played {
                audio::play("take_damage");
                state.is_played = true;
            }
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
                state.is_played = false;
            }

        }

        // SLEEP EVENT
        if spawn_event_val == 2 && state.sleep_event_active == false && state.sleep_fail_anim_playing == false && state.sleep_success_anim_playing == false {
            log!("sleep event active");
            state.sleep_event_active = true;
            state.sleep_event_start_time = state.time_elapsed;
        }
        if state.sleep_event_active == true {
            
            state.sleep_event_time_left = state.sleep_event_timer + state.sleep_event_start_time - state.time_elapsed;
            //crate::println!("{:?} {}", {}, state.time_elapsed);
            //if the timer is up, make you lose health and make the icon disappear
            if state.sleep_event_time_left <= 0.0 || state.sleep_event_time_left > state.sleep_event_start_time{
                state.sleep_event_active = false;
                //TAKE DAMAGE
                state.sleep_fail_anim_playing = true;
                state.sleep_fail_start_time = state.time_elapsed;
            }
            //When the event is active, make it show the icon
            let transparency = (4.0 * (state.time_elapsed - state.sleep_event_start_time)/state.sleep_event_time_left).clamp(0.0,1.0);
            let alpha = (transparency * 255.0).round() as u32;

            circ!(
                d = 28.0 + (state.sleep_event_time_left/state.sleep_event_timer) * 40.0,
                x = 200.0 - (28.0 + (state.sleep_event_time_left/state.sleep_event_timer) * 40.0) / 2.0,
                y = 29.0 - (28.0 + (state.sleep_event_time_left/state.sleep_event_timer) * 40.0) / 2.0,
                
                color = (0xadd8e6 & 0xffffff00) | alpha,
            );
            sprite!("button_sleep",
                x = 180.0,
                y = 10.0,
                scale = 0.3,
                color = 0xffffff00 | (alpha * 2).clamp(0, 255),
            );
            text!("SLEEP",
                x = 188.0,
                y = 3.0,
                color = 0xffffff00 | (alpha * 2).clamp(0, 255),
            );
            text!("S",
                x = 196.0,
                y = 45.0,
                font = "large",
                color = 0xffffff00 | (alpha * 2).clamp(0, 255),
            );
        }
        if state.sleep_event_active == true && gamepad(0).down.pressed() {
            state.score += 1;
            state.sleep_event_active = false;
            state.sleep_success_anim_playing = true;
            state.sleep_success_start_time = state.time_elapsed;
        }
        // spawn the green circle and fade-out when you get a success
        if state.sleep_success_anim_playing == true {
            if !state.is_played_point {
                audio::play("earn_point");
                state.is_played_point = true;
            }
            state.sleep_success_anim_time_left = (state.sleep_success_start_time + state.sleep_success_anim_timer) - state.time_elapsed;

            // fade-out icons
            if state.sleep_success_anim_time_left > 0.0 && state.sleep_success_anim_time_left <= state.sleep_success_anim_timer {
                let transparency = 1.0 - (2.0 * (state.time_elapsed - state.sleep_success_start_time)/state.sleep_success_anim_time_left).clamp(0.0,1.0);
                let alpha = (transparency * 255.0).round() as u32;
                let circtransparency = 1.0 - (0.15 * ((state.time_elapsed - state.sleep_success_start_time)/state.sleep_success_anim_time_left)).clamp(0.0,1.0);
                let circalpha = (circtransparency * 255.0).round() as u32;
                sprite!("button_sleep",
                    x = 180.0,
                    y = 10,
                    scale = 0.3,
                    color = 0xffffff00 | (alpha).clamp(0, 255),
                );
                text!("SLEEP",
                    x = 188.0,
                    y = 3.0,
                    color = 0xffffff00 | (alpha).clamp(0, 255),
                );
                text!("S",
                    x = 196.0,
                    y = 45.0,
                    font = "large",
                    color = 0xffffff00 | (alpha).clamp(0, 255),
                );

                // grow the green circle
                if ((state.sleep_success_anim_timer - state.sleep_success_anim_time_left)/state.sleep_success_anim_timer) * 180.0 < 60.0 {
                    circ!(
                        d = (((state.sleep_success_anim_timer - state.sleep_success_anim_time_left)/state.sleep_success_anim_timer) * 180.0).clamp(0.0, 60.0),
                        x = 200.0 - ((((state.sleep_success_anim_timer - state.sleep_success_anim_time_left)/state.sleep_success_anim_timer) * 180.0).clamp(0.0, 60.0)) / 2.0,
                        y = 29.0 - ((((state.sleep_success_anim_timer - state.sleep_success_anim_time_left)/state.sleep_success_anim_timer) * 180.0).clamp(0.0, 60.0)) / 2.0,
                        
                        color = (0x1bff0000 & 0xffffff00) | circalpha,
                    );
                }
                else {
                    circ!(
                        d = 60.0,
                        x = 170.0,
                        y = -1.0,
                        
                        color = (0x1bff0000 & 0xffffff00) | circalpha,
                    );
                }
                
            }
            else {
                state.sleep_success_anim_playing = false;
                state.is_played_point = false;
            }

        }

        // spawn the red circle and fade-out when you get a failure
        if state.sleep_fail_anim_playing == true {
            if !state.is_played {
                audio::play("take_damage");
                state.is_played = true;
            }
            state.sleep_fail_anim_time_left = (state.sleep_fail_start_time + state.sleep_fail_anim_timer) - state.time_elapsed;
            
            // fade-out icons
            if state.sleep_fail_anim_time_left > 0.0 && state.sleep_fail_anim_time_left <= state.sleep_fail_anim_timer {
                let transparency = 1.0 - (2.0 * (state.time_elapsed - state.sleep_fail_start_time)/state.sleep_fail_anim_time_left).clamp(0.0,1.0);
                let alpha = (transparency * 255.0).round() as u32;
                let circtransparency = 1.0 - (0.15 * ((state.time_elapsed - state.sleep_fail_start_time)/state.sleep_fail_anim_time_left)).clamp(0.0,1.0);
                let circalpha = (circtransparency * 255.0).round() as u32;
                sprite!("button_sleep",
                    x = 180.0,
                    y = 10.0,
                    scale = 0.3,
                    color = 0xffffff00 | (alpha).clamp(0, 255),
                );
                text!("SLEEP",
                    x = 188.0,
                    y = 3.0,
                    color = 0xffffff00 | (alpha).clamp(0, 255),
                );
                text!("S",
                    x = 196.0,
                    y = 45.0,
                    font = "large",
                    color = 0xffffff00 | (alpha).clamp(0, 255),
                );

            // grow the red circle
            if ((state.sleep_fail_anim_timer - state.sleep_fail_anim_time_left)/state.sleep_fail_anim_timer) * 180.0 < 60.0 {
                circ!(
                    d = (((state.sleep_fail_anim_timer - state.sleep_fail_anim_time_left)/state.sleep_fail_anim_timer) * 180.0).clamp(0.0, 60.0),
                    x = 200.0 - ((((state.sleep_fail_anim_timer - state.sleep_fail_anim_time_left)/state.sleep_fail_anim_timer) * 180.0).clamp(0.0, 60.0)) / 2.0,
                    y = 29.0 - ((((state.sleep_fail_anim_timer - state.sleep_fail_anim_time_left)/state.sleep_fail_anim_timer) * 180.0).clamp(0.0, 60.0)) / 2.0,
                    
                    color = (0xff000000 & 0xffffff00) | circalpha,
                );
            }
            else {
                circ!(
                    d = 60.0,
                    x = 170.0,
                    y = -1.0,
                    
                    color = (0xff000000 & 0xffffff00) | circalpha,
                );
            }
            
            }
            else {
                state.sleep_fail_anim_playing = false;
                state.is_played = false;
            }

        }

        // WATER EVENT
        if spawn_event_val == 3 && state.water_event_active == false && state.water_fail_anim_playing == false && state.water_success_anim_playing == false {
            log!("water event active");
            state.water_event_active = true;
            state.water_event_start_time = state.time_elapsed;
        }
        if state.water_event_active == true {
            
            state.water_event_time_left = state.water_event_timer + state.water_event_start_time - state.time_elapsed;
            //crate::println!("{:?} {}", {}, state.time_elapsed);
            //if the timer is up, make you lose health and make the icon disappear
            if state.water_event_time_left <= 0.0 || state.water_event_time_left > state.water_event_start_time{
                state.water_event_active = false;
                //TAKE DAMAGE
                // if the event passes, health decreases by 10
                state.health -= 10.0;
                // if the current health is less than 0, keep the health at zero so it doesn't go negative
                if state.health < 0.0 { state.health = 0.0;}
                state.water_fail_anim_playing = true;
                state.water_fail_start_time = state.time_elapsed;
            }
            //When the event is active, make it show the icon
            let transparency = (4.0 * (state.time_elapsed - state.water_event_start_time)/state.water_event_time_left).clamp(0.0,1.0);
            let alpha = (transparency * 255.0).round() as u32;

            circ!(
                d = 28.0 + (state.water_event_time_left/state.water_event_timer) * 40.0,
                x = 60.0 - (28.0 + (state.water_event_time_left/state.water_event_timer) * 40.0) / 2.0,
                y = 109.0 - (28.0 + (state.water_event_time_left/state.water_event_timer) * 40.0) / 2.0,
                
                color = (0xadd8e6 & 0xffffff00) | alpha,
            );
            sprite!("button_water",
                x = 40,
                y = 90,
                scale = 0.3,
                color = 0xffffff00 | (alpha * 2).clamp(0, 255),
            );
            text!("WATER",
                x = 48,
                y = 83,
                color = 0xffffff00 | (alpha * 2).clamp(0, 255),
            );
            text!("A",
                x = 56,
                y = 125,
                font = "large",
                color = 0xffffff00 | (alpha * 2).clamp(0, 255),
            );
        }
        if state.water_event_active == true && gamepad(0).left.pressed() {
            state.score += 1;
            state.water_event_active = false;
            state.water_success_anim_playing = true;
            state.water_success_start_time = state.time_elapsed;
        }
        // spawn the green circle and fade-out when you get a success
        if state.water_success_anim_playing == true {
            if !state.is_played_point {
                audio::play("earn_point");
                state.is_played_point = true;
            }
            state.water_success_anim_time_left = (state.water_success_start_time + state.water_success_anim_timer) - state.time_elapsed;

            // fade-out icons
            if state.water_success_anim_time_left > 0.0 && state.water_success_anim_time_left <= state.water_success_anim_timer {
                let transparency = 1.0 - (2.0 * (state.time_elapsed - state.water_success_start_time)/state.water_success_anim_time_left).clamp(0.0,1.0);
                let alpha = (transparency * 255.0).round() as u32;
                let circtransparency = 1.0 - (0.15 * ((state.time_elapsed - state.water_success_start_time)/state.water_success_anim_time_left)).clamp(0.0,1.0);
                let circalpha = (circtransparency * 255.0).round() as u32;
                sprite!("button_water",
                x = 40,
                y = 90,
                scale = 0.3,
                color = 0xffffff00 | (alpha).clamp(0, 255),
            );
            text!("WATER",
                x = 48,
                y = 83,
                color = 0xffffff00 | (alpha).clamp(0, 255),
            );
            text!("A",
                x = 56,
                y = 125,
                font = "large",
                color = 0xffffff00 | (alpha).clamp(0, 255),
            );

            // grow the green circle
            if ((state.water_success_anim_timer - state.water_success_anim_time_left)/state.water_success_anim_timer) * 180.0 < 60.0 {
                circ!(
                    d = (((state.water_success_anim_timer - state.water_success_anim_time_left)/state.water_success_anim_timer) * 180.0).clamp(0.0, 60.0),
                    x = 60.0 - ((((state.water_success_anim_timer - state.water_success_anim_time_left)/state.water_success_anim_timer) * 180.0).clamp(0.0, 60.0)) / 2.0,
                    y = 109.0 - ((((state.water_success_anim_timer - state.water_success_anim_time_left)/state.water_success_anim_timer) * 180.0).clamp(0.0, 60.0)) / 2.0,
                    
                    color = (0x1bff0000 & 0xffffff00) | circalpha,
                );
            }
            else {
                circ!(
                    d = 60.0,
                    x = 30.0,
                    y = 79.0,
                    
                    color = (0x1bff0000 & 0xffffff00) | circalpha,
                );
            }
            
            }
            else {
                state.water_success_anim_playing = false;
                state.is_played_point = false;
            }

        }

        // spawn the red circle and fade-out when you get a failure
        if state.water_fail_anim_playing == true {
            if !state.is_played {
                audio::play("take_damage");
                state.is_played = true;
            }
            state.water_fail_anim_time_left = (state.water_fail_start_time + state.water_fail_anim_timer) - state.time_elapsed;
            
            // fade-out icons
            if state.water_fail_anim_time_left > 0.0 && state.water_fail_anim_time_left <= state.water_fail_anim_timer {
                let transparency = 1.0 - (2.0 * (state.time_elapsed - state.water_fail_start_time)/state.water_fail_anim_time_left).clamp(0.0,1.0);
                let alpha = (transparency * 255.0).round() as u32;
                let circtransparency = 1.0 - (0.15 * ((state.time_elapsed - state.water_fail_start_time)/state.water_fail_anim_time_left)).clamp(0.0,1.0);
                let circalpha = (circtransparency * 255.0).round() as u32;
                sprite!("button_water",
                x = 40,
                y = 90,
                scale = 0.3,
                color = 0xffffff00 | (alpha).clamp(0, 255),
            );
            text!("WATER",
                x = 48,
                y = 83,
                color = 0xffffff00 | (alpha).clamp(0, 255),
            );
            text!("A",
                x = 56,
                y = 125,
                font = "large",
                color = 0xffffff00 | (alpha).clamp(0, 255),
            );

            // grow the red circle
            if ((state.water_fail_anim_timer - state.water_fail_anim_time_left)/state.water_fail_anim_timer) * 180.0 < 60.0 {
                circ!(
                    d = (((state.water_fail_anim_timer - state.water_fail_anim_time_left)/state.water_fail_anim_timer) * 180.0).clamp(0.0, 60.0),
                    x = 60.0 - ((((state.water_fail_anim_timer - state.water_fail_anim_time_left)/state.water_fail_anim_timer) * 180.0).clamp(0.0, 60.0)) / 2.0,
                    y = 109.0 - ((((state.water_fail_anim_timer - state.water_fail_anim_time_left)/state.water_fail_anim_timer) * 180.0).clamp(0.0, 60.0)) / 2.0,
                    
                    color = (0xff000000 & 0xffffff00) | circalpha,
                );
            }
            else {
                circ!(
                    d = 60.0,
                    x = 30.0,
                    y = 79.0,
                    
                    color = (0xff000000 & 0xffffff00) | circalpha,
                );
            }
            
            }
            else {
                state.water_fail_anim_playing = false;
                state.is_played = false;
            }

        }

        // GRASS EVENT
        if spawn_event_val == 4 && state.grass_event_active == false && state.grass_fail_anim_playing == false && state.grass_success_anim_playing == false {
            log!("grass event active");
            state.grass_event_active = true;
            state.grass_event_start_time = state.time_elapsed;
        }
        if state.grass_event_active == true {
            
            state.grass_event_time_left = state.grass_event_timer + state.grass_event_start_time - state.time_elapsed;
            //crate::println!("{:?} {}", {}, state.time_elapsed);
            //if the timer is up, make you lose health and make the icon disappear
            if state.grass_event_time_left <= 0.0 || state.grass_event_time_left > state.grass_event_start_time{
                state.grass_event_active = false;
                //TAKE DAMAGE
                // if the event passes, health decreases by 10
                state.health -= 10.0;
                // if the current health is less than 0, keep the health at zero so it doesn't go negative
                if state.health < 0.0 { state.health = 0.0;}
                state.grass_fail_anim_playing = true;
                state.grass_fail_start_time = state.time_elapsed;
            }
            //When the event is active, make it show the icon
            let transparency = (4.0 * (state.time_elapsed - state.grass_event_start_time)/state.grass_event_time_left).clamp(0.0,1.0);
            let alpha = (transparency * 255.0).round() as u32;

            circ!(
                d = 28.0 + (state.grass_event_time_left/state.grass_event_timer) * 40.0,
                x = 200.0 - (28.0 + (state.grass_event_time_left/state.grass_event_timer) * 40.0) / 2.0,
                y = 109.0 - (28.0 + (state.grass_event_time_left/state.grass_event_timer) * 40.0) / 2.0,
                
                color = (0xadd8e6 & 0xffffff00) | alpha,
            );
            sprite!("button_touch_grass",
                x = 180.0,
                y = 90,
                scale = 0.3,
                color = 0xffffff00 | (alpha * 2).clamp(0, 255),
            );
            text!("TOUCH GRASS",
                x = 172.0,
                y = 83,
                color = 0xffffff00 | (alpha * 2).clamp(0, 255),
            );
            text!("D",
                x = 196.0,
                y = 125,
                font = "large",
                color = 0xffffff00 | (alpha * 2).clamp(0, 255),
            );
        }
        if state.grass_event_active == true && gamepad(0).right.pressed() {
            state.score += 1;
            state.grass_event_active = false;
            state.grass_success_anim_playing = true;
            state.grass_success_start_time = state.time_elapsed;
        }
        // spawn the green circle and fade-out when you get a success
        if state.grass_success_anim_playing == true {
            if !state.is_played_point {
                audio::play("earn_point");
                state.is_played_point = true;
            }
            state.grass_success_anim_time_left = (state.grass_success_start_time + state.grass_success_anim_timer) - state.time_elapsed;

            // fade-out icons
            if state.grass_success_anim_time_left > 0.0 && state.grass_success_anim_time_left <= state.grass_success_anim_timer {
                let transparency = 1.0 - (2.0 * (state.time_elapsed - state.grass_success_start_time)/state.grass_success_anim_time_left).clamp(0.0,1.0);
                let alpha = (transparency * 255.0).round() as u32;
                let circtransparency = 1.0 - (0.15 * ((state.time_elapsed - state.grass_success_start_time)/state.grass_success_anim_time_left)).clamp(0.0,1.0);
                let circalpha = (circtransparency * 255.0).round() as u32;
                sprite!("button_touch_grass",
                x = 180.0,
                y = 90,
                scale = 0.3,
                color = 0xffffff00 | (alpha).clamp(0, 255),
            );
            text!("TOUCH GRASS",
                x = 172,
                y = 83,
                color = 0xffffff00 | (alpha).clamp(0, 255),
            );
            text!("D",
                x = 196.0,
                y = 125,
                font = "large",
                color = 0xffffff00 | (alpha).clamp(0, 255),
            );

            // grow the green circle
            if ((state.grass_success_anim_timer - state.grass_success_anim_time_left)/state.grass_success_anim_timer) * 180.0 < 60.0 {
                circ!(
                    d = (((state.grass_success_anim_timer - state.grass_success_anim_time_left)/state.grass_success_anim_timer) * 180.0).clamp(0.0, 60.0),
                    x = 200.0 - ((((state.grass_success_anim_timer - state.grass_success_anim_time_left)/state.grass_success_anim_timer) * 180.0).clamp(0.0, 60.0)) / 2.0,
                    y = 109.0 - ((((state.grass_success_anim_timer - state.grass_success_anim_time_left)/state.grass_success_anim_timer) * 180.0).clamp(0.0, 60.0)) / 2.0,
                    
                    color = (0x1bff0000 & 0xffffff00) | circalpha,
                );
            }
            else {
                circ!(
                    d = 60.0,
                    x = 170.0,
                    y = 79.0,
                    
                    color = (0x1bff0000 & 0xffffff00) | circalpha,
                );
            }
            
            }
            else {
                state.grass_success_anim_playing = false;
                state.is_played_point = false;
            }

        }

        // spawn the red circle and fade-out when you get a failure
        if state.grass_fail_anim_playing == true {
            if !state.is_played {
                audio::play("take_damage");
                state.is_played = true;
            }
            state.grass_fail_anim_time_left = (state.grass_fail_start_time + state.grass_fail_anim_timer) - state.time_elapsed;
            
            // fade-out icons
            if state.grass_fail_anim_time_left > 0.0 && state.grass_fail_anim_time_left <= state.grass_fail_anim_timer {
                let transparency = 1.0 - (2.0 * (state.time_elapsed - state.grass_fail_start_time)/state.grass_fail_anim_time_left).clamp(0.0,1.0);
                let alpha = (transparency * 255.0).round() as u32;
                let circtransparency = 1.0 - (0.15 * ((state.time_elapsed - state.grass_fail_start_time)/state.grass_fail_anim_time_left)).clamp(0.0,1.0);
                let circalpha = (circtransparency * 255.0).round() as u32;
                sprite!("button_touch_grass",
                x = 180.0,
                y = 90,
                scale = 0.3,
                color = 0xffffff00 | (alpha).clamp(0, 255),
            );
            text!("TOUCH GRASS",
                x = 172,
                y = 83,
                color = 0xffffff00 | (alpha).clamp(0, 255),
            );
            text!("D",
                x = 196.0,
                y = 125,
                font = "large",
                color = 0xffffff00 | (alpha).clamp(0, 255),
            );

            // grow the red circle
            if ((state.grass_fail_anim_timer - state.grass_fail_anim_time_left)/state.grass_fail_anim_timer) * 180.0 < 60.0 {
                circ!(
                    d = (((state.grass_fail_anim_timer - state.grass_fail_anim_time_left)/state.grass_fail_anim_timer) * 180.0).clamp(0.0, 60.0),
                    x = 200.0 - ((((state.grass_fail_anim_timer - state.grass_fail_anim_time_left)/state.grass_fail_anim_timer) * 180.0).clamp(0.0, 60.0)) / 2.0,
                    y = 109.0 - ((((state.grass_fail_anim_timer - state.grass_fail_anim_time_left)/state.grass_fail_anim_timer) * 180.0).clamp(0.0, 60.0)) / 2.0,
                    
                    color = (0xff000000 & 0xffffff00) | circalpha,
                );
            }
            else {
                circ!(
                    d = 60.0,
                    x = 170.0,
                    y = 79.0,
                    
                    color = (0xff000000 & 0xffffff00) | circalpha,
                );
            }
            
            }
            else {
                state.grass_fail_anim_playing = false;
                state.is_played = false;
            }

        }

        if (state.eat_event_active == true && gamepad(0).up.pressed()) || (true) {

        }

    }

    if (state.misinput_debounce == false) && (((state.eat_event_active == false && gamepad(0).up.pressed()) && (state.eat_fail_anim_playing == false && gamepad(0).up.pressed()) && (state.eat_success_anim_playing == false && gamepad(0).up.pressed())) || ((state.sleep_event_active == false && gamepad(0).down.pressed()) && (state.sleep_fail_anim_playing == false && gamepad(0).down.pressed()) && (state.sleep_success_anim_playing == false && gamepad(0).down.pressed())) || ((state.water_event_active == false && gamepad(0).left.pressed()) && (state.water_fail_anim_playing == false && gamepad(0).left.pressed()) && (state.water_success_anim_playing == false && gamepad(0).left.pressed())) || ((state.grass_event_active == false && gamepad(0).right.pressed()) && (state.grass_fail_anim_playing == false && gamepad(0).right.pressed()) && (state.grass_success_anim_playing == false && gamepad(0).right.pressed()))){
            state.health -= 5.0;
            state.misinput_debounce = true;
            state.debounce_start_time = state.time_elapsed;
            audio::play("take_damage");
        }
        if state.time_elapsed - state.debounce_start_time > 0.5 {
            state.misinput_debounce = false;
        }

    state.frame += 1;
    state.time_elapsed = state.frame as f64 / 60.0;
    state.save();
}

