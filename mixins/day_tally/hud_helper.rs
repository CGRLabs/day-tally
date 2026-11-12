// The tally in the bottom left corner: the day of the world, big, in the
// game's own font.
//
// It appears when the day turns and shrinks away again a few seconds later.

fn on_hud(
    game: &mut crate::game::Game<'_>,
    batch: &mut horizon_client::hud::HudBatch,
    corners: &mut horizon_client::extras_hud::Corners,
) {
    use horizon_client::day_tally as dt;
    use horizon_client::extras::Corner;

    // The day the world is on, and whether the tally has been noted.
    dt::note(dt::day_of(game.world.day_time));
    let Some((day, fade)) = dt::visible() else {
        return;
    };
    // The debug screen takes the top left; do not draw over anything else.
    if !game.native_corner_shown(Corner::BottomLeft) {
        return;
    }

    // The day, as a plain number.
    let text = day.to_string();

    // How big the number is, and the space around it.
    const BIG: f32 = 3.4;
    const PAD: f32 = 6.0;
    // A glyph cell is eight GUI pixels tall; a digit fills seven of them.
    const CELL: f32 = 8.0;
    const INK: f32 = 7.0;

    let width = game.native_text_width(&text) * BIG;
    let height = INK * BIG;
    let (x, y) = corners.place(Corner::BottomLeft, width + PAD * 2.0, height + PAD * 2.0);

    // The middle of the number.
    let cx = x + (width + PAD * 2.0) / 2.0;
    let cy = y + (height + PAD * 2.0) / 2.0;

    // The number is drawn in the font's own size and then grown about its own
    // middle, so it lands exactly where the corner put it.
    let start = batch.vertices.len();
    game.native_text(
        batch,
        &text,
        [
            cx - game.native_text_width(&text) / 2.0,
            cy - CELL / 2.0,
        ],
        0xFFFFFF,
        true,
    );
    batch.scale_since(start, cx, cy, BIG, BIG);
}
