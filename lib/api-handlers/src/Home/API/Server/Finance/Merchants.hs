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

fromDbMerchant :: Entity Db.Merchant -> Merchant
fromDbMerchant (Entity key Db.Merchant{..}) = MkMerchant (Just key) merchantName

-- | `getMerchants` lists all known merchants.
getMerchants :: ApiHandler [Merchant]
getMerchants = do
    merchants <- runQuery $ select $ do
        m <- from $ table @Db.Merchant
        pure m

    pure $ map fromDbMerchant merchants

-- | `getMerchant` @key@ gets the merchant identifier by @key@.
getMerchant :: Key Db.Merchant -> ApiHandler Merchant
getMerchant merchantId = do
    merchant <- selectOneOr404 $ do
        m <- from $ table @Db.Merchant
        where_ $ m ^. Db.MerchantId ==. val merchantId
        pure m

    pure $ fromDbMerchant merchant

-- | `putMerchant` @merchant@ adds @merchant@ to the database and returns
-- @merchant@ with the assigned key.
putMerchant :: Merchant -> ApiHandler Merchant
putMerchant MkMerchant{..} = do
    -- Don't accept empty strings
    validateNonEmpty "The merchant name" merchantName

    -- Create the new merchant
    newMerchantId <- runQuery $ insert $ Db.Merchant merchantName

    -- Return all available information about the new merchant
    pure $ MkMerchant (Just newMerchantId) merchantName

merchantHandlers :: ServerT MerchantsAPI ApiHandler
merchantHandlers = getMerchants :<|> getMerchant :<|> putMerchant

--------------------------------------------------------------------------------
