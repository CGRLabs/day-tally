// The server half: tell the Day Counter content mod that the corner tally is
// here, so the day is not shown twice.

fn on_level_tick_post(game: &mut crate::game::Game<'_>, dimension: &str) {
    use horizon_client::day_tally as dt;

    let _ = dimension;
    // Once the flag is set, there is nothing more to do.
    if dt::flag_set() {
        return;
    }
    // The content mod's scoreboard holds the flag. Without the content mod
    // there is no objective, so the command fails — try a few times, then
    // leave it alone.
    if dt::tries() >= 40 {
        return;
    }
    dt::note_a_try();
    // "scoreboard players set #day_tally daycounter 1" — the content mod's
    // objective, which only exists when the content mod is installed.
    if game.native_run_command("scoreboard players set #day_tally daycounter 1") {
        dt::set_flag();
    }
}
