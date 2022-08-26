pub struct AccountInfo<'a> {
    pub authorities_seeds: &'a [&'a str],
    pub pre_funded_seeds: &'a [&'a str],
    pub sudo_account_seed: &'a str
}

pub fn get_account_info() -> AccountInfo<'static> {
    AccountInfo {
        authorities_seeds:,
        pre_funded_seeds:,
        sudo_account_seed:
    }
}
