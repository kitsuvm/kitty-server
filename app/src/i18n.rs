//! This module provides functionality for internationalization (i18n) in the application.

use std::{cell::RefCell, rc::Rc};

use i18n_embed::{
    DesktopLanguageRequester,
    fluent::{FluentLanguageLoader, fluent_language_loader},
};
use rust_embed::Embed;

use crate::{Error, resources::app_config::AppLanguage};

/// A type alias for a reference-counted, mutable `FluentLanguageLoader`.
pub type I18n = Rc<RefCell<FluentLanguageLoader>>;

#[derive(Embed)]
#[folder = "i18n"]
/// A struct that represents the embedded locales for the application.
pub struct Locales;

/// Initializes the i18n system;
pub fn init(language: AppLanguage) -> Result<I18n, Error> {
    let loader = fluent_language_loader!();

    let requested_languages = if language == AppLanguage::System {
        DesktopLanguageRequester::requested_languages()
    } else {
        vec![language.into()]
    };

    i18n_embed::select(&loader, &Locales, &requested_languages)
        .map(|_| Rc::new(RefCell::new(loader)))
        .map_err(|e| {
            tracing::error!("Failed to initialize i18n: {}", e);
            Error::I18nInit
        })
}

/// Changes the language of the application at runtime.
pub fn change_language(loader: &I18n, language: AppLanguage) -> Result<(), Error> {
    let requested_languages = if language == AppLanguage::System {
        DesktopLanguageRequester::requested_languages()
    } else {
        vec![language.into()]
    };

    i18n_embed::select(&*loader.borrow(), &Locales, &requested_languages)
        .map(|_| ())
        .map_err(|e| {
            tracing::error!("Failed to change language: {}", e);
            Error::I18nInit
        })
}
