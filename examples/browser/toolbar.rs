use cacao::objc::{msg_send, sel, sel_impl};

use cacao::button::Button;
use cacao::input::{TextField, TextFieldDelegate};

use cacao::appkit::toolbar::{ItemIdentifier, Toolbar, ToolbarDelegate, ToolbarDisplayMode, ToolbarItem};

use super::Action;

const BACK_BUTTON: &str = "BackButton";
const FWDS_BUTTON: &str = "FwdsButton";
const URL_BAR: &str = "URLBar";

#[derive(Debug)]
pub struct URLBar;

impl TextFieldDelegate for URLBar {
    const NAME: &'static str = "URLBar";

    fn text_did_end_editing(&self, value: &str) {
        Action::Load(value.to_string()).dispatch();
    }
}

#[derive(Debug)]
pub struct BrowserToolbar {
    back_item: ToolbarItem,
    forwards_item: ToolbarItem,
    url_bar: TextField<URLBar>,
    url_bar_item: ToolbarItem
}

impl BrowserToolbar {
    pub fn new() -> Self {
        let back_button = Button::new("Back");
        let mut back_item = ToolbarItem::new(BACK_BUTTON);
        back_item.set_button(back_button);
        back_item.set_action(|| Action::Back.dispatch());

        let forwards_button = Button::new("Forwards");
        let mut forwards_item = ToolbarItem::new(FWDS_BUTTON);
        forwards_item.set_button(forwards_button);
        forwards_item.set_action(|| Action::Forwards.dispatch());

        let url_bar = TextField::with(URLBar);
        let url_bar_item = ToolbarItem::new(URL_BAR);

        // We cheat for now to link these, as there's no API for Toolbar yet
        // to support arbitrary view types. The framework is designed to support this kind of
        // cheating, though: it's not outlandish to need to just manage things yourself when it
        // comes to Objective-C/AppKit sometimes.
        //
        // As long as we keep hold of things here and they all drop together, it's relatively safe.
        url_bar.objc.with_mut(|obj| unsafe {
            let _: () = msg_send![&*url_bar_item.objc, setView:&*obj];
        });

        BrowserToolbar {
            back_item,
            forwards_item,
            url_bar,
            url_bar_item
        }
    }

    pub fn set_url(&self, url: &str) {
        self.url_bar.set_text(url);
    }

    fn item_identifiers(&self) -> Vec<ItemIdentifier> {
        vec![
            ItemIdentifier::Custom(BACK_BUTTON),
            ItemIdentifier::Custom(FWDS_BUTTON),
            ItemIdentifier::Space,
            ItemIdentifier::Custom(URL_BAR),
            ItemIdentifier::Space,
        ]
    }
}

impl ToolbarDelegate for BrowserToolbar {
    const NAME: &'static str = "BrowserToolbar";

    fn did_load(&mut self, toolbar: Toolbar) {
        toolbar.set_display_mode(ToolbarDisplayMode::IconOnly);
    }

    fn allowed_item_identifiers(&self) -> Vec<ItemIdentifier> {
        self.item_identifiers()
    }

    fn default_item_identifiers(&self) -> Vec<ItemIdentifier> {
        self.item_identifiers()
    }

    fn item_for(&self, identifier: &str) -> &ToolbarItem {
        match identifier {
            BACK_BUTTON => &self.back_item,
            FWDS_BUTTON => &self.forwards_item,
            URL_BAR => &self.url_bar_item,
            _ => {
                std::unreachable!();
            }
        }
    }
}
