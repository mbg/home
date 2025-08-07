-- | Implements the handlers for the merchants API.
module Home.API.Server.Finance.Merchants (
    merchantHandlers
) where

--------------------------------------------------------------------------------

import Data.Text ( strip )

import Database.Esqueleto.Experimental

import Home.API.Finance.Merchants
import Home.API.Server.Handler
import Home.Db.Finance.Merchant qualified as Db

--------------------------------------------------------------------------------

fromDbMerchant :: Entity Db.Merchant -> Keyed MerchantInfo
fromDbMerchant (Entity key Db.Merchant{..}) =
    mkKeyed key $ MkMerchantInfo{
        merchantName
    }

-- | `merchantById` @key@ is a `SqlQuery` for retrieving a `Db.Merchant` entity
-- with the given @key@.
merchantById :: Key Db.Merchant -> SqlQuery (SqlExpr (Entity Db.Merchant))
merchantById merchantId = do
    m <- from $ table @Db.Merchant
    where_ $ m ^. Db.MerchantId ==. val merchantId
    pure m

-- | `merchantByName` @name@ is a `SqlQuery` for retrieving a `Db.Merchant` entity
-- with the given @name@.
merchantByName :: Text -> SqlQuery (SqlExpr (Entity Db.Merchant))
merchantByName name = do
    m <- from $ table @Db.Merchant
    where_ $ m ^. Db.MerchantName ==. val name
    pure m

-- | `getMerchants` lists all known merchants.
getMerchants :: ApiHandler [Keyed MerchantInfo]
getMerchants = do
    merchants <- runQuery $ select $ do
        m <- from $ table @Db.Merchant
        pure m

    pure $ map fromDbMerchant merchants

-- | `getMerchant` @key@ gets the merchant identifier by @key@.
getMerchant
    :: Key Db.Merchant
    -> ApiHandler (Keyed MerchantInfo)
getMerchant merchantId = do
    merchant <- selectOneOr404 $ merchantById merchantId

    pure $ fromDbMerchant merchant

-- | `putMerchant` @merchant@ adds @merchant@ to the database and returns
-- @merchant@ with the assigned key.
putMerchant :: MerchantInfo -> ApiHandler (Keyed MerchantInfo)
putMerchant MkMerchantInfo{..} = do
    -- Don't accept empty strings
    validateNonEmpty "The merchant name" merchantName
    -- Don't accept merchants with identical names
    let trimmedName = strip merchantName
    validateNoEntity
        ("A merchant named '" <> trimmedName <> "' already exists.")
        (merchantByName trimmedName)

    -- Create the new merchant
    now <- liftIO getCurrentTime
    newMerchantId <- runQuery $ insert $
        Db.Merchant trimmedName now

    -- Return all available information about the new merchant
    pure $ mkKeyed newMerchantId $ MkMerchantInfo{
        merchantName = trimmedName
    }

merchantHandlers :: ServerT MerchantsAPI ApiHandler
merchantHandlers = getMerchants :<|> getMerchant :<|> putMerchant

--------------------------------------------------------------------------------
