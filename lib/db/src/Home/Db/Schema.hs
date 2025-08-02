--------------------------------------------------------------------------------

-- | This module defines an overall `Migration` for the entire database schema.
module Home.Db.Schema ( migrateAll ) where

--------------------------------------------------------------------------------

import Database.Persist.Sql ( Migration, runSqlCommand, rawExecute )
import Database.Persist.TH ( migrateModels )

import Home.Db.Finance ( financeModels )

--------------------------------------------------------------------------------

-- | `migrateAll` is a `Migration` for all database models.
migrateAll :: Migration
migrateAll = do
    -- ensure that the UUID extension is enabled
    runSqlCommand $
        rawExecute "CREATE EXTENSION IF NOT EXISTS \"uuid-ossp\"" []

    -- Run the migrations for the tables.
    migrateModels $ concat
        [ financeModels
        ]

--------------------------------------------------------------------------------
