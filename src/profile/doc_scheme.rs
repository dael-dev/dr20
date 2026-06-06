
use serde;
use serde::Deserialize;

const SAMPLE_YAML: &str = include_str!("../sample.yaml");

#[derive(Debug, Deserialize)]
pub(super) struct DocModel
{
    pub(super)vars: Vec<VarModel>,
    pub(super)rolls: Vec<DiceGroupModel>
}
impl DocModel {
    pub(super) fn demo() -> Self {
        serde_saphyr::from_str::<DocModel>(&SAMPLE_YAML)
        .unwrap_or_else(|err| {
            eprintln!("Failed to load profile: {err}");

            DocModel {
                vars: Vec::new(),
                rolls: Vec::new(),
            }
        })
    }
}     

#[derive(Debug, Deserialize)]
pub struct DiceGroupModel
{
    pub(super)name: String,
    pub(super)shortcut: char,

    pub(super)base: Option<DiceExprModel>,
    pub(super)shift: Option<DiceExprModel>,
    pub(super)alt: Option<DiceExprModel>,
    pub(super)command: Option<DiceExprModel>,
    pub(super)shift_command: Option<DiceExprModel>,
    pub(super)shift_alt: Option<DiceExprModel>   
}

#[derive(Debug, Deserialize)]
pub struct DiceExprModel
{
    pub(super)label: String,
    pub(super)expression: String
}

#[derive(Debug, Deserialize)]
pub struct VarModel
{
    pub name: String,
    pub value: i32,
}