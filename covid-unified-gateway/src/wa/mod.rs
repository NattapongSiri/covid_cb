//! Module to create session and send user message to Watson Assistant
//! 
//! Use [WASession struct](struct.WASession.html) to create new session
//! or use [re_attach](struct.WASession.html#method.re_attach) to reuse 
//! existing session_id.
//! 
//! There's couple of `send` method to use depending on complexity of input.
//! The simplest one is [send_txt](struct.WASession.html#method.send_txt) 
//! where it only take user message as parameter.
//! Another one is [send_txt_with_context](struct.WASession.html#method.send_txt_with_context) 
//! where it take user message along with `context`. 
//! The most complex one is [send](struct.WASession.html#method.send) where
//! it take [UserInput](struct.UserInput.html) object.
//! 
//! There's [UserInputBuilder](struct.UserInputBuilder.html) to help build
//! UserInput object to send.
//! To Obtain [UserInput](struct.UserInput.html) object, call 
//! [build](struct.UserInputBuilder.html#method.build) method

use super::utils::{delete, post_json, CurlErr};
use serde::{Deserialize, Serialize};
use serde_json::value::{RawValue, Value};
use std::boxed::Box;
use std::collections::HashMap;

#[derive(Debug)]
pub struct WASession {
    api_key: String,
    pub session_id: String,
    session_url: String,
    delete_url: String,
    send_url: String
}

impl WASession {
    /// Construct a new session.
    /// It will immediately establish a session with WA.
    pub async fn new(endpoint_url: String, api_key: String, assistant_id: String, version: String) -> Result<WASession, CurlErr> {
        let session_url = format!("{}/v2/assistants/{}/sessions?version={}", endpoint_url, assistant_id, version);
        let result = post_json::<(), HashMap<String, String>>(&session_url, &api_key, None)?;
        let session_id = result["session_id"].to_owned();
        let delete_url = format!("{}/v2/assistants/{}/sessions/{}version={}", &endpoint_url, &assistant_id, &session_id, &version);
        let send_url = format!("{}/v2/assistants/{}/sessions/{}/message?version={}", &endpoint_url, &assistant_id, &session_id, &version);

        Ok(WASession {
            api_key,
            session_id,
            session_url,
            send_url,
            delete_url
        })
    }

    /// Construct WASession reusing established session.
    /// It take all parameters required to create new session along with session_id which is string
    /// that can be found in `WASession.session_id`.
    /// It doesn't check whether the `session_id` is valid, nor usable.
    pub fn re_attach(endpoint_url: String, api_key: String, assistant_id: String, version: String, session_id: String) -> WASession {
        let session_url = format!("{}/v2/assistants/{}/sessions?version={}", endpoint_url, assistant_id, version);
        let session_id = session_id;
        let delete_url = format!("{}/v2/assistants/{}/sessions/{}version={}", &endpoint_url, &assistant_id, &session_id, &version);
        let send_url = format!("{}/v2/assistants/{}/sessions/{}/message?version={}", &endpoint_url, &assistant_id, &session_id, &version);

        WASession {
            api_key,
            session_id,
            session_url,
            send_url,
            delete_url
        }
    }

    /// Create new session and replace old session with new session.
    pub async fn renew(&mut self) -> Result<(), CurlErr> {
        let result = post_json::<(), HashMap<String, String>>(&self.session_url, &self.api_key, None)?;
        self.session_id = result["session_id"].to_owned();
        Ok(())
    }

    /// Primitive function to send user input.
    pub async fn send<'a>(&self, message: &UserInput<'a>) -> Result<WAResponse, CurlErr> {
        post_json(&self.send_url, &self.api_key, Some(message))
    }

    /// User friendly function to let user send simple text message to WA
    pub async fn send_txt(&self, input: &str) -> Result<WAResponse, CurlErr> {
        post_json(&self.send_url, &self.api_key, Some(&UserInputBuilder::builder().text(input).options(InputOptions::default()).build()))
    }

    /// User friendly function to let user simple text message along with message context to WA
    pub async fn send_txt_with_context(&self, input: &str, context: UnknownType) -> Result<WAResponse, CurlErr> {
        post_json(&self.send_url, &self.api_key, Some(&UserInputBuilder::builder().text(input).options(InputOptions::default()).context(ContextBuilder::builder().user_defined(context).build()).build()))
    }

    /// Terminate the session.
    pub async fn close(self) -> Result<(), CurlErr> {
        delete(&self.delete_url, &self.api_key)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct InputOptions {
    debug: bool,
    restart: bool,
    alternate_intents: bool,
    return_context: bool,
    export: bool
}

impl Default for InputOptions {
    fn default() -> Self {
        InputOptions {
            debug: false,
            restart: false,
            alternate_intents: true,
            return_context: true,
            export: false
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Intent {
    intent: String,
    confidence: f32
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum UnknownType {
    Raw(Box<RawValue>),
    Value(Value)
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Entity {
    entity: String,
    location: [usize;2],
    value: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    confidence: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<UnknownType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    groups: Option<Vec<CapturedGrouup>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    interpretation: Option<Interpretation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    alternatives: Option<Vec<AlternativeEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    role: Option<EntityRole>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CapturedGrouup {
    group: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    location: Option<(usize, usize)>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Interpretation {
    #[serde(skip_serializing_if = "Option::is_none")]
    calendar_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    datetime_link: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    festival: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    granularity: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    range_link: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    range_modifier: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    relative_day: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    relative_month: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    relative_week: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    relative_weekend: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    relative_year: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    specific_day: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    specific_day_of_week: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    specific_month: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    specific_quarter: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    specific_year: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    numeric_value: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    subtype: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    part_of_day: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    relative_hour: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    relative_minute: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    relative_second: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    specific_hour: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    specific_minute: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    specific_second: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    timezone: Option<String>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AlternativeEntity {
    value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    confidence: Option<f32>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EntityRole {
    #[serde(rename = "type")]
    entity_type: Option<String>
}

#[derive(Debug, Serialize)]
pub struct InputMessage<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    message_type: Option<InputType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    options: Option<InputOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    intents: Option<&'a [&'a Intent]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    entities: Option<Vec<Entity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    suggestion_id: Option<&'a str>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ContextGlobalSystem {
    #[serde(skip_serializing_if = "Option::is_none")]
    timezone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    turn_count: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    locale: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reference_time: Option<String>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ContextGlobal {
    #[serde(skip_serializing_if = "Option::is_none")]
    system: Option<ContextGlobalSystem>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ContextSkill {
    #[serde(skip_serializing_if = "Option::is_none")]
    user_defined: Option<UnknownType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    system: Option<HashMap<String, String>>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Context {
    #[serde(skip_serializing_if = "Option::is_none")]
    global: Option<ContextGlobal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    skills: Option<HashMap<String, ContextSkill>>
}

#[derive(Debug, Serialize)]
pub struct UserInput<'a> {
    input: InputMessage<'a>,
    #[serde(skip_serializing_if = "Option::is_none")]
    context: Option<Context>
}

impl<'a> UserInput<'a> {
    /// Attach a context to this input. It will consume current input
    /// and move all input into new UserInput with new context.
    pub fn attach(self, context: Context) -> UserInput<'a> {
        UserInput {
            input: self.input,
            context: Some(context)
        }
    }
}

#[derive(Debug)]
pub struct UserInputBuilder<'a> {
    user_input: UserInput<'a>
}

impl<'a> UserInputBuilder<'a> {
    fn builder() -> UserInputBuilder<'a> {
        UserInputBuilder {
            user_input: UserInput {
                input: InputMessage {
                    message_type: None,
                    text: None,
                    options: None,
                    intents: None,
                    entities: None,
                    suggestion_id: None
                },
                context: None
            }
        }
    }

    pub fn message_type<'b: 'a>(mut self, msg_type: InputType) -> UserInputBuilder<'a> {
        self.user_input.input.message_type = Some(msg_type);
        self
    }

    pub fn text<'b: 'a>(mut self, text: &'b str) -> UserInputBuilder<'a> {
        self.user_input.input.text = Some(text);
        self
    }

    pub fn options(mut self, options: InputOptions) -> UserInputBuilder<'a> {
        self.user_input.input.options = Some(options);
        self
    }

    pub fn intents(mut self, intents: &'a[&'a Intent]) -> UserInputBuilder<'a> {
        self.user_input.input.intents = Some(intents);
        self
    }

    pub fn suggestion_id(mut self, suggestion_id: &'a str) -> UserInputBuilder<'a> {
        self.user_input.input.suggestion_id = Some(suggestion_id);
        self
    }

    pub fn entities(self, entities: Vec<Entity>) -> UserInputBuilder<'a> {
        UserInputBuilder {
            user_input: UserInput {
                input: InputMessage {
                    message_type: self.user_input.input.message_type,
                    text: self.user_input.input.text,
                    options: self.user_input.input.options,
                    intents: self.user_input.input.intents,
                    entities: Some(entities),
                    suggestion_id: self.user_input.input.suggestion_id
                },
                context: self.user_input.context
            }
        }
    }

    pub fn context(self, context: Context) -> UserInputBuilder<'a> {
        UserInputBuilder {
            user_input: UserInput {
                input: self.user_input.input,
                context: Some(context)
            }
        }
    }

    pub fn build(self) -> UserInput<'a> {
        self.user_input
    }
}

#[derive(Debug)]
pub struct EntityBuilder {
    entity: Entity
}

impl<'a> EntityBuilder {
    pub fn builder(entity: String, location: [usize;2], value: String) -> EntityBuilder {
        EntityBuilder {
            entity: Entity {
                entity: entity,
                location: location,
                value: value,
                confidence: None,
                metadata: None,
                groups: None,
                interpretation: None,
                alternatives: None,
                role: None
            }
        }
    }

    pub fn confidence(mut self, c: f32) -> EntityBuilder {
        self.entity.confidence = Some(c);
        self
    }

    pub fn metadata(self, meta: UnknownType) -> EntityBuilder {
        EntityBuilder {
            entity: Entity {
                entity: self.entity.entity,
                location: self.entity.location,
                value: self.entity.value,
                confidence: self.entity.confidence,
                metadata: Some(meta),
                groups: self.entity.groups,
                interpretation: self.entity.interpretation,
                alternatives: self.entity.alternatives,
                role: self.entity.role
            }
        }
    }

    pub fn groups(mut self, groups: Vec<CapturedGrouup>) -> EntityBuilder {
        self.entity.groups = Some(groups);
        self
    }

    pub fn interpretation(mut self, interpretation: Interpretation) -> EntityBuilder {
        self.entity.interpretation = Some(interpretation);
        self
    }

    pub fn alternatives(mut self, alternatives: Vec<AlternativeEntity>) -> EntityBuilder {
        self.entity.alternatives = Some(alternatives);
        self
    }

    pub fn role(mut self, role: EntityRole) -> EntityBuilder {
        self.entity.role = Some(role);
        self
    }

    pub fn build(self) -> Entity {
        self.entity
    }
}


#[derive(Debug)]
pub struct ContextBuilder {
    timezone: Option<String>,
    user_id: Option<String>,
    turn_count: Option<usize>,
    locale: Option<String>,
    reference_time: Option<String>,
    user_defined: Option<UnknownType>
}

impl ContextBuilder {

    pub fn builder() -> ContextBuilder {
        ContextBuilder {
            timezone: None,
            user_id: None,
            turn_count: None,
            locale: None,
            reference_time: None,
            user_defined: None
        }
    }

    pub fn timezone(mut self, timezone_name: String) -> ContextBuilder {
        self.timezone = Some(timezone_name);
        self
    }

    pub fn user_id(mut self, user_id: String) -> ContextBuilder {
        self.user_id = Some(user_id);
        self
    }

    pub fn turn_count(mut self, count: usize) -> ContextBuilder {
        self.turn_count = Some(count);
        self
    }

    pub fn locale(mut self, locale: String) -> ContextBuilder {
        self.locale = Some(locale);
        self
    }

    pub fn reference_time(mut self, reference_time: String) -> ContextBuilder {
        self.reference_time = Some(reference_time);
        self
    }

    pub fn user_defined(self, context: UnknownType) -> ContextBuilder {
        ContextBuilder {
            timezone: self.timezone,
            user_id: self.user_id,
            turn_count: self.turn_count,
            locale: self.locale,
            reference_time: self.reference_time,
            user_defined: Some(context)
        }
    }

    pub fn build(self) -> Context {
        let mut skills = HashMap::new();
        skills.insert("main skill".to_owned(), ContextSkill {
            user_defined: self.user_defined,
            system: None
        });
        Context {
            global: Some(ContextGlobal {
                system: Some(ContextGlobalSystem {
                    timezone: self.timezone,
                    user_id: self.user_id,
                    turn_count: self.turn_count,
                    locale: self.locale,
                    reference_time: self.reference_time
                })
            }),
            skills: Some(skills)
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ResponseType {
    Text,
    Pause,
    Image,
    Option,
    ConnectToAgent,
    Suggestion,
    Search
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum OptionPreference {
    Dropdown,
    Button
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum InputType {
    Text
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OptionInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_type: Option<InputType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<InputOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intents: Option<Vec<Intent>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entities: Option<Vec<Entity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggestion_id: Option<String>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OptionElm {
    pub input: OptionInput
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OutputOption {
    pub label: String,
    pub value: OptionElm
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SuggestionInputType {
    Text,
    Option,
    Image
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SuggestionInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_type: Option<SuggestionInputType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<OutputOption>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intents: Option<Vec<Intent>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entities: Option<Vec<Entity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggestion_id: Option<String>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GenericSuggestion {
    #[serde(skip_serializing_if = "Option::is_none")]
    generic: Option<Vec<SuggestionInput>>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Suggestion {
    pub label: String,
    pub value: OptionElm,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<GenericSuggestion>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ResultMetadata {
    pub confidence: f64,
    pub score: f64
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SearchResult {
    pub id: String,
    pub result_metadata: ResultMetadata,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub highlight: Option<HashMap<String, Vec<String>>>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ResponseGeneric {
    pub response_type: ResponseType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub typing: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preference: Option<OptionPreference>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<OutputOption>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_to_human_agent: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggestions: Option<Vec<Suggestion>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<SearchResult>>
}

#[derive(Debug, Deserialize, Serialize)]
pub enum ActionType {
    #[serde(rename = "client")]
    Client,
    #[serde(rename = "server")]
    Server,
    #[serde(rename = "web-action")]
    WebAction,
    #[serde(rename = "cloud-function")]
    CloudFunction
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Action {
    pub name: String,
    pub result_variable: String,
    #[serde(rename = "type")]
    pub action_type: ActionType,
    pub parameters: Box<RawValue>,
    pub credentials: String
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NodeInfo {
    pub dialog_node: String,
    pub title: String,
    pub conditions: String
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LogInfo {
    pub level: String,
    pub message: String
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum BranchExitReason {
    Completed,
    Fallback
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DebugInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_visisted: Option<Vec<NodeInfo>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log_messages: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch_exited: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch_exited_reason: Option<BranchExitReason>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GenericMessageOutput {
    pub generic: Vec<ResponseGeneric>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intents: Option<Vec<Intent>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entities: Option<Vec<Entity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<Action>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub debug: Option<DebugInfo>
}

// /// A response where it doesn't return any meta, nor actions parameters, nor context
// pub type SimpleWAResponse = WAResponse<(), (), (), (), (), ()>;

// /// A response that will have context associated but have no meta nor actions parameters
// pub type WAResponseWithContext<C> = WAResponse<(), (), (), (), (), C>;

#[derive(Debug, Deserialize, Serialize)]
pub struct WAResponse {
    pub output: GenericMessageOutput,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<Context>
}

#[cfg(test)]
mod test;