mod ui;
mod wallet;

use std::env;

use anyhow::Result;
use std::time::Duration;
use tuirealm::terminal::TerminalBridge;
use tuirealm::{application::PollStrategy, Application, EventListenerCfg, NoUserEvent, Update};
use ui::data::Msg;
use ui::main_menu::MainMenu;
use ui::wallet_actions::WalletActions;
use wallet::{
    core::Wallet,
    evm::{address_from_pubkey, establish_web3_connection},
};
// tui
use tuirealm::tui::layout::{Constraint, Direction as LayoutDirection, Layout};

// Let's define the component ids for our application
#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub enum Id {
    MainMenu,
    WalletActions,
}

struct WoletState {
    wallet: Option<Wallet>,
}

impl Default for WoletState {
    fn default() -> Self {
        Self { wallet: None }
    }
}

impl WoletState {
    fn new_wallet(&mut self) {
        // TODO remove unwrap
        let new_wallet = Wallet::new().unwrap();
        self.wallet = Some(new_wallet);
    }

    fn load_wallet_from_file(&mut self) {
        // TODO remove unwrap or default
        let loaded_wallet = Wallet::from_file().unwrap();
        self.wallet = Some(loaded_wallet);
    }

    async fn test_connection(&mut self) {
        // TODO remove all unwraps
        let endpoint = env::var("TESTNET_WS").unwrap();
        let web3_con = establish_web3_connection(&endpoint).await.unwrap();
        let block_number = web3_con.eth().block_number().await.unwrap();
        println!("block number: {}", &block_number);
    }
}

struct Wolet {
    // ui fields
    quit: bool,   // Becomes true when the user presses <ESC>
    redraw: bool, // Tells whether to refresh the UI; performance optimization
    app: Application<Id, Msg, NoUserEvent>,
    states: WoletState,
}

// ui elements
impl Default for Wolet {
    fn default() -> Self {
        // Setup app
        let mut app: Application<Id, Msg, NoUserEvent> = Application::init(
            EventListenerCfg::default().default_input_listener(Duration::from_millis(10)),
        );
        assert!(app
            .mount(Id::MainMenu, Box::new(MainMenu::default()), vec![])
            .is_ok());
        assert!(app
            .mount(
                Id::WalletActions,
                Box::new(WalletActions::default()),
                vec![]
            )
            .is_ok());
        // We need to give focus to input then
        assert!(app.active(&Id::MainMenu).is_ok());
        assert!(app.active(&Id::WalletActions).is_ok());
        Self {
            quit: false,
            redraw: true,
            states: WoletState { wallet: None },
            app,
        }
    }
}

impl Wolet {
    fn view(&mut self, terminal: &mut TerminalBridge) {
        let _ = terminal.raw_mut().draw(|f| {
            // Prepare chunks
            let chunks = Layout::default()
                .direction(LayoutDirection::Vertical)
                .margin(1)
                .constraints(
                    [
                        Constraint::Length(10),
                        Constraint::Length(6),
                        Constraint::Length(1),
                    ]
                    .as_ref(),
                )
                .split(f.size());
            if self.states.wallet.is_some() {
                // show main menu
                self.app.view(&Id::WalletActions, f, chunks[0]);
            } else {
                // show wallet actions menu
                self.app.view(&Id::MainMenu, f, chunks[0]);
            }
        });
    }
}

impl Update<Msg> for Wolet {
    fn update(&mut self, msg: Option<Msg>) -> Option<Msg> {
        self.redraw = true;
        match msg.unwrap_or(Msg::None) {
            Msg::AppClose => {
                self.quit = true;
                None
            }
            Msg::MainMenuBlur => None,
            Msg::WalletActionsBlur => None,
            Msg::OptionSelected(val) => {
                if val == 1 {
                    self.states.new_wallet();
                    None
                } else {
                    self.states.load_wallet_from_file();
                    None
                }
            }

            Msg::None => None,
        }
    }
}

#[tokio::main]
async fn main() {
    // infra setup
    dotenv::dotenv().ok();

    // set up CLI
    let mut terminal = TerminalBridge::new().expect("Cannot create terminal bridge");
    let mut model = Wolet::default();
    let _ = terminal.enable_raw_mode();
    let _ = terminal.enter_alternate_screen();
    // Now we use the Model struct to keep track of some states

    // main ui loop
    while !model.quit {
        // Tick
        if let Ok(messages) = model.app.tick(PollStrategy::Once) {
            for msg in messages.into_iter() {
                let mut msg = Some(msg);
                while msg.is_some() {
                    msg = model.update(msg);
                }
            }
        }
        // You can flag redraw to be true for rerenderings based on state
        if model.redraw {
            model.view(&mut terminal);
            model.redraw = false;
        }
    }
    // Terminate terminal
    let _ = terminal.leave_alternate_screen();
    let _ = terminal.disable_raw_mode();
    let _ = terminal.clear_screen();
}
