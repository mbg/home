--------------------------------------------------------------------------------

-- | Defines `Merchant`, which represents known merchants that might be
-- associated with transactions.
module Home.Db.Finance.Merchant (
    Merchant(..),
    merchantModel
) where

--------------------------------------------------------------------------------

import Home.Db.Model

--------------------------------------------------------------------------------

dbModel "merchantModel" $(discoverEntities) [persistLowerCase|
Merchant sql=merchant
    Id
    name Text
    deriving Eq Show
|]

--------------------------------------------------------------------------------
