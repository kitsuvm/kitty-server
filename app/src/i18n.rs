//! This module provides functionality for internationalization (i18n) in the application.

use i18n_embed::{
    DesktopLanguageRequester,
    fluent::{FluentLanguageLoader, fluent_language_loader},
};
use rust_embed::Embed;

use crate::{Error, config::application::Language};

#[derive(Embed)]
#[folder = "i18n"]
/// A struct that represents the embedded locales for the application.
struct Locales;

/// Initializes the i18n system;
pub fn init(language: Language) -> Result<FluentLanguageLoader, Error> {
    let loader = fluent_language_loader!();

    let requested_languages = if language == Language::System {
        DesktopLanguageRequester::requested_languages()
    } else {
        vec![language.into()]
    };

    i18n_embed::select(&loader, &Locales, &requested_languages)
        .map(|_| loader)
        .map_err(|e| {
            tracing::error!("Failed to initialize i18n: {}", e);
            Error::I18nInit
        })
}
