use std::collections::HashMap;

use iced::widget::{
    button, column, container, horizontal_space, row, scrollable, text, text_input,
};
use iced::{Element, Length, Padding};

use axiom_protocol::config::{ConfigField, ConfigFieldType, ConfigSection, ConfigValue};

use crate::app::{ConfigurationState, Message};
use crate::settings::UiSettings;

pub fn config_view<'a>(
    config: &'a ConfigurationState,
    _settings: &'a UiSettings,
) -> Element<'a, Message> {
    let left = section_panel(&config.sections, &config.active_section_id);
    let right = field_panel(
        &config.sections,
        &config.active_section_id,
        &config.pending_changes,
        &config.validation_errors,
    );

    let has_pending = config.pending_changes.contains_key(&config.active_section_id);

    let bottom = row![
        horizontal_space(),
        button(text("Discard").size(13))
            .on_press_maybe(if has_pending { Some(Message::ConfigDiscard) } else { None })
            .style(button::secondary),
        button(text("Apply").size(13))
            .on_press_maybe(if has_pending {
                Some(Message::ConfigApply { section_id: config.active_section_id.clone() })
            } else {
                None
            })
            .style(button::primary),
    ]
    .spacing(8)
    .padding([8u16, 12u16]);

    column![
        row![
            container(left).width(200).height(Length::Fill),
            container(right).width(Length::Fill).height(Length::Fill),
        ]
        .height(Length::Fill),
        bottom,
    ]
    .into()
}

// ── Section navigation ─────────────────────────────────────────────────────

fn section_panel<'a>(
    sections: &'a [ConfigSection],
    active_id: &'a str,
) -> Element<'a, Message> {
    let items: Vec<Element<Message>> = sections
        .iter()
        .flat_map(|s| section_entries(s, active_id, 0))
        .collect();

    scrollable(column(items).spacing(2).padding([4u16, 0u16])).into()
}

fn section_entries<'a>(
    section: &'a ConfigSection,
    active_id: &'a str,
    depth: usize,
) -> Vec<Element<'a, Message>> {
    let indent = (depth as u16) * 12;
    let is_active = section.id == active_id;

    let btn = button(
        row![
            iced::widget::horizontal_space().width(indent),
            text(section.label.as_str()).size(13),
        ],
    )
    .on_press(Message::ConfigSectionSelected(section.id.clone()))
    .style(if is_active { button::primary } else { button::text })
    .width(Length::Fill);

    let mut entries: Vec<Element<Message>> = vec![btn.into()];

    for sub in &section.subsections {
        entries.extend(section_entries(sub, active_id, depth + 1));
    }

    entries
}

// ── Field rendering ────────────────────────────────────────────────────────

fn field_panel<'a>(
    sections: &'a [ConfigSection],
    active_id: &'a str,
    pending: &'a HashMap<String, HashMap<String, ConfigValue>>,
    errors: &'a HashMap<String, String>,
) -> Element<'a, Message> {
    let section = find_section(sections, active_id);

    let content: Element<Message> = match section {
        None => text("Select a section").size(14).into(),
        Some(sec) if sec.fields.is_empty() => {
            text("No configurable fields in this section.").size(14).into()
        }
        Some(sec) => {
            let pending_fields = pending.get(active_id);
            let rows: Vec<Element<Message>> = sec
                .fields
                .iter()
                .map(|f| field_row(f, active_id, pending_fields, errors))
                .collect();
            scrollable(column(rows).spacing(16).padding(16)).into()
        }
    };

    container(content).padding(8).into()
}

fn field_row<'a>(
    field: &'a ConfigField,
    section_id: &'a str,
    pending: Option<&'a HashMap<String, ConfigValue>>,
    errors: &'a HashMap<String, String>,
) -> Element<'a, Message> {
    let pending_value = pending.and_then(|p| p.get(&field.id));
    let display_value = pending_value.unwrap_or(&field.current_value);
    let is_pending = pending_value.is_some();
    let error = errors.get(&field.id);

    let label_text = if is_pending {
        format!("{}  ●", field.label)
    } else {
        field.label.clone()
    };

    let label = text(label_text).size(13);
    let description = field.description.as_deref().map(|d| text(d).size(11));

    let control = if field.readonly {
        text(value_display(display_value)).size(13).into()
    } else {
        field_control(field, section_id, display_value)
    };

    let error_row: Option<Element<Message>> = error.map(|e| {
        text(e.as_str()).size(11).color([0.85, 0.3, 0.3]).into()
    });

    let mut col = column![label, control].spacing(4);
    if let Some(desc) = description {
        col = column![col, desc].spacing(2);
    }
    if let Some(err) = error_row {
        col = column![col, err].spacing(2);
    }

    container(col)
        .padding(Padding { top: 0.0, right: 0.0, bottom: 8.0, left: 0.0 })
        .into()
}

fn field_control<'a>(
    field: &'a ConfigField,
    section_id: &'a str,
    current: &'a ConfigValue,
) -> Element<'a, Message> {
    let sid = section_id.to_string();
    let fid = field.id.clone();

    match &field.field_type {
        ConfigFieldType::Bool => {
            let checked = matches!(current, ConfigValue::Bool(true));
            iced::widget::checkbox("", checked)
                .on_toggle(move |v| Message::ConfigFieldChanged {
                    section_id: sid.clone(),
                    field_id: fid.clone(),
                    value: ConfigValue::Bool(v),
                })
                .into()
        }
        ConfigFieldType::String { .. } => {
            let val = match current {
                ConfigValue::String(s) => s.as_str(),
                _ => "",
            };
            text_input("", val)
                .on_input(move |s| Message::ConfigFieldChanged {
                    section_id: sid.clone(),
                    field_id: fid.clone(),
                    value: ConfigValue::String(s),
                })
                .size(13)
                .into()
        }
        ConfigFieldType::Integer { .. } | ConfigFieldType::UInt { .. } => {
            let val = value_display(current);
            text_input("", val.as_str())
                .on_input(move |s| {
                    let parsed = s.parse::<i64>()
                        .map(ConfigValue::Integer)
                        .unwrap_or(ConfigValue::String(s));
                    Message::ConfigFieldChanged {
                        section_id: sid.clone(),
                        field_id: fid.clone(),
                        value: parsed,
                    }
                })
                .size(13)
                .into()
        }
        ConfigFieldType::Float { .. } => {
            let val = value_display(current);
            text_input("", val.as_str())
                .on_input(move |s| {
                    let parsed = s.parse::<f64>()
                        .map(ConfigValue::Float)
                        .unwrap_or(ConfigValue::String(s));
                    Message::ConfigFieldChanged {
                        section_id: sid.clone(),
                        field_id: fid.clone(),
                        value: parsed,
                    }
                })
                .size(13)
                .into()
        }
        ConfigFieldType::Enum { variants } => {
            let btns: Vec<Element<Message>> = variants
                .iter()
                .map(|v| {
                    let is_selected = matches!(current, ConfigValue::EnumVariant(s) if s == v);
                    button(text(v.as_str()).size(12))
                        .on_press(Message::ConfigFieldChanged {
                            section_id: sid.clone(),
                            field_id: fid.clone(),
                            value: ConfigValue::EnumVariant(v.clone()),
                        })
                        .style(if is_selected { button::primary } else { button::secondary })
                        .into()
                })
                .collect();
            row(btns).spacing(4).into()
        }
        ConfigFieldType::Duration | ConfigFieldType::Domain => {
            let val = value_display(current);
            text_input("", val.as_str())
                .on_input(move |s| {
                    let parsed = s.parse::<u64>()
                        .map(ConfigValue::Duration)
                        .unwrap_or(ConfigValue::String(s));
                    Message::ConfigFieldChanged {
                        section_id: sid.clone(),
                        field_id: fid.clone(),
                        value: parsed,
                    }
                })
                .size(13)
                .into()
        }
    }
}

// ── Helpers ────────────────────────────────────────────────────────────────

fn value_display(v: &ConfigValue) -> String {
    match v {
        ConfigValue::Bool(b) => b.to_string(),
        ConfigValue::Integer(i) => i.to_string(),
        ConfigValue::UInt(u) => u.to_string(),
        ConfigValue::Float(f) => format!("{:.4}", f),
        ConfigValue::String(s) => s.clone(),
        ConfigValue::EnumVariant(s) => s.clone(),
        ConfigValue::Duration(d) => d.to_string(),
        ConfigValue::Domain(d) => d.to_string(),
    }
}

fn find_section<'a>(sections: &'a [ConfigSection], id: &str) -> Option<&'a ConfigSection> {
    for s in sections {
        if s.id == id {
            return Some(s);
        }
        if let Some(found) = find_section(&s.subsections, id) {
            return Some(found);
        }
    }
    None
}

pub fn build_workstation_section(settings: &UiSettings) -> ConfigSection {
    use axiom_protocol::config::{ConfigCategory, ConfigField, ConfigFieldType};
    ConfigSection {
        id: "workstation.connection".to_string(),
        label: "Connection".to_string(),
        category: ConfigCategory::Workstation,
        fields: vec![ConfigField {
            id: "engine_address".to_string(),
            label: "Engine Address".to_string(),
            description: Some("host:port of the Engine WebSocket".to_string()),
            field_type: ConfigFieldType::String { max_length: 255 },
            current_value: ConfigValue::String(settings.engine_address.clone()),
            default_value: ConfigValue::String("127.0.0.1:9876".to_string()),
            hot_reloadable: true,
            readonly: false,
        }],
        subsections: vec![],
    }
}
