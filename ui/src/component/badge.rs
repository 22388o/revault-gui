use crate::{
    color,
    icon::{
        bitcoin_icon, block_icon, deposit_icon, network_icon, person_check_icon, send_icon,
        shield_check_icon, shield_icon, shield_notif_icon, turnback_icon,
    },
};

use iced::{container, Container, Length};

pub fn bitcoin_core<'a, T: 'a>() -> Container<'a, T> {
    let icon = bitcoin_icon().width(Length::Units(20));
    Container::new(icon)
        .width(Length::Units(30))
        .height(Length::Units(30))
        .style(BitcoinCoreBadgeStyle)
        .align_x(iced::Align::Center)
        .align_y(iced::Align::Center)
}

struct BitcoinCoreBadgeStyle;
impl container::StyleSheet for BitcoinCoreBadgeStyle {
    fn style(&self) -> container::Style {
        container::Style {
            border_radius: 30.0,
            background: iced::Color::BLACK.into(),
            text_color: iced::Color::WHITE.into(),
            ..container::Style::default()
        }
    }
}

pub fn network<'a, T: 'a>() -> Container<'a, T> {
    let icon = network_icon().width(Length::Units(20));
    Container::new(icon)
        .width(Length::Units(40))
        .height(Length::Units(40))
        .align_x(iced::Align::Center)
        .align_y(iced::Align::Center)
}

pub fn person_check<'a, T: 'a>() -> Container<'a, T> {
    let icon = person_check_icon().width(Length::Units(20));
    Container::new(icon)
        .width(Length::Units(40))
        .height(Length::Units(40))
        .style(PersonBadgeStyle)
        .align_x(iced::Align::Center)
        .align_y(iced::Align::Center)
}

struct PersonBadgeStyle;
impl container::StyleSheet for PersonBadgeStyle {
    fn style(&self) -> container::Style {
        container::Style {
            border_radius: 40.0,
            background: color::FOREGROUND.into(),
            text_color: color::CANCEL.into(),
            ..container::Style::default()
        }
    }
}

pub fn shield<'a, T: 'a>() -> Container<'a, T> {
    let icon = shield_icon().width(Length::Units(20));
    Container::new(icon)
        .width(Length::Units(40))
        .height(Length::Units(40))
        .style(ShieldBadgeStyle)
        .align_x(iced::Align::Center)
        .align_y(iced::Align::Center)
}

struct ShieldBadgeStyle;
impl container::StyleSheet for ShieldBadgeStyle {
    fn style(&self) -> container::Style {
        container::Style {
            border_radius: 40.0,
            background: color::FOREGROUND.into(),
            text_color: color::CANCEL.into(),
            ..container::Style::default()
        }
    }
}

pub fn shield_success<'a, T: 'a>() -> Container<'a, T> {
    let icon = shield_check_icon().width(Length::Units(20));
    Container::new(icon)
        .width(Length::Units(40))
        .height(Length::Units(40))
        .style(ShieldSuccessBadgeStyle)
        .align_x(iced::Align::Center)
        .align_y(iced::Align::Center)
}

struct ShieldSuccessBadgeStyle;
impl container::StyleSheet for ShieldSuccessBadgeStyle {
    fn style(&self) -> container::Style {
        container::Style {
            border_radius: 40.0,
            background: color::FOREGROUND.into(),
            text_color: color::SUCCESS.into(),
            ..container::Style::default()
        }
    }
}

pub fn shield_notif<'a, T: 'a>() -> Container<'a, T> {
    let icon = shield_notif_icon().width(Length::Units(20));
    Container::new(icon)
        .width(Length::Units(40))
        .height(Length::Units(40))
        .style(ShieldNotifBadgeStyle)
        .align_x(iced::Align::Center)
        .align_y(iced::Align::Center)
}

struct ShieldNotifBadgeStyle;
impl container::StyleSheet for ShieldNotifBadgeStyle {
    fn style(&self) -> container::Style {
        container::Style {
            border_radius: 40.0,
            background: color::FOREGROUND.into(),
            text_color: color::CANCEL.into(),
            ..container::Style::default()
        }
    }
}

pub fn block<'a, T: 'a>() -> Container<'a, T> {
    let icon = block_icon().width(Length::Units(20));
    Container::new(icon)
        .width(Length::Units(40))
        .height(Length::Units(40))
        .style(BlockBadgeStyle)
        .align_x(iced::Align::Center)
        .align_y(iced::Align::Center)
}

struct BlockBadgeStyle;
impl container::StyleSheet for BlockBadgeStyle {
    fn style(&self) -> container::Style {
        container::Style {
            border_radius: 40.0,
            background: color::PRIMARY_LIGHT.into(),
            text_color: color::PRIMARY.into(),
            ..container::Style::default()
        }
    }
}

pub fn tx_deposit<'a, T: 'a>() -> Container<'a, T> {
    let icon = deposit_icon().width(Length::Units(20));
    Container::new(icon)
        .width(Length::Units(40))
        .height(Length::Units(40))
        .style(TxDepositBadgeStyle)
        .align_x(iced::Align::Center)
        .align_y(iced::Align::Center)
}

struct TxDepositBadgeStyle;
impl container::StyleSheet for TxDepositBadgeStyle {
    fn style(&self) -> container::Style {
        container::Style {
            border_radius: 40.0,
            background: color::INFO_LIGHT.into(),
            text_color: color::INFO.into(),
            ..container::Style::default()
        }
    }
}

pub fn vault_unconfirmed<'a, T: 'a>() -> Container<'a, T> {
    let icon = deposit_icon().width(Length::Units(20));
    Container::new(icon)
        .width(Length::Units(40))
        .height(Length::Units(40))
        .style(WarningBadgeStyle)
        .align_x(iced::Align::Center)
        .align_y(iced::Align::Center)
}

pub fn vault_unvaulting<'a, T: 'a>() -> Container<'a, T> {
    let icon = send_icon().width(Length::Units(20));
    Container::new(icon)
        .width(Length::Units(40))
        .height(Length::Units(40))
        .style(WarningBadgeStyle)
        .align_x(iced::Align::Center)
        .align_y(iced::Align::Center)
}

pub fn vault_canceling<'a, T: 'a>() -> Container<'a, T> {
    let icon = turnback_icon().width(Length::Units(20));
    Container::new(icon)
        .width(Length::Units(40))
        .height(Length::Units(40))
        .style(WarningBadgeStyle)
        .align_x(iced::Align::Center)
        .align_y(iced::Align::Center)
}

pub fn vault_spending<'a, T: 'a>() -> Container<'a, T> {
    let icon = send_icon().width(Length::Units(20));
    Container::new(icon)
        .width(Length::Units(40))
        .height(Length::Units(40))
        .style(WarningBadgeStyle)
        .align_x(iced::Align::Center)
        .align_y(iced::Align::Center)
}

struct WarningBadgeStyle;
impl container::StyleSheet for WarningBadgeStyle {
    fn style(&self) -> container::Style {
        container::Style {
            border_radius: 40.0,
            background: color::ALERT_LIGHT.into(),
            text_color: color::ALERT.into(),
            ..container::Style::default()
        }
    }
}

pub fn vault_canceled<'a, T: 'a>() -> Container<'a, T> {
    let icon = turnback_icon().width(Length::Units(20));
    Container::new(icon)
        .width(Length::Units(40))
        .height(Length::Units(40))
        .style(AlertBadgeStyle)
        .align_x(iced::Align::Center)
        .align_y(iced::Align::Center)
}

struct AlertBadgeStyle;
impl container::StyleSheet for AlertBadgeStyle {
    fn style(&self) -> container::Style {
        container::Style {
            border_radius: 40.0,
            background: color::ALERT_LIGHT.into(),
            text_color: color::ALERT.into(),
            ..container::Style::default()
        }
    }
}

pub fn vault_spent<'a, T: 'a>() -> Container<'a, T> {
    let icon = send_icon().width(Length::Units(20));
    Container::new(icon)
        .width(Length::Units(40))
        .height(Length::Units(40))
        .style(SuccessBadgeStyle)
        .align_x(iced::Align::Center)
        .align_y(iced::Align::Center)
}

struct SuccessBadgeStyle;
impl container::StyleSheet for SuccessBadgeStyle {
    fn style(&self) -> container::Style {
        container::Style {
            border_radius: 40.0,
            background: color::SUCCESS_LIGHT.into(),
            text_color: color::SUCCESS.into(),
            ..container::Style::default()
        }
    }
}

pub fn pending_spent_tx<'a, T: 'a>() -> Container<'a, T> {
    let icon = send_icon().width(Length::Units(20));
    Container::new(icon)
        .width(Length::Units(40))
        .height(Length::Units(40))
        .style(InactiveBadgeStyle)
        .align_x(iced::Align::Center)
        .align_y(iced::Align::Center)
}

struct InactiveBadgeStyle;
impl container::StyleSheet for InactiveBadgeStyle {
    fn style(&self) -> container::Style {
        container::Style {
            border_radius: 40.0,
            background: color::BACKGROUND.into(),
            ..container::Style::default()
        }
    }
}