// The tally in the bottom left corner: a dark plaque with a gold frame, and the
// day in Roman numerals, big, on it.
//
// The layout is worked out at its final size and then drawn at that size, so
// nothing drifts when the tally shrinks away at the end.

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

    let numeral = dt::roman(day);
    let label = "DAY";

    // How big the numeral is, the space inside the plaque, and the space
    // between the numeral and the label under it.
    const BIG: f32 = 3.0;
    const PAD: f32 = 8.0;
    const GAP: f32 = 4.0;
    // A glyph cell is eight GUI pixels tall; a capital letter fills seven of
    // them, so the ink sits between the top of the cell and the baseline.
    const CELL: f32 = 8.0;
    const INK: f32 = 7.0;

    // The plaque, in GUI pixels.
    let numeral_ink = INK * BIG;
    let label_ink = INK;
    let plaque_width = (game.native_text_width(&numeral) * BIG)
        .max(game.native_text_width(label))
        .max(34.0)
        + PAD * 2.0;
    let plaque_height = PAD + numeral_ink + GAP + label_ink + PAD;
    let (x, y) = corners.place(Corner::BottomLeft, plaque_width, plaque_height);

    // Where the numeral and the label sit, in GUI pixels.
    let numeral_x = x + plaque_width / 2.0;
    let numeral_y = y + PAD + numeral_ink / 2.0;
    let label_x = x + plaque_width / 2.0;
    let label_y = y + PAD + numeral_ink + GAP + label_ink / 2.0;

    let start = batch.vertices.len();

    // The plaque: a gold frame around a dark board, with a thin gold line
    // inside it, the way a framed page is ruled.
    game.native_fill(batch, [x, y, plaque_width, plaque_height], 0xF0C08A3C);
    game.native_fill(
        batch,
        [x + 2.0, y + 2.0, plaque_width - 4.0, plaque_height - 4.0],
        0xD0181008,
    );
    game.native_fill(
        batch,
        [x + 5.0, y + 5.0, plaque_width - 10.0, plaque_height - 10.0],
        0xF0C08A3C,
    );
    game.native_fill(
        batch,
        [x + 6.0, y + 6.0, plaque_width - 12.0, plaque_height - 12.0],
        0xD0181008,
    );

    // The day, in numerals. It is drawn in the font's own size and then grown
    // about its own middle, so it lands exactly where the plaque expects it.
    let numeral_start = batch.vertices.len();
    game.native_text(
        batch,
        &numeral,
        [
            numeral_x - game.native_text_width(&numeral) / 2.0,
            numeral_y - CELL / 2.0,
        ],
        0xF0D28C,
        false,
    );
    batch.scale_since(numeral_start, numeral_x, numeral_y, BIG, BIG);

    // And what it counts, small under it.
    game.native_text(
        batch,
        label,
        [
            label_x - game.native_text_width(label) / 2.0,
            label_y - CELL / 2.0,
        ],
        0xC0A878,
        false,
    );

    // The whole thing shrinks into the middle of the plaque as it goes.
    batch.scale_since(
        start,
        x + plaque_width / 2.0,
        y + plaque_height / 2.0,
        fade,
        fade,
    );
}
