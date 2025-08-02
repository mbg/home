
-- | Implements the merchants API.
module Home.API.Finance.Merchants (
    MerchantsAPI,
    Merchant(..)
) where

--------------------------------------------------------------------------------

import Servant
import Home.Db.Types
import Home.API.JSON

--------------------------------------------------------------------------------

data Merchant = MkMerchant {
    merchantId :: Id,
    merchantName :: Text
} deriving (Generic, Eq, Show)
  deriving (FromJSON, ToJSON) via CustomJSON (JSONOptions "merchant") Merchant

--------------------------------------------------------------------------------

-- | Describes the API for merchants.
type MerchantsAPI
    = Get '[JSON] [Merchant]
 :<|> Capture "id" Id :> Get '[JSON] Merchant

--------------------------------------------------------------------------------
