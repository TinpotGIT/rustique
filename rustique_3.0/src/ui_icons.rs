use egui::{Color32, RichText};

pub struct ToolIcons;

impl ToolIcons {
    pub fn brush() -> RichText {
        RichText::new("🖌️").size(20.0)
    }
    
    pub fn eraser() -> RichText {
        RichText::new("🧹").size(20.0)
    }
    
    pub fn paint_bucket() -> RichText {
        RichText::new("🪣").size(20.0)
    }
    
    pub fn color_picker() -> RichText {
        RichText::new("🎨").size(20.0)
    }
    
    pub fn line() -> RichText {
        RichText::new("📏").size(20.0)
    }
    
    pub fn undo() -> RichText {
        RichText::new("↶").size(18.0).color(Color32::WHITE)
    }
    
    pub fn redo() -> RichText {
        RichText::new("↷").size(18.0).color(Color32::WHITE)
    }
    
    pub fn save() -> RichText {
        RichText::new("💾").size(18.0)
    }
    
    pub fn home() -> RichText {
        RichText::new("🏠").size(18.0).color(Color32::WHITE)
    }
    
    pub fn layer_visible() -> RichText {
        RichText::new("👁").size(16.0)
    }
    
    pub fn layer_hidden() -> RichText {
        RichText::new("🚫").size(16.0)
    }
    
    pub fn add() -> RichText {
        RichText::new("➕").size(16.0).color(Color32::WHITE)
    }
    
    pub fn remove() -> RichText {
        RichText::new("➖").size(16.0).color(Color32::WHITE)
    }
    
    pub fn move_up() -> RichText {
        RichText::new("⬆").size(16.0).color(Color32::WHITE)
    }
    
    pub fn move_down() -> RichText {
        RichText::new("⬇").size(16.0).color(Color32::WHITE)
    }
    
    pub fn edit() -> RichText {
        RichText::new("✏️").size(14.0)
    }
}
