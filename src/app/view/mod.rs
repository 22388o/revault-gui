mod deposit;
mod emergency;
mod home;
mod layout;
pub mod manager;
mod settings;
mod sidebar;
pub mod sign;
pub mod spend_transaction;
pub mod stakeholder;
pub mod vault;
mod vaults;
mod warning;

pub use deposit::DepositView;
pub use emergency::EmergencyView;
pub use home::{ManagerHomeView, StakeholderHomeView};
pub use settings::SettingsView;
pub use spend_transaction::{SpendTransactionListItemView, SpendTransactionView};
pub use stakeholder::{StakeholderCreateVaultsView, StakeholderDelegateFundsView};
pub use vault::VaultView;
pub use vaults::VaultsView;

use iced::{scrollable, Column, Container, Element, Length, Row};

use revault_ui::component::{button, card, scroll, text::Text, ContainerBackgroundStyle};

use crate::{
    app::{context::Context, error::Error, menu::Menu, message::Message},
    daemon::client::Client,
};

#[derive(Debug)]
pub struct LoadingDashboard {
    dashboard: layout::Dashboard,
}

impl LoadingDashboard {
    pub fn new() -> Self {
        LoadingDashboard {
            dashboard: layout::Dashboard::new(),
        }
    }

    pub fn view<'a, C: Client>(
        &'a mut self,
        ctx: &Context<C>,
        warning: Option<&Error>,
    ) -> Element<'a, Message> {
        self.dashboard.view(ctx, warning, Column::new())
    }
}

#[derive(Debug)]
pub struct LoadingModal {
    scroll: scrollable::State,
    close_button: iced::button::State,
}

impl LoadingModal {
    pub fn new() -> Self {
        LoadingModal {
            scroll: scrollable::State::new(),
            close_button: iced::button::State::new(),
        }
    }

    pub fn view<'a, C: Client>(
        &'a mut self,
        _ctx: &Context<C>,
        warning: Option<&Error>,
        close_redirect: Menu,
    ) -> Element<'a, Message> {
        let mut col = Column::new()
            .push(
                Row::new().push(Column::new().width(Length::Fill)).push(
                    Container::new(
                        button::close_button(&mut self.close_button)
                            .on_press(Message::Menu(close_redirect)),
                    )
                    .width(Length::Shrink),
                ),
            )
            .spacing(50);

        if let Some(error) = warning {
            col = col.push(card::alert_warning(Container::new(Text::new(&format!(
                "{}",
                error
            )))))
        }

        Container::new(scroll(&mut self.scroll, Container::new(col)))
            .width(Length::Fill)
            .height(Length::Fill)
            .style(ContainerBackgroundStyle)
            .padding(20)
            .into()
    }
}
