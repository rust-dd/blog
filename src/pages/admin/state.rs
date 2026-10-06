use crate::{
    app::Route,
    ssr::admin::{
        admin_delete, admin_publish, admin_save,
        types::{AdminPost, PostInput},
    },
};
use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct EditorState {
    pub post: Option<AdminPost>,
    pub input: PostInput,
    pub saved: PostInput,
    pub busy: bool,
    pub error: String,
    pub notice: String,
    pub confirm_delete: bool,
}

impl EditorState {
    pub fn dirty(&self) -> bool {
        self.input != self.saved
    }
    pub fn published(&self) -> bool {
        self.post.as_ref().is_some_and(|post| post.is_published)
    }
}

#[derive(Clone, Copy)]
pub enum Action {
    Save,
    Publish,
    Unpublish,
}

pub async fn perform(mut state: Signal<EditorState>, action: Action) {
    if state().busy {
        return;
    }
    let snapshot = state();
    let id = snapshot.post.as_ref().map(|post| post.id.clone());
    if matches!(action, Action::Unpublish) && id.is_none() {
        return;
    }
    state.write().busy = true;
    state.write().error.clear();
    state.write().notice.clear();
    let mut saved_for_publication = false;
    let result = match action {
        Action::Unpublish => admin_publish(id.unwrap(), false).await,
        Action::Save | Action::Publish => match admin_save(id.clone(), snapshot.input.clone()).await {
            Ok(post) if matches!(action, Action::Publish) => {
                {
                    let mut next = state.write();
                    next.input = post.input.clone();
                    next.saved = post.input.clone();
                    next.post = Some(post.clone());
                }
                saved_for_publication = true;
                admin_publish(post.id, true).await
            }
            result => result,
        },
    };
    state.write().busy = false;
    match result {
        Ok(post) => {
            let mut next = state.write();
            if !matches!(action, Action::Unpublish) {
                next.input = post.input.clone();
            }
            next.saved = post.input.clone();
            next.post = Some(post.clone());
            next.notice = match action {
                Action::Save => "Saved.",
                Action::Publish => "Published. Your post is live.",
                Action::Unpublish => "Moved to drafts. This post is private.",
            }
            .into();
            drop(next);
            if !matches!(action, Action::Unpublish) {
                navigator().replace(Route::AdminEdit { id: post.id });
            }
        }
        Err(error) => {
            let mut next = state.write();
            next.error = super::error_message(&error.into());
            if saved_for_publication {
                next.notice = "Your draft is saved. You can try publishing again.".into();
            }
        }
    }
}

pub async fn remove(mut state: Signal<EditorState>) {
    if state().busy {
        return;
    }
    let Some(post) = state().post else { return };
    state.write().busy = true;
    match admin_delete(post.id).await {
        Ok(()) => {
            let input = state().input;
            state.write().saved = input;
            let _ = document::eval("window.rdAdminDirty = false;");
            navigator().replace(Route::Admin {});
        }
        Err(error) => {
            state.write().busy = false;
            state.write().confirm_delete = false;
            state.write().error = super::error_message(&error.into());
        }
    }
}
