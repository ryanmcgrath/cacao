//! Implements an example toolbar for a Preferences app. Could be cleaner, probably worth cleaning
//! up at some point.

use cacao::macos::toolbar::{Toolbar, ToolbarDelegate, ToolbarItem};
use cacao::image::{Image, MacSystemIcon};

use crate::storage::{dispatch_ui, Message};

#[derive(Debug)]
pub struct PreferencesToolbar((ToolbarItem, ToolbarItem));

impl Default for PreferencesToolbar {
    fn default() -> Self {
        PreferencesToolbar(({
            let mut item = ToolbarItem::new("general");
            item.set_title("General");

            let icon = Image::system_icon(MacSystemIcon::PreferencesGeneral, "General");
            item.set_image(icon);
            
            item.set_action(|| {
                dispatch_ui(Message::SwitchPreferencesToGeneralPane);
            });

            item
        }, {
            let mut item = ToolbarItem::new("advanced");
            item.set_title("Advanced");
            
            let icon = Image::system_icon(MacSystemIcon::PreferencesAdvanced, "Advanced");
            item.set_image(icon);
            
            item.set_action(|| {
                dispatch_ui(Message::SwitchPreferencesToAdvancedPane);
            });
            
            item
        }))
    }
}

impl ToolbarDelegate for PreferencesToolbar {
    const NAME: &'static str = "PreferencesToolbar";
    
    fn did_load(&mut self, toolbar: Toolbar) {
        toolbar.set_selected("general");
    }

    fn allowed_item_identifiers(&self) -> Vec<&'static str> {
        vec!["general", "advanced"]
    }

    fn default_item_identifiers(&self) -> Vec<&'static str> {
        vec!["general", "advanced"]
    }

    fn selectable_item_identifiers(&self) -> Vec<&'static str> {
        vec!["general", "advanced"]
    }

    fn item_for(&self, identifier: &str) -> &ToolbarItem {
        match identifier {
            "general" => &self.0.0,
            "advanced" => &self.0.1,
            _ => { unreachable!(); }
        }
    }
}
