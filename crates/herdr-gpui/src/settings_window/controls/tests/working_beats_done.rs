use super::*;
use crate::config::preferences::Preference;
use crate::settings_window::controls::preferences::PreferenceIo;
use std::sync::{Arc, Mutex};

#[gpui::test]
fn working_beats_done_toggle_saves_the_setting(cx: &mut TestAppContext) {
    let (view, cx) = cx.add_window_view(skill_fixture);
    let edits = Arc::new(Mutex::new(Vec::new()));
    let captured = edits.clone();
    view.update(cx, |view, cx| {
        view.controls.preference_io = Some(PreferenceIo {
            write: Arc::new(move |edit| {
                captured.lock().unwrap().push(edit);
                Ok(())
            }),
            load: Arc::new(skill_load),
        });
        view.section = Section::Sidebar;
        cx.notify();
    });
    cx.simulate_resize(size(px(960.), px(2200.)));
    cx.update(|window, cx| crate::sidebar::layout_tests::full_draw(window, cx).clear(cx));
    let toggle = cx
        .debug_bounds("sidebar-working-beats-done-toggle")
        .unwrap();
    cx.simulate_click(toggle.center(), Default::default());
    cx.run_until_parked();
    assert_eq!(
        edits.lock().unwrap().as_slice(),
        [Preference::SidebarWorkingBeatsDone(true)]
    );
}
