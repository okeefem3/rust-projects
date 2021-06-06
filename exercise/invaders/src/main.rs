use crossterm::{
    cursor::{Hide, Show},
    event::{self, Event, KeyCode},
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use invaders::{
    frame::{self, new_frame, Drawable},
    invader::Invaders,
    player::Player,
    render,
};
use std::{error::Error, io, thread, time::Duration};
use std::{sync::mpsc, time::Instant};

fn main() -> Result<(), Box<dyn Error>> {
    // let mut audio = Audio::new();

    // audio.add("explode", "explode.wav");
    // audio.add("lose", "lose.wav");
    // audio.add("move", "move.wav");
    // audio.add("pew", "pew.wav");
    // audio.add("startup", "startup.wav");
    // audio.add("win", "win.wav");

    // audio.play("startup");

    // // Cleanup
    // audio.wait();

    // ? is the error propagation expression operator
    // it unwraps valid values or returns error values propagating to the caller
    // applies to Result and Option
    // Will unwrap the value from an Ok, or return the error

    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?; // Open alt screen
    stdout.execute(Hide)?; // Hide cursor

    // Render Loop

    let (render_tx, render_rx) = mpsc::channel(); // Use a cross beam channel irl

    let render_handle = thread::spawn(move || {
        let mut last_frame = frame::new_frame();
        let mut stdout = io::stdout();
        render::render(&mut stdout, &last_frame, &last_frame, true);

        loop {
            let curr_frame = match render_rx.recv() {
                Ok(x) => x,
                Err(_) => break, // break the loop and shut down the thread
            }; // Need ; here because assigning as a statement
            render::render(&mut stdout, &last_frame, &curr_frame, false);
            last_frame = curr_frame;
        }
    });

    // Game Loop

    let mut player = Player::new();

    let mut instant = Instant::now();

    let mut invaders = Invaders::new();

    'gameLoop: loop {
        // Per-frame init
        let mut curr_frame = new_frame();
        let delta = instant.elapsed();

        instant = Instant::now();

        // Init

        // Note, the ? operator acts a match for Result, if Ok, unwraps and gives the inner value, if error, return from the function currently in
        // it is basically the same as the try! macro, this is so highly used in idomatic rust, that it was given it's own operator
        while event::poll(Duration::default())? {
            // If the event is read succesfully and is a key event, create a key event from it named key_event
            // I believe we know it is a key event because the construction passes properly?
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Left => player.move_left(),
                    KeyCode::Right => player.move_right(),
                    KeyCode::Char(' ') | KeyCode::Enter => player.shoot(),
                    KeyCode::Esc | KeyCode::Char('q') => {
                        // Play lose music and end game
                        break 'gameLoop;
                    }
                    _ => {}
                }
            }
        }

        // Updates

        player.update(delta);

        invaders.update(delta);

        player.detect_hits(&mut invaders);

        // Draw and render

        // This will fail at first because the game loop will run faster than the
        // the set up of the channels, so want to ignore the error
        // player.draw(&mut curr_frame);
        // invaders.draw(&mut curr_frame);

        let drawables: Vec<&dyn Drawable> = vec![&player, &invaders];

        for drawable in drawables {
            drawable.draw(&mut curr_frame);
        }
        let _ = render_tx.send(curr_frame);
        // Game loop is faster than the render loop, so do this to allow a little time
        // seems hacky haha
        thread::sleep(Duration::from_millis(1));

        // Win or Lose
        if invaders.all_killed() {
            break 'gameLoop;
        } else if invaders.reached_bottom() {
            break 'gameLoop;
        }
    }

    // Cleanup
    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    drop(render_tx);
    render_handle.join().unwrap();

    Ok(())
}
