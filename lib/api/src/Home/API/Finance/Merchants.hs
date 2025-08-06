
-- | Implements the merchants API.
module Home.API.Finance.Merchants (
    MerchantsAPI,
    MerchantInfo(..)
) where

--------------------------------------------------------------------------------

import Servant

import Home.Db.Types
import Home.Db.Finance.Merchant qualified as Db
import Home.JSON

--------------------------------------------------------------------------------

data MerchantInfo = MkMerchantInfo {
    merchantId :: Maybe (Key Db.Merchant),
    merchantName :: Text
} deriving (Generic, Eq, Show)
  deriving (FromJSON, ToJSON)
    via CustomJSON (JSONOptions "merchant") MerchantInfo

--------------------------------------------------------------------------------

-- | Describes the API for merchants.
type MerchantsAPI
    = Get '[JSON] [MerchantInfo]
 :<|> Capture "id" (Key Db.Merchant) :>
      Get '[JSON] (MerchantInfo)
 :<|> ReqBody '[JSON] MerchantInfo :>
      Put '[JSON] MerchantInfo

--------------------------------------------------------------------------------
