-- | Implements the handlers for the merchants API.
module Home.API.Server.Finance.Merchants (
    merchantHandlers
) where

--------------------------------------------------------------------------------

import Control.Monad

import Data.Text qualified as T

import Database.Esqueleto.Experimental

import Home.API.Finance.Merchants
import Home.API.Server.Handler
import Home.Db.Finance.Merchant qualified as Db

--------------------------------------------------------------------------------

fromDbMerchant :: Entity Db.Merchant -> MerchantInfo
fromDbMerchant (Entity key Db.Merchant{..}) =
    MkMerchantInfo (Just key) merchantName

-- | `getMerchants` lists all known merchants.
getMerchants :: ApiHandler [MerchantInfo]
getMerchants = do
    merchants <- runQuery $ select $ do
        m <- from $ table @Db.Merchant
        pure m

    pure $ map fromDbMerchant merchants

-- | `getMerchant` @key@ gets the merchant identifier by @key@.
getMerchant :: Key Db.Merchant -> ApiHandler MerchantInfo
getMerchant merchantId = do
    merchant <- selectOneOr404 $ do
        m <- from $ table @Db.Merchant
        where_ $ m ^. Db.MerchantId ==. val merchantId
        pure m

    pure $ fromDbMerchant merchant

-- | `putMerchant` @merchant@ adds @merchant@ to the database and returns
-- @merchant@ with the assigned key.
putMerchant :: MerchantInfo -> ApiHandler MerchantInfo
putMerchant MkMerchantInfo{..} = do
    -- Don't accept empty strings
    validateNonEmpty "The merchant name" merchantName

    -- Create the new merchant
    now <- liftIO getCurrentTime
    newMerchantId <- runQuery $ insert $
        Db.Merchant merchantName now

    -- Return all available information about the new merchant
    pure $ MkMerchantInfo (Just newMerchantId) merchantName

merchantHandlers :: ServerT MerchantsAPI ApiHandler
merchantHandlers = getMerchants :<|> getMerchant :<|> putMerchant

--------------------------------------------------------------------------------
