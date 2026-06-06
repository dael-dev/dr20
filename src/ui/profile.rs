use iced::widget::{Column, Grid, button, column, row, text};
use iced::{Element, Length};


use crate::profile::{ExprId, FullExpression, Profile, ReplacementVariable, RollGroup, ExprSlot};
use crate::message::Message;

pub(super) fn view_profile(prof: &Profile) -> Element<'_, Message> {
    column![
        view_replacment_vars(&prof.vars),
        view_dice_groups(&prof.rolls)
    ].into()
}

fn view_replacment_vars<'a>(vs :&'a[ReplacementVariable]) -> Element<'a, Message> {
    vs.iter().fold(
        Grid::new()
            .columns(4)
            .spacing(8),
        |grid, v| {
            grid.push(
                column![
                    text(&v.name),
                    text(&v.value)
                ]
            )
        },
    )
    .into()
}

fn view_dice_groups<'a>(vs: &'a [RollGroup]) -> Element<'a, Message> {
    vs.iter()
        .enumerate()
        .fold(Column::new().spacing(8), |col, (gid, v)| {
            [
                ("".to_string(), ExprSlot::Base, &v.base),
                ("⇧".to_string(), ExprSlot::Shift, &v.shift),
                ("⌥".to_string(), ExprSlot::Alt, &v.alt),
                ("⇧⌥".to_string(), ExprSlot::ShiftAlt, &v.shift_alt),
                ("⌘".to_string(), ExprSlot::Cmd, &v.command),
                ("⇧⌘".to_string(), ExprSlot::ShiftCmd, &v.shift_command),
            ]
            .into_iter()
            .fold(col.push(text(&v.name)), |col, (prefix, slot, ex)| {
                match ex {
                    Some(ex) => col.push(view_dice_expr(
                        gid,
                        slot,
                        format!("{prefix}{}", v.shortcut_label),
                        ex,
                    )),
                    None => col,
                }
            })
        })
        .into()
}

fn view_dice_expr<'a>(
    group : usize,
    slot: ExprSlot,
    shortcut_str: String,
    ex: &'a FullExpression,
) -> Element<'a, Message> {
    row![
        button(text(shortcut_str))
            .width(Length::Fixed(60.0))
            .height(Length::Fixed(40.0))
            .on_press(Message::RollExpression(
                ExprId {group, slot}
            )
         ),
        column![
            text(&ex.label),
            text(&ex.expression),
        ]
    ]
    .spacing(8)
    .into()
}