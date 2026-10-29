use super::*;
use crate::config::preferences::Preference;

#[test]
fn working_beats_done_defaults_off_and_round_trips() -> anyhow::Result<()> {
    for config in [Config::default(), Config::parse(DEFAULT_CONFIG)?] {
        assert!(!config.sidebar_style.working_beats_done);
    }
    assert!(
        Config::parse("[sidebar]\nworking_beats_done = true")?
            .sidebar_style
            .working_beats_done
    );
    assert!(
        !Config::parse("[sidebar]\nworking_beats_done = false")?
            .sidebar_style
            .working_beats_done
    );
    assert!(Config::parse("[sidebar]\nworking_beats_done = \"true\"").is_err());

    let dir = tempfile::tempdir()?;
    let path = dir.path().join("local.toml");
    fs::write(&path, "[sidebar]\nworking_beats_done = false # local\n")?;
    Config::save_preference_path(Preference::SidebarWorkingBeatsDone(true), &path)?;
    let text = fs::read_to_string(&path)?;
    assert!(text.contains("working_beats_done = true # local"), "{text}");
    assert!(Config::parse(&text)?.sidebar_style.working_beats_done);
    Ok(())
}
