use ratatui::{
    style::{Modifier, Style},
    text::{Line, Span, Text},
};

pub fn to_text_with_cursor(input: &tui_input::Input, width: u16) -> ratatui::text::Text {
    let scroll = input.visual_scroll(width as usize - 1); // 1 is for the cursor
    let value = input.value();
    let cursor_pos = input.visual_cursor();

    // cut off scrolled off content
    let invisible = value.chars().take(scroll).map(|c| c.len_utf8()).sum();
    let (_, visible) = value.split_at(invisible);

    // extract part that goes before the cursor
    let first_split = visible
        .chars()
        .take(cursor_pos - scroll)
        .map(|c| c.len_utf8())
        .sum();
    let (part1, remainder) = visible.split_at(first_split);

    // extract the part under the cursor and the remainder
    let second_split = remainder.chars().next().map(|c| c.len_utf8()).unwrap_or(0);
    let (mut part2, part3) = remainder.split_at(second_split);
    if part2.is_empty() {
        part2 = " "
    };

    Text::from(Line::from(vec![
        Span::from(part1),
        Span::from(part2).style(Style::default().add_modifier(Modifier::REVERSED)),
        Span::from(part3),
    ]))
}
