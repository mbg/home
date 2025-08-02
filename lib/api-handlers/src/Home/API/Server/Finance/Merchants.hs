-- | Implements the handlers for the merchants API.
module Home.API.Server.Finance.Merchants (
    merchantHandlers
) where

--------------------------------------------------------------------------------

import Home.API.Finance.Merchants
import Home.API.Server.Handler

--------------------------------------------------------------------------------

getMerchants :: ApiHandler [Merchant]
getMerchants = pure []

getMerchant :: Id -> ApiHandler Merchant
getMerchant _ = pure $ MkMerchant nilId "Test"

merchantHandlers :: ServerT MerchantsAPI ApiHandler
merchantHandlers = getMerchants :<|> getMerchant

--------------------------------------------------------------------------------
