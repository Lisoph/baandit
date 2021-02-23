#[cfg(debug_assertions)]
macro_rules! include_str_reload {
    ($path:literal, $on_change:expr) => {{
		use notify::{Watcher, RecommendedWatcher, RecursiveMode};
		use std::fs;
		use std::path::Path;
		use std::time::Duration;
		use std::sync::mpsc;
		use std::thread;
        use std::borrow::Cow;

        let on_change = $on_change;

        let path = Path::new("src").join($path);
        let initial_source = fs::read_to_string(&path).expect("Read initial file contents");

        (on_change)(Cow::<'static, str>::Owned(initial_source));

        thread::spawn(move || {
            let (tx, rx) = mpsc::channel();
            let mut watcher: RecommendedWatcher =
                Watcher::new(tx, Duration::from_millis(500))
                .expect("Filesystem watcher");
            watcher.watch(&path, RecursiveMode::NonRecursive).expect("Watch target file");

            loop {
                use notify::DebouncedEvent::*;
                match rx.recv() {
                    Ok(e) => match e {
                        Write(_) => {
                            let new_source = fs::read_to_string(&path).expect("Reread contents");
                            (on_change)(Cow::<'static, str>::Owned(new_source));
                        }
                        _ => {}
                    },
                    Err(_err) => break,
                }
            }
        });
    }}
}

#[cfg(not(debug_assertions))]
macro_rules! include_str_reload {
    ($path:literal, $on_change:expr) => {
        ($on_change)(Cow::<'static, str>::Borrowed(include_str!($path)));
    }
}