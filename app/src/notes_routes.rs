use askama_axum::Template;
use axum::{
    body::Body,
    response::{IntoResponse, Response},
    Form,
    extract::Path,
};
use axum::extract::Query;
use pulldown_cmark::{Event, html, Options, Parser};
use serde::{Deserialize, Serialize};
use crate::{
    notes_model::Note,
    notes_service::NotesService,
};

use validator::{Validate};
use crate::axum_extractors::UserId;
use axum_cloudflare_adapter_macros::worker_route_compat;
use http::header::CONTENT_TYPE;

#[cfg(feature = "spa")]
const ENABLE_SPA: bool = true;

#[cfg(not(feature = "spa"))]
const ENABLE_SPA: bool = false;


pub struct BaseTemplate {
    pub enable_spa: bool,
}

impl Default for BaseTemplate {
    fn default() -> Self {
        BaseTemplate {
            enable_spa: ENABLE_SPA,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NoteListItem {
    pub id: i64,
    pub title: String,
}

impl NoteListItem {
    pub fn from(note: &Note) -> Self {
        NoteListItem {
            id: note.id,
            title: first_20_chars(&note.content),
        }
    }
}

#[derive(Validate, Debug, Serialize, Deserialize, Clone, Default)]
pub struct NoteForm {
    pub id: Option<i64>,

    #[validate(length(min = 1, message = "Content is too short. It must be at least 1 characters long."))]
    #[validate(length(max = 1000, message = "Content is too long. It must be no more than 1000 characters long."))]
    pub content: String,

    pub content_error: Option<String>,
}

impl NoteForm {
    pub fn from(note: &Note) -> Self {
        NoteForm {
            id: Some(note.id),
            content: note.content.clone(),
            content_error: None,
        }
    }
}


impl NoteForm {
    pub fn is_valid(&mut self) -> bool {
        let result = self.validate();
        if result.is_err() {
            self.content_error = Some(result.unwrap_err().to_string());
            false
        } else {
            self.content_error = None;
            true
        }
    }
}

#[derive(Template)]
#[template(path = "notes/index.html")]
pub struct IndexTemplate {
    pub note_list: Vec<NoteListItem>,
    pub note_form: NoteForm,
    pub base_template: BaseTemplate,
}

fn content_to_markdown(content: &str) -> String {
    let parser = Parser::new(content);
    let mut markdown_output = String::new();

    let filtered_parser = parser.into_iter().filter(|event| {
        !matches!(event, Event::Html(ref html) | Event::Html(ref html) if html.contains("<script"))
    });

    html::push_html(&mut markdown_output, filtered_parser);
    markdown_output
}

fn first_20_chars(markdown_input: &str) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    let parser = Parser::new_ext(markdown_input, options);

    let mut plain_text = String::new();
    const LENGTH: usize = 20;

    for event in parser {
        match event {
            Event::Text(text) => plain_text.push_str(&text),
            Event::Code(code) => plain_text.push_str(&code),
            _ => {}
        }
        if plain_text.len() >= LENGTH {
            break;
        }
    }

    plain_text.truncate(LENGTH);
    plain_text
}

#[worker_route_compat]
pub async fn index(
    user_id: UserId,
    note_service: NotesService,
) -> impl IntoResponse {
    let notes = note_service.all_notes(user_id.0).await.unwrap();

    IndexTemplate {
        note_list: notes.iter().map(NoteListItem::from).collect(),
        note_form: NoteForm::default(),
        base_template: Default::default(),
    }
}

#[worker_route_compat]
pub async fn create_note(
    user_id: UserId,
    note_service: NotesService,
    note_form: Form<NoteForm>,
) -> impl IntoResponse {
    let mut note_form = note_form.0;

    if !note_form.is_valid() {
        let notes = note_service.all_notes(user_id.0).await.unwrap();

        let index_template = IndexTemplate {
            note_list: notes.iter().map(NoteListItem::from).collect(),
            note_form,
            base_template: Default::default(),
        };

        let html = index_template.render().unwrap();

        Response::builder()
            .status(200)
            .header(CONTENT_TYPE, "text/html")
            .body(html.into())
            .unwrap()
    } else {
        let note = note_service.create_note(
            note_form.content,
            user_id.0,
        ).await.unwrap();

        let location = format!("/show/{}", note.id);

        Response::builder()
            .header("Location", location)
            .status(303)
            .body(Body::empty())
            .unwrap()
    }
}

#[worker_route_compat]
pub async fn update_note(
    user_id: UserId,
    note_service: NotesService,
    note_form: Form<NoteForm>,
) -> impl IntoResponse {
    let mut note_form = note_form.0;
    if !note_form.is_valid() {
        let notes = note_service.all_notes(user_id.0).await.unwrap();

        let index_template = IndexTemplate {
            note_list: notes.iter().map(NoteListItem::from).collect(),
            note_form,
            base_template: Default::default(),
        };

        let html = index_template.render().unwrap();

        Response::builder()
            .status(200)
            .body(html.into())
            .unwrap()
    } else {
        let note_id = note_form.id.unwrap();
        note_service.update_note(note_form.content, note_id, user_id.0).await.unwrap();
        let location = format!("/show/{}", note_id);

        Response::builder()
            .header("Location", location)
            .status(303)
            .body(Body::empty())
            .unwrap()
    }
}

#[worker_route_compat]
pub async fn show_note(
    Path(id): Path<i64>,
    user_id: UserId,
    note_service: NotesService,
) -> impl IntoResponse {
    let notes = note_service.all_notes(user_id.0).await.unwrap();
    let note_by_id = note_service.by_id(id, user_id.0).await;

    if let Ok(note) = note_by_id {
        let preview = content_to_markdown(&note.content);

        let show_template = ShowTemplate {
            note_list: notes.iter().map(NoteListItem::from).collect(),
            preview,
            selected_note: note,
            base_template: Default::default(),
        };

        let html: String = show_template.render().unwrap();

        Response::builder()
            .status(200)
            .header(CONTENT_TYPE, "text/html")
            .body(html.into())
            .unwrap()
    } else {
        Response::builder()
            .status(404)
            .header(CONTENT_TYPE, "text/html")
            .body(Body::from("Note not found"))
            .unwrap()
    }
}


#[derive(Template)]
#[template(path = "notes/show.html")]
pub struct ShowTemplate {
    pub note_list: Vec<NoteListItem>,
    pub preview: String,
    pub selected_note: Note,
    pub base_template: BaseTemplate,
}


#[worker_route_compat]
pub async fn edit_note(
    Path(id): Path<i64>,
    user_id: UserId,
    note_service: NotesService,
) -> impl IntoResponse {
    let notes = note_service.all_notes(user_id.0).await.unwrap();
    let note_by_id = notes.iter().find(|note| note.id == id).cloned();

    if let Some(note) = note_by_id {
        let show_template = EditTemplate {
            note_list: notes.iter().map(NoteListItem::from).collect(),
            note_form: NoteForm::from(&note),
            base_template: Default::default(),
        };

        let html: String = show_template.render().unwrap();

        Response::builder()
            .status(200)
            .header(CONTENT_TYPE, "text/html")
            .body(html.into())
            .unwrap()
    } else {
        Response::builder()
            .status(404)
            .header(CONTENT_TYPE, "text/html")
            .body(Body::from("Note not found"))
            .unwrap()
    }
}

#[derive(Template)]
#[template(path = "notes/edit.html")]
pub struct EditTemplate {
    pub note_list: Vec<NoteListItem>,
    pub note_form: NoteForm,
    pub base_template: BaseTemplate,
}


#[derive(Deserialize)]
pub struct SearchQuery {
    search: String,
}

#[worker_route_compat]
pub async fn search_note(
    Query(SearchQuery { search }): Query<SearchQuery>,
    user_id: UserId,
    note_service: NotesService,
) -> impl IntoResponse {
    let notes = note_service.all_notes(user_id.0).await.unwrap();

    let filtered_notes: Vec<NoteSearchPreview> = notes
        .iter()
        .filter(|note| note.content.to_lowercase().contains(&search.to_lowercase()))
        .map(|note| {
            let preview = content_to_markdown(&note.content);
            NoteSearchPreview {
                id: note.id,
                preview,
            }
        })
        .collect();

    let search_template = SearchTemplate {
        note_list: notes.iter().map(NoteListItem::from).collect(),
        filtered_notes,
        search,
        base_template: Default::default(),
    };
    let html = search_template.render().unwrap();

    Response::builder()
        .status(200)
        .header(CONTENT_TYPE, "text/html")
        .body(html)
        .unwrap()
}

pub struct NoteSearchPreview {
    pub id: i64,
    pub preview: String,
}


#[derive(Template)]
#[template(path = "notes/search.html")]
pub struct SearchTemplate {
    pub note_list: Vec<NoteListItem>,
    pub filtered_notes: Vec<NoteSearchPreview>,
    pub search: String,
    pub base_template: BaseTemplate,
}

