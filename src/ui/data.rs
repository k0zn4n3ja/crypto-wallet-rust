#[derive(Debug, PartialEq)]
pub enum Msg {
    AppClose,
    MainMenuBlur,
    WalletActionsBlur,
    OptionSelected(usize),
    None,
}
