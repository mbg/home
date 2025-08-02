--------------------------------------------------------------------------------

-- | Provides `financeModels`, which is a list of all finance-related database
-- models.
module Home.Db.Finance ( financeModels ) where

--------------------------------------------------------------------------------

import Database.Persist.Sql ( EntityDef )

import Home.Db.Finance.Merchant ( merchantModel )

--------------------------------------------------------------------------------

-- | `financeModels` is a list of all finance-related database models.
financeModels :: [EntityDef]
financeModels = concat
    [ merchantModel
    ]

--------------------------------------------------------------------------------
