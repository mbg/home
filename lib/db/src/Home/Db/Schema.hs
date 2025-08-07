--------------------------------------------------------------------------------

-- | This module defines an overall `Migration` for the entire database schema.
module Home.Db.Schema ( migrateAll ) where

--------------------------------------------------------------------------------

import Database.Persist.Sql ( Migration, runSqlCommand, rawExecute )
import Database.Persist.TH ( migrateModels )

import Home.Db.Index
import Home.Db.Finance ( financeModels )
import Home.Db.Finance.Merchant

--------------------------------------------------------------------------------

-- | `migrateAll` is a `Migration` for all database models.
migrateAll :: Migration
migrateAll = do
    -- ensure that the UUID extension is enabled
    runSqlCommand $
        rawExecute "CREATE EXTENSION IF NOT EXISTS \"uuid-ossp\"" []

    createIndex [indexField asc Nothing MerchantName]

    -- Run the migrations for the tables.
    migrateModels $ concat
        [ financeModels
        ]

--------------------------------------------------------------------------------
