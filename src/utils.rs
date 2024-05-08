use std::fs;

use chrono::{DateTime, Local};
use fltk::{
    app::Sender,
    browser::HoldBrowser,
    button::{Button, CheckButton},
    enums::CallbackTrigger,
    frame,
    input::Input,
    prelude::{BrowserExt, ButtonExt, InputExt, WidgetExt},
};
use fltk_theme::widget_themes;
use resolve_path::PathResolveExt;

use crate::{
    constants::{DATA_PATH, WIDGET_HEIGHT, WIDGET_PADDING, WIDGET_WIDTH},
    controller::TodolistRS,
    models::ListItem,
};

#[derive(Clone, Copy)]
pub enum Message {
    Create,
    Update,
    Delete,
    Select,
    Filter,
}

pub struct MainWindow {
    pub completed_input: CheckButton,
    pub create_button: Button,
    pub delete_button: Button,
    pub description_input: Input,
    pub filter_input: Input,
    pub list_browser: HoldBrowser,
    pub update_button: Button,
}

/// Load data from storage file
pub fn load_list_items() -> Vec<ListItem> {
    let data: Vec<u8> = fs::read(DATA_PATH.resolve())
        .map_err(|err| eprintln!("{err:?}"))
        .unwrap_or_default();
    // Bail since we found no data.
    if data.is_empty() {
        return vec![];
    }

    rmp_serde::from_slice::<Vec<ListItem>>(&data).unwrap()
}

/// Save the data to the storage file
pub fn dump_list_items(model: &Vec<ListItem>) {
    fs::write(DATA_PATH.resolve(), rmp_serde::to_vec(model).unwrap())
        .map_err(|err| eprintln!("{err:?}"))
        .unwrap_or_default();
}

/// Gets the current date and time
pub fn get_datetime() -> String {
    let current_local: DateTime<Local> = Local::now();
    current_local.format("%d-%m-%Y • %H:%M:%S").to_string()
}

/// Configure UI Items
pub fn draw_ui(sender: Sender<Message>) -> MainWindow {
    let mut filter_input = Input::default()
        .with_size(WIDGET_WIDTH, WIDGET_HEIGHT)
        .with_pos(WIDGET_PADDING + WIDGET_WIDTH * 2, WIDGET_PADDING)
        .with_label("Filter prefix:");
    filter_input.set_trigger(CallbackTrigger::Changed);
    filter_input.emit(sender, Message::Filter);

    let mut list_browser = HoldBrowser::default()
        .with_pos(
            WIDGET_PADDING,
            filter_input.y() + filter_input.height() + WIDGET_PADDING,
        )
        .with_size(WIDGET_WIDTH * 7, WIDGET_HEIGHT * 4);
    list_browser.set_column_widths(&[WIDGET_WIDTH * 2, WIDGET_WIDTH * 3, WIDGET_WIDTH]);
    list_browser.set_column_char('\t');
    list_browser.emit(sender, Message::Select);

    let description_input = Input::default()
        .with_size(WIDGET_WIDTH * 2, WIDGET_HEIGHT)
        .with_pos(
            list_browser.x() + list_browser.width() + WIDGET_PADDING * 3 + WIDGET_WIDTH,
            list_browser.y(),
        )
        .with_label("Description:");

    let label_completed = frame::Frame::default().with_label("Completed:").with_pos(
        list_browser.x() + list_browser.width() + WIDGET_PADDING * 5,
        list_browser.y() + list_browser.height() - WIDGET_PADDING,
    );
    let completed_input = CheckButton::default().with_size(20, 20).with_pos(
        label_completed.x() + label_completed.width() + WIDGET_PADDING * 5,
        label_completed.y() - 10,
    );

    let mut create_button = Button::default()
        .with_size(WIDGET_WIDTH, WIDGET_HEIGHT)
        .with_pos(
            WIDGET_PADDING,
            list_browser.y() + list_browser.height() + WIDGET_PADDING,
        )
        .with_label("Create");
    create_button.set_frame(widget_themes::OS_MINI_BUTTON_UP_BOX);
    create_button.emit(sender, Message::Create);

    let mut update_button = Button::default()
        .with_size(WIDGET_WIDTH, WIDGET_HEIGHT)
        .right_of(&create_button, WIDGET_PADDING)
        .with_label("Update");
    update_button.set_frame(widget_themes::OS_MINI_BUTTON_UP_BOX);
    update_button.emit(sender, Message::Update);
    update_button.deactivate();

    let mut delete_button = Button::default()
        .with_size(WIDGET_WIDTH, WIDGET_HEIGHT)
        .right_of(&update_button, WIDGET_PADDING)
        .with_label("Delete");
    delete_button.set_frame(widget_themes::OS_MINI_BUTTON_UP_BOX);
    delete_button.emit(sender, Message::Delete);
    delete_button.deactivate();

    MainWindow {
        completed_input,
        create_button,
        delete_button,
        description_input,
        filter_input,
        list_browser,
        update_button,
    }
}

/// Create the message waiting loop
/// and start the application
pub fn message_waiting_loop(app: &mut TodolistRS) {
    let MainWindow {
        completed_input,
        create_button,
        delete_button,
        description_input,
        filter_input,
        list_browser,
        update_button,
    } = &mut app.m_window;

    while app.a.wait() {
        match app.r.recv() {
            Some(Message::Create) => {
                // Do not allow empty TO-DO
                if description_input.value().trim() != "" {
                    app.model.insert(
                        0,
                        ListItem {
                            completed: false,
                            description: description_input.value(),
                            datetime: get_datetime(),
                        },
                    );
                }
                dump_list_items(&app.model);
                description_input.set_value("");
                app.s.send(Message::Filter);
            }
            Some(Message::Update) => {
                let selected_name = list_browser.text(list_browser.value()).unwrap();
                let datetime = selected_name.split("\t").collect::<Vec<&str>>()[1];
                let index = app
                    .model
                    .iter()
                    .position(|s| s.datetime == datetime)
                    .unwrap();
                let item = &mut app.model[index];
                item.completed = completed_input.value();
                dump_list_items(&app.model);
                // description_input.set_value("");
                app.s.send(Message::Filter);
            }
            Some(Message::Delete) => {
                let selected_name = list_browser.text(list_browser.value()).unwrap();
                let datetime = selected_name.split("\t").collect::<Vec<&str>>()[1];
                // println!("{datetime}");
                let index = app
                    .model
                    .iter()
                    .position(|s| s.datetime == datetime)
                    .unwrap();
                app.model.remove(index);
                dump_list_items(&app.model);
                app.s.send(Message::Filter);
                app.s.send(Message::Select)
            }
            Some(Message::Select) => {
                if list_browser.value() == 0 || list_browser.value() == 1 {
                    create_button.activate();
                    description_input.set_value("");
                    description_input.set_readonly(false);
                    description_input.set_tooltip("");
                    update_button.deactivate();
                    delete_button.deactivate();
                    completed_input.set_value(false);
                    completed_input.deactivate();
                } else {
                    create_button.deactivate();
                    completed_input.activate();
                    let selected_name = list_browser.text(list_browser.value()).unwrap();
                    let datetime = selected_name.split("\t").collect::<Vec<&str>>()[1];
                    let index = app
                        .model
                        .iter()
                        .position(|s| s.datetime == datetime)
                        .unwrap();
                    completed_input.set_value(app.model[index].completed);
                    description_input.set_value(&app.model[index].description);
                    description_input.set_readonly(true);
                    description_input.set_tooltip(&app.model[index].description);
                    update_button.activate();
                    delete_button.activate();
                }
            }
            Some(Message::Filter) => {
                let prefix = filter_input.value().to_lowercase();
                list_browser.clear();
                list_browser.add("@C221DESCRIPTION\t@C221DATETIME\t@C221COMPLETED");
                for item in &app.model {
                    if item.description.to_lowercase().starts_with(&prefix) {
                        let content = format!(
                            "{}\t{}\t{}",
                            item.description,
                            item.datetime,
                            match item.completed {
                                true => "✅",
                                false => "❌",
                            }
                        );
                        list_browser.add(&content);
                    }
                }
                app.s.send(Message::Select)
            }
            None => {}
        }
    }
}
