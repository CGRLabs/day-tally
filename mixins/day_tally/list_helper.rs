// Day Tally's row in Options > Mods.
fn on_mods(mods: &mut Vec<crate::game::native_hooks::NativeMod>) {
    use horizon_client::day_tally as dt;
    mods.push(crate::game::native_hooks::NativeMod {
        id: dt::ID.into(),
        title: format!("Day Tally {}", dt::VERSION),
        description: dt::DESCRIPTION.into(),
        keys: vec![],
    });
}
