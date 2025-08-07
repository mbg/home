
-- | Implements the merchants API.
module Home.API.Finance.Merchants (
    MerchantsAPI,
    MerchantInfo(..)
) where

--------------------------------------------------------------------------------

import Servant

import Home.API.Response
import Home.Db.Types
import Home.Db.Finance.Merchant qualified as Db
import Home.JSON

--------------------------------------------------------------------------------

data MerchantInfo = MkMerchantInfo {
    merchantName :: Text
} deriving (Generic, Eq, Show)
  deriving (FromJSON, ToJSON)
    via CustomJSON (JSONOptions "merchant") MerchantInfo

instance HasDbKey MerchantInfo where
    type DbKey MerchantInfo = Key Db.Merchant

--------------------------------------------------------------------------------

-- | Describes the API for merchants.
type MerchantsAPI
    = Get '[JSON] [Keyed MerchantInfo]
 :<|> Capture "id" (Key Db.Merchant) :>
      Get '[JSON] (Keyed MerchantInfo)
 :<|> ReqBody '[JSON] MerchantInfo :>
      Put '[JSON] (Keyed MerchantInfo)

--------------------------------------------------------------------------------
