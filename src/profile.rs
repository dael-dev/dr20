use serde::Deserialize;

static  SAMPLE_YAML: &str  = r#"
    vars:
        - name: str
          value: +4
        - name: int
          value: -1
        - name: con
          value: 0
        - name: PB
          value: +2

    rolls:
        - name: Sword
          shortcut: s
          base:
            label: HIT
            expression: 1d20+{str}+{PB}
            
          shift:
            label: ADV
            expression: 2d20kh+{str}+{PB}

          alt:
            label: DIS,
            expression: 2d20kl+{str}+{PB}

          command:
            label: DMG
            expression: 1d8+{str}

          shift_command:
            label: CRIT
            expression: 2d8+{str}
"#;

#[derive(Debug, Deserialize)]
pub struct Profile
{
    vars: Vec<ReplacementVariable>,
    rolls: Vec<RollGroup>
}

impl Default for Profile {
    fn default() -> Self {
        serde_saphyr::from_str::<Profile>(SAMPLE_YAML)
        .unwrap_or_else(|err| {
            eprintln!("Failed to load profile: {err}");

            Profile {
                vars: Vec::new(),
                rolls: Vec::new(),
            }
        })
    }
}


#[derive(Debug, Default, Deserialize)]
struct ReplacementVariable
{
    name: String,
    value: i32
}

#[derive(Debug, Deserialize)]
struct RollGroup
{
    name: String,
    shortcut: char,

    base: Option<TaggedExpression>,
    shift: Option<TaggedExpression>,
    alt: Option<TaggedExpression>,
    command: Option<TaggedExpression>,
    shift_command: Option<TaggedExpression>,
    shift_alt: Option<TaggedExpression>
}

#[derive(Debug, Deserialize)]
struct TaggedExpression
{
    label: String,
    expression: String
}