use std::convert::TryFrom;

pub use iced::keyboard::key::Code;
use crate::{app::KeyPress, profile::doc_scheme::{DiceExprModel, VarModel}};

mod doc_scheme;
use doc_scheme::{DocModel, DiceGroupModel};

mod  utility;

#[derive(Debug)]
pub struct Profile
{
    pub vars: Vec<ReplacementVariable>,
    pub rolls: Vec<RollGroup>
}

impl TryFrom<DocModel> for Profile {
    type Error = String;

    fn try_from(doc: DocModel) -> Result<Self, Self::Error> {
        let vars: Vec<ReplacementVariable> = doc.vars
            .into_iter()
            .map(ReplacementVariable::from)
            .collect();

        let rolls = doc.rolls
            .into_iter()
            .map(|group| RollGroup::try_from_model(group, &vars))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Profile {
            vars,
            rolls,
        })
    }
}

impl Profile {
    pub(super) fn demo() -> Self {
        let dm = DocModel::demo();
        Profile::try_from(dm).unwrap() //TODO: handle potential error
    }

    pub(crate) fn get_expression(&self, id: ExprId) -> Option<&FullExpression> {
        let group = self.rolls.get(id.group)?;

        match id.slot {
            ExprSlot::Base => group.base.as_ref(),
            ExprSlot::Shift => group.shift.as_ref(),
            ExprSlot::Alt => group.alt.as_ref(),
            ExprSlot::ShiftAlt => group.shift_alt.as_ref(),
            ExprSlot::Ctrl => group.command.as_ref(),
            ExprSlot::ShiftCtrl => group.shift_command.as_ref(),
            ExprSlot::Cmd => group.command.as_ref(),
            ExprSlot::ShiftCmd => group.shift_command.as_ref(),
            ExprSlot::Logo => group.command.as_ref(),
            ExprSlot::ShiftLogo => group.shift_command.as_ref(),
        }
    }

    pub fn get_expression_for_keypress(
        &self,
        key: &KeyPress,
    ) -> Option<&FullExpression> {
        let group = self.rolls.iter()
            .find(|g| g.shortcut == key.code)?;

        if key.modifiers.shift() && key.modifiers.command() {
            return group.shift_command.as_ref();
        }

        if key.modifiers.command() {
            return group.command.as_ref();
        }

        if key.modifiers.shift() && key.modifiers.alt() {
            return group.shift_alt.as_ref();
        }

        if key.modifiers.alt() {
            return group.alt.as_ref();
        }

        if key.modifiers.shift() {
            return group.shift.as_ref();
        }

        group.base.as_ref()
    }
}

#[derive(Debug, Default)]
pub struct ReplacementVariable
{
    pub name: String,
    pub value: i32,
}

impl From<VarModel> for ReplacementVariable
{
    fn from(model: VarModel) -> Self {
        ReplacementVariable {
            name: model.name,
            value: model.value
        }
    }
}


#[derive(Debug)]
pub struct RollGroup
{
    pub name: String,
    pub shortcut_label: char,
    pub shortcut: Option<Code>,

    pub base: Option<FullExpression>,
    pub shift: Option<FullExpression>,
    pub alt: Option<FullExpression>,
    pub command: Option<FullExpression>,
    pub shift_command: Option<FullExpression>,
    pub shift_alt: Option<FullExpression>
}

impl RollGroup
{
    //type Error = String;
    fn try_from_model(value: DiceGroupModel, vars: &[ReplacementVariable]) -> Result<Self, String> {
        let mut group = RollGroup {
            name: value.name,
            shortcut_label: value.shortcut,
            shortcut: utility::char_to_code(value.shortcut),

            base: FullExpression::try_from_model(value.base, vars),
            alt: FullExpression::try_from_model(value.alt, vars),
            command: FullExpression::try_from_model(value.command, vars), 
            shift: FullExpression::try_from_model(value.shift, vars),
            shift_command: FullExpression::try_from_model(value.shift_command, vars), 
            shift_alt: FullExpression::try_from_model(value.shift_alt, vars) 
        };

        group.set_item_group();
        Ok(group)
    }

    fn set_item_group(&mut self) {
        let group_name = self.name.clone();

        for expr in [
            &mut self.base,
            &mut self.shift,
            &mut self.alt,
            &mut self.command,
            &mut self.shift_command,
        ] {
            if let Some(expr) = expr {
                expr.group = group_name.clone();
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct FullExpression
{
    pub group: String,

    pub label: String,
    pub expression: String,

    pub prepared: Option<String>
}

impl FullExpression {
    fn try_from_model(value: Option<DiceExprModel>, vars: &[ReplacementVariable]) -> Option<Self> {
        match value {
            Some(exp) => {
                let exp_copy = exp.expression.clone();
                Some( FullExpression {
                    group: String::default(),
                    label: exp.label,
                    expression: exp.expression,
                    prepared: replace_variables(exp_copy, vars),
                })    
            }
            None => None
        }
    }
}

fn replace_variables(mut exp: String, vars: &[ReplacementVariable]) -> Option<String>{
    for var in vars {
        let brace_pattern = format!("{{{}}}", var.name);
        let dollar_pattern = format!("[{}]", var.name);

        exp = exp.replace(&brace_pattern, &format!("({})", var.value));
        exp = exp.replace(&dollar_pattern, &format!("{}", var.value));
    }
    Some(exp)
}

#[derive(Debug, Clone)]
pub struct ExprId
{
    pub group: usize,
    pub slot: ExprSlot
}

#[derive(Debug, Clone)]
pub enum ExprSlot
{
    Base,
    Shift,
    Alt,
    ShiftAlt,
    Cmd,
    ShiftCmd,
    Ctrl,
    ShiftCtrl,
    Logo,
    ShiftLogo
}
