use anyhow;
use gpui::Pixels;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::Settings;

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema, PartialEq)]
pub enum OpenAIModel {
    #[serde(rename = "gpt-3.5-turbo-0613")]
    ThreePointFiveTurbo,
    #[serde(rename = "gpt-4-0613")]
    Four,
    #[serde(rename = "gpt-4-1106-preview")]
    FourTurbo,
}

impl OpenAIModel {
    pub fn full_name(&self) -> &'static str {
        match self {
            OpenAIModel::ThreePointFiveTurbo => "gpt-3.5-turbo-0613",
            OpenAIModel::Four => "gpt-4-0613",
            OpenAIModel::FourTurbo => "gpt-4-1106-preview",
        }
    }

    pub fn short_name(&self) -> &'static str {
        match self {
            OpenAIModel::ThreePointFiveTurbo => "gpt-3.5-turbo",
            OpenAIModel::Four => "gpt-4",
            OpenAIModel::FourTurbo => "gpt-4-turbo",
        }
    }

    pub fn cycle(&self) -> Self {
        match self {
            OpenAIModel::ThreePointFiveTurbo => OpenAIModel::Four,
            OpenAIModel::Four => OpenAIModel::FourTurbo,
            OpenAIModel::FourTurbo => OpenAIModel::ThreePointFiveTurbo,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum AssistantDockPosition {
    Left,
    Right,
    Bottom,
}

#[derive(Deserialize, Debug)]
pub struct AssistantSettings {
    pub button: bool,
    pub dock: AssistantDockPosition,
    pub default_width: Pixels,
    pub default_height: Pixels,
    pub default_open_ai_model: OpenAIModel,
}

/// Assistant panel settings
#[derive(Clone, Default, Serialize, Deserialize, JsonSchema, Debug)]
pub struct AssistantSettingsContent {
    /// Whether to show the assistant panel button in the status bar.
    ///
    /// Default: true
    pub button: Option<bool>,
    /// Where to dock the assistant.
    ///
    /// Default: right
    pub dock: Option<AssistantDockPosition>,
    /// Default width in pixels when the assistant is docked to the left or right.
    ///
    /// Default: 640
    pub default_width: Option<f32>,
    /// Default height in pixels when the assistant is docked to the bottom.
    ///
    /// Default: 320
    pub default_height: Option<f32>,
    /// The default OpenAI model to use when starting new conversations.
    ///
    /// Default: gpt-4-1106-preview
    pub default_open_ai_model: Option<OpenAIModel>,
}

impl Settings for AssistantSettings {
    const KEY: Option<&'static str> = Some("assistant");

    type FileContent = AssistantSettingsContent;

    fn load(
        default_value: &Self::FileContent,
        user_values: &[&Self::FileContent],
        _: &mut gpui::AppContext,
    ) -> anyhow::Result<Self> {
        Self::load_via_json_merge(default_value, user_values)
    }
}
