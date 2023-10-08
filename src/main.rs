mod ui;
mod wallet;

use std::env;

use anyhow::Result;
use secp256k1::SecretKey;
use std::time::Duration;
use tuirealm::{AttrValue, Attribute, StateValue};
use ui::main_menu::MainMenu;
use ui::wallet_actions::WalletActions;
use wallet::evm::{address_from_pubkey, establish_web3_connection, generate_keypair, Wallet};

use tui_realm_stdlib::{List, Table};
use tuirealm::command::{Cmd, CmdResult, Direction, Position};
use tuirealm::props::{
    Alignment, BorderType, Borders, Color, TableBuilder, TextModifiers, TextSpan,
};
use tuirealm::terminal::TerminalBridge;
use tuirealm::{
    application::PollStrategy,
    event::{Key, KeyEvent},
    Application, Component, Event, EventListenerCfg, MockComponent, NoUserEvent, Update,
};
// tui
use tuirealm::tui::layout::{Constraint, Direction as LayoutDirection, Layout};

const WALLET_FILE_PATH: &str = "crypto_wallet.json";

#[derive(Debug, PartialEq)]
pub enum Msg {
    AppClose,
    MainMenuBlur,
    WalletActionsBlur,
    OptionSelected(usize),
    None,
}

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
        let (private, public) = generate_keypair();
        // println!("private and public keys");
        // println!("{}", private.display_secret().to_string());
        // println!("{}", public);

        let address = address_from_pubkey(&public);

        // println!("address: ");
        // println!("{:?}", address);

        let crypto_wallet = Wallet::new(&private, &public);
        // println!("crypto_wallet: {:?}", &crypto_wallet);

        // TODO remove unwrap or default, errors etc
        crypto_wallet
            // TODO wallet save location added to settings
            .save_to_file(&WALLET_FILE_PATH)
            .unwrap_or_default();
        self.wallet = Some(crypto_wallet);
    }

    fn load_wallet_from_file(&mut self) {
        // TODO remove unwrap or default
        let loaded_wallet = Wallet::from_file(&WALLET_FILE_PATH).unwrap();
        println!("loaded_wallet: {:?}", loaded_wallet);
        self.wallet = Some(loaded_wallet);
    }

    async fn test_wallet(&mut self) {
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

#[tokio::main]
async fn main() {
    // infra setup
    dotenv::dotenv().ok();

    let mut terminal = TerminalBridge::new().expect("Cannot create terminal bridge");
    let mut model = Wolet::default();
    let _ = terminal.enable_raw_mode();
    let _ = terminal.enter_alternate_screen();
    // Now we use the Model struct to keep track of some states

    // let's loop until quit is true
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
        // Redraw
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
                    // check how to load a new menu
                    None
                } else {
                    None
                }
            }

            Msg::None => None,
        }
    }
}
