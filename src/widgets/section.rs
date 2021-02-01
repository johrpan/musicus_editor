use gtk::prelude::*;
use gtk_macros::get_widget;

/// A widget displaying a title, a framed child widget and, if needed, some
/// actions.
pub struct Section {
    /// The actual GTK widget.
    pub widget: gtk::Box,

    /// The box containing the title and action buttons.
    title_box: gtk::Box,
}

impl Section {
    /// Create a new section.
    pub fn new<W: IsA<gtk::Widget>>(title: &str, content: &W) -> Self {
        let builder = gtk::Builder::from_resource("/de/johrpan/musicus/ui/section.ui");

        get_widget!(builder, gtk::Box, widget);
        get_widget!(builder, gtk::Box, title_box);
        get_widget!(builder, gtk::Label, title_label);
        get_widget!(builder, gtk::Frame, frame);

        title_label.set_label(title);
        frame.set_child(Some(content));

        Self {
            widget,
            title_box,
        }
    }

    /// Add an action button. This should by definition be something that is
    /// doing something with the child widget that is applicable in all
    /// situations where the widget is visible. The new button will be packed
    /// to the end of the title box.
    pub fn add_action<F: Fn() + 'static>(&self, icon_name: &str, cb: F) {
        let button = gtk::ButtonBuilder::new()
            .has_frame(false)
            .valign(gtk::Align::Center)
            .icon_name(icon_name)
            .build();

        button.connect_clicked(move |_| cb());

        self.title_box.append(&button);
    }
}