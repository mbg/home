--------------------------------------------------------------------------------

-- | This module provides the foundation for all database models. It defines
-- `dbModel` which can be used in a top-level expression to generate models
-- using our custom settings from persistent entity definitions.
module Home.Db.Model (
    module Database.Persist.TH,
    module Home.Db.Types,
    dbModel,
    PersistEntity(..)
) where

--------------------------------------------------------------------------------

import Database.Persist.Class
import Database.Persist.Quasi.Internal ( UnboundEntityDef )
import Database.Persist.Sql
import Database.Persist.TH

import Language.Haskell.TH (Dec, Q)

import Home.Db.Types

--------------------------------------------------------------------------------

-- | `customSqlSettings` represents the default `MkPersistSettings` we use.
-- Specifically, we use UUIDs as primary keys and this configures persistent
-- to add those automatically unless we specify something else for a given
-- database table.
customSqlSettings :: MkPersistSettings
customSqlSettings =
    setImplicitIdDef uuidDef sqlSettings

-- | `dbModel` @name inScopeEntities entityDefs@ applies default modifiers
-- to all elements of @entityDefs@. The @name@ is used to generate a Haskell
-- definition for the resulting list of entities that can be consumed elsewhere
-- later. @inScopeEntities@ should be a list of other `EntityDef` values which
-- are in scope, for use in foreign key resolution. This should normally be the
-- result of @$(discoverEntities)@.
dbModel :: String -> [EntityDef] -> [UnboundEntityDef] -> Q [Dec]
dbModel name inScope = share
    [ mkPersistWith customSqlSettings inScope
    , mkEntityDefList name
    ]

--------------------------------------------------------------------------------
