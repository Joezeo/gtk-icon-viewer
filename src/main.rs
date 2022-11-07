use std::sync::atomic::AtomicBool;

use gtk::{
    glib::{self, clone, Type},
    prelude::*,
    Application, ApplicationWindow, CellAreaBox, CellRendererPixbuf, CellRendererText,
    GestureClick, ListStore, ScrolledWindow, TreeView, TreeViewColumn,
};

const APP_ID: &str = "com.toocol.icon_viewer";
static RIGHT_CLICKED: AtomicBool = AtomicBool::new(false);

fn main() {
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(build_ui);

    app.run();
}

fn build_ui(app: &Application) {
    let tree_view = TreeView::builder().build();

    setup_columns(&tree_view);

    let list_store = ListStore::new(&[Type::STRING, Type::STRING]);

    tree_view.set_model(Some(&list_store));

    for icon_name in ICON_NAMES {
        let iter = list_store.append();
        list_store.set_value(&iter, 0, &icon_name.to_value());
        list_store.set_value(&iter, 1, &icon_name.to_value());
    }

    let scrolled_window = ScrolledWindow::builder()
        .min_content_height(200)
        .min_content_height(400)
        .child(&tree_view)
        .build();

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Icon Viewer")
        .child(&scrolled_window)
        .build();

    tree_view.connect_cursor_changed(|tree_view| {
        if RIGHT_CLICKED.load(std::sync::atomic::Ordering::SeqCst) {
            let selection = tree_view.selection();
            if let Some((model, iter)) = selection.selected() {
                let icon_name = model
                    .get_value(&iter, 1)
                    .get::<String>()
                    .expect("TreeView selection column 1 error.");
                let clipboard = tree_view.clipboard();
                clipboard.set_text(&icon_name);
                println!("Icon name={}, clipboard is_local={}", icon_name, clipboard.is_local());
            }
            RIGHT_CLICKED.store(false, std::sync::atomic::Ordering::SeqCst);
        }
    });

    let gesture_right_click = GestureClick::new();
    gesture_right_click.set_button(3);
    gesture_right_click.connect_pressed(
        clone!(@weak tree_view => move |_gesture, n_press, _x, _y| {
            if n_press == 1 {
                RIGHT_CLICKED.store(true, std::sync::atomic::Ordering::SeqCst);
            }
        }),
    );
    tree_view.add_controller(&gesture_right_click);

    window.present();
}

fn setup_columns(tree_view: &TreeView) {
    let cell_renderer = CellRendererPixbuf::new();
    let cell_area = CellAreaBox::builder()
        .orientation(gtk::Orientation::Vertical)
        .build();
    cell_area.pack_end(&cell_renderer, true, true, true);
    let column = TreeViewColumn::builder()
        .cell_area(&cell_area)
        .title("icon")
        .build();
    column.add_attribute(&cell_renderer, "icon-name", 0);
    tree_view.append_column(&column);

    let cell_renderer = CellRendererText::new();
    let cell_area = CellAreaBox::builder()
        .orientation(gtk::Orientation::Vertical)
        .build();
    cell_area.pack_end(&cell_renderer, true, true, true);
    let column = TreeViewColumn::builder()
        .cell_area(&cell_area)
        .title("icon-name")
        .build();
    column.add_attribute(&cell_renderer, "text", 1);
    tree_view.append_column(&column);
}

const ICON_NAMES: [&str; 275] = [
    "application-exit",
    "appointment-new",
    "call-start",
    "call-stop",
    "contact-new",
    "document-new",
    "document-open",
    "document-open-recent",
    "document-page-setup",
    "document-print",
    "document-print-preview",
    "document-properties",
    "document-revert",
    "document-save",
    "document-save-as",
    "document-send",
    "edit-clear",
    "edit-copy",
    "edit-cut",
    "edit-delete",
    "edit-find",
    "edit-find-replace",
    "edit-paste",
    "edit-redo",
    "edit-select-all",
    "edit-undo",
    "folder-new",
    "format-indent-less",
    "format-indent-more",
    "format-justify-center",
    "format-justify-fill",
    "format-justify-left",
    "format-justify-right",
    "format-text-direction-ltr",
    "format-text-direction-rtl",
    "format-text-bold",
    "format-text-italic",
    "format-text-underline",
    "format-text-strikethrough",
    "go-bottom",
    "go-down",
    "go-first",
    "go-home",
    "go-jump",
    "go-last",
    "go-next",
    "go-previous",
    "go-top",
    "go-up",
    "help-about",
    "help-contents",
    "help-faq",
    "insert-image",
    "insert-link",
    "insert-object",
    "insert-text",
    "list-add",
    "list-remove",
    "mail-forward",
    "mail-mark-important",
    "mail-mark-junk",
    "mail-mark-notjunk",
    "mail-mark-read",
    "mail-mark-unread",
    "mail-message-new",
    "mail-reply-all",
    "mail-reply-sender",
    "mail-send",
    "mail-send-receive",
    "media-eject",
    "media-playback-pause",
    "media-playback-start",
    "media-playback-stop",
    "media-record",
    "media-seek-backward",
    "media-seek-forward",
    "media-skip-backward",
    "media-skip-forward",
    "object-flip-horizontal",
    "object-flip-vertical",
    "object-rotate-left",
    "object-rotate-right",
    "process-stop",
    "system-lock-screen",
    "system-log-out",
    "system-run",
    "system-search",
    "system-reboot",
    "system-shutdown",
    "tools-check-spelling",
    "view-fullscreen",
    "view-refresh",
    "view-restore",
    "view-sort-ascending",
    "view-sort-descending",
    "window-close",
    "window-new",
    "zoom-fit-best",
    "zoom-in",
    "zoom-original",
    "zoom-out",
    "accessories-character-map",
    "accessories-dictionary",
    "accessories-text-editor",
    "help-browser",
    "multimedia-volume-control",
    "preferences-desktop-accessibility",
    "preferences-desktop-font",
    "preferences-desktop-keyboard",
    "preferences-desktop-locale",
    "preferences-desktop-multimedia",
    "preferences-desktop-screensaver",
    "preferences-desktop-theme",
    "preferences-desktop-wallpaper",
    "system-file-manager",
    "system-software-install",
    "system-software-update",
    "utilities-system-monitor",
    "utilities-terminal",
    "applications-development",
    "applications-engineering",
    "applications-games",
    "applications-graphics",
    "applications-internet",
    "applications-multimedia",
    "applications-office",
    "applications-other",
    "applications-science",
    "applications-system",
    "applications-utilities",
    "preferences-desktop",
    "preferences-desktop-peripherals",
    "preferences-desktop-personal",
    "preferences-other",
    "preferences-system",
    "preferences-system-network",
    "system-help",
    "audio-input-microphone",
    "battery",
    "camera-photo",
    "camera-video",
    "camera-web",
    "computer",
    "drive-harddisk",
    "drive-optical",
    "drive-removable-media",
    "input-gaming",
    "input-keyboard",
    "input-mouse",
    "input-tablet",
    "media-flash",
    "media-floppy",
    "media-optical",
    "media-tape",
    "modem",
    "multimedia-player",
    "network-wired",
    "network-wireless",
    "pda",
    "phone",
    "printer",
    "scanner",
    "video-display",
    "emblem-documents",
    "emblem-downloads",
    "emblem-favorite",
    "emblem-important",
    "emblem-mail",
    "emblem-photos",
    "emblem-readonly",
    "emblem-shared",
    "emblem-symbolic-link",
    "emblem-synchronized",
    "emblem-system",
    "emblem-unreadable",
    "face-angry",
    "face-cool",
    "face-crying",
    "face-devilish",
    "face-embarrassed",
    "face-kiss",
    "face-laugh",
    "face-monkey",
    "face-plain",
    "face-raspberry",
    "face-sad",
    "face-sick",
    "face-smile",
    "face-smile-big",
    "face-smirk",
    "face-surprise",
    "face-tired",
    "face-uncertain",
    "face-wink",
    "face-worried",
    "audio-x-generic",
    "font-x-generic",
    "image-x-generic",
    "package-x-generic",
    "text-html",
    "text-x-generic",
    "text-x-generic-template",
    "text-x-script",
    "video-x-generic",
    "x-office-address-book",
    "x-office-calendar",
    "x-office-document",
    "x-office-presentation",
    "x-office-spreadsheet",
    "folder-remote",
    "network-server",
    "network-workgroup",
    "start-here",
    "user-bookmarks",
    "user-desktop",
    "user-home",
    "user-trash",
    "appointment-soon",
    "audio-volume-high",
    "audio-volume-low",
    "audio-volume-medium",
    "audio-volume-muted",
    "battery-caution",
    "battery-low",
    "dialog-error",
    "dialog-information",
    "dialog-password",
    "dialog-question",
    "dialog-warning",
    "folder-drag-accept",
    "folder-open",
    "folder-visiting",
    "image-loading",
    "image-missing",
    "mail-attachment",
    "mail-unread",
    "mail-read",
    "mail-replied",
    "mail-signed",
    "mail-signed-verified",
    "media-playlist-repeat",
    "media-playlist-shuffle",
    "network-error",
    "network-idle",
    "network-offline",
    "network-receive",
    "network-transmit",
    "network-transmit-receive",
    "printer-error",
    "printer-printing",
    "security-high",
    "security-medium",
    "security-low",
    "software-update-available",
    "software-update-urgent",
    "sync-error",
    "sync-synchronizing",
    "task-due",
    "task-past-due",
    "user-available",
    "user-away",
    "user-idle",
    "user-offline",
    "user-trash-full",
    "weather-clear",
    "weather-clear-night",
    "weather-few-clouds",
    "weather-few-clouds-night",
    "weather-fog",
    "weather-overcast",
    "weather-severe-alert",
    "weather-showers",
    "weather-showers-scattered",
    "weather-snow",
    "weather-storm",
];
