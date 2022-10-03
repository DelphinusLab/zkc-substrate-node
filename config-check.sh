{ read ADMINS; read -e AUTHORITIES_SEEDS; read -e PRE_FUNDED_SEEDS; read -e SUDO_ACCOUNT_SEED; } <<< `node ../deployment/config/get-account-config.js`
if echo "$(<generated_config/account_config.rs)" | grep -Fq "${AUTHORITIES_SEEDS}" &&
   echo "$(<generated_config/account_config.rs)" | grep -Fq "${PRE_FUNDED_SEEDS}" &&
   echo "$(<generated_config/account_config.rs)" | grep -Fq "${SUDO_ACCOUNT_SEED}"
then
   echo "account_config.rs check: Success"
else
   echo "account_config.rs check: Failed, Please run generate-config.sh in substrate-node to generate configs"
   exit 0
fi

if
   echo "$(<generated_config/admins_config.rs)" | grep -Fq "$ADMINS"; 
then
   echo "admins_config.rs check: Success"
else
   echo "admins_config.rs check: Failed, Please run generate-config.sh in substrate-node to generate configs"
   exit 0
fi

echo "Config Check Success"