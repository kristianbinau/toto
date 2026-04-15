use std::thread;
use std::time::{Duration, Instant};

use toto_engine::action::{Delay, MouseClick};
use toto_engine::backend::{Backend, MockBackend, MockEvent};
use toto_engine::{Action, Direction, Engine, EngineError, MouseButton, Repeat, Script, ScriptId};

/// Shared-handle factory: every runner thread gets its own `MockBackend`
/// handle but all handles share a single event buffer, so the test can
/// observe events from outside the runner.
fn shared_mock() -> (
    MockBackend,
    impl Fn() -> Result<Box<dyn Backend>, EngineError> + Send + Sync,
) {
    let observer = MockBackend::new();
    let for_factory = observer.clone();
    let factory =
        move || -> Result<Box<dyn Backend>, EngineError> { Ok(Box::new(for_factory.clone())) };
    (observer, factory)
}

fn click_script(id: &str, repeat: Repeat, delay_ms: f64) -> Script {
    Script {
        id: ScriptId::new(id),
        actions: vec![
            Action::Click(MouseClick {
                button: MouseButton::Left,
                direction: Direction::Click,
            }),
            Action::Delay(Delay { ms: delay_ms }),
        ],
        repeat,
    }
}

#[test]
fn runs_bounded_script_to_completion() {
    let (observer, factory) = shared_mock();
    let engine = Engine::new(factory);

    engine
        .start(click_script("five", Repeat::Times(5), 10.0))
        .unwrap();

    // Wait for completion (5 iterations * ~10ms + slack).
    let deadline = Instant::now() + Duration::from_secs(2);
    while engine.is_running(&ScriptId::new("five")) {
        if Instant::now() > deadline {
            panic!("script did not finish within deadline");
        }
        thread::sleep(Duration::from_millis(10));
    }

    let clicks = observer
        .events()
        .iter()
        .filter(|e| matches!(e, MockEvent::Click(_, _)))
        .count();
    assert_eq!(clicks, 5);
}

#[test]
fn infinite_script_stops_promptly() {
    let (observer, factory) = shared_mock();
    let engine = Engine::new(factory);

    engine
        .start(click_script("inf", Repeat::Infinite, 20.0))
        .unwrap();
    thread::sleep(Duration::from_millis(120));

    let stop_start = Instant::now();
    engine.stop(&ScriptId::new("inf")).unwrap();
    let stop_elapsed = stop_start.elapsed();

    assert!(
        stop_elapsed < Duration::from_millis(200),
        "stop took too long: {:?}",
        stop_elapsed
    );
    assert!(!engine.is_running(&ScriptId::new("inf")));

    let click_count = observer
        .events()
        .iter()
        .filter(|e| matches!(e, MockEvent::Click(_, _)))
        .count();
    assert!(click_count >= 1);
    assert!(click_count < 100, "ran away: {click_count}");
}

#[test]
fn concurrent_scripts_are_independent() {
    let (observer, factory) = shared_mock();
    let engine = Engine::new(factory);

    engine
        .start(click_script("a", Repeat::Infinite, 20.0))
        .unwrap();
    engine
        .start(click_script("b", Repeat::Infinite, 20.0))
        .unwrap();

    thread::sleep(Duration::from_millis(100));
    assert!(engine.is_running(&ScriptId::new("a")));
    assert!(engine.is_running(&ScriptId::new("b")));

    engine.stop(&ScriptId::new("a")).unwrap();
    assert!(!engine.is_running(&ScriptId::new("a")));
    assert!(engine.is_running(&ScriptId::new("b")));

    let after_a_stop = observer.len();
    thread::sleep(Duration::from_millis(80));
    let after_wait = observer.len();
    assert!(
        after_wait > after_a_stop,
        "script b should still be producing events"
    );

    engine.stop_all();
    assert!(!engine.is_running(&ScriptId::new("b")));
}

#[test]
fn starting_running_script_errors() {
    let (_observer, factory) = shared_mock();
    let engine = Engine::new(factory);

    engine
        .start(click_script("dup", Repeat::Infinite, 20.0))
        .unwrap();
    let err = engine.start(click_script("dup", Repeat::Infinite, 20.0));
    assert!(matches!(err, Err(EngineError::AlreadyRunning(_))));
    engine.stop(&ScriptId::new("dup")).unwrap();
}

#[test]
fn toggle_flips_state() {
    let (_observer, factory) = shared_mock();
    let engine = Engine::new(factory);

    let s = click_script("tog", Repeat::Infinite, 20.0);
    assert!(engine.toggle(s.clone()).unwrap());
    assert!(engine.is_running(&ScriptId::new("tog")));
    assert!(!engine.toggle(s).unwrap());
    assert!(!engine.is_running(&ScriptId::new("tog")));
}

#[test]
fn script_round_trips_through_json() {
    let script = click_script("json", Repeat::Times(3), 15.0);
    let json = serde_json::to_string(&script).unwrap();
    let back: Script = serde_json::from_str(&json).unwrap();
    assert_eq!(back.id, ScriptId::new("json"));
    assert_eq!(back.actions.len(), 2);
}
