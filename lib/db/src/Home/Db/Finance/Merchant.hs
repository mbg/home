--------------------------------------------------------------------------------

-- | Defines `Merchant`, which represents known merchants that might be
-- associated with transactions.
module Home.Db.Finance.Merchant where

--------------------------------------------------------------------------------

import Home.Db.Model

--------------------------------------------------------------------------------

dbModel "merchantModel" $(discoverEntities) [persistLowerCase|
Merchant sql=merchant
    name Text
    createdAt UTCTime sql=created_at default=now()
    deriving Eq Show
|]

--------------------------------------------------------------------------------
