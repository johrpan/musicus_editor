use crate::editors::RecordingEditor;
use crate::navigator::{NavigationHandle, NavigatorWindow, Screen};
use crate::widgets;
use crate::widgets::{List, Section, Widget};
use gettextrs::gettext;
use glib::clone;
use gtk::prelude::*;
use libadwaita::prelude::*;
use musicus_backend::db::{Recording, Track};
use std::cell::RefCell;
use std::rc::Rc;

/// A screen for showing a recording.
pub struct RecordingScreen {
    handle: NavigationHandle<()>,
    recording: Recording,
    widget: widgets::Screen,
    list: Rc<List>,
    tracks: RefCell<Vec<Track>>,
}

impl Screen<Recording, ()> for RecordingScreen {
    /// Create a new recording screen for the specified recording and load the
    /// contents asynchronously.
    fn new(recording: Recording, handle: NavigationHandle<()>) -> Rc<Self> {
        let widget = widgets::Screen::new();
        widget.set_title(&recording.work.get_title());
        widget.set_subtitle(&recording.get_performers());

        let list = List::new();
        let section = Section::new(&gettext("Tracks"), &list.widget);
        widget.add_content(&section.widget);

        let this = Rc::new(Self {
            handle,
            recording,
            widget,
            list,
            tracks: RefCell::new(Vec::new()),
        });

        section.add_action(
            "media-playback-start-symbolic",
            clone!(@weak this =>  move || {
                for track in &*this.tracks.borrow() {
                    this.handle.backend.pl().add_item(track.clone()).unwrap();
                }
            }),
        );

        this.widget.set_back_cb(clone!(@weak this =>  move || {
            this.handle.pop(None);
        }));

        this.widget.add_action(
            &gettext("Edit recording"),
            clone!(@weak this =>  move || {
                spawn!(@clone this, async move {
                    let window = NavigatorWindow::new(this.handle.backend.clone());
                    replace!(window.navigator, RecordingEditor, Some(this.recording.clone())).await;
                });
            }),
        );

        this.widget.add_action(
            &gettext("Delete recording"),
            clone!(@weak this =>  move || {
                spawn!(@clone this, async move {
                    this.handle.backend.db().delete_recording(&this.recording.id).await.unwrap();
                    this.handle.backend.library_changed();
                });
            }),
        );

        this.list
            .set_make_widget_cb(clone!(@weak this =>  @default-panic, move |index| {
                let track = &this.tracks.borrow()[index];

                let mut title_parts = Vec::<String>::new();
                for part in &track.work_parts {
                    title_parts.push(this.recording.work.parts[*part].title.clone());
                }

                let title = if title_parts.is_empty() {
                    gettext("Unknown")
                } else {
                    title_parts.join(", ")
                };

                let row = libadwaita::ActionRow::new();
                row.set_title(Some(&title));

                row.upcast()
            }));

        // Load the content asynchronously.

        spawn!(@clone this, async move {
            let tracks = this.handle
                .backend
                .db()
                .get_tracks(&this.recording.id)
                .await
                .unwrap();

            this.show_tracks(tracks);
            this.widget.ready();
        });

        this
    }
}

impl RecordingScreen {
    /// Update the tracks variable as well as the user interface.
    fn show_tracks(&self, tracks: Vec<Track>) {
        let length = tracks.len();
        self.tracks.replace(tracks);
        self.list.update(length);
    }
}

impl Widget for RecordingScreen {
    fn get_widget(&self) -> gtk::Widget {
        self.widget.widget.clone().upcast()
    }
}
