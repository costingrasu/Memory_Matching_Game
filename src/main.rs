slint::include_modules!();
use std::time::{Duration, Instant};

fn duration_to_float(duration: Duration) -> f32 {
    let secs = duration.as_secs() as f32;
    let nanos = duration.subsec_nanos() as f32 / 1e9; 
    secs + nanos                                     
}

fn update_memory_tiles(main_window: &MainWindow, nr: usize) -> std::rc::Rc<slint::VecModel<TileData>> {
    use slint::Model;

    let mut tiles: Vec<TileData> = main_window.get_memory_tiles().iter().collect();
    let mut memoryvec = Vec::new();
    
    for i in 0..nr {
        memoryvec.push(tiles[i].clone());
    }
    
    memoryvec.extend(memoryvec.clone()); // Duplicate for pairs
    
    // Shuffle the tiles for randomness
    use rand::seq::SliceRandom;
    let mut rng = rand::thread_rng();
    memoryvec.shuffle(&mut rng);
    
    let tiles_model = std::rc::Rc::new(slint::VecModel::from(memoryvec));
    main_window.set_memory_tiles(tiles_model.clone().into());
    
    tiles_model
}

fn main() {
    use slint::Model;

    let main_window = MainWindow::new().unwrap();
    let mut timer = main_window.get_timp();
    let start_time = Instant::now();

    let main_window_weak1 = main_window.as_weak();
    main_window.on_asign_dif(move || {
        let main_window = main_window_weak1.unwrap();
        let dif = main_window.get_dif();
        
        let nr = match dif {
            2 => 2,
            3 => 4,
            _ => 8,
        };
        
        let tiles_model = update_memory_tiles(&main_window, nr);
    });

    let main_window_weak = main_window.as_weak();
    main_window.on_check_if_pair_solved(move || {
        let main_window = main_window_weak.unwrap();
        let mut scor = main_window.get_scuor();
        let tiles_model = main_window.get_memory_tiles();
        let mut flipped_tiles =
            tiles_model.iter().enumerate().filter(|(_, tile)| tile.image_visible && !tile.solved);

        if let (Some((t1_idx, mut t1)), Some((t2_idx, mut t2))) =
            (flipped_tiles.next(), flipped_tiles.next())
        {
            let is_pair_solved = t1 == t2;
            if is_pair_solved {
                t1.solved = true;
                tiles_model.set_row_data(t1_idx, t1);
                t2.solved = true;
                tiles_model.set_row_data(t2_idx, t2);
                scor += 10;
                main_window.set_scuor(scor);
            } else {
                let main_window = main_window_weak.unwrap();
                main_window.set_disable_tiles(true);
                let tiles_model = tiles_model.clone();
                slint::Timer::single_shot(std::time::Duration::from_secs(1), move || {
                    main_window.set_disable_tiles(false);
                    t1.image_visible = false;
                    tiles_model.set_row_data(t1_idx, t1);
                    t2.image_visible = false;
                    tiles_model.set_row_data(t2_idx, t2);
                });
            }
        }
    });

    let main_window_weak2 = main_window.as_weak();
    main_window.on_check_if_won(move || {
        let main_window = main_window_weak2.unwrap();
        let tiles_model = main_window.get_memory_tiles().clone();
        let mut tiles_left = tiles_model.iter().enumerate().filter(|(_, tile)| !tile.solved);
        let mut nr: i8 = 0;
        for i in tiles_left{
            nr += 1;
        }
        
        if nr == 0 {
            let duration = start_time.elapsed();
            timer = duration_to_float(duration);
            main_window.set_timp(timer);
            main_window.set_won(true);
        }
    });

    main_window.run().unwrap();
}