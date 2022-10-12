#!/usr/bin/env bash
{ read ADMINS; read -r AUTHORITIES_SEEDS; read -r PRE_FUNDED_SEEDS; read -r SUDO_ACCOUNT_SEED; } <<< `node ../deployment/config/get-account-config.js`
echo "ADMINS: " $ADMINS;
echo " "
echo "AUTHORITIES_SEEDS: " $AUTHORITIES_SEEDS
echo " "
echo "PRE_FUNDED_SEEDS: " $PRE_FUNDED_SEEDS
echo " "
echo "SUDO_ACCOUNT_SEEDS: " $SUDO_ACCOUNT_SEED
echo " "
sed -i "s/vec.*$/$ADMINS/" generated_config/admins_config.rs
sed -i "/pub/!s/authorities_seeds.*$/$AUTHORITIES_SEEDS/" generated_config/account_config.rs
sed -i "/pub/!s/pre_funded_seeds.*$/$PRE_FUNDED_SEEDS/" generated_config/account_config.rs
sed -i "/pub/!s/sudo_account_seed.*$/$SUDO_ACCOUNT_SEED/" generated_config/account_config.rs
echo "admins_config.rs: "
echo "$(<generated_config/admins_config.rs)"
echo " "
echo "account_config.rs: "
echo "$(<generated_config/account_config.rs)"
